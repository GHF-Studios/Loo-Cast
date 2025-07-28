use std::sync::Arc;

use tracing::{Event, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan};

use crate::{functions::now_since_start_ns, log::types::*};

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
        segments.push(SpanSegment { name: span.name().to_string() });
        cur = span.parent();
    }

    segments.reverse();

    if segments.is_empty() {
        SpanPath::default()
    } else {
        SpanPath { spans: segments }
    }
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
    let metadata = event.metadata();
    let lvl: LogLevel = (*metadata.level()).into();
    let msg = Arc::from(event.metadata().name());
    let entry = LogEntry { ts, lvl, msg, metadata };
    let span_path = span_path_from_ctx(ctx);
    let (module_path, physical_path) = parse_paths(
        metadata.module_path().unwrap(), 
        metadata.file().unwrap(), 
        metadata.line().unwrap()
    );

    (log_id, entry, span_path, module_path, physical_path)
}

fn parse_paths(module_path: &str, file_path: &str, line: u32) -> (ModulePath, PhysicalStoragePath) {
    enum CrateOrigin {
        Local { crate_name: String },
        Registry { crate_name: String, version: String },
    }

    struct PathPatternMatch {
        origin: CrateOrigin,
        folders: Vec<String>,
        file_name: String,
        line: u32,
    }

    fn match_file_path(path: &str) -> Option<PathPatternMatch> {
        if cfg!(not(windows)) {
            panic!("Only Windows is supported for now");
        }

        let raw_path = path.replace('\\', "/");
        let parts = raw_path.split('/').collect::<Vec<_>>();

        parse_registry_path(&parts)
            .or_else(|| parse_local_path(&parts))
    }

    fn parse_registry_path(parts: &[&str]) -> Option<PathPatternMatch> {
        if parts.len() < 10 {
            return None;
        }

        let drive = parts[0];
        if !drive.ends_with(':') || drive.len() != 2 {
            return None;
        }

        if parts[1] != "Users" {
            return None;
        }

        let _ = parts[2]; // Discard the username

        if parts[3] != ".cargo" || parts[4] != "registry" || parts[5] != "src" {
            return None;
        }

        let (boilerplate_stuff, hash_id_thing) = parts[6].rsplit_once('-')?;
        if boilerplate_stuff != "index.crates.io" || hash_id_thing.len() != 16 {
            return None;
        }

        let crate_id = parts[7];
        let (crate_name, version) = crate_id.rsplit_once('-')?;
        let (major_version, version_rest) = version.split_once('.')?;
        let (minor_version, patch_version) = version_rest.split_once('.')?;
        let major_version = major_version.parse::<u32>().ok()?;
        let minor_version = minor_version.parse::<u32>().ok()?;
        let patch_version = patch_version.parse::<u32>().ok()?;
        let version = format!("{}.{}.{}", major_version, minor_version, patch_version);

        if parts[8] != "src" {
            return None;
        }

        let relative_parts = &parts[9..];
        if relative_parts.is_empty() {
            return None;
        }

        let folders = relative_parts[..relative_parts.len() - 1]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let file_part = relative_parts.last()?.to_string();
        let (file_id_part, line_part) = file_part.rsplit_once(':')?;
        let (file_name, file_format) = file_id_part.rsplit_once('.')?;
        let line = line_part.parse::<u32>().ok()?;
        if file_format != "rs" {
            return None;
        }

        Some(PathPatternMatch {
            origin: CrateOrigin::Registry {
                crate_name: crate_name.to_string(),
                version,
            },
            folders,
            file_name: file_name.to_string(),
            line
        })
    }

    fn parse_local_path(parts: &[&str]) -> Option<PathPatternMatch> {
        if parts.len() < 3 {
            return None;
        }

        let crate_name = parts[0];

        if parts[1] != "src" {
            return None;
        }

        let relative_parts = &parts[2..];
        if relative_parts.is_empty() {
            return None;
        }

        let folders = relative_parts[..relative_parts.len() - 1]
            .iter()
            .map(|s| s.to_string())
            .collect();

        let file_part = relative_parts.last()?.to_string();
        let (file_id_part, line_part) = file_part.rsplit_once(':')?;
        let (file_name, file_format) = file_id_part.rsplit_once('.')?;
        let line = line_part.parse::<u32>().ok()?;
        if file_format != "rs" {
            return None;
        }

        Some(PathPatternMatch {
            origin: CrateOrigin::Local {
                crate_name: crate_name.to_string(),
            },
            folders,
            file_name: file_name.to_string(),
            line
        })
    }

    let Some(path_match) = match_file_path(file_path) else {
        return (ModulePath::default(), PhysicalStoragePath::default());
    };
    
    if path_match.line != line {
        return (ModulePath::default(), PhysicalStoragePath::default());
    }
    
    let crate_folder = match &path_match.origin {
        CrateOrigin::Registry { crate_name, .. } => {
            CrateFolderSegment { name: crate_name.clone() }
        }
        CrateOrigin::Local { crate_name } => {
            CrateFolderSegment { name: crate_name.clone() }
        }
    };
    let folders = path_match.folders
        .iter()
        .map(|name| FolderSegment { name: name.clone() })
        .collect();
    let file = FileSegment { name: path_match.file_name.clone() };
    let line = LineSegment { number: path_match.line };

    let physical_storage_path = PhysicalStoragePath {
        crate_folder,
        folders,
        file,
        line,
    };

    let module_path = module_path.replace("::", "/");
    let module_parts = module_path.split('/').collect::<Vec<_>>();

    let Some(crate_name) = module_parts.first() else {
        return (ModulePath::default(), PhysicalStoragePath::default());
    };

    let declared_path = &module_parts[1..]; // skip crate

    let final_module = match path_match.file_name.as_str() {
        "mod" => {
            match declared_path.last().cloned() {
                Some(p) => p,
                None => return (ModulePath::default(), PhysicalStoragePath::default())
            }
        },                          // use last folder name
        other => other, // use file stem
    };

    let split_idx = declared_path.iter().position(|&s| s == final_module).unwrap_or(0);

    let modules = declared_path[..=split_idx]
        .iter()
        .map(|s| ModuleSegment { name: s.to_string() })
        .collect::<Vec<_>>();

    let sub_modules = declared_path[split_idx + 1..]
        .iter()
        .map(|s| SubModuleSegment { name: s.to_string() })
        .collect::<Vec<_>>();

    let crate_module = CrateModuleSegment {
        name: crate_name.to_string(),
    };

    let module_path = ModulePath {
        crate_module,
        modules,
        sub_modules,
    };

    (module_path, physical_storage_path)
}
