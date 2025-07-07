use std::sync::Arc;

use tracing::{span::{Attributes, Id}, Event, Metadata, Subscriber};
use tracing_subscriber::{layer::Context, registry::{LookupSpan, SpanRef}};

use crate::{functions::now_since_start_ns, log_NEW::types::*};

pub(in super) fn extract_log_identity<S>(
    event: &Event<'_>,
    ctx: &Context<'_, S>,
) -> (
    LogId,
    LogEntry,
    SpanPath,
    ModulePath,
    PhysicalStoragePath
)
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>
{
    let log_id = LogId::new();
    let ts = now_since_start_ns();
    let lvl: LogLevel = (*event.metadata().level()).into();
    let msg = Arc::from(event.metadata().name());

    let entry = LogEntry { ts, lvl, msg };

    let span_path = span_path_from_ctx(ctx);
    let module_path = module_path_from_event(event);
    let physical_path = physical_path_from_event(event);

    (log_id, entry, span_path, module_path, physical_path)
}

pub(in super) fn extract_span_identity<S>(
    ctx: &Context<'_, S>
) -> SpanPath 
where
    S: tracing::Subscriber + for<'lookup> LookupSpan<'lookup>
{
    span_path_from_ctx(ctx)
}

fn span_path_from_ctx<S>(ctx: &Context<'_, S>) -> SpanPath
where
    S: Subscriber + for<'lookup> LookupSpan<'lookup>
{
    let mut segments = Vec::new();
    let mut cur = ctx.lookup_current();

    while let Some(span) = cur {
        segments.push(SpanPathSegment(span.name().to_string()));
        cur = span.parent();
    }

    segments.reverse();

    if segments.is_empty() {
        SpanPath::UNCATEGORIZED
    } else {
        SpanPath(segments)
    }
}

fn module_path_from_event(event: &Event<'_>) -> ModulePath {
    let meta = event.metadata();
    let crate_name = extract_crate_name(event).unwrap_or_else(|| "unknown_crate".to_string());

    if let Some(path) = meta.module_path() {
        let segments: Vec<_> = path
            .strip_prefix("crate::").unwrap_or(path)
            .split("::")
            .map(|s| ModulePathSegment { name: s.to_string() })
            .collect();

        return ModulePath {
            _crate_: CrateModulePathSegment { name: crate_name },
            modules: segments,
            sub_modules: vec![],
        };
    }

    ModulePath::UNCATEGORIZED
}

fn physical_path_from_event(event: &Event<'_>) -> PhysicalStoragePath {
    let meta = event.metadata();
    let crate_name = extract_crate_name(event).unwrap_or_else(|| "unknown_crate".to_string());

    if let (Some(file), Some(line)) = (meta.file(), meta.line()) {
        let mut parts: Vec<&str> = file.split('/').collect();
        if let Some(file_name) = parts.pop() {
            let folders = parts
                .into_iter()
                .map(|s| FolderPathSegment { name: s.to_string() })
                .collect();

            return PhysicalStoragePath {
                _crate_: CrateFolderPathSegment { name: crate_name },
                folders,
                file: FilePathSegment { name: file_name.to_string() },
                line: LinePathSegment { number: line },
            };
        }
    }

    PhysicalStoragePath::UNCATEGORIZED
}

fn extract_message(event: &Event<'_>) -> Arc<str> {
    let mut msg = None;

    event.record(&mut |field: &tracing::field::Field, value: &dyn std::fmt::Debug| {
        if field.name() == "message" {
            msg = Some(Arc::from(format!("{:?}", value)));
        }
    });

    msg.unwrap_or_else(|| Arc::from("<NO MESSAGE>"))
}

fn extract_crate_name(event: &Event<'_>) -> Option<String> {
    let mut out = None;
    event.record(&mut |field: &tracing::field::Field, value: &dyn std::fmt::Debug| {
        if field.name() == "crate_name" {
            out = Some(format!("{:?}", value));
        }
    });
    out
}
