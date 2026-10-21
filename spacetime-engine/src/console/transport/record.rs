//! Structured records shared by overlay, tracing and terminal frontends.

use crate::console::ConsoleCommandSource;
use bevy::log::tracing::Level as TracingLevel;
use std::time::{SystemTime, UNIX_EPOCH};
use vapor_telemetry::{ConsoleField, ConsoleLevel, ConsoleRecord as SharedConsoleRecord};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::console) enum ConsoleRecordOrigin {
    Command,
    Tracing,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::console) enum ConsoleLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl ConsoleLogLevel {
    pub(super) fn from_tracing(level: &TracingLevel) -> Self {
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

}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::console) enum ConsoleRecordKind {
    Command,
    Output,
    Error,
    Trace(ConsoleLogLevel),
    Clear,
}

#[derive(Debug, Clone)]
pub(in crate::console) struct ConsoleRecord {
    pub(in crate::console) origin: ConsoleRecordOrigin,
    pub(in crate::console) kind: ConsoleRecordKind,
    pub(in crate::console) source: Option<ConsoleCommandSource>,
    pub(in crate::console) semantic: SharedConsoleRecord,
}

impl ConsoleRecord {
    pub(in crate::console) fn command(source: ConsoleCommandSource, raw: &str) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Command,
            source: Some(source),
            semantic: SharedConsoleRecord::new(
                capture_timestamp_millis(),
                ConsoleLevel::Input,
                "console",
                raw,
            )
            .target(source.label()),
        }
    }

    pub(in crate::console) fn output(text: impl Into<String>) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Output,
            source: None,
            semantic: SharedConsoleRecord::new(
                capture_timestamp_millis(),
                ConsoleLevel::Info,
                "console",
                text,
            ),
        }
    }

    pub(in crate::console) fn error(text: impl Into<String>) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Error,
            source: None,
            semantic: SharedConsoleRecord::new(
                capture_timestamp_millis(),
                ConsoleLevel::Error,
                "console",
                text,
            ),
        }
    }

    pub(in crate::console) fn clear() -> Self {
        Self {
            origin: ConsoleRecordOrigin::Command,
            kind: ConsoleRecordKind::Clear,
            source: None,
            semantic: SharedConsoleRecord::new(
                capture_timestamp_millis(),
                ConsoleLevel::System,
                "console",
                "",
            ),
        }
    }

    pub(super) fn tracing(
        level: ConsoleLogLevel,
        target: &str,
        text: String,
        fields: Vec<(String, String)>,
    ) -> Self {
        Self {
            origin: ConsoleRecordOrigin::Tracing,
            kind: ConsoleRecordKind::Trace(level),
            source: None,
            semantic: SharedConsoleRecord {
                timestamp_unix_ms: capture_timestamp_millis(),
                level: match level {
                    ConsoleLogLevel::Trace => ConsoleLevel::Trace,
                    ConsoleLogLevel::Debug => ConsoleLevel::Debug,
                    ConsoleLogLevel::Info => ConsoleLevel::Info,
                    ConsoleLogLevel::Warn => ConsoleLevel::Warn,
                    ConsoleLogLevel::Error => ConsoleLevel::Error,
                },
                source: vapor_telemetry::ConsoleSource("tracing".to_owned()),
                target: Some(target.to_owned()),
                message: text,
                fields: fields
                    .into_iter()
                    .map(|(key, value)| ConsoleField { key, value })
                    .collect(),
            },
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

pub(in crate::console) fn timestamp_label(timestamp_millis: u64) -> String {
    let day_millis = timestamp_millis % 86_400_000;
    let hours = day_millis / 3_600_000;
    let minutes = (day_millis / 60_000) % 60;
    let seconds = (day_millis / 1_000) % 60;
    let millis = day_millis % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
}
