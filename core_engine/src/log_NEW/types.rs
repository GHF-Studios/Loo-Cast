use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tracing::{span::Id, Level as TracingLevel, Metadata, Subscriber};
use tracing_subscriber::{layer::Context, registry::LookupSpan};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}
impl From<TracingLevel> for LogLevel {
    fn from(value: TracingLevel) -> Self {
        match value {
            TracingLevel::TRACE => LogLevel::Trace,
            TracingLevel::DEBUG => LogLevel::Debug,
            TracingLevel::INFO  => LogLevel::Info,
            TracingLevel::WARN  => LogLevel::Warn,
            TracingLevel::ERROR => LogLevel::Error,
        }
    }
}
impl Into<TracingLevel> for LogLevel {
    fn into(self) -> TracingLevel {
        match self {
            LogLevel::Trace => TracingLevel::TRACE,
            LogLevel::Debug => TracingLevel::DEBUG,
            LogLevel::Info  => TracingLevel::INFO,
            LogLevel::Warn  => TracingLevel::WARN,
            LogLevel::Error => TracingLevel::ERROR,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub ts: u64,
    pub lvl: LogLevel,
    pub msg: Arc<str>,
}

static LOG_ID_COUNTER: OnceLock<AtomicU64> = OnceLock::new();

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogId(pub u64);
impl LogId {
    pub fn new() -> Self {
        let counter = LOG_ID_COUNTER.get_or_init(|| AtomicU64::new(1));
        let id = counter.fetch_add(1, Ordering::Relaxed);
        LogId(id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogPath {
    Span(SpanPath),
    Module(ModulePath),
    Physical(PhysicalPath)
}
impl std::fmt::Display for LogPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogPath::Span(path) => write!(f, "LogPath({})", path),
            LogPath::Module(path) => write!(f, "LogPath({})", path),
            LogPath::Physical(path) => write!(f, "LogPath({})", path),
        }
    }
}

#[derive(Default)]
pub struct LogRegistry {
    pub logs: HashMap<LogId, LogEntry>,
    pub span_index: SpanPathIndex,
    pub module_index: ModulePathIndex,
    pub physical_index: PhysicalPathIndex,
}
impl LogRegistry {
    pub fn insert_without_log(
        &mut self,
        span_path: &SpanPath,
    ) {
        self.span_index.insert_without_log(span_path);
    }

    pub fn insert_log(
        &mut self,
        log_id: LogId,
        log_entry: LogEntry,
        span_path: SpanPath,
        module_path: ModulePath,
        physical_path: PhysicalPath,
    ) {
        self.logs.insert(log_id, log_entry);
        self.span_index.insert(&span_path, log_id);
        self.module_index.insert(&module_path, log_id);
        self.physical_index.insert(&physical_path, log_id);
    }

    pub fn get_log(&self, id: &LogId) -> Option<&LogEntry> {
        self.logs.get(id)
    }

    pub fn resolve_span_path(&self, path: &SpanPath) -> Option<&Vec<LogId>> {
        self.span_index.resolve(path)
    }

    pub fn resolve_module_path(&self, path: &ModulePath) -> Option<&Vec<LogId>> {
        self.module_index.resolve(path)
    }

    pub fn resolve_physical_path(&self, path: &PhysicalPath) -> Option<&Vec<LogId>> {
        self.physical_index.resolve(path)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanPath(pub Vec<SpanPathSegment>);
impl SpanPath {
    pub const UNCATEGORIZED: Self = SpanPath(Vec::new());

    pub fn from_context<S>(id: &Id, ctx: &Context<'_, S>) -> SpanPath
    where
        S: Subscriber + for<'lookup> LookupSpan<'lookup>,
    {
        let mut segments = Vec::new();
        if let Some(span_ref) = ctx.span(id) {
            let mut current = Some(span_ref);
            while let Some(span) = current {
                segments.push(SpanPathSegment(span.name().to_string()));
                current = span.parent();
            }
            segments.reverse();
            SpanPath(segments)
        } else {
            SpanPath::UNCATEGORIZED
        }
    }
}
impl std::fmt::Display for SpanPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanPath({})", self.0.iter().map(|s| s.0.as_str()).collect::<Vec<_>>().join("/"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanPathSegment(pub String);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePath {
    pub _crate_: ModuleCratePathSegment,
    pub modules: Vec<ModulePathSegment>,
    pub sub_modules: Vec<SubModulePathSegment>
}
impl ModulePath {
    pub const UNCATEGORIZED: Self = ModulePath {
        _crate_: ModuleCratePathSegment { name: String::new() },
        modules: Vec::new(),
        sub_modules: Vec::new(),
    };

    pub fn from_metadata(meta: &Metadata<'_>, crate_name: &str) -> ModulePath {
        match meta.module_path() {
            Some(path) => {
                let segments: Vec<_> = path
                    .strip_prefix("crate::").unwrap_or(path)
                    .split("::")
                    .map(|s| ModulePathSegment { name: s.to_string() })
                    .collect();

                ModulePath {
                    _crate_: ModuleCratePathSegment { name: crate_name.to_string() },
                    modules: segments,
                    sub_modules: vec![],
                }
            }
            None => ModulePath::UNCATEGORIZED,
        }
    }
}
impl std::fmt::Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModulePath({}/{}/{})",
            self._crate_.name,
            self.modules.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
            self.sub_modules.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleCratePathSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePathSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubModulePathSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalPath {
    pub _crate_: PhysicalCratePathSegment,
    pub folders: Vec<FolderPathSegment>,
    pub file: FilePathSegment,
    pub line: LinePathSegment,
}
impl PhysicalPath {
    pub const UNCATEGORIZED: Self = PhysicalPath {
        _crate_: PhysicalCratePathSegment { name: String::new() },
        folders: Vec::new(),
        file: FilePathSegment { name: String::new() },
        line: LinePathSegment { number: 0 }
    };

    pub fn from_metadata(meta: &Metadata<'_>, crate_name: &str) -> PhysicalPath {
        match (meta.file(), meta.line()) {
            (Some(file), Some(line)) => {
                let mut parts: Vec<&str> = file.split('/').collect();
                if let Some(file_name) = parts.pop() {
                    let folders = parts.into_iter()
                        .map(|s| FolderPathSegment { name: s.to_string() })
                        .collect();
                
                    PhysicalPath {
                        _crate_: PhysicalCratePathSegment { name: crate_name.to_string() },
                        folders,
                        file: FilePathSegment { name: file_name.to_string() },
                        line: LinePathSegment { number: line },
                    }
                } else {
                    PhysicalPath::UNCATEGORIZED
                }
            }
            _ => PhysicalPath::UNCATEGORIZED,
        }
    }
}
impl std::fmt::Display for PhysicalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhysicalPath({}/{}/{}:{})", 
            self._crate_.name,
            self.folders.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
            self.file.name,
            self.line.number
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalCratePathSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FolderPathSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FilePathSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LinePathSegment {
    pub number: u32,
}

#[derive(Default)]
pub struct SpanPathIndex {
    pub span_roots: HashMap<SpanPathSegment, SpanPathNode>,
}
impl SpanPathIndex {
    pub fn insert(&mut self, path: &SpanPath, log_id: LogId) {
        let mut current = self.span_roots.entry(path.0[0].clone()).or_default();

        for segment in &path.0[1..] {
            current = current.span_children.entry(segment.clone()).or_default();
        }

        current.logs.push(log_id);
    }

    pub fn insert_without_log(&mut self, path: &SpanPath) {
        let mut current = self.span_roots.entry(path.0[0].clone()).or_default();

        for segment in &path.0[1..] {
            current = current.span_children.entry(segment.clone()).or_default();
        }
    }

    pub fn resolve(&self, path: &SpanPath) -> Option<&Vec<LogId>> {
        let mut current = self.span_roots.get(&path.0[0])?;
        for segment in &path.0[1..] {
            current = current.span_children.get(segment)?;
        }
        Some(&current.logs)
    }
}

#[derive(Default)]
pub struct SpanPathNode {
    pub span_children: HashMap<SpanPathSegment, SpanPathNode>,
    pub logs: Vec<LogId>,
}

#[derive(Default)]
pub struct ModulePathIndex {
    pub crates: HashMap<ModuleCratePathSegment, ModuleCratePathNode>,
}
impl ModulePathIndex {
    pub fn insert(&mut self, path: &ModulePath, log_id: LogId) {
        let _crate_ = self.crates.entry(path._crate_.clone()).or_default();

        if path.modules.is_empty() {
            _crate_.logs.push(log_id);
            return;
        }

        let mut current_module = _crate_.modules.entry(path.modules[0].clone()).or_default();

        for segment in &path.modules[1..] {
            current_module = _crate_.modules.entry(segment.clone()).or_default();
        }

        if path.sub_modules.is_empty() {
            current_module.logs.push(log_id);
            return;
        }

        let mut current_sub_module = current_module.sub_modules.entry(path.sub_modules[0].clone()).or_default();

        for segment in &path.sub_modules[1..] {
            current_sub_module = current_sub_module.sub_modules.entry(segment.clone()).or_default();
        }

        current_sub_module.logs.push(log_id);
    }

    pub fn insert_without_log(&mut self, path: &ModulePath) {
        let _crate_ = self.crates.entry(path._crate_.clone()).or_default();

        if path.modules.is_empty() {
            return;
        }

        let mut current_module = _crate_.modules.entry(path.modules[0].clone()).or_default();

        for segment in &path.modules[1..] {
            current_module = current_module.modules.entry(segment.clone()).or_default();
        }

        if path.sub_modules.is_empty() {
            return;
        }

        let mut current_sub_module = current_module.sub_modules.entry(path.sub_modules[0].clone()).or_default();

        for segment in &path.sub_modules[1..] {
            current_sub_module = current_sub_module.sub_modules.entry(segment.clone()).or_default();
        }
    }

    pub fn resolve(&self, path: &ModulePath) -> Option<&Vec<LogId>> {
        let _crate_ = self.crates.get(&path._crate_)?;

        if path.modules.is_empty() {
            return Some(&_crate_.logs);
        }

        let mut current_module = _crate_.modules.get(&path.modules[0])?;
        
        for segment in &path.modules[1..] {
            current_module = current_module.modules.get(segment)?;
        }

        if path.sub_modules.is_empty() {
            return Some(&current_module.logs);
        }

        let mut current_sub_module = current_module.sub_modules.get(&path.sub_modules[0])?;

        for segment in &path.sub_modules[1..] {
            current_sub_module = current_sub_module.sub_modules.get(segment)?;
        }

        Some(&current_sub_module.logs)
    }
}

#[derive(Default)]
pub struct ModuleCratePathNode {
    pub modules: HashMap<ModulePathSegment, ModulePathNode>,
    pub logs: Vec<LogId>,
}

#[derive(Default)]
pub struct ModulePathNode {
    pub modules: HashMap<ModulePathSegment, ModulePathNode>,
    pub sub_modules: HashMap<SubModulePathSegment, SubModulePathNode>,
    pub logs: Vec<LogId>,
}

#[derive(Default)]
pub struct SubModulePathNode {
    pub sub_modules: HashMap<SubModulePathSegment, SubModulePathNode>,
    pub logs: Vec<LogId>,
}

#[derive(Default)]
pub struct PhysicalPathIndex {
    pub crates: HashMap<PhysicalCratePathSegment, PhysicalCratePathNode>,
}
impl PhysicalPathIndex {
    pub fn insert(&mut self, path: &PhysicalPath, log_id: LogId) {
        let file = {
            let _crate_ = self.crates.entry(path._crate_.clone()).or_default();

            if path.folders.is_empty() {
                _crate_.files.entry(path.file.clone()).or_default()
            } else {
                let mut current_folder = _crate_.folders.entry(path.folders[0].clone()).or_default();

                for segment in &path.folders[1..] {
                    current_folder = _crate_.folders.entry(segment.clone()).or_default();
                }

                current_folder.files.entry(path.file.clone()).or_default()
            }
        };

        let line = file.lines.entry(path.line.clone()).or_default();

        line.logs.push(log_id);
    }

    pub fn insert_without_log(&mut self, path: &PhysicalPath) {
        let file = {
            let _crate_ = self.crates.entry(path._crate_.clone()).or_default();

            if path.folders.is_empty() {
                _crate_.files.entry(path.file.clone()).or_default()
            } else {
                let mut current_folder = _crate_.folders.entry(path.folders[0].clone()).or_default();

                for segment in &path.folders[1..] {
                    current_folder = _crate_.folders.entry(segment.clone()).or_default();
                }

                current_folder.files.entry(path.file.clone()).or_default()
            }
        };

        let _line = file.lines.entry(path.line.clone()).or_default();
    }

    pub fn resolve(&self, path: &PhysicalPath) -> Option<&Vec<LogId>> {
        let _crate_ = self.crates.get(&path._crate_)?;

        let file = if path.folders.is_empty() {
            _crate_.files.get(&path.file)?
        } else {
            let mut current_folder = _crate_.folders.get(&path.folders[0])?;

            for segment in &path.folders[1..] {
                current_folder = current_folder.folders.get(segment)?;
            }

            current_folder.files.get(&path.file)?
        };

        let line = file.lines.get(&path.line)?;

        Some(&line.logs)
    }
}

#[derive(Default)]
pub struct PhysicalCratePathNode {
    pub folders: HashMap<FolderPathSegment, FolderPathNode>,
    pub files: HashMap<FilePathSegment, FilePathNode>,
}

#[derive(Default)]
pub struct FolderPathNode {
    pub folders: HashMap<FolderPathSegment, FolderPathNode>,
    pub files: HashMap<FilePathSegment, FilePathNode>,
}

#[derive(Default)]
pub struct FilePathNode {
    pub lines: HashMap<LinePathSegment, LinePathNode>,
}

#[derive(Default)]
pub struct LinePathNode {
    pub logs: Vec<LogId>,
}
