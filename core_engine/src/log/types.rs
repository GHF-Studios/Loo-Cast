use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Level as TracingLevel;

use crate::log::statics::LOG_ID_COUNTER;

// === Basics ===

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
impl From<LogLevel> for TracingLevel {
    fn from(value: LogLevel) -> TracingLevel {
        match value {
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
impl std::fmt::Display for LogId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum LogPath {
    Span(SpanPath),
    Module(ModulePath),
    Physical(PhysicalStoragePath)
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
    pub span_registry: SpanRegistry,
    pub module_registry: ModuleRegistry,
    pub physical_registry: PhysicalRegistry,
}
impl LogRegistry {
    pub fn insert_without_log(
        &mut self,
        span_path: &SpanPath,
    ) {
        self.span_registry.insert_without_log(span_path);
    }

    pub fn insert_log(
        &mut self,
        log_id: LogId,
        log_entry: LogEntry,
        span_path: SpanPath,
        module_path: ModulePath,
        physical_path: PhysicalStoragePath,
    ) {
        self.logs.insert(log_id, log_entry);
        self.span_registry.insert(&span_path, log_id);
        self.module_registry.insert(&module_path, log_id);
        self.physical_registry.insert(&physical_path, log_id);
    }

    pub fn get_log(&self, id: &LogId) -> Option<&LogEntry> {
        self.logs.get(id)
    }

    pub fn resolve_span_path(&self, path: &SpanPath) -> Option<&Vec<LogId>> {
        self.span_registry.resolve(path)
    }

    pub fn resolve_module_path(&self, path: &ModulePath) -> Option<&Vec<LogId>> {
        self.module_registry.resolve(path)
    }

    pub fn resolve_physical_path(&self, path: &PhysicalStoragePath) -> Option<&Vec<LogId>> {
        self.physical_registry.resolve(path)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogSelection {
    pub state: LogSelectionState,
    pub privilege: LogSelectionPrivilege,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSelectionState {
    #[default]
    InheritedOrDefault,
    Selected,
    Deselected,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogSelectionPrivilege {
    #[default]
    None,
    User,
    Sudo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogSelectionCommand {
    ResetToInheritedOrDefault(LogSelectionPrivilege),
    Select(LogSelectionPrivilege),
    Deselect(LogSelectionPrivilege),
    RecursiveResetToInheritedOrDefault(LogSelectionPrivilege),
    RecursiveSelect(LogSelectionPrivilege),
    RecursiveDeselect(LogSelectionPrivilege),
}
impl LogSelectionCommand {
    pub fn unpack(self) -> (LogSelectionState, LogSelectionPrivilege, bool) {
        match self {
            LogSelectionCommand::ResetToInheritedOrDefault(required) => {
                (LogSelectionState::InheritedOrDefault, required, false)
            }
            LogSelectionCommand::Select(required) => {
                (LogSelectionState::Selected, required, false)
            }
            LogSelectionCommand::Deselect(required) => {
                (LogSelectionState::Deselected, required, false)
            }
            LogSelectionCommand::RecursiveResetToInheritedOrDefault(required) => {
                (LogSelectionState::InheritedOrDefault, required, true)
            }
            LogSelectionCommand::RecursiveSelect(required) => {
                (LogSelectionState::Selected, required, true)
            }
            LogSelectionCommand::RecursiveDeselect(required) => {
                (LogSelectionState::Deselected, required, true)
            }
        }
    }

    pub fn run(
        current_privilege: &mut LogSelectionPrivilege, 
        current_selection_state: &mut LogSelectionState, 
        required_privilege: LogSelectionPrivilege, 
        requested_selection_state: LogSelectionState
    ) -> Result<(), LogSelectionCommandError> {
        if required_privilege > *current_privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: *current_privilege,
            })?
        }
    
        if *current_selection_state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        *current_privilege = required_privilege;
        *current_selection_state = requested_selection_state;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LogSelectionCommandError {
    InsufficientPrivilege {
        required: LogSelectionPrivilege,
        actual: LogSelectionPrivilege,
    },
    AlreadyAtState(LogSelectionState),
    SpanPathNotFound(SpanPath),
    ModulePathNotFound(ModulePath),
    PhysicalPathNotFound(PhysicalSelectionPath),
}

#[derive(Default)]
pub struct NodeMetadata {
    pub ui_collapsed: bool,
}

// === Path Types ===

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanPath {
    pub spans: Vec<SpanSegment>
}
impl SpanPath {
    pub const UNCATEGORIZED: Self = SpanPath { spans: Vec::new() };
}
impl std::fmt::Display for SpanPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanPath({})", self.spans.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePath {
    pub crate_module: CrateModuleSegment,
    pub modules: Vec<ModuleSegment>,
    pub sub_modules: Vec<SubModuleSegment>
}
impl ModulePath {
    pub const UNCATEGORIZED: Self = ModulePath {
        crate_module: CrateModuleSegment { name: String::new() },
        modules: Vec::new(),
        sub_modules: Vec::new(),
    };
}
impl std::fmt::Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModulePath({}/{}/{})",
            self.crate_module.name,
            self.modules.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
            self.sub_modules.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/")
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalStoragePath {
    pub crate_folder: CrateFolderSegment,
    pub folders: Vec<FolderSegment>,
    pub file: FileSegment,
    pub line: LineSegment,
}
impl PhysicalStoragePath {
    pub const UNCATEGORIZED: Self = PhysicalStoragePath {
        crate_folder: CrateFolderSegment { name: String::new() },
        folders: Vec::new(),
        file: FileSegment { name: String::new() },
        line: LineSegment { number: 0 }
    };
}
impl std::fmt::Display for PhysicalStoragePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhysicalStoragePath({}/{}/{}:{})", 
            self.crate_folder.name,
            self.folders.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
            self.file.name,
            self.line.number
        )
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalSelectionPath {
    pub crate_folder: CrateFolderSegment,
    pub folders: Vec<FolderSegment>,
    pub file: Option<FileSegment>,
    pub line: Option<LineSegment>,
}
impl PhysicalSelectionPath {
    pub const UNCATEGORIZED: Self = PhysicalSelectionPath {
        crate_folder: CrateFolderSegment { name: String::new() },
        folders: Vec::new(),
        file: Some(FileSegment { name: String::new() }),
        line: Some(LineSegment { number: 0 })
    };
}
impl std::fmt::Display for PhysicalSelectionPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let crate_name = self.crate_folder.name.clone();

        if self.folders.is_empty() {
            return write!(f, "PhysicalSelectionPath({})", 
                self.crate_folder.name
            );
        }

        let mut folder_names = Vec::with_capacity(self.folders.len());

        for folder in &self.folders {
            folder_names.push(folder.name.clone());
        }

        let file_name = match self.file {
            Some(ref segment) => segment.name.clone(),
            None => {
                return write!(f, "PhysicalSelectionPath({}/{})", 
                    crate_name,
                    folder_names.join("/"),
                );
            }
        };

        let line_number = match self.line {
            Some(ref segment) => segment.number,
            None => {
                return write!(f, "PhysicalSelectionPath({}/{}/{})", 
                    crate_name,
                    folder_names.join("/"),
                    file_name,
                );
            }
        };

        write!(f, "PhysicalSelectionPath({}/{}/{}:{})",
            crate_name,
            folder_names.join("/"),
            file_name,
            line_number,
        )
    }
}

// === Segment Types ===

// --- Span ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanSegment {
    pub name: String
}

// --- Module ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateModuleSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubModuleSegment {
    pub name: String,
}

// --- Physical ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateFolderSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FolderSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileSegment {
    pub name: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LineSegment {
    pub number: u32,
}

// === Registry Types ===

#[derive(Default)]
pub struct SpanRegistry {
    pub span_roots: HashMap<SpanSegment, SpanNode>,
}
impl SpanRegistry {
    pub fn insert(&mut self, path: &SpanPath, log_id: LogId) {
        println!("SpanRegistry: Trying to insert path `{path}` with log `{log_id}`");

        let root_span_segment = if path.spans.is_empty() {
            SpanSegment { name: "[UNCATEGORIZED]".to_string() }
        } else {
            path.spans[0].clone()
        };
        let mut current = self.span_roots.entry(root_span_segment).or_default();

        for segment in path.spans.get(1..).unwrap_or_default() {
            current = current.span_children.entry(segment.clone()).or_default();
        }

        current.logs.push(log_id);
    }

    pub fn insert_without_log(&mut self, path: &SpanPath) {
        println!("SpanRegistry: Trying to insert path `{path}` without log");

        let root_span_segment = if path.spans.is_empty() {
            SpanSegment { name: "[UNCATEGORIZED]".to_string() }
        } else {
            path.spans[0].clone()
        };
        let mut current = self.span_roots.entry(root_span_segment).or_default();

        for segment in path.spans.get(1..).unwrap_or_default() {
            current = current.span_children.entry(segment.clone()).or_default();
        }
    }

    pub fn resolve(&self, path: &SpanPath) -> Option<&Vec<LogId>> {
        let mut current = self.span_roots.get(&path.spans[0])?;
        for segment in path.spans.get(1..).unwrap_or_default() {
            current = current.span_children.get(segment)?;
        }
        Some(&current.logs)
    }
}

#[derive(Default)]
pub struct ModuleRegistry {
    pub crates: HashMap<CrateModuleSegment, CrateModuleNode>,
}
impl ModuleRegistry {
    pub fn insert(&mut self, path: &ModulePath, log_id: LogId) {
        // println!("ModuleRegistry: Trying to insert path `{path}` with log `{log_id}`");

        let crate_module_segment = if path.crate_module.name.is_empty() {
            CrateModuleSegment { name: "[UNCATEGORIZED]".to_string() }
        } else {
            path.crate_module.clone()
        };
        let crate_module = self.crates.entry(crate_module_segment).or_default();

        if path.modules.is_empty() {
            crate_module.logs.push(log_id);
            return;
        }

        let mut current_module = crate_module.modules.entry(path.modules[0].clone()).or_default();

        for segment in path.modules.get(1..).unwrap_or_default() {
            current_module = crate_module.modules.entry(segment.clone()).or_default();
        }

        if path.sub_modules.is_empty() {
            current_module.logs.push(log_id);
            return;
        }

        let mut current_sub_module = current_module.sub_modules.entry(path.sub_modules[0].clone()).or_default();

        for segment in path.sub_modules.get(1..).unwrap_or_default() {
            current_sub_module = current_sub_module.sub_modules.entry(segment.clone()).or_default();
        }

        current_sub_module.logs.push(log_id);
    }

    pub fn insert_without_log(&mut self, path: &ModulePath) {
        // println!("ModuleRegistry: Trying to insert path `{path}` without log");

        let crate_module_segment = if path.crate_module.name.is_empty() {
            CrateModuleSegment { name: "[UNCATEGORIZED]".to_string() }
        } else {
            path.crate_module.clone()
        };
        let crate_module = self.crates.entry(crate_module_segment).or_default();

        if path.modules.is_empty() {
            return;
        }

        let mut current_module = crate_module.modules.entry(path.modules[0].clone()).or_default();

        for segment in path.modules.get(1..).unwrap_or_default() {
            current_module = current_module.modules.entry(segment.clone()).or_default();
        }

        if path.sub_modules.is_empty() {
            return;
        }

        let mut current_sub_module = current_module.sub_modules.entry(path.sub_modules[0].clone()).or_default();

        for segment in path.sub_modules.get(1..).unwrap_or_default() {
            current_sub_module = current_sub_module.sub_modules.entry(segment.clone()).or_default();
        }
    }

    pub fn resolve(&self, path: &ModulePath) -> Option<&Vec<LogId>> {
        let crate_module = self.crates.get(&path.crate_module)?;

        if path.modules.is_empty() {
            return Some(&crate_module.logs);
        }

        let mut current_module = crate_module.modules.get(&path.modules[0])?;
        
        for segment in path.modules.get(1..).unwrap_or_default() {
            current_module = current_module.modules.get(segment)?;
        }

        if path.sub_modules.is_empty() {
            return Some(&current_module.logs);
        }

        let mut current_sub_module = current_module.sub_modules.get(&path.sub_modules[0])?;

        for segment in path.sub_modules.get(1..).unwrap_or_default() {
            current_sub_module = current_sub_module.sub_modules.get(segment)?;
        }

        Some(&current_sub_module.logs)
    }
}

#[derive(Default)]
pub struct PhysicalRegistry {
    pub crates: HashMap<CrateFolderSegment, CrateFolderNode>,
}
impl PhysicalRegistry {
    pub fn insert(&mut self, path: &PhysicalStoragePath, log_id: LogId) {
        // println!("PhysicalRegistry: Trying to insert path `{path}` with log `{log_id}`");

        let file = {
            let crate_folder_segment = if path.crate_folder.name.is_empty() {
                CrateFolderSegment { name: "[UNCATEGORIZED]".to_string() }
            } else {
                path.crate_folder.clone()
            };
            let crate_folder = self.crates.entry(crate_folder_segment).or_default();

            if path.folders.is_empty() {
                crate_folder.files.entry(path.file.clone()).or_default()
            } else {
                let mut current_folder = crate_folder.folders.entry(path.folders[0].clone()).or_default();

                for segment in path.folders.get(1..).unwrap_or_default() {
                    current_folder = crate_folder.folders.entry(segment.clone()).or_default();
                }

                current_folder.files.entry(path.file.clone()).or_default()
            }
        };

        let line = file.lines.entry(path.line.clone()).or_default();

        line.logs.push(log_id);
    }

    pub fn insert_without_log(&mut self, path: &PhysicalStoragePath) {
        // println!("PhysicalRegistry: Trying to insert path `{path}` without log");

        let file = {
            let crate_folder_segment = if path.crate_folder.name.is_empty() {
                CrateFolderSegment { name: "[UNCATEGORIZED]".to_string() }
            } else {
                path.crate_folder.clone()
            };
            let crate_folder = self.crates.entry(crate_folder_segment).or_default();

            if path.folders.is_empty() {
                crate_folder.files.entry(path.file.clone()).or_default()
            } else {
                let mut current_folder = crate_folder.folders.entry(path.folders[0].clone()).or_default();

                for segment in path.folders.get(1..).unwrap_or_default() {
                    current_folder = crate_folder.folders.entry(segment.clone()).or_default();
                }

                current_folder.files.entry(path.file.clone()).or_default()
            }
        };

        let _line = file.lines.entry(path.line.clone()).or_default();
    }

    pub fn resolve(&self, path: &PhysicalStoragePath) -> Option<&Vec<LogId>> {
        let crate_folder = self.crates.get(&path.crate_folder)?;

        let file = if path.folders.is_empty() {
            crate_folder.files.get(&path.file)?
        } else {
            let mut current_folder = crate_folder.folders.get(&path.folders[0])?;

            for segment in path.folders.get(1..).unwrap_or_default() {
                current_folder = current_folder.folders.get(segment)?;
            }

            current_folder.files.get(&path.file)?
        };

        let line = file.lines.get(&path.line)?;

        Some(&line.logs)
    }
}

// === Node Types ===

// --- Span ---

#[derive(Default)]
pub struct SpanNode {
    pub span_children: HashMap<SpanSegment, SpanNode>,
    pub logs: Vec<LogId>,
}

// --- Module ---

#[derive(Default)]
pub struct CrateModuleNode {
    pub modules: HashMap<ModuleSegment, ModuleNode>,
    pub logs: Vec<LogId>,
}

#[derive(Default)]
pub struct ModuleNode {
    pub modules: HashMap<ModuleSegment, ModuleNode>,
    pub sub_modules: HashMap<SubModuleSegment, SubModuleNode>,
    pub logs: Vec<LogId>,
}

#[derive(Default)]
pub struct SubModuleNode {
    pub sub_modules: HashMap<SubModuleSegment, SubModuleNode>,
    pub logs: Vec<LogId>,
}

// --- Physical ---

#[derive(Default)]
pub struct CrateFolderNode {
    pub folders: HashMap<FolderSegment, FolderNode>,
    pub files: HashMap<FileSegment, FileNode>,
}

#[derive(Default)]
pub struct FolderNode {
    pub folders: HashMap<FolderSegment, FolderNode>,
    pub files: HashMap<FileSegment, FileNode>,
}

#[derive(Default)]
pub struct FileNode {
    pub lines: HashMap<LineSegment, LineNode>,
}

#[derive(Default)]
pub struct LineNode {
    pub logs: Vec<LogId>,
}

// === PathSelections Types ===

#[derive(Default)]
pub struct SpanPathSelections {
    pub span_roots: HashMap<SpanSegment, SpanNodeSelection>,
}
impl SpanPathSelections {
    pub fn select(&mut self, path: &SpanPath, command: LogSelectionCommand) -> Result<(), LogSelectionCommandError> {
        let span = self.get_span_mut(path).ok_or(LogSelectionCommandError::SpanPathNotFound(path.clone()))?;
        let (required_privilege, requested_selection_state, recursive) = command.unpack();

        span.select(requested_selection_state, required_privilege, recursive)
    }

    pub fn collect_logs(&self, registry: &LogRegistry) -> Vec<LogId> {
        let mut out = Vec::new();

        for (root_segment, root_node_selection) in &self.span_roots {
            let root_node = registry
                .span_registry
                .span_roots
                .get(root_segment)
                .unwrap_or_else(|| unreachable!("Selection path root {:?} not found in registry", root_segment));

            let effective_selection_state = match root_node_selection.selection.state {
                LogSelectionState::InheritedOrDefault => LogSelectionState::Deselected,
                explicit => explicit,
            };

            Self::collect_logs_from_span(
                root_node_selection,
                root_node,
                effective_selection_state,
                &mut out,
            );
        }

        out
    }

    fn collect_logs_from_span(
        selection: &SpanNodeSelection,
        parent_node: &SpanNode,
        inherited_state: LogSelectionState,
        out: &mut Vec<LogId>,
    ) {
        let effective_selection_state = match selection.selection.state {
            LogSelectionState::InheritedOrDefault => inherited_state,
            explicit => explicit,
        };

        if effective_selection_state == LogSelectionState::Selected {
            out.extend(&parent_node.logs);
        }

        for (child_segment, child_selection) in &selection.span_children {
            let child_node = parent_node
                .span_children
                .get(child_segment)
                .unwrap_or_else(|| unreachable!("Child segment {:?} not found in registry", child_segment));

            Self::collect_logs_from_span(child_selection, child_node, effective_selection_state, out);
        }
    }

    pub fn is_selected(&self, path: &SpanPath) -> bool {
        self.get_span(path).is_some()
    }

    pub fn get_span(&self, path: &SpanPath) -> Option<&SpanNodeSelection> {
        let mut current = self.span_roots.get(&path.spans[0])?;
        
        for segment in path.spans.get(1..).unwrap_or_default() {
            current = current.span_children.get(segment)?;
        }
        
        Some(current)
    }
    fn get_span_mut(&mut self, path: &SpanPath) -> Option<&mut SpanNodeSelection> {
        let mut current = self.span_roots.get_mut(&path.spans[0])?;
        
        for segment in path.spans.get(1..).unwrap_or_default() {
            current = current.span_children.get_mut(segment)?;
        }
        
        Some(current)
    }
}

#[derive(Default)]
pub struct ModulePathSelections {
    pub crates: HashMap<CrateModuleSegment, CrateModuleNodeSelection>,
}
impl ModulePathSelections {
    pub fn select(&mut self, path: &ModulePath, command: LogSelectionCommand) -> Result<(), LogSelectionCommandError> {
        match (path.modules.is_empty(), path.sub_modules.is_empty()) {
            (false, false) => {
                let sub_module = self.get_sub_module_mut(path).ok_or(LogSelectionCommandError::ModulePathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                sub_module.select(required_privilege, requested_selection_state, recursive)
            }
            (false, true) => {
                let module = self.get_module_mut(path).ok_or(LogSelectionCommandError::ModulePathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                module.select(required_privilege, requested_selection_state, recursive)
            }
            (true, false) => unreachable!("Invalid path: No module has been selected, yet a sub_module has been selected"),
            (true, true) => {
                let crate_module = self.get_crate_module_mut(path).ok_or(LogSelectionCommandError::ModulePathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                crate_module.select(required_privilege, requested_selection_state, recursive)
            }
        }
    }

    pub fn collect_logs(&self, registry: &LogRegistry) -> Vec<LogId> {
        let mut out = Vec::new();

        for (crate_segment, crate_node_selection) in &self.crates {
            let crate_node = registry
                .module_registry
                .crates
                .get(crate_segment)
                .unwrap_or_else(|| unreachable!("Crate {:?} not found in module registry", crate_segment));

            let effective_selection_state = match crate_node_selection.selection.state {
                LogSelectionState::InheritedOrDefault => LogSelectionState::Deselected,
                explicit => explicit,
            };

            if effective_selection_state == LogSelectionState::Selected {
                out.extend(&crate_node.logs);
            }

            for (module_segment, module_selection) in &crate_node_selection.modules {
                let module_node = crate_node
                    .modules
                    .get(module_segment)
                    .unwrap_or_else(|| unreachable!("Module {:?} not found in registry", module_segment));
                Self::collect_logs_from_module(module_selection, module_node, effective_selection_state, &mut out);
            }
        }

        out
    }

    fn collect_logs_from_module(
        selection: &ModuleNodeSelection,
        module_node: &ModuleNode,
        inherited_state: LogSelectionState,
        out: &mut Vec<LogId>,
    ) {
        let effective_selection_state = match selection.selection.state {
            LogSelectionState::InheritedOrDefault => inherited_state,
            explicit => explicit,
        };

        if effective_selection_state == LogSelectionState::Selected {
            out.extend(&module_node.logs);
        }

        for (module_segment, module_selection) in &selection.modules {
            let module_node = module_node
                .modules
                .get(module_segment)
                .unwrap_or_else(|| unreachable!("Nested module {:?} not found", module_segment));
            Self::collect_logs_from_module(module_selection, module_node, effective_selection_state, out);
        }

        for (sub_module_segment, sub_module_selection) in &selection.sub_modules {
            let sub_module_node = module_node
                .sub_modules
                .get(sub_module_segment)
                .unwrap_or_else(|| unreachable!("SubModule {:?} not found", sub_module_segment));
            Self::collect_logs_from_submodule(sub_module_selection, sub_module_node, effective_selection_state, out);
        }
    }

    fn collect_logs_from_submodule(
        selection: &SubModuleNodeSelection,
        sub_module_node: &SubModuleNode,
        inherited: LogSelectionState,
        out: &mut Vec<LogId>,
    ) {
        let effective_selection_state = match selection.selection.state {
            LogSelectionState::InheritedOrDefault => inherited,
            explicit => explicit,
        };

        if effective_selection_state == LogSelectionState::Selected {
            out.extend(&sub_module_node.logs);
        }

        for (sub_module_segment, sub_module_selection) in &selection.sub_modules {
            let sub_module_node = sub_module_node
                .sub_modules
                .get(sub_module_segment)
                .unwrap_or_else(|| unreachable!("Nested submodule {:?} not found", sub_module_segment));
            Self::collect_logs_from_submodule(sub_module_selection, sub_module_node, effective_selection_state, out);
        }
    }

    pub fn get_crate_module(&self, path: &ModulePath) -> Option<&CrateModuleNodeSelection> {
        self.crates.get(&path.crate_module)
    }
    fn get_crate_module_mut(&mut self, path: &ModulePath) -> Option<&mut CrateModuleNodeSelection> {
        self.crates.get_mut(&path.crate_module)
    }

    pub fn get_module(&self, path: &ModulePath) -> Option<&ModuleNodeSelection> {
        let crate_module = self.crates.get(&path.crate_module)?;
        let mut module = crate_module.modules.get(&path.modules[0])?;

        for segment in path.modules.get(1..).unwrap_or_default() {
            module = module.modules.get(segment)?;
        }

        Some(module)
    }
    fn get_module_mut(&mut self, path: &ModulePath) -> Option<&mut ModuleNodeSelection> {
        let crate_module = self.crates.get_mut(&path.crate_module)?;
        let mut module = crate_module.modules.get_mut(&path.modules[0])?;

        for segment in path.modules.get(1..).unwrap_or_default() {
            module = module.modules.get_mut(segment)?;
        }

        Some(module)
    }

    pub fn get_sub_module(&self, path: &ModulePath) -> Option<&SubModuleNodeSelection> {
        let crate_module = self.crates.get(&path.crate_module)?;
        let mut module = crate_module.modules.get(&path.modules[0])?;

        for segment in path.modules.get(1..).unwrap_or_default() {
            module = module.modules.get(segment)?;
        }

        let mut sub_module = module.sub_modules.get(&path.sub_modules[0])?;

        for segment in path.sub_modules.get(1..).unwrap_or_default() {
            sub_module = sub_module.sub_modules.get(segment)?;
        }

        Some(sub_module)
    }
    fn get_sub_module_mut(&mut self, path: &ModulePath) -> Option<&mut SubModuleNodeSelection> {
        let crate_module = self.crates.get_mut(&path.crate_module)?;
        let mut module = crate_module.modules.get_mut(&path.modules[0])?;

        for segment in path.modules.get(1..).unwrap_or_default() {
            module = module.modules.get_mut(segment)?;
        }

        let mut sub_module = module.sub_modules.get_mut(&path.sub_modules[0])?;

        for segment in path.sub_modules.get(1..).unwrap_or_default() {
            sub_module = sub_module.sub_modules.get_mut(segment)?;
        }

        Some(sub_module)
    }
}

#[derive(Default)]
pub struct PhysicalPathSelections {
    pub crates: HashMap<CrateFolderSegment, CrateFolderNodeSelection>,
}
impl PhysicalPathSelections {
    pub fn select(&mut self, path: &PhysicalSelectionPath, command: LogSelectionCommand) -> Result<(), LogSelectionCommandError> {
        match (path.folders.is_empty(), path.file.is_none(), path.line.is_none()) {
            (false, false, false) => {
                let line = self.get_line_mut(path).ok_or(LogSelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, _recursive) = command.unpack();
                
                line.select(required_privilege, requested_selection_state)
            }
            (false, false, true) => {
                let file = self.get_file_mut(path).ok_or(LogSelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                file.select(required_privilege, requested_selection_state, recursive)
            }
            (false, true, false) => unreachable!("Invalid path: No file has been, yet a line has been selected"),
            (false, true, true) => {
                let folder = self.get_folder_mut(path).ok_or(LogSelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                folder.select(required_privilege, requested_selection_state, recursive)
            }
            (true, false, false) => {
                let line = self.get_line_mut(path).ok_or(LogSelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, _recursive) = command.unpack();

                line.select(required_privilege, requested_selection_state)
            }
            (true, false, true) => {
                let file = self.get_file_mut(path).ok_or(LogSelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();

                file.select(required_privilege, requested_selection_state, recursive)
            }
            (true, true, false) => unreachable!("Invalid path: No file has been selected, yet a line has been selected"),
            (true, true, true) => {
                let crate_folder = self.get_crate_folder_mut(path).ok_or(LogSelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();

                crate_folder.select(required_privilege, requested_selection_state, recursive)
            }
        }
    }

    pub fn collect_logs(&self, registry: &LogRegistry) -> Vec<LogId> {
        let mut out = Vec::new();

        for (crate_segment, crate_node_selection) in &self.crates {
            let crate_node = registry
                .physical_registry
                .crates
                .get(crate_segment)
                .unwrap_or_else(|| unreachable!("Crate {:?} not found in physical registry", crate_segment));

            let effective_selection_state = match crate_node_selection.selection.state {
                LogSelectionState::InheritedOrDefault => LogSelectionState::Deselected,
                explicit => explicit,
            };

            if effective_selection_state == LogSelectionState::Selected {
                out.extend(crate_node.files.values().flat_map(|file| file.lines.values().flat_map(|line| &line.logs)));
            }

            for (folder_segment, folder_selection) in &crate_node_selection.folders {
                let folder_node = crate_node.folders.get(folder_segment)
                    .unwrap_or_else(|| unreachable!("Folder {:?} not found in registry", folder_segment));
                Self::collect_logs_from_folder(folder_selection, folder_node, effective_selection_state, &mut out);
            }
        }

        out
    }

    fn collect_logs_from_folder(
        selection: &FolderNodeSelection,
        folder_node: &FolderNode,
        inherited_state: LogSelectionState,
        out: &mut Vec<LogId>,
    ) {
        let effective_selection_state = match selection.selection.state {
            LogSelectionState::InheritedOrDefault => inherited_state,
            explicit => explicit,
        };

        if effective_selection_state == LogSelectionState::Selected {
            out.extend(folder_node.files.values().flat_map(|file| file.lines.values().flat_map(|line| &line.logs)));
        }

        for (folder_segment, folder_selection) in &selection.folders {
            let folder_node = folder_node
                .folders
                .get(folder_segment)
                .unwrap_or_else(|| unreachable!("Nested folder {:?} not found", folder_segment));
            Self::collect_logs_from_folder(folder_selection, folder_node, effective_selection_state, out);
        }

        for (file_segment, file_selection) in &selection.files {
            let file_node = folder_node
                .files
                .get(file_segment)
                .unwrap_or_else(|| unreachable!("File {:?} not found", file_segment));
            Self::collect_logs_from_file(file_selection, file_node, effective_selection_state, out);
        }
    }

    fn collect_logs_from_file(
        selection: &FileNodeSelection,
        file_node: &FileNode,
        inherited_state: LogSelectionState,
        out: &mut Vec<LogId>,
    ) {
        let effective_selection_state = match selection.selection.state {
            LogSelectionState::InheritedOrDefault => inherited_state,
            explicit => explicit,
        };

        if effective_selection_state == LogSelectionState::Selected {
            out.extend(file_node.lines
                .values()
                .into_iter()
                .flat_map(|line| &line.logs)
            );
        }

        for (line_segment, line_selection) in &selection.lines {
            let line_node = file_node
                .lines
                .get(line_segment)
                .unwrap_or_else(|| unreachable!("Line {:?} not found", line_segment));
            Self::collect_logs_from_line(line_selection, line_node, effective_selection_state, out);
        }
    }

    fn collect_logs_from_line(
        selection: &LineNodeSelection,
        line_node: &LineNode,
        inherited_state: LogSelectionState,
        out: &mut Vec<LogId>,
    ) {
        let effective_selection_state = match selection.selection.state {
            LogSelectionState::InheritedOrDefault => inherited_state,
            explicit => explicit,
        };

        if effective_selection_state == LogSelectionState::Selected {
            out.extend(&line_node.logs);
        }
    }

    pub fn get_crate_folder(&self, path: &PhysicalSelectionPath) -> Option<&CrateFolderNodeSelection> {
        self.crates.get(&path.crate_folder)
    }
    fn get_crate_folder_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut CrateFolderNodeSelection> {
        self.crates.get_mut(&path.crate_folder)
    }
    
    pub fn get_folder(&self, path: &PhysicalSelectionPath) -> Option<&FolderNodeSelection> {
        let crate_folder = self.crates.get(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get(&path.folders[0])?;

        for segment in path.folders.get(1..).unwrap_or_default() {
            folder = folder.folders.get(segment)?;
        }

        Some(folder)
    }
    fn get_folder_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut FolderNodeSelection> {
        let crate_folder = self.crates.get_mut(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get_mut(&path.folders[0])?;

        for segment in path.folders.get(1..).unwrap_or_default() {
            folder = folder.folders.get_mut(segment)?;
        }

        Some(folder)
    }
    
    pub fn get_file(&self, path: &PhysicalSelectionPath) -> Option<&FileNodeSelection> {
        let crate_folder = self.crates.get(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get(&path.folders[0])?;

        for segment in path.folders.get(1..).unwrap_or_default() {
            folder = folder.folders.get(segment)?;
        }

        let file = folder.files.get(&path.file.clone()?)?;

        Some(file)
    }
    fn get_file_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut FileNodeSelection> {
        let crate_folder = self.crates.get_mut(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get_mut(&path.folders[0])?;

        for segment in path.folders.get(1..).unwrap_or_default() {
            folder = folder.folders.get_mut(segment)?;
        }

        let file = folder.files.get_mut(&path.file.clone()?)?;

        Some(file)
    }
    
    pub fn get_line(&self, path: &PhysicalSelectionPath) -> Option<&LineNodeSelection> {
        let crate_folder = self.crates.get(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get(&path.folders[0])?;

        for segment in path.folders.get(1..).unwrap_or_default() {
            folder = folder.folders.get(segment)?;
        }

        let file = folder.files.get(&path.file.clone()?)?;

        let line = file.lines.get(&path.line.clone()?)?;

        Some(line)
    }
    fn get_line_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut LineNodeSelection> {
        let crate_folder = self.crates.get_mut(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get_mut(&path.folders[0])?;

        for segment in path.folders.get(1..).unwrap_or_default() {
            folder = folder.folders.get_mut(segment)?;
        }

        let file = folder.files.get_mut(&path.file.clone()?)?;

        let line = file.lines.get_mut(&path.line.clone()?)?;

        Some(line)
    }
}

// === PathSelection Types ===

// --- Span ---

pub struct SpanNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
    pub span_children: HashMap<SpanSegment, SpanNodeSelection>,
}
impl SpanNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState, recursive: bool) -> Result<(), LogSelectionCommandError> {
        LogSelectionCommand::run(
            &mut self.selection.privilege, 
            &mut self.selection.state, 
            required_privilege, 
            requested_selection_state
        )?;

        if recursive {
            for child in self.span_children.values_mut() {
                child.select(required_privilege, requested_selection_state, true)?;
            }
        }

        Ok(())
    }
}

// --- Module ---

pub struct CrateModuleNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
    pub modules: HashMap<ModuleSegment, ModuleNodeSelection>,
}
impl CrateModuleNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState, recursive: bool) -> Result<(), LogSelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        if recursive {
            for module in self.modules.values_mut() {
                module.select(required_privilege, requested_selection_state, true)?;
            }
        }

        Ok(())
    }
}

pub struct ModuleNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
    pub modules: HashMap<ModuleSegment, ModuleNodeSelection>,
    pub sub_modules: HashMap<SubModuleSegment, SubModuleNodeSelection>,
}
impl ModuleNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState, recursive: bool) -> Result<(), LogSelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        if recursive {
            for module in self.modules.values_mut() {
                module.select(required_privilege, requested_selection_state, true)?;
            }

            for sub_module in self.sub_modules.values_mut() {
                sub_module.select(required_privilege, requested_selection_state, true)?;
            }
        }

        Ok(())
    }
}

pub struct SubModuleNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
    pub sub_modules: HashMap<SubModuleSegment, SubModuleNodeSelection>,
}
impl SubModuleNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState, recursive: bool) -> Result<(), LogSelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        if recursive {
            for sub_module in self.sub_modules.values_mut() {
                sub_module.select(required_privilege, requested_selection_state, true)?;
            }
        }

        Ok(())
    }
}

// --- Physical ---

pub struct CrateFolderNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
    pub folders: HashMap<FolderSegment, FolderNodeSelection>,
    pub files: HashMap<FileSegment, FileNodeSelection>,
}
impl CrateFolderNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState, recursive: bool) -> Result<(), LogSelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        if recursive {
            for folder in self.folders.values_mut() {
                folder.select(required_privilege, requested_selection_state, true)?;
            }

            for file in self.files.values_mut() {
                file.select(required_privilege, requested_selection_state, true)?;
            }
        }

        Ok(())
    }
}

pub struct FolderNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
    pub folders: HashMap<FolderSegment, FolderNodeSelection>,
    pub files: HashMap<FileSegment, FileNodeSelection>,
}
impl FolderNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState, recursive: bool) -> Result<(), LogSelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        if recursive {
            for folder in self.folders.values_mut() {
                folder.select(required_privilege, requested_selection_state, true)?;
            }

            for file in self.files.values_mut() {
                file.select(required_privilege, requested_selection_state, true)?;
            }
        }

        Ok(())
    }
}

pub struct FileNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
    pub lines: HashMap<LineSegment, LineNodeSelection>,
}
impl FileNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState, recursive: bool) -> Result<(), LogSelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        if recursive {
            for line in self.lines.values_mut() {
                line.select(required_privilege, requested_selection_state)?;
            }
        }

        Ok(())
    }
}

pub struct LineNodeSelection {
    pub selection: LogSelection,
    pub metadata: NodeMetadata,
}
impl LineNodeSelection {
    pub fn select(&mut self, required_privilege: LogSelectionPrivilege, requested_selection_state: LogSelectionState) -> Result<(), LogSelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(LogSelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(LogSelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        Ok(())
    }
}
