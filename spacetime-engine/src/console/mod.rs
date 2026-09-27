//! Shared developer-console command and diagnostic transport.
//!
//! The console has one command registry/dispatcher and multiple frontends:
//! - the in-game egui overlay;
//! - native stdin.
//!
//! Command results become structured console records consumed by both the
//! overlay and terminal sinks. Bevy/tracing diagnostics remain tracing events;
//! the overlay observes them through an additional [`bevy::log::LogPlugin`]
//! layer while Bevy's normal formatter continues to own terminal log output.
//!
//! Raw process stdout/stderr are deliberately not intercepted. Engine code that
//! should participate in the shared diagnostic stream should use tracing.

use std::{
    collections::{BTreeMap, VecDeque},
    fmt,
    sync::{Arc, Mutex, MutexGuard},
    time::{SystemTime, UNIX_EPOCH},
};

#[cfg(not(target_arch = "wasm32"))]
use std::{
    io::{BufRead, IsTerminal, Write},
    thread,
};

use bevy::{
    log::{
        BoxedLayer,
        tracing::{
            Event as TracingEvent, Level as TracingLevel,
            field::{Field, Visit},
        },
        tracing_subscriber::{
            Layer as TracingLayer,
            layer::Context as TracingContext,
            registry::Registry as TracingRegistry,
        },
    },
    prelude::*,
};
use bevy_egui::{EguiContext, EguiPrimaryContextPass, PrimaryEguiContext, egui};

use crate::input_focus::{InputFocus, InputFocusSet};

const CONSOLE_FOCUS_OWNER: &str = "developer_console";
const MAX_SCROLLBACK: usize = 512;
const MAX_HISTORY: usize = 128;
const MAX_PENDING_RECORDS: usize = 4_096;

pub type ConsoleCommandHandler =
    fn(&mut World, &ConsoleCommandInvocation) -> ConsoleCommandResult;

#[derive(Debug, Clone, Copy)]
pub struct ConsoleCommandSpec {
    pub name: &'static str,
    pub aliases: &'static [&'static str],
    pub usage: &'static str,
    pub summary: &'static str,
}

#[derive(Debug, Clone, Copy)]
struct RegisteredConsoleCommand {
    spec: ConsoleCommandSpec,
    handler: ConsoleCommandHandler,
}

#[derive(Resource, Default)]
pub struct ConsoleCommandRegistry {
    commands: BTreeMap<String, RegisteredConsoleCommand>,
    aliases: BTreeMap<String, String>,
}

impl ConsoleCommandRegistry {
    pub fn register(&mut self, spec: ConsoleCommandSpec, handler: ConsoleCommandHandler) {
        let canonical = normalize_name(spec.name);
        assert!(!canonical.is_empty(), "console command names must not be empty");
        assert!(
            !self.commands.contains_key(&canonical),
            "duplicate console command `{canonical}`"
        );

        for &alias in spec.aliases {
            let alias = normalize_name(alias);
            assert!(
                !alias.is_empty() && !self.aliases.contains_key(&alias),
                "duplicate/empty console command alias `{alias}`"
            );
            self.aliases.insert(alias, canonical.clone());
        }

        self.commands
            .insert(canonical, RegisteredConsoleCommand { spec, handler });
    }

    fn resolve(&self, name: &str) -> Option<RegisteredConsoleCommand> {
        let name = normalize_name(name);
        if let Some(command) = self.commands.get(&name) {
            return Some(*command);
        }
        let canonical = self.aliases.get(&name)?;
        self.commands.get(canonical).copied()
    }

    fn command_names(&self) -> impl Iterator<Item = &str> {
        self.commands.keys().map(String::as_str)
    }

    fn completions(&self, prefix: &str) -> Vec<String> {
        let prefix = normalize_name(prefix);
        self.command_names()
            .filter(|name| name.starts_with(&prefix))
            .map(ToOwned::to_owned)
            .collect()
    }
}

pub trait AppConsoleExt {
    fn register_console_command(
        &mut self,
        spec: ConsoleCommandSpec,
        handler: ConsoleCommandHandler,
    ) -> &mut Self;
}

impl AppConsoleExt for App {
    fn register_console_command(
        &mut self,
        spec: ConsoleCommandSpec,
        handler: ConsoleCommandHandler,
    ) -> &mut Self {
        self.init_resource::<ConsoleCommandRegistry>();
        self.world_mut()
            .resource_mut::<ConsoleCommandRegistry>()
            .register(spec, handler);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleCommandSource {
    Overlay,
    Terminal,
}

impl ConsoleCommandSource {
    fn label(self) -> &'static str {
        match self {
            Self::Overlay => "overlay",
            Self::Terminal => "stdin",
        }
    }
}

#[derive(Debug, Clone)]
struct ConsoleCommandSubmission {
    source: ConsoleCommandSource,
    raw: String,
}

#[derive(Debug, Clone)]
pub struct ConsoleCommandInvocation {
    raw: String,
    name: String,
    args: Vec<String>,
    source: ConsoleCommandSource,
}

impl ConsoleCommandInvocation {
    pub fn raw(&self) -> &str {
        &self.raw
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn args(&self) -> &[String] {
        &self.args
    }

    pub const fn source(&self) -> ConsoleCommandSource {
        self.source
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConsoleFocusDisposition {
    KeepConsole,
    ReturnToGameplay,
}

#[derive(Debug)]
pub enum ConsoleCommandResult {
    Silent,
    Success {
        lines: Vec<String>,
        focus: ConsoleFocusDisposition,
    },
    Error(String),
    Clear,
}

impl ConsoleCommandResult {
    pub fn success(line: impl Into<String>) -> Self {
        Self::Success {
            lines: vec![line.into()],
            focus: ConsoleFocusDisposition::KeepConsole,
        }
    }

    pub fn success_and_return_to_gameplay(line: impl Into<String>) -> Self {
        Self::Success {
            lines: vec![line.into()],
            focus: ConsoleFocusDisposition::ReturnToGameplay,
        }
    }

    pub fn lines(lines: impl IntoIterator<Item = impl Into<String>>) -> Self {
        Self::Success {
            lines: lines.into_iter().map(Into::into).collect(),
            focus: ConsoleFocusDisposition::KeepConsole,
        }
    }

    pub fn error(message: impl Into<String>) -> Self {
        Self::Error(message.into())
    }

    pub const fn clear() -> Self {
        Self::Clear
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConsoleRecordOrigin {
    Command,
    Tracing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConsoleLogLevel {
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

    const fn label(self) -> &'static str {
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
enum ConsoleRecordKind {
    Command,
    Output,
    Error,
    Trace(ConsoleLogLevel),
    Clear,
}

#[derive(Debug, Clone)]
struct ConsoleRecord {
    origin: ConsoleRecordOrigin,
    kind: ConsoleRecordKind,
    timestamp_millis: u64,
    source: Option<ConsoleCommandSource>,
    target: Option<String>,
    text: String,
    fields: Vec<(String, String)>,
}

impl ConsoleRecord {
    fn command(source: ConsoleCommandSource, raw: &str) -> Self {
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

    fn output(text: impl Into<String>) -> Self {
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

    fn error(text: impl Into<String>) -> Self {
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

    fn clear() -> Self {
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
            duration
                .as_millis()
                .min(u128::from(u64::MAX)) as u64
        })
}

fn timestamp_label(timestamp_millis: u64) -> String {
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
struct ConsoleTransport {
    commands: Arc<Mutex<VecDeque<ConsoleCommandSubmission>>>,
    records: Arc<Mutex<VecDeque<ConsoleRecord>>>,
}

impl ConsoleTransport {
    fn submit(&self, source: ConsoleCommandSource, raw: impl Into<String>) {
        let raw = raw.into();
        if raw.trim().is_empty() {
            return;
        }
        lock_recover(&self.commands).push_back(ConsoleCommandSubmission { source, raw });
    }

    fn drain_commands(&self) -> Vec<ConsoleCommandSubmission> {
        lock_recover(&self.commands).drain(..).collect()
    }

    fn publish(&self, record: ConsoleRecord) {
        let mut records = lock_recover(&self.records);
        while records.len() >= MAX_PENDING_RECORDS {
            records.pop_front();
        }
        records.push_back(record);
    }

    fn drain_records(&self) -> Vec<ConsoleRecord> {
        lock_recover(&self.records).drain(..).collect()
    }
}

fn lock_recover<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
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
    fn on_event(
        &self,
        event: &TracingEvent<'_>,
        _context: TracingContext<'_, TracingRegistry>,
    ) {
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
    Some(Box::new(ConsoleTracingLayer { transport }))
}

#[derive(Resource)]
struct ConsoleOverlay {
    open: bool,
    opened_this_frame: bool,
    input: String,
    history: Vec<String>,
    history_cursor: Option<usize>,
    scrollback: VecDeque<ConsoleRecord>,
}

impl Default for ConsoleOverlay {
    fn default() -> Self {
        let mut scrollback = VecDeque::new();
        scrollback.push_back(ConsoleRecord::output(
            "Spacetime Engine developer console — overlay + stdin + Bevy tracing.",
        ));
        Self {
            open: false,
            opened_this_frame: false,
            input: String::new(),
            history: Vec::new(),
            history_cursor: None,
            scrollback,
        }
    }
}

impl ConsoleOverlay {
    fn push(&mut self, record: ConsoleRecord) {
        self.scrollback.push_back(record);
        while self.scrollback.len() > MAX_SCROLLBACK {
            self.scrollback.pop_front();
        }
    }

    fn submit(&mut self) -> Option<String> {
        let command = self.input.trim().to_string();
        self.input.clear();
        self.history_cursor = None;
        if command.is_empty() {
            return None;
        }

        if self.history.last() != Some(&command) {
            self.history.push(command.clone());
            if self.history.len() > MAX_HISTORY {
                self.history.remove(0);
            }
        }

        Some(command)
    }

    fn history_up(&mut self) {
        if self.history.is_empty() {
            return;
        }
        let next = match self.history_cursor {
            None => self.history.len() - 1,
            Some(0) => 0,
            Some(index) => index - 1,
        };
        self.history_cursor = Some(next);
        self.input.clone_from(&self.history[next]);
    }

    fn history_down(&mut self) {
        let Some(index) = self.history_cursor else {
            return;
        };
        if index + 1 >= self.history.len() {
            self.history_cursor = None;
            self.input.clear();
        } else {
            let next = index + 1;
            self.history_cursor = Some(next);
            self.input.clone_from(&self.history[next]);
        }
    }
}

pub struct DeveloperConsolePlugin;

impl Plugin for DeveloperConsolePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleTransport>()
            .init_resource::<ConsoleOverlay>()
            .init_resource::<ConsoleCommandRegistry>()
            .init_resource::<InputFocus>()
            .add_systems(PreUpdate, toggle_console.before(InputFocusSet::Resolve))
            .add_systems(Update, dispatch_console_commands)
            .add_systems(PostUpdate, flush_console_records)
            .add_systems(EguiPrimaryContextPass, draw_console);

        #[cfg(not(target_arch = "wasm32"))]
        app.add_systems(Startup, start_terminal_input);

        app.register_console_command(
            ConsoleCommandSpec {
                name: "help",
                aliases: &["?", "commands"],
                usage: "help [command]",
                summary: "List commands or show detailed help for one command.",
            },
            help_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "clear",
                aliases: &["cls"],
                usage: "clear",
                summary: "Clear console frontends.",
            },
            clear_command,
        )
        .register_console_command(
            ConsoleCommandSpec {
                name: "echo",
                aliases: &[],
                usage: "echo <text...>",
                summary: "Print text to all console frontends.",
            },
            echo_command,
        );
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn start_terminal_input(transport: Res<ConsoleTransport>) {
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

fn toggle_console(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut console: ResMut<ConsoleOverlay>,
    mut focus: ResMut<InputFocus>,
) {
    if keyboard.just_pressed(KeyCode::Backquote) {
        console.open = !console.open;
        console.opened_this_frame = console.open;
        console.history_cursor = None;
    } else if console.open && keyboard.just_pressed(KeyCode::Escape) {
        console.open = false;
        console.history_cursor = None;
    }

    focus.set_modal_claim(CONSOLE_FOCUS_OWNER, console.open);
}

fn dispatch_console_commands(world: &mut World) {
    let submissions = world.resource::<ConsoleTransport>().drain_commands();

    for submission in submissions {
        let transport = world.resource::<ConsoleTransport>().clone();
        transport.publish(ConsoleRecord::command(submission.source, &submission.raw));

        let invocation = match parse_command(&submission.raw, submission.source) {
            Ok(invocation) => invocation,
            Err(error) => {
                transport.publish(ConsoleRecord::error(error));
                continue;
            }
        };

        let command = {
            let registry = world.resource::<ConsoleCommandRegistry>();
            registry.resolve(invocation.name())
        };

        let Some(command) = command else {
            transport.publish(ConsoleRecord::error(format!(
                "unknown command `{}` — type `help` to list commands",
                invocation.name()
            )));
            continue;
        };

        match (command.handler)(world, &invocation) {
            ConsoleCommandResult::Silent => {}
            ConsoleCommandResult::Success { lines, focus } => {
                for line in lines {
                    transport.publish(ConsoleRecord::output(line));
                }
                apply_focus_disposition(world, invocation.source(), focus);
            }
            ConsoleCommandResult::Error(error) => {
                transport.publish(ConsoleRecord::error(error));
            }
            ConsoleCommandResult::Clear => {
                transport.publish(ConsoleRecord::clear());
            }
        }
    }
}

fn apply_focus_disposition(
    world: &mut World,
    source: ConsoleCommandSource,
    focus: ConsoleFocusDisposition,
) {
    if source != ConsoleCommandSource::Overlay
        || focus != ConsoleFocusDisposition::ReturnToGameplay
    {
        return;
    }

    {
        let mut console = world.resource_mut::<ConsoleOverlay>();
        console.open = false;
        console.history_cursor = None;
    }

    let mut input_focus = world.resource_mut::<InputFocus>();
    input_focus.set_modal_claim(CONSOLE_FOCUS_OWNER, false);
    input_focus.request_gameplay_resume();
}

fn flush_console_records(
    transport: Res<ConsoleTransport>,
    mut overlay: ResMut<ConsoleOverlay>,
) {
    for record in transport.drain_records() {
        if record.origin == ConsoleRecordOrigin::Command {
            write_terminal_record(&record);
        }

        if record.kind == ConsoleRecordKind::Clear {
            overlay.scrollback.clear();
        } else {
            overlay.push(record);
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn write_terminal_record(record: &ConsoleRecord) {
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
            let source = record
                .source
                .map_or("command", ConsoleCommandSource::label);
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
fn write_terminal_record(_: &ConsoleRecord) {}

fn console_record_layout(record: &ConsoleRecord) -> egui::text::LayoutJob {
    let mut job = egui::text::LayoutJob::default();
    let font = egui::FontId::new(12.5, egui::FontFamily::Monospace);

    let timestamp = egui::Color32::from_rgb(105, 112, 120);
    let separator = egui::Color32::from_rgb(92, 100, 108);
    let target = egui::Color32::from_rgb(100, 184, 214);
    let message = egui::Color32::from_rgb(218, 222, 226);
    let field_key = egui::Color32::from_rgb(192, 146, 224);
    let field_value = egui::Color32::from_rgb(218, 188, 116);
    let command = egui::Color32::from_rgb(96, 190, 220);
    let output = egui::Color32::from_rgb(202, 208, 214);
    let error = egui::Color32::from_rgb(248, 105, 96);

    let mut append = |text: &str, color: egui::Color32| {
        job.append(
            text,
            0.0,
            egui::text::TextFormat {
                font_id: font.clone(),
                color,
                ..default()
            },
        );
    };

    append(&timestamp_label(record.timestamp_millis), timestamp);
    append("  ", separator);

    match record.kind {
        ConsoleRecordKind::Trace(level) => {
            append(&format!("{:<5}", level.label()), level_color(level));
            append("  ", separator);
            append(record.target.as_deref().unwrap_or("<unknown>"), target);
            append("  ", separator);
            append(&record.text, message);

            for (key, value) in &record.fields {
                append("  ", separator);
                append(key, field_key);
                append("=", separator);
                append(value, field_value);
            }
        }
        ConsoleRecordKind::Command => {
            append("CMD  ", command);
            append(
                &format!("[{}]", record.source.map_or("?", ConsoleCommandSource::label)),
                separator,
            );
            append("  › ", command);
            append(&record.text, message);
        }
        ConsoleRecordKind::Output => {
            append("OUT  ", separator);
            append(&record.text, output);
        }
        ConsoleRecordKind::Error => {
            append("ERR  ", error);
            append(&record.text, error);
        }
        ConsoleRecordKind::Clear => {}
    }

    job
}

fn level_color(level: ConsoleLogLevel) -> egui::Color32 {
    match level {
        ConsoleLogLevel::Trace => egui::Color32::from_rgb(172, 132, 205),
        ConsoleLogLevel::Debug => egui::Color32::from_rgb(104, 158, 220),
        ConsoleLogLevel::Info => egui::Color32::from_rgb(108, 190, 126),
        ConsoleLogLevel::Warn => egui::Color32::from_rgb(236, 185, 82),
        ConsoleLogLevel::Error => egui::Color32::from_rgb(248, 105, 96),
    }
}

fn draw_console(
    mut contexts: Query<&mut EguiContext, With<PrimaryEguiContext>>,
    mut console: ResMut<ConsoleOverlay>,
    registry: Res<ConsoleCommandRegistry>,
    transport: Res<ConsoleTransport>,
) {
    if !console.open {
        return;
    }

    let Ok(mut context) = contexts.single_mut() else {
        return;
    };
    let ctx = context.get_mut();
    let content_rect = ctx.input(|input| input.content_rect());
    let console_height = (content_rect.height() * 0.46).clamp(220.0, 560.0);
    let console_width = content_rect.width();

    egui::Area::new(egui::Id::new("spacetime_developer_console"))
        .order(egui::Order::Foreground)
        .fixed_pos(content_rect.left_top())
        .default_size(egui::vec2(console_width, console_height))
        .show(ctx, |ui| {
            ui.set_width(console_width);
            ui.set_height(console_height);

            egui::Frame::new()
                .fill(egui::Color32::from_rgba_unmultiplied(10, 12, 14, 248))
                .stroke(egui::Stroke::new(
                    1.0_f32,
                    egui::Color32::from_rgb(72, 82, 92),
                ))
                .show(ui, |ui| {
                    ui.set_width(console_width);
                    ui.set_height(console_height);

                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new("SPACETIME CONSOLE")
                                .monospace()
                                .strong()
                                .color(egui::Color32::from_rgb(220, 224, 228)),
                        );
                        ui.separator();
                        ui.label(
                            egui::RichText::new(
                                "` toggle   ↑/↓ history   Tab complete   Ctrl+C copy   stdin + Bevy logs mirrored",
                            )
                            .monospace()
                            .small()
                            .color(egui::Color32::from_rgb(130, 140, 150)),
                        );
                    });
                    ui.separator();

                    egui::ScrollArea::vertical()
                        .stick_to_bottom(true)
                        .auto_shrink([false, false])
                        .max_height((console_height - 62.0).max(80.0))
                        .show(ui, |ui| {
                            for record in &console.scrollback {
                                ui.add(
                                    egui::Label::new(console_record_layout(record))
                                        .selectable(true),
                                );
                            }
                        });

                    ui.separator();

                    let response = ui.add(
                        egui::TextEdit::singleline(&mut console.input)
                            .font(egui::TextStyle::Monospace)
                            .desired_width(f32::INFINITY)
                            .hint_text("command"),
                    );

                    if console.opened_this_frame {
                        console.input.clear();
                        response.request_focus();
                        console.opened_this_frame = false;
                    }

                    let prompt_active = response.has_focus() || response.lost_focus();
                    if prompt_active {
                        let history_up =
                            ui.input(|input| input.key_pressed(egui::Key::ArrowUp));
                        let history_down =
                            ui.input(|input| input.key_pressed(egui::Key::ArrowDown));
                        let complete =
                            ui.input(|input| input.key_pressed(egui::Key::Tab));
                        let submit =
                            ui.input(|input| input.key_pressed(egui::Key::Enter));

                        if history_up {
                            console.history_up();
                            response.request_focus();
                        }
                        if history_down {
                            console.history_down();
                            response.request_focus();
                        }
                        if complete {
                            complete_command_input(&mut console.input, &registry);
                            response.request_focus();
                        }
                        if submit {
                            if let Some(command) = console.submit() {
                                transport.submit(ConsoleCommandSource::Overlay, command);
                            }
                            response.request_focus();
                        }
                    }

                    let prefix = command_prefix(&console.input);
                    let completions = registry.completions(prefix);
                    if !prefix.is_empty() && !completions.is_empty() {
                        let preview = completions
                            .into_iter()
                            .take(8)
                            .collect::<Vec<_>>()
                            .join("  ");
                        ui.label(
                            egui::RichText::new(preview)
                                .monospace()
                                .small()
                                .color(egui::Color32::from_rgb(108, 136, 160)),
                        );
                    }
                });
        });
}

fn complete_command_input(input: &mut String, registry: &ConsoleCommandRegistry) {
    let prefix = command_prefix(input);
    if prefix.is_empty() {
        return;
    }

    let completions = registry.completions(prefix);
    if completions.is_empty() {
        return;
    }

    let completed = if completions.len() == 1 {
        completions[0].clone()
    } else {
        common_prefix(&completions)
    };
    if completed.len() <= prefix.len() {
        return;
    }

    let slash = input.trim_start().starts_with('/');
    *input = format!("{}{} ", if slash { "/" } else { "" }, completed);
}

fn command_prefix(input: &str) -> &str {
    input
        .trim_start()
        .strip_prefix('/')
        .unwrap_or_else(|| input.trim_start())
        .split_whitespace()
        .next()
        .unwrap_or("")
}

fn common_prefix(values: &[String]) -> String {
    let Some(first) = values.first() else {
        return String::new();
    };
    let mut length = first.len();
    for value in &values[1..] {
        length = first
            .bytes()
            .zip(value.bytes())
            .take_while(|(left, right)| left == right)
            .count()
            .min(length);
    }
    first[..length].to_string()
}

fn help_command(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    let registry = world.resource::<ConsoleCommandRegistry>();

    if let Some(name) = invocation.args().first() {
        let Some(command) = registry.resolve(name) else {
            return ConsoleCommandResult::error(format!("unknown command `{name}`"));
        };
        let mut lines = vec![format!(
            "{} — {}",
            command.spec.usage, command.spec.summary
        )];
        if !command.spec.aliases.is_empty() {
            lines.push(format!("aliases: {}", command.spec.aliases.join(", ")));
        }
        return ConsoleCommandResult::lines(lines);
    }

    ConsoleCommandResult::lines(registry.commands.values().map(|command| {
        format!("{:<34} {}", command.spec.usage, command.spec.summary)
    }))
}

fn clear_command(_: &mut World, _: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    ConsoleCommandResult::clear()
}

fn echo_command(_: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    ConsoleCommandResult::success(invocation.args().join(" "))
}

fn parse_command(
    raw: &str,
    source: ConsoleCommandSource,
) -> Result<ConsoleCommandInvocation, String> {
    let source_text = raw.trim();
    let source_text = source_text
        .strip_prefix('/')
        .unwrap_or(source_text)
        .trim();
    if source_text.is_empty() {
        return Err("empty command".to_string());
    }

    let tokens = tokenize(source_text)?;
    let Some((name, args)) = tokens.split_first() else {
        return Err("empty command".to_string());
    };

    Ok(ConsoleCommandInvocation {
        raw: raw.to_string(),
        name: normalize_name(name),
        args: args.to_vec(),
        source,
    })
}

fn tokenize(source: &str) -> Result<Vec<String>, String> {
    let mut tokens = Vec::new();
    let mut current = String::new();
    let mut quote = None;
    let mut escaped = false;

    for character in source.chars() {
        if escaped {
            current.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' {
            escaped = true;
            continue;
        }
        if let Some(active_quote) = quote {
            if character == active_quote {
                quote = None;
            } else {
                current.push(character);
            }
            continue;
        }

        match character {
            '"' | '\'' => quote = Some(character),
            character if character.is_whitespace() => {
                if !current.is_empty() {
                    tokens.push(std::mem::take(&mut current));
                }
            }
            _ => current.push(character),
        }
    }

    if escaped {
        current.push('\\');
    }
    if quote.is_some() {
        return Err("unterminated quoted argument".to_string());
    }
    if !current.is_empty() {
        tokens.push(current);
    }
    Ok(tokens)
}

fn normalize_name(name: &str) -> String {
    name.trim().trim_start_matches('/').to_ascii_lowercase()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_supports_slash_quotes_escapes_and_source() {
        let parsed = parse_command(
            r#"/echo "hello universe" moon\ base"#,
            ConsoleCommandSource::Terminal,
        )
        .unwrap();

        assert_eq!(parsed.name(), "echo");
        assert_eq!(
            parsed.args(),
            &["hello universe".to_string(), "moon base".to_string()]
        );
        assert_eq!(parsed.source(), ConsoleCommandSource::Terminal);
    }

    #[test]
    fn shared_transport_preserves_command_frontend() {
        let transport = ConsoleTransport::default();
        transport.submit(ConsoleCommandSource::Overlay, "echo overlay");
        transport.submit(ConsoleCommandSource::Terminal, "echo terminal");

        let commands = transport.drain_commands();
        assert_eq!(commands.len(), 2);
        assert_eq!(commands[0].source, ConsoleCommandSource::Overlay);
        assert_eq!(commands[1].source, ConsoleCommandSource::Terminal);
        assert!(transport.drain_commands().is_empty());
    }

    #[test]
    fn command_output_and_tracing_share_records_without_losing_origin() {
        let transport = ConsoleTransport::default();
        transport.publish(ConsoleRecord::output("command result"));
        transport.publish(ConsoleRecord::tracing(
            ConsoleLogLevel::Info,
            "test_target",
            "trace event".to_string(),
            vec![("answer".to_string(), "42".to_string())],
        ));

        let records = transport.drain_records();
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].origin, ConsoleRecordOrigin::Command);
        assert_eq!(records[1].origin, ConsoleRecordOrigin::Tracing);
        assert_eq!(
            records[1].fields,
            vec![("answer".to_string(), "42".to_string())]
        );
    }

    #[test]
    fn timestamp_label_matches_tracing_style_clock_width() {
        assert_eq!(timestamp_label(0), "00:00:00.000");
        assert_eq!(timestamp_label(86_399_999), "23:59:59.999");
        assert_eq!(timestamp_label(86_400_000), "00:00:00.000");
    }
}
