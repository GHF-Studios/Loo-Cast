use tracing::{field::{Visit, Field}, span::Attributes, span::Id, Event};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::{LookupSpan, SpanRef},
};
use std::sync::Arc;
use std::fmt::Debug;

use crate::{config::statics::CONFIG, log::{
    arena::Level,
    resources::LogTreeHandle,
}};

pub struct MsgAndMetaVisitor {
    pub message: Option<Arc<str>>,
    pub meta_fields: Vec<(String, String)>,
    pub target: Option<String>,
    pub module_path: Option<String>,
    pub file: Option<String>,
    pub line: Option<String>,
}

impl MsgAndMetaVisitor {
    pub fn new() -> Self {
        Self {
            message: None,
            meta_fields: Vec::new(),
            target: None,
            module_path: None,
            file: None,
            line: None,
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
                self.meta_fields
                    .push((name.to_string(), format!("{value:?}")));
            }
        }
    }
}

pub struct LogTreeTracingLayer {
    pub handle: LogTreeHandle,
}

impl<S> Layer<S> for LogTreeTracingLayer
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span_path = span_chain(attrs, id, &ctx);
        let module_path: Vec<&'static str> =
            attrs.metadata().module_path().unwrap_or_default().split("::").collect();
        let file = attrs.metadata().file().unwrap_or("unknown");
        let line = attrs.metadata().line().unwrap_or(0);

        self.handle.0.insert(
            &span_path,
            &module_path,
            file,
            line,
            0,
            Level::Trace,
            "", // no message
        );
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let scope  = ctx.lookup_current();
        let span_path = scope_path(scope);

        let meta = event.metadata();
        let module_path: Vec<&'static str> =
            meta.module_path().unwrap_or_default().split("::").collect();
        let file  = meta.file().unwrap_or("unknown");
        let line  = meta.line().unwrap_or(0);
        
        let mut visitor = MsgAndMetaVisitor::new();
        event.record(&mut visitor);

        let mut msg = visitor
            .message
            .unwrap_or_else(|| Arc::from("<no message>"));

        // === Optional: append [META] if enabled ===
        let mut meta_parts = Vec::new();

        if CONFIG.get::<bool>("log/show_target") {
            meta_parts.push(format!("target={}", meta.target()));
        }
        if CONFIG.get::<bool>("log/show_module") {
            if let Some(module) = meta.module_path() {
                meta_parts.push(format!("module={}", module));
            }
        }
        if CONFIG.get::<bool>("log/show_file") {
            if let Some(file) = meta.file() {
                meta_parts.push(format!("file={}", file));
            }
        }
        if CONFIG.get::<bool>("log/show_line") {
            if let Some(line) = meta.line() {
                meta_parts.push(format!("line={}", line));
            }
        }

        if CONFIG.get::<bool>("log/show_other_fields") {
            for (k, v) in visitor.meta_fields {
                meta_parts.push(format!("{k}={v}"));
            }
        }

        if !meta_parts.is_empty() {
            let meta_str = meta_parts.join(", ");
            let combined = format!("{msg} [META] {{ {meta_str} }}");
            msg = Arc::from(combined);
        }

        let lvl = match *meta.level() {
            tracing::Level::TRACE => Level::Trace,
            tracing::Level::DEBUG => Level::Debug,
            tracing::Level::INFO  => Level::Info,
            tracing::Level::WARN  => Level::Warn,
            tracing::Level::ERROR => Level::Error,
        };

        self.handle.0.insert(
            &span_path,
            &module_path,
            file,
            line,
            0,
            lvl,
            msg,
        );
    }
}

/// Walk from the new span up to root, collecting names (root-first order).
fn span_chain<S>(
    attrs: &Attributes<'_>,
    id: &Id,
    ctx: &Context<'_, S>,
) -> Vec<&'static str>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    let mut out = Vec::new();

    if let Some(span_ref) = ctx.span(id) {
        let mut cur = Some(span_ref);
        while let Some(s) = cur {
            out.push(s.name());
            cur = s.parent();
        }
    } else {
        out.push(attrs.metadata().name());
    }

    out.reverse();
    out
}

/// Same as `span_chain` but for the current scope (if any).
fn scope_path<'a, C>(scope: Option<SpanRef<'a, C>>) -> Vec<&'static str>
where
    C: LookupSpan<'a>,
{
    let mut out = Vec::new();
    let mut cur = scope;
    while let Some(s) = cur {
        out.push(s.name());
        cur = s.parent();
    }
    out.reverse();
    out
}
