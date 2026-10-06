//! Bevy tracing adapter for the in-game overlay.

use super::{
    queue::ConsoleTransport,
    record::{ConsoleLogLevel, ConsoleRecord},
};
use bevy::{
    log::{
        BoxedLayer,
        tracing::{
            Event as TracingEvent,
            field::{Field, Visit},
        },
        tracing_subscriber::{
            Layer as TracingLayer, filter::FilterFn, layer::Context as TracingContext,
            registry::Registry as TracingRegistry,
        },
    },
    prelude::*,
};
use std::fmt;

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
