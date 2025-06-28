use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::log::arena::{Level};

#[derive(Clone, Debug)]
pub struct LogEntry {
    pub ts: u64,
    pub lvl: Level,
    pub msg: Arc<str>,
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LogId(pub u64);

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
