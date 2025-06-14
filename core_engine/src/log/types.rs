use std::collections::HashMap;
use tracing_subscriber::layer::{Context, Layer};
use tracing::{span::{Attributes, Id}, Event};
use tracing::metadata::Level;

use crate::log::resources::LogTreeHandle;

#[derive(Debug)]
pub struct LogSpan {
    pub id: tracing::span::Id,
    pub name: String,
    pub parent: Option<tracing::span::Id>,
    pub fields: HashMap<String, String>,
    pub children: Vec<tracing::span::Id>,
    pub logs: Vec<LogEvent>,
    pub is_closed: bool,
}

#[derive(Debug)]
pub struct LogEvent {
    pub message: String,
    pub time: std::time::SystemTime,
    pub level: tracing::Level,
    pub fields: HashMap<String, String>,
}

#[derive(Default)]
pub struct LogTree {
    pub spans: HashMap<tracing::span::Id, LogSpan>,
    pub root_spans: Vec<tracing::span::Id>,
}

pub struct LogTreeTracingLayer {
    pub log_tree: LogTreeHandle,
}

impl<S> Layer<S> for LogTreeTracingLayer
where
    S: tracing::Subscriber,
    S: for<'a> tracing_subscriber::registry::LookupSpan<'a>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let mut tree = self.log_tree.0.lock().unwrap();
        let mut fields = HashMap::new();
        
        attrs.record(&mut |field: &tracing::field::Field, value: &dyn std::fmt::Debug| {
            fields.insert(field.to_string(), format!("{:?}", value));
        });

        let parent = ctx.span(id).and_then(|span_ref| span_ref.parent().map(|p| p.id()));
        tree.spans.insert(id.clone(), LogSpan {
            id: id.clone(),
            name: attrs.metadata().name().to_string(),
            parent: parent.clone(),
            fields,
            children: vec![],
            logs: vec![],
            is_closed: false,
        });

        if let Some(parent_id) = parent {
            if let Some(parent_span) = tree.spans.get_mut(&parent_id) {
                parent_span.children.push(id.clone());
            }
        } else {
            tree.root_spans.push(id.clone());
        }
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let mut fields = HashMap::new();

        event.record(&mut |field: &tracing::field::Field, value: &dyn std::fmt::Debug| {
            fields.insert(field.to_string(), format!("{:?}", value));
        });

        let level = *event.metadata().level();
        if level == Level::TRACE || level == Level::DEBUG || level == Level::INFO {
            return;
        }

        let log = LogEvent {
            message: format!("{:?}", event),
            level,
            time: std::time::SystemTime::now(),
            fields,
        };

        let mut tree = self.log_tree.0.lock().unwrap();
        if let Some(scope) = ctx.lookup_current() {
            if let Some(span) = tree.spans.get_mut(&scope.id()) {
                span.logs.push(log);
            }
        }
    }

    fn on_close(&self, id: Id, _ctx: Context<'_, S>) {
        let mut tree = self.log_tree.0.lock().unwrap();
        if let Some(span) = tree.spans.get_mut(&id) {
            span.is_closed = true;
        }
    }
}
