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
    config::statics::CONFIG,
    log::{
        arena::Level,
        resources::LogTreeHandle,
        functions::resolve_log_location, // assume exists
    },
};

pub struct MsgAndMetaVisitor {
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

pub struct LogTreeTracingLayer {
    pub handle: LogTreeHandle,
}

impl<S> Layer<S> for LogTreeTracingLayer
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span_path = span_chain(attrs, id, &ctx);
        self.handle.0.insert_span(&span_path);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let scope = ctx.lookup_current();
        let span_path = scope_path(scope);

        let meta = event.metadata();

        // === LOCATION ===
        let Some((crate_name, module_path, file_path)) = resolve_log_location(meta) else {
            return;
        };
        let module_path: Vec<&str> = module_path.iter().map(String::as_str).collect();
        let line = meta.line().unwrap_or(0);

        // === MESSAGE ===
        let mut visitor = MsgAndMetaVisitor::new();
        event.record(&mut visitor);

        let mut msg = visitor
            .message
            .unwrap_or_else(|| Arc::from("<no message>"));

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
            TracingLevel::TRACE => Level::Trace,
            TracingLevel::DEBUG => Level::Debug,
            TracingLevel::INFO => Level::Info,
            TracingLevel::WARN => Level::Warn,
            TracingLevel::ERROR => Level::Error,
        };

        self.handle.0.insert_log(
            &span_path,
            &crate_name,
            &module_path,
            &file_path,
            line,
            0,
            lvl,
            msg,
        );
    }
}

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
