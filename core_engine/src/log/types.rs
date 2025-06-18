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
        arena::Level, functions::resolve_log_location_path, resources::{LocationTreeHandle, LogStorageHandle, SpanTreeHandle} // assume exists
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogPath {
    Span(Vec<&'static str>),
    Loc(Vec<LocationPathSegment>),
}
impl std::fmt::Display for LogPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogPath::Span(path) => write!(f, "SpanPath({})", path.join(" > ")),
            LogPath::Loc(path) => write!(f, "LocPath({})", path.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" > ")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LocationPathSegment {
    Crate(String),
    Module(String),
    File(String),
    Line(u32),
    SubModule(String),
}
impl std::fmt::Display for LocationPathSegment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LocationPathSegment::Crate(name) => write!(f, "Crate({})", name),
            LocationPathSegment::Module(name) => write!(f, "Module({})", name),
            LocationPathSegment::File(name) => write!(f, "File({})", name),
            LocationPathSegment::Line(line) => write!(f, "Line({})", line),
            LocationPathSegment::SubModule(name) => write!(f, "SubModule({})", name),
        }
    }
}

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
    pub storage: LogStorageHandle,
    pub location_tree: LocationTreeHandle,
    pub span_tree: SpanTreeHandle,
}

impl<S> Layer<S> for LogTreeTracingLayer
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span_path = span_chain(attrs, id, &ctx);
        self.span_tree.0.insert_path(span_path);
    }

    fn on_event(&self, event: &Event<'_>, ctx: Context<'_, S>) {
        let scope = ctx.lookup_current();
        let span_path = scope_path(scope);

        let meta = event.metadata();
        let location_path = resolve_log_location_path(meta);

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
            TracingLevel::INFO  => Level::Info,
            TracingLevel::WARN  => Level::Warn,
            TracingLevel::ERROR => Level::Error,
        };

        let log_id = self.storage.0.insert_log(lvl, msg);
        self.span_tree.0.insert_log(span_path, log_id);
        self.location_tree.0.insert_log(location_path, log_id);
    }
}

fn span_chain<S>(
    attrs: &Attributes<'_>,
    id: &Id,
    ctx: &Context<'_, S>,
) -> Vec<String>
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    let mut out = Vec::new();

    if let Some(span_ref) = ctx.span(id) {
        let mut cur = Some(span_ref);
        while let Some(s) = cur {
            out.push(s.name().to_string());
            cur = s.parent();
        }
    } else {
        out.push(attrs.metadata().name().to_string());
    }

    out.reverse();
    out
}

fn scope_path<'a, C>(scope: Option<SpanRef<'a, C>>) -> Vec<String>
where
    C: LookupSpan<'a>,
{
    let mut out = Vec::new();
    let mut cur = scope;
    while let Some(s) = cur {
        out.push(s.name().to_string());
        cur = s.parent();
    }
    out.reverse();

    out
}
