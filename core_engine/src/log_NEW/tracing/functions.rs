use std::sync::Arc;

use tracing::{span::{Attributes, Id}, Event, Metadata, Subscriber};
use tracing_subscriber::{layer::Context, registry::{LookupSpan, SpanRef}};

use crate::{functions::now_since_start_ns, log_NEW::types::{LogEntry, LogId, ModuleCratePathSegment, ModulePath, ModulePathSegment, PhysicalPath, SpanPath}};

use super::types::PathResolution;

pub(in super) fn extract_log_identity<S>(
    event: &Event<'_>,
    ctx: &Context<'_, S>,
) -> (
    LogId,
    LogEntry,
    SpanPath,
    ModulePath,
    PhysicalPath
)
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>
{
    (
        next_log_id(), 
        LogEntry { 
            ts: now_since_start_ns(), 
            lvl: (*attrs.metadata().level()).into(), 
            msg: Arc::from(attrs.metadata().name())
        }, 
        SpanPath::from_context(id, ctx), 
        ModulePath::from_metadata(attrs.metadata(), crate_name), 
        PhysicalPath::from_metadata(attrs.metadata(), crate_name)
    )
}


pub(in super) fn extract_span_identity<S>(
    id: &Id, 
    ctx: &Context<'_, S>
) -> SpanPath 
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>
{
    SpanPath::from_context(id, ctx)
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