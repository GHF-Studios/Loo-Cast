use tracing::{span::Attributes, span::Id, Event};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry::{LookupSpan, SpanRef},
};

use crate::log::{
    arena::{Level},
    resources::LogTreeHandle,
};

pub struct LogTreeTracingLayer {
    pub handle: LogTreeHandle,
}

impl<S> Layer<S> for LogTreeTracingLayer
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let span_path      = span_chain(attrs, id, &ctx);
        let module_path: Vec<&'static str> =
            attrs.metadata().module_path().unwrap_or_default().split("::").collect();
        let file           = attrs.metadata().file().unwrap_or("unknown");
        let line           = attrs.metadata().line().unwrap_or(0);

        self.handle.0.insert(
            &span_path,
            &module_path,
            file,
            line,
            0,
            Level::Trace,
            format!("span_open {:?}", id),
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
            format!("{:?}", event),
        );
    }
}

/* ------------------------------------------------------------------------- */
/* Helpers                                                                   */
/* ------------------------------------------------------------------------- */

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
