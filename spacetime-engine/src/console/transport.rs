//! Structured diagnostic transport and native terminal frontend.

use super::ConsoleCommandSource;
use bevy::{
    log::{
        BoxedLayer,
        tracing::{
            Event as TracingEvent, Level as TracingLevel,
            field::{Field, Visit},
        },
        tracing_subscriber::{
            Layer as TracingLayer, filter::FilterFn, layer::Context as TracingContext,
            registry::Registry as TracingRegistry,
        },
    },
    prelude::*,
};
use std::{
    collections::VecDeque,
    fmt,
    sync::{Arc, Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};
#[cfg(not(target_arch = "wasm32"))]
use std::{
    io::{BufRead, IsTerminal, Write},
    thread,
};
const MAX_PENDING_RECORDS: usize = 4_096;

#[derive(Debug, Clone)]
pub(super) struct ConsoleCommandSubmission {
    pub(super) source: ConsoleCommandSource,
    pub(super) raw: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConsoleRecordOrigin {
    Command,
    Tracing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConsoleLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl ConsoleLogLevel {
    fn from_tracing(level: &TracingLevel) -> Self {
        if *level == TracingLevel::ERROR {
            Self::Error
        } else if *level == TracingLevel::WARN {
            Self::Warn
        } else if *level == TracingLevel::INFO {
            Self::Info
        } else if *level == TracingLevel::DEBUG {
            Self::Debug
        } else {
            Self::Trace
        }
    }

    pub(super) const fn label(self) -> &'static str {
        match self {
            Self::Trace => "TRACE",
            Self::Debug => "DEBUG",
            Self::Info => "INFO",
            Self::Warn => "WARN",
            Self::Error => "ERROR",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum ConsoleRecordKind {
    Command,
    Output,
    Error,
    Trace(ConsoleLogLevel),
    Clear,
}

#[derive(Debug, Clone)]
pub(super) struct ConsoleRecord {
    pub(super) origin: ConsoleRecordOrigin,
    pub(super) kind: ConsoleRecordKind,
    pub(super) timestamp_millis: u64,
    pub(super) source: Option<ConsoleCommandSource>,
    pub(super) target: Option<String>,
    pub(super) text: String,
    pub(super) fields: Vec<(String, String)>,
}

impl ConsoleRecord {
    pub(super) fn command(source: ConsoleCommandSource, raw: &str) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Command,
            timestamp_millis: capture_timestamp_millis(),
            source: Some(source),
            target: None,
            text: raw.to_string(),
            fields: Vec::new(),
        }
    }

    pub(super) fn output(text: impl Into<String>) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Output,
            timestamp_millis: capture_timestamp_millis(),
            source: None,
            target: None,
            text: text.into(),
            fields: Vec::new(),
        }
    }

    pub(super) fn error(text: impl Into<String>) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Error,
            timestamp_millis: capture_timestamp_millis(),
            source: None,
            target: None,
            text: text.into(),
            fields: Vec::new(),
        }
    }

    pub(super) fn clear() -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Clear,
            timestamp_millis: capture_timestamp_millis(),
            source: None,
            target: None,
            text: String::new(),
            fields: Vec::new(),
        }
    }

    fn tracing(
        level: ConsoleLogLevel,
        target: &str,
        text: String,
        fields: Vec<(String, String)>,
    ) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Tracing,
            kind: ConsoleRecordKind::Trace(level),
            timestamp_millis: capture_timestamp_millis(),
            source: None,
            target: Some(target.to_string()),
            text,
            fields,
        }
    }
}

fn capture_timestamp_millis() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| {
            duration.as_millis().min(u128::from(u64::MAX)) as u64
        })
}

pub(super) fn timestamp_label(timestamp_millis: u64) -> String {
    let day_millis = timestamp_millis % 86_400_000;
    let hours = day_millis / 3_600_000;
    let minutes = (day_millis / 60_000) % 60;
    let seconds = (day_millis / 1_000) % 60;
    let millis = day_millis % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
}

/// Thread-safe transport shared by tracing, stdin, ECS dispatch and the overlay.
///
/// The queues contain no ECS values and never borrow the [`World`].
#[derive(Resource, Clone, Default)]
pub(crate) struct ConsoleTransport {
    commands: Arc<Mutex<VecDeque<ConsoleCommandSubmission>>>,
    records: Arc<Mutex<VecDeque<ConsoleRecord>>>,
}

impl ConsoleTransport {
    pub(crate) fn submit(&self, source: ConsoleCommandSource, raw: impl Into<String>) {
        let raw = raw.into();
        if raw.trim().is_empty() {
            return;
        }
        lock_recover(&self.commands).push_back(ConsoleCommandSubmission { source, raw });
    }

    pub(super) fn drain_commands(&self) -> Vec<ConsoleCommandSubmission> {
        lock_recover(&self.commands).drain(..).collect()
    }

    pub(super) fn publish(&self, record: ConsoleRecord) {
        let mut records = lock_recover(&self.records);
        while records.len() >= MAX_PENDING_RECORDS {
            records.pop_front();
        }
        records.push_back(record);
    }

    pub(super) fn drain_records(&self) -> Vec<ConsoleRecord> {
        lock_recover(&self.records).drain(..).collect()
    }
}

fn lock_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

#[derive(Default)]
struct TracingFields {
    message: Option<String>,
    fields: Vec<(String, String)>,
}

impl TracingFields {
    fn record_value(&mut self, field: &Field, value: impl fmt::Display) {
        let value = value.to_string();
        if field.name() == "message" {
            self.message = Some(value);
        } else {
            self.fields.push((field.name().to_string(), value));
        }
    }

    fn finish(self, fallback: &str) -> (String, Vec<(String, String)>) {
        (
            self.message.unwrap_or_else(|| fallback.to_string()),
            self.fields,
        )
    }
}

impl Visit for TracingFields {
    fn record_debug(&mut self, field: &Field, value: &dyn fmt::Debug) {
        self.record_value(field, format_args!("{value:?}"));
    }

    fn record_str(&mut self, field: &Field, value: &str) {
        self.record_value(field, value);
    }

    fn record_bool(&mut self, field: &Field, value: bool) {
        self.record_value(field, value);
    }

    fn record_i64(&mut self, field: &Field, value: i64) {
        self.record_value(field, value);
    }

    fn record_u64(&mut self, field: &Field, value: u64) {
        self.record_value(field, value);
    }

    fn record_f64(&mut self, field: &Field, value: f64) {
        self.record_value(field, value);
    }
}

struct ConsoleTracingLayer {
    transport: ConsoleTransport,
}

impl TracingLayer<TracingRegistry> for ConsoleTracingLayer {
    fn on_event(&self, event: &TracingEvent<'_>, _context: TracingContext<'_, TracingRegistry>) {
        let metadata = event.metadata();
        let mut fields = TracingFields::default();
        event.record(&mut fields);

        let (message, fields) = fields.finish(metadata.name());
        self.transport.publish(ConsoleRecord::tracing(
            ConsoleLogLevel::from_tracing(metadata.level()),
            metadata.target(),
            message,
            fields,
        ));
    }
}

/// Extra Bevy tracing layer used only by the in-game overlay.
///
/// Bevy's normal formatted terminal layer remains installed and authoritative.
pub(crate) fn console_log_layer(app: &mut App) -> Option<BoxedLayer> {
    app.init_resource::<ConsoleTransport>();
    let transport = app.world().resource::<ConsoleTransport>().clone();

    // Match Bevy's own formatted-log policy: Tracy's per-frame marker is
    // instrumentation metadata, not a human-facing log record. Filtering at the
    // layer boundary prevents it from reaching on_event at all.
    let filter = FilterFn::new(|metadata| metadata.fields().field("tracy.frame_mark").is_none());
    Some(Box::new(
        ConsoleTracingLayer { transport }.with_filter(filter),
    ))
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn start_terminal_input(transport: Res<ConsoleTransport>) {
    let transport = transport.clone();

    if let Err(error) = thread::Builder::new()
        .name("spacetime-console-stdin".to_string())
        .spawn(move || {
            let stdin = std::io::stdin();
            for line in stdin.lock().lines() {
                match line {
                    Ok(line) => transport.submit(ConsoleCommandSource::Terminal, line),
                    Err(error) => {
                        bevy::log::error!(
                            target: "developer_console",
                            error = %error,
                            "stdin console frontend stopped"
                        );
                        break;
                    }
                }
            }
        })
    {
        bevy::log::error!(
            target: "developer_console",
            error = %error,
            "failed to spawn stdin console frontend"
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
pub(super) fn write_terminal_record(record: &ConsoleRecord) {
    match record.kind {
        ConsoleRecordKind::Clear => {
            let stdout = std::io::stdout();
            if stdout.is_terminal() {
                let mut stdout = stdout.lock();
                let _ = write!(stdout, "\x1b[2J\x1b[H");
                let _ = stdout.flush();
            }
        }
        ConsoleRecordKind::Error => {
            let stderr = std::io::stderr();
            let mut stderr = stderr.lock();
            let _ = writeln!(stderr, "{}", record.text);
        }
        ConsoleRecordKind::Command => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let source = record.source.map_or("command", ConsoleCommandSource::label);
            let _ = writeln!(stdout, "[{source}] {}", record.text);
        }
        ConsoleRecordKind::Output => {
            let stdout = std::io::stdout();
            let mut stdout = stdout.lock();
            let _ = writeln!(stdout, "{}", record.text);
        }
        ConsoleRecordKind::Trace(_) => {
            // Bevy's normal tracing formatter already owns terminal log output.
        }
    }
}

#[cfg(target_arch = "wasm32")]
pub(super) fn write_terminal_record(_: &ConsoleRecord) {}
