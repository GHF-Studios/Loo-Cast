use tracing::{
    field::{Visit, Field},
    span::Attributes,
    span::Id,
    Event,
    Metadata,
    Level as TracingLevel,
};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::{LookupSpan, SpanRef},
};
use std::sync::Arc;
use std::fmt::Debug;

use crate::{
    config::statics::CONFIG, functions::now_since_start_ns, log_NEW::{
        resources::LogRegistryHandle, tracing::functions::extract_span_identity, types::LogLevel 
    }
};

use super::functions::extract_log_identity;

#[derive(Debug, Clone)]
pub enum PathResolution<T> {
    Resolved(T),
    Uncategorized,
}

pub struct LogTreeTracingLayer {
    pub registry: LogRegistryHandle,
}
impl<S> Layer<S> for LogTreeTracingLayer
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span_path = extract_span_identity(id, &ctx);
        self.registry.0.lock().unwrap().insert_without_log(&span_path);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let (
            log_id, 
            log_entry, 
            span_path, 
            module_path, 
            physical_path
        ) = extract_log_identity(event, &ctx);

        self.registry.0.lock().unwrap().insert_log(log_id, log_entry, span_path, module_path, physical_path);
    }
}

struct MsgAndMetaVisitor {
    pub message: Option<Arc<str>>,
    pub meta_fields: Vec<(String, String)>,
}
impl MsgAndMetaVisitor {
    pub fn new() -> Self {
        Self {
            message: None,
            meta_fields: Vec::new(),
        }
    }
}
impl Visit for MsgAndMetaVisitor {
    fn record_str(&mut self, field: &Field, value: &str) {
        match field.name() {
            "message" => {
                self.message = Some(Arc::from(value));
            }
            name => {
                self.meta_fields.push((name.to_string(), value.to_string()));
            }
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn Debug) {
        match field.name() {
            "message" => {
                self.message = Some(Arc::from(format!("{value:?}")));
            }
            name => {
                self.meta_fields.push((name.to_string(), format!("{value:?}")));
            }
        }
    }
}