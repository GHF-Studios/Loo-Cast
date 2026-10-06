//! Structured records shared by overlay, tracing and terminal frontends.

use crate::console::ConsoleCommandSource;
use bevy::log::tracing::Level as TracingLevel;
use std::time::{SystemTime, UNIX_EPOCH};

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

    pub(in crate::console) const fn label(self) -> &'static str {
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
    pub(in crate::console) timestamp_millis: u64,
    pub(in crate::console) source: Option<ConsoleCommandSource>,
    pub(in crate::console) target: Option<String>,
    pub(in crate::console) text: String,
    pub(in crate::console) fields: Vec<(String, String)>,
}

impl ConsoleRecord {
    pub(in crate::console) fn command(source: ConsoleCommandSource, raw: &str) -> Self {
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

    pub(in crate::console) fn output(text: impl Into<String>) -> Self {
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

    pub(in crate::console) fn error(text: impl Into<String>) -> Self {
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

    pub(in crate::console) fn clear() -> Self {
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

    pub(super) fn tracing(
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

pub(in crate::console) fn timestamp_label(timestamp_millis: u64) -> String {
    let day_millis = timestamp_millis % 86_400_000;
    let hours = day_millis / 3_600_000;
    let minutes = (day_millis / 60_000) % 60;
    let seconds = (day_millis / 1_000) % 60;
    let millis = day_millis % 1_000;
    format!("{hours:02}:{minutes:02}:{seconds:02}.{millis:03}")
}
