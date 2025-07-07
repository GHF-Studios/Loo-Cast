use std::marker::PhantomData;
use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::Level as TracingLevel;

use crate::log_NEW::statics::LOG_ID_COUNTER;

// Basics

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
        physical_path: PhysicalStoragePath,
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

    pub fn resolve_physical_path(&self, path: &PhysicalStoragePath) -> Option<&Vec<LogId>> {
        self.physical_index.resolve(path)
    }
}

// PathSegment Types

// Span
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanPathSegment(pub String);

// Module
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateModulePathSegment {
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

// Physical
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateFolderPathSegment {
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

// Path Types

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanPath(pub Vec<SpanPathSegment>);
impl SpanPath {
    pub const UNCATEGORIZED: Self = SpanPath(Vec::new());
}
impl std::fmt::Display for SpanPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanPath({})", self.0.iter().map(|s| s.0.as_str()).collect::<Vec<_>>().join("/"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePath {
    pub crate_module: CrateModulePathSegment,
    pub modules: Vec<ModulePathSegment>,
    pub sub_modules: Vec<SubModulePathSegment>
}
impl ModulePath {
    pub const UNCATEGORIZED: Self = ModulePath {
        crate_module: CrateModulePathSegment { name: String::new() },
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
    pub crate_folder: CrateFolderPathSegment,
    pub folders: Vec<FolderPathSegment>,
    pub file: FilePathSegment,
    pub line: LinePathSegment,
}
impl PhysicalStoragePath {
    pub const UNCATEGORIZED: Self = PhysicalStoragePath {
        crate_folder: CrateFolderPathSegment { name: String::new() },
        folders: Vec::new(),
        file: FilePathSegment { name: String::new() },
        line: LinePathSegment { number: 0 }
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
    pub crate_folder: CrateFolderPathSegment,
    pub folders: Vec<FolderPathSegment>,
    pub file: Option<FilePathSegment>,
    pub line: Option<LinePathSegment>,
}
impl PhysicalSelectionPath {
    pub const UNCATEGORIZED: Self = PhysicalSelectionPath {
        crate_folder: CrateFolderPathSegment { name: String::new() },
        folders: Vec::new(),
        file: Some(FilePathSegment { name: String::new() }),
        line: Some(LinePathSegment { number: 0 })
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

// SpanPathIndex

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

// ModulePathIndex

#[derive(Default)]
pub struct ModulePathIndex {
    pub crates: HashMap<CrateModulePathSegment, CrateModulePathNode>,
}
impl ModulePathIndex {
    pub fn insert(&mut self, path: &ModulePath, log_id: LogId) {
        let crate_module = self.crates.entry(path.crate_module.clone()).or_default();

        if path.modules.is_empty() {
            crate_module.logs.push(log_id);
            return;
        }

        let mut current_module = crate_module.modules.entry(path.modules[0].clone()).or_default();

        for segment in &path.modules[1..] {
            current_module = crate_module.modules.entry(segment.clone()).or_default();
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
        let crate_module = self.crates.entry(path.crate_module.clone()).or_default();

        if path.modules.is_empty() {
            return;
        }

        let mut current_module = crate_module.modules.entry(path.modules[0].clone()).or_default();

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
        let crate_module = self.crates.get(&path.crate_module)?;

        if path.modules.is_empty() {
            return Some(&crate_module.logs);
        }

        let mut current_module = crate_module.modules.get(&path.modules[0])?;
        
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
pub struct CrateModulePathNode {
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

// PhysicalPathIndex

#[derive(Default)]
pub struct PhysicalPathIndex {
    pub crates: HashMap<CrateFolderPathSegment, PhysicalCratePathNode>,
}
impl PhysicalPathIndex {
    pub fn insert(&mut self, path: &PhysicalStoragePath, log_id: LogId) {
        let file = {
            let crate_folder = self.crates.entry(path.crate_folder.clone()).or_default();

            if path.folders.is_empty() {
                crate_folder.files.entry(path.file.clone()).or_default()
            } else {
                let mut current_folder = crate_folder.folders.entry(path.folders[0].clone()).or_default();

                for segment in &path.folders[1..] {
                    current_folder = crate_folder.folders.entry(segment.clone()).or_default();
                }

                current_folder.files.entry(path.file.clone()).or_default()
            }
        };

        let line = file.lines.entry(path.line.clone()).or_default();

        line.logs.push(log_id);
    }

    pub fn insert_without_log(&mut self, path: &PhysicalStoragePath) {
        let file = {
            let crate_folder = self.crates.entry(path.crate_folder.clone()).or_default();

            if path.folders.is_empty() {
                crate_folder.files.entry(path.file.clone()).or_default()
            } else {
                let mut current_folder = crate_folder.folders.entry(path.folders[0].clone()).or_default();

                for segment in &path.folders[1..] {
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

// Selection Basics

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    pub state: SelectionState,
    pub privilege: SelectionPrivilege,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionState {
    #[default]
    InheritedOrDefault,
    Selected,
    Deselected,
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SelectionPrivilege {
    #[default]
    None,
    User,
    Sudo,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelectionCommand {
    ResetToInheritedOrDefault(SelectionPrivilege),
    Select(SelectionPrivilege),
    Deselect(SelectionPrivilege),
    RecursiveResetToInheritedOrDefault(SelectionPrivilege),
    RecursiveSelect(SelectionPrivilege),
    RecursiveDeselect(SelectionPrivilege),
}
impl SelectionCommand {
    pub fn unpack(self) -> (SelectionState, SelectionPrivilege, bool) {
        match self {
            SelectionCommand::ResetToInheritedOrDefault(required) => {
                (SelectionState::InheritedOrDefault, required, false)
            }
            SelectionCommand::Select(required) => {
                (SelectionState::Selected, required, false)
            }
            SelectionCommand::Deselect(required) => {
                (SelectionState::Deselected, required, false)
            }
            SelectionCommand::RecursiveResetToInheritedOrDefault(required) => {
                (SelectionState::InheritedOrDefault, required, true)
            }
            SelectionCommand::RecursiveSelect(required) => {
                (SelectionState::Selected, required, true)
            }
            SelectionCommand::RecursiveDeselect(required) => {
                (SelectionState::Deselected, required, true)
            }
        }
    }

    pub fn run(
        current_privilege: &mut SelectionPrivilege, 
        current_selection_state: &mut SelectionState, 
        required_privilege: SelectionPrivilege, 
        requested_selection_state: SelectionState
    ) -> Result<(), SelectionCommandError> {
        if required_privilege > *current_privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: *current_privilege,
            })?
        }
    
        if *current_selection_state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        *current_privilege = required_privilege;
        *current_selection_state = requested_selection_state;

        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelectionCommandError {
    InsufficientPrivilege {
        required: SelectionPrivilege,
        actual: SelectionPrivilege,
    },
    AlreadyAtState(SelectionState),
    SpanPathNotFound(SpanPath),
    ModulePathNotFound(ModulePath),
    PhysicalPathNotFound(PhysicalSelectionPath),
}

// SpanPathSelection

#[derive(Default)]
pub struct SpanPathSelection {
    pub span_roots: HashMap<SpanPathSegment, SpanPathNodeSelection>,
}
impl SpanPathSelection {
    pub fn select(&mut self, path: &SpanPath, command: SelectionCommand) -> Result<(), SelectionCommandError> {
        let span = self.get_span_mut(path).ok_or(SelectionCommandError::SpanPathNotFound(path.clone()))?;

        let (requested_selection_state, required_privilege, recursive) = command.unpack();
        span.select(required_privilege, requested_selection_state, recursive)
    }

    fn get_span(&self, path: &SpanPath) -> Option<&SpanPathNodeSelection> {
        let mut current = self.span_roots.get(&path.0[0])?;
        
        for segment in &path.0[1..] {
            current = current.span_children.get(segment)?;
        }
        
        Some(current)
    }
    fn get_span_mut(&mut self, path: &SpanPath) -> Option<&mut SpanPathNodeSelection> {
        let mut current = self.span_roots.get_mut(&path.0[0])?;
        
        for segment in &path.0[1..] {
            current = current.span_children.get_mut(segment)?;
        }
        
        Some(current)
    }
}

pub struct SpanPathNodeSelection {
    pub selection: Selection,
    pub span_children: HashMap<SpanPathSegment, SpanPathNodeSelection>,
}
impl SpanPathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState, recursive: bool) -> Result<(), SelectionCommandError> {
        SelectionCommand::run(
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

// ModulePathSelection

#[derive(Default)]
pub struct ModulePathSelection {
    pub crates: HashMap<CrateModulePathSegment, CrateModulePathNodeSelection>,
}
impl ModulePathSelection {
    pub fn select(&mut self, path: &ModulePath, command: SelectionCommand) -> Result<(), SelectionCommandError> {
        match (path.modules.is_empty(), path.sub_modules.is_empty()) {
            (false, false) => {
                let sub_module = self.get_sub_module_mut(path).ok_or(SelectionCommandError::ModulePathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                sub_module.select(required_privilege, requested_selection_state, recursive)?;
            }
            (false, true) => {
                let module = self.get_module_mut(path).ok_or(SelectionCommandError::ModulePathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                module.select(required_privilege, requested_selection_state, recursive)?;
            }
            (true, false) => unreachable!("Invalid path: No module has been selected, yet a sub_module has been selected"),
            (true, true) => {
                let crate_module = self.get_crate_module_mut(path).ok_or(SelectionCommandError::ModulePathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                crate_module.select(required_privilege, requested_selection_state, recursive)?;
            }
        }

        Ok(())
    }

    fn get_crate_module(&self, path: &ModulePath) -> Option<&CrateModulePathNodeSelection> {
        self.crates.get(&path.crate_module)
    }
    fn get_crate_module_mut(&mut self, path: &ModulePath) -> Option<&mut CrateModulePathNodeSelection> {
        self.crates.get_mut(&path.crate_module)
    }

    fn get_module(&self, path: &ModulePath) -> Option<&ModulePathNodeSelection> {
        let crate_module = self.crates.get(&path.crate_module)?;
        let mut module = crate_module.modules.get(&path.modules[0])?;

        for segment in &path.modules[1..] {
            module = module.modules.get(segment)?;
        }

        Some(module)
    }
    fn get_module_mut(&mut self, path: &ModulePath) -> Option<&mut ModulePathNodeSelection> {
        let crate_module = self.crates.get_mut(&path.crate_module)?;
        let mut module = crate_module.modules.get_mut(&path.modules[0])?;

        for segment in &path.modules[1..] {
            module = module.modules.get_mut(segment)?;
        }

        Some(module)
    }

    fn get_sub_module(&self, path: &ModulePath) -> Option<&SubModulePathNodeSelection> {
        let crate_module = self.crates.get(&path.crate_module)?;
        let mut module = crate_module.modules.get(&path.modules[0])?;

        for segment in &path.modules[1..] {
            module = module.modules.get(segment)?;
        }

        let mut sub_module = module.sub_modules.get(&path.sub_modules[0])?;

        for segment in &path.sub_modules[1..] {
            sub_module = sub_module.sub_modules.get(segment)?;
        }

        Some(sub_module)
    }
    fn get_sub_module_mut(&mut self, path: &ModulePath) -> Option<&mut SubModulePathNodeSelection> {
        let crate_module = self.crates.get_mut(&path.crate_module)?;
        let mut module = crate_module.modules.get_mut(&path.modules[0])?;

        for segment in &path.modules[1..] {
            module = module.modules.get_mut(segment)?;
        }

        let mut sub_module = module.sub_modules.get_mut(&path.sub_modules[0])?;

        for segment in &path.sub_modules[1..] {
            sub_module = sub_module.sub_modules.get_mut(segment)?;
        }

        Some(sub_module)
    }
}

pub struct CrateModulePathNodeSelection {
    pub selection: Selection,
    pub modules: HashMap<ModulePathSegment, ModulePathNodeSelection>,
}
impl CrateModulePathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState, recursive: bool) -> Result<(), SelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
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

pub struct ModulePathNodeSelection {
    pub selection: Selection,
    pub modules: HashMap<ModulePathSegment, ModulePathNodeSelection>,
    pub sub_modules: HashMap<SubModulePathSegment, SubModulePathNodeSelection>,
}
impl ModulePathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState, recursive: bool) -> Result<(), SelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
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

pub struct SubModulePathNodeSelection {
    pub selection: Selection,
    pub sub_modules: HashMap<SubModulePathSegment, SubModulePathNodeSelection>,
}
impl SubModulePathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState, recursive: bool) -> Result<(), SelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
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

// PhysicalPathSelection

#[derive(Default)]
pub struct PhysicalPathSelection {
    pub crates: HashMap<CrateFolderPathSegment, CrateFolderPathNodeSelection>,
}
impl PhysicalPathSelection {
    pub fn select(&mut self, path: &PhysicalSelectionPath, command: SelectionCommand) -> Result<(), SelectionCommandError> {
        match (path.folders.is_empty(), path.file.is_none(), path.line.is_none()) {
            (false, false, false) => {
                let line = self.get_line_mut(path).ok_or(SelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, _recursive) = command.unpack();
                
                line.select(required_privilege, requested_selection_state)?;
            }
            (false, false, true) => {
                let file = self.get_file_mut(path).ok_or(SelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                file.select(required_privilege, requested_selection_state, recursive)?;
            }
            (false, true, false) => unreachable!("Invalid path: No file has been, yet a line has been selected"),
            (false, true, true) => {
                let folder = self.get_folder_mut(path).ok_or(SelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();
                
                folder.select(required_privilege, requested_selection_state, recursive)?;
            }
            (true, false, false) => {
                let line = self.get_line_mut(path).ok_or(SelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, _recursive) = command.unpack();

                line.select(required_privilege, requested_selection_state)?;
            }
            (true, false, true) => {
                let file = self.get_file_mut(path).ok_or(SelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();

                file.select(required_privilege, requested_selection_state, recursive)?;
            }
            (true, true, false) => unreachable!("Invalid path: No file has been selected, yet a line has been selected"),
            (true, true, true) => {
                let crate_folder = self.get_crate_folder_mut(path).ok_or(SelectionCommandError::PhysicalPathNotFound(path.clone()))?;
                let (requested_selection_state, required_privilege, recursive) = command.unpack();

                crate_folder.select(required_privilege, requested_selection_state, recursive)?;
            }
        }

        Ok(())
    }

    fn get_crate_folder(&self, path: &PhysicalSelectionPath) -> Option<&CrateFolderPathNodeSelection> {
        self.crates.get(&path.crate_folder)
    }
    fn get_crate_folder_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut CrateFolderPathNodeSelection> {
        self.crates.get_mut(&path.crate_folder)
    }
    
    fn get_folder(&self, path: &PhysicalSelectionPath) -> Option<&FolderPathNodeSelection> {
        let crate_folder = self.crates.get(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get(&path.folders[0])?;

        for segment in &path.folders[1..] {
            folder = folder.folders.get(segment)?;
        }

        Some(folder)
    }
    fn get_folder_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut FolderPathNodeSelection> {
        let crate_folder = self.crates.get_mut(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get_mut(&path.folders[0])?;

        for segment in &path.folders[1..] {
            folder = folder.folders.get_mut(segment)?;
        }

        Some(folder)
    }
    
    fn get_file(&self, path: &PhysicalSelectionPath) -> Option<&FilePathNodeSelection> {
        let crate_folder = self.crates.get(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get(&path.folders[0])?;

        for segment in &path.folders[1..] {
            folder = folder.folders.get(segment)?;
        }

        let file = folder.files.get(&path.file.clone()?)?;

        Some(file)
    }
    fn get_file_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut FilePathNodeSelection> {
        let crate_folder = self.crates.get_mut(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get_mut(&path.folders[0])?;

        for segment in &path.folders[1..] {
            folder = folder.folders.get_mut(segment)?;
        }

        let file = folder.files.get_mut(&path.file.clone()?)?;

        Some(file)
    }
    
    fn get_line(&self, path: &PhysicalSelectionPath) -> Option<&LinePathNodeSelection> {
        let crate_folder = self.crates.get(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get(&path.folders[0])?;

        for segment in &path.folders[1..] {
            folder = folder.folders.get(segment)?;
        }

        let file = folder.files.get(&path.file.clone()?)?;

        let line = file.lines.get(&path.line.clone()?)?;

        Some(line)
    }
    fn get_line_mut(&mut self, path: &PhysicalSelectionPath) -> Option<&mut LinePathNodeSelection> {
        let crate_folder = self.crates.get_mut(&path.crate_folder)?;
        let mut folder = crate_folder.folders.get_mut(&path.folders[0])?;

        for segment in &path.folders[1..] {
            folder = folder.folders.get_mut(segment)?;
        }

        let file = folder.files.get_mut(&path.file.clone()?)?;

        let line = file.lines.get_mut(&path.line.clone()?)?;

        Some(line)
    }
}

pub struct CrateFolderPathNodeSelection {
    pub selection: Selection,
    pub folders: HashMap<FolderPathSegment, FolderPathNodeSelection>,
    pub files: HashMap<FilePathSegment, FilePathNodeSelection>,
}
impl CrateFolderPathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState, recursive: bool) -> Result<(), SelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
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

pub struct FolderPathNodeSelection {
    pub selection: Selection,
    pub folders: HashMap<FolderPathSegment, FolderPathNodeSelection>,
    pub files: HashMap<FilePathSegment, FilePathNodeSelection>,
}
impl FolderPathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState, recursive: bool) -> Result<(), SelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
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

pub struct FilePathNodeSelection {
    pub selection: Selection,
    pub lines: HashMap<LinePathSegment, LinePathNodeSelection>,
}
impl FilePathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState, recursive: bool) -> Result<(), SelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
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

pub struct LinePathNodeSelection {
    pub selection: Selection,
}
impl LinePathNodeSelection {
    pub fn select(&mut self, required_privilege: SelectionPrivilege, requested_selection_state: SelectionState) -> Result<(), SelectionCommandError> {
        if required_privilege > self.selection.privilege {
            Err(SelectionCommandError::InsufficientPrivilege {
                required: required_privilege,
                actual: self.selection.privilege,
            })?
        }

        if self.selection.state == requested_selection_state {
            Err(SelectionCommandError::AlreadyAtState(requested_selection_state))?
        }

        self.selection.privilege = required_privilege;
        self.selection.state = requested_selection_state;

        Ok(())
    }
}
