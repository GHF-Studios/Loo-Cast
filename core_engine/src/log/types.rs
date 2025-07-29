use std::sync::atomic::{AtomicU64, Ordering};
use std::collections::HashMap;
use std::sync::Arc;
use tracing::{Level as TracingLevel, Metadata};

use crate::log::resources::LogRegistry;
use crate::log::statics::LOG_ID_COUNTER;
use crate::log::ui::types::SelectionMode;
use crate::ui::custom_egui_widgets::tri_checkbox::TriState;























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
    pub metadata: &'static Metadata<'static>
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

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExplicitSelectionState {
    #[default]
    Selected,
    Deselected,
}
impl ExplicitSelectionState {
    pub fn consolidate(self, is_partial: bool) -> EffectiveSelectionState {
        if is_partial {
            EffectiveSelectionState::PartiallySelected
        } else {
            match self {
                ExplicitSelectionState::Selected => EffectiveSelectionState::Selected,
                ExplicitSelectionState::Deselected => EffectiveSelectionState::Deselected
            }
        }
    }

    pub fn is_selected(&self) -> bool {
        matches!(self, ExplicitSelectionState::Selected)
    }

    pub fn is_deselected(&self) -> bool {
        matches!(self, ExplicitSelectionState::Deselected)
    }
}

#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum EffectiveSelectionState {
    #[default]
    Selected,
    Deselected,
    PartiallySelected
}
impl From<EffectiveSelectionState> for TriState {
    fn from(value: EffectiveSelectionState) -> Self {
        match value {
            EffectiveSelectionState::Selected => TriState::Checked,
            EffectiveSelectionState::PartiallySelected => TriState::Indeterminate,
            EffectiveSelectionState::Deselected => TriState::Unchecked,
        }
    }
}

#[derive(Default)]
pub struct NodeMetadata {
    pub explicit_selection_state: ExplicitSelectionState,
}

// === Path Types ===

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanPath {
    pub spans: Vec<SpanSegment>
}
impl std::fmt::Display for SpanPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanPath({})", self.spans.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"))
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModulePath {
    pub crate_module: CrateModuleSegment,
    pub modules: Vec<ModuleSegment>,
    pub sub_modules: Vec<SubModuleSegment>
}
impl std::fmt::Display for ModulePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (self.modules.is_empty(), self.sub_modules.is_empty()) {
            (false, false) => {
                write!(f, "ModulePath({}/{}/{})",
                    self.crate_module.name,
                    self.modules.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
                    self.sub_modules.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
                )
            },
            (false, true) => {
                write!(f, "ModulePath({}/{})",
                    self.crate_module.name,
                    self.modules.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
                )
            },
            (true, false) => {
                unreachable!()
            },
            (true, true) => {
                write!(f, "ModulePath({})",
                    self.crate_module.name,
                )
            },
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalStoragePath {
    pub crate_folder: CrateFolderSegment,
    pub folders: Vec<FolderSegment>,
    pub file: FileSegment,
    pub line: LineSegment,
}
impl std::fmt::Display for PhysicalStoragePath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.folders.is_empty() {
            write!(f, "PhysicalStoragePath({}/{}:{})", 
                self.crate_folder.name,
                self.file.name,
                self.line.number
            )
        } else {
            write!(f, "PhysicalStoragePath({}/{}/{}:{})", 
                self.crate_folder.name,
                self.folders.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
                self.file.name,
                self.line.number
            )
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PhysicalSelectionPath {
    pub crate_folder: CrateFolderSegment,
    pub folders: Vec<FolderSegment>,
    pub file: Option<FileSegment>,
    pub line: Option<LineSegment>,
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
impl Default for SpanSegment {
    fn default() -> Self {
        Self { name: "[UNKNOWN]".to_string() }
    }
}

// --- Module ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateModuleSegment {
    pub name: String,
}
impl Default for CrateModuleSegment {
    fn default() -> Self {
        Self { name: "[UNKNOWN]".to_string() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ModuleSegment {
    pub name: String,
}
impl Default for ModuleSegment {
    fn default() -> Self {
        Self { name: "[UNKNOWN]".to_string() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SubModuleSegment {
    pub name: String,
}
impl Default for SubModuleSegment {
    fn default() -> Self {
        Self { name: "[UNKNOWN]".to_string() }
    }
}

// --- Physical ---

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrateFolderSegment {
    pub name: String,
}
impl Default for CrateFolderSegment {
    fn default() -> Self {
        Self { name: "[UNKNOWN]".to_string() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FolderSegment {
    pub name: String,
}
impl Default for FolderSegment {
    fn default() -> Self {
        Self { name: "[UNKNOWN]".to_string() }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FileSegment {
    pub name: String,
}
impl Default for FileSegment {
    fn default() -> Self {
        Self { name: "[UNKNOWN]".to_string() }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
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
        let root_span_segment = if path.spans.is_empty() {
            SpanSegment::default()
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
        let root_span_segment = if path.spans.is_empty() {
            SpanSegment::default()
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
        let crate_module_segment = if path.crate_module.name.is_empty() {
            CrateModuleSegment::default()
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
        let crate_module_segment = if path.crate_module.name.is_empty() {
            CrateModuleSegment::default()
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
        let file = {
            let crate_folder_segment = if path.crate_folder.name.is_empty() {
                CrateFolderSegment::default()
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
        let file = {
            let crate_folder_segment = if path.crate_folder.name.is_empty() {
                CrateFolderSegment::default()
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
    pub fn insert(&mut self, path: &SpanPath) {
        let root_span_segment = if path.spans.is_empty() {
            SpanSegment::default()
        } else {
            path.spans[0].clone()
        };
        let mut current = self.span_roots.entry(root_span_segment).or_default();

        for segment in path.spans.get(1..).unwrap_or_default() {
            current = current.span_children.entry(segment.clone()).or_default();
        }
    }

    pub fn collect_logs(&self, registry: &LogRegistry) -> Vec<LogId> {
        let mut out = Vec::new();

        for (root_segment, root_node_selection) in &self.span_roots {
            let root_node = registry
                .span_registry
                .span_roots
                .get(root_segment)
                .unwrap_or_else(|| unreachable!("Selection path root {:?} not found in registry", root_segment));

            Self::collect_logs_from_span(
                root_node_selection,
                root_node,
                &mut out,
            );
        }

        out
    }

    fn collect_logs_from_span(
        selection: &SpanNodeSelection,
        parent_node: &SpanNode,
        out: &mut Vec<LogId>,
    ) {
        if selection.metadata.explicit_selection_state == ExplicitSelectionState::Selected {
            out.extend(&parent_node.logs);
        }

        for (child_segment, child_selection) in &selection.span_children {
            let child_node = parent_node
                .span_children
                .get(child_segment)
                .unwrap_or_else(|| unreachable!("Child segment {:?} not found in registry", child_segment));

            Self::collect_logs_from_span(child_selection, child_node, out);
        }
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
    pub fn insert(&mut self, path: &ModulePath) {
        let crate_module_segment = if path.crate_module.name.is_empty() {
            CrateModuleSegment::default()
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

    pub fn collect_logs(&self, registry: &LogRegistry) -> Vec<LogId> {
        let mut out = Vec::new();

        for (crate_segment, crate_node_selection) in &self.crates {
            let crate_node = registry
                .module_registry
                .crates
                .get(crate_segment)
                .unwrap_or_else(|| unreachable!("Crate {:?} not found in module registry", crate_segment));

            if crate_node_selection.metadata.explicit_selection_state == ExplicitSelectionState::Selected {
                out.extend(&crate_node.logs);
            }

            for (module_segment, module_selection) in &crate_node_selection.modules {
                let module_node = crate_node
                    .modules
                    .get(module_segment)
                    .unwrap_or_else(|| unreachable!("Module {:?} not found in registry", module_segment));
                Self::collect_logs_from_module(module_selection, module_node, &mut out);
            }
        }

        out
    }

    fn collect_logs_from_module(
        selection: &ModuleNodeSelection,
        module_node: &ModuleNode,
        out: &mut Vec<LogId>,
    ) {
        if selection.metadata.explicit_selection_state == ExplicitSelectionState::Selected {
            out.extend(&module_node.logs);
        }

        for (module_segment, module_selection) in &selection.modules {
            let module_node = module_node
                .modules
                .get(module_segment)
                .unwrap_or_else(|| unreachable!("Nested module '{}' not found. Full printout: {:?}", module_segment.name, module_node.modules.keys()));
            Self::collect_logs_from_module(module_selection, module_node, out);
        }

        for (sub_module_segment, sub_module_selection) in &selection.sub_modules {
            let sub_module_node = module_node
                .sub_modules
                .get(sub_module_segment)
                .unwrap_or_else(|| unreachable!("SubModule '{}' not found. Full printout: {:?}", sub_module_segment.name, module_node.sub_modules.keys()));
            Self::collect_logs_from_submodule(sub_module_selection, sub_module_node, out);
        }
    }

    fn collect_logs_from_submodule(
        selection: &SubModuleNodeSelection,
        sub_module_node: &SubModuleNode,
        out: &mut Vec<LogId>,
    ) {
        if selection.metadata.explicit_selection_state == ExplicitSelectionState::Selected {
            out.extend(&sub_module_node.logs);
        }

        for (sub_module_segment, sub_module_selection) in &selection.sub_modules {
            let sub_module_node = sub_module_node
                .sub_modules
                .get(sub_module_segment)
                .unwrap_or_else(|| unreachable!("Nested submodule '{}' not found", sub_module_segment.name));
            Self::collect_logs_from_submodule(sub_module_selection, sub_module_node, out);
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
    pub fn insert(&mut self, path: &PhysicalStoragePath) {
        let crate_folder_segment = if path.crate_folder.name.is_empty() {
            CrateFolderSegment::default()
        } else {
            path.crate_folder.clone()
        };
        let crate_folder = self.crates.entry(crate_folder_segment).or_default();

        if path.folders.is_empty() {
            let file = crate_folder.files.entry(path.file.clone()).or_default();
            file.lines.entry(path.line.clone()).or_default();
            return;
        }

        let mut current_folder = crate_folder.folders.entry(path.folders[0].clone()).or_default();

        for segment in path.folders.get(1..).unwrap_or_default() {
            current_folder = current_folder.folders.entry(segment.clone()).or_default();
        }

        let file = current_folder.files.entry(path.file.clone()).or_default();
        file.lines.entry(path.line.clone()).or_default();
    }

    pub fn collect_logs(&self, registry: &LogRegistry) -> Vec<LogId> {
        let mut out = Vec::new();

        for (crate_segment, crate_node_selection) in &self.crates {
            let crate_node = registry
                .physical_registry
                .crates
                .get(crate_segment)
                .unwrap_or_else(|| unreachable!("Crate {:?} not found in physical registry", crate_segment));

            if crate_node_selection.metadata.explicit_selection_state == ExplicitSelectionState::Selected {
                out.extend(crate_node.files.values().flat_map(|file| file.lines.values().flat_map(|line| &line.logs)));
            }

            for (folder_segment, folder_selection) in &crate_node_selection.folders {
                let folder_node = crate_node.folders.get(folder_segment)
                    .unwrap_or_else(|| unreachable!("Folder {:?} not found in registry", folder_segment));
                Self::collect_logs_from_folder(folder_selection, folder_node, &mut out);
            }
        }

        out
    }

    fn collect_logs_from_folder(
        selection: &FolderNodeSelection,
        folder_node: &FolderNode,
        out: &mut Vec<LogId>,
    ) {
        if selection.metadata.explicit_selection_state == ExplicitSelectionState::Selected {
            out.extend(folder_node.files.values().flat_map(|file| file.lines.values().flat_map(|line| &line.logs)));
        }

        for (folder_segment, folder_selection) in &selection.folders {
            let folder_node = folder_node
                .folders
                .get(folder_segment)
                .unwrap_or_else(|| unreachable!("Nested folder '{}' not found", folder_segment.name));
            Self::collect_logs_from_folder(folder_selection, folder_node, out);
        }

        for (file_segment, file_selection) in &selection.files {
            let file_node = folder_node
                .files
                .get(file_segment)
                .unwrap_or_else(|| unreachable!("File '{}' not found", file_segment.name));
            Self::collect_logs_from_file(file_selection, file_node, out);
        }
    }

    fn collect_logs_from_file(
        selection: &FileNodeSelection,
        file_node: &FileNode,
        out: &mut Vec<LogId>,
    ) {
        if selection.metadata.explicit_selection_state == ExplicitSelectionState::Selected {
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
                .unwrap_or_else(|| unreachable!("Line '{}' not found", line_segment.number));
            Self::collect_logs_from_line(line_selection, line_node, out);
        }
    }

    fn collect_logs_from_line(
        selection: &LineNodeSelection,
        line_node: &LineNode,
        out: &mut Vec<LogId>,
    ) {
        let effective_selection_state = selection.metadata.explicit_selection_state;

        if effective_selection_state == ExplicitSelectionState::Selected {
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

#[derive(Default)]
pub struct SpanNodeSelection {
    pub metadata: NodeMetadata,
    pub span_children: HashMap<SpanSegment, SpanNodeSelection>,
}
impl SpanNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;

                for child_span_sel in self.span_children.values_mut() {
                    child_span_sel.deselect()
                }
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;

                for child_span_sel in self.span_children.values_mut() {
                    child_span_sel.select()
                }
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub fn is_partial(&self) -> bool {
        let mut has_selected_children = false;
        let mut has_deselected_children = false;

        for child_span_sel in self.span_children.values() {
            if child_span_sel.is_partial() {
                return true;
            }

            if child_span_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        false
    }
}

// --- Module ---

#[derive(Default)]
pub struct CrateModuleNodeSelection {
    pub metadata: NodeMetadata,
    pub modules: HashMap<ModuleSegment, ModuleNodeSelection>,
}
impl CrateModuleNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;

                for module_sel in self.modules.values_mut() {
                    module_sel.deselect();
                }
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;

                for module_sel in self.modules.values_mut() {
                    module_sel.select();
                }
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub fn is_partial(&self) -> bool {
        let mut has_selected_children = false;
        let mut has_deselected_children = false;

        for module_sel in self.modules.values() {
            if module_sel.is_partial() {
                return true;
            }

            if module_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        false
    }
}

#[derive(Default)]
pub struct ModuleNodeSelection {
    pub metadata: NodeMetadata,
    pub modules: HashMap<ModuleSegment, ModuleNodeSelection>,
    pub sub_modules: HashMap<SubModuleSegment, SubModuleNodeSelection>,
}
impl ModuleNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;

                for module_sel in self.modules.values_mut() {
                    module_sel.deselect();
                }
                for sub_module_sel in self.sub_modules.values_mut() {
                    sub_module_sel.deselect();
                }
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;

                for module_sel in self.modules.values_mut() {
                    module_sel.select();
                }
                for sub_module_sel in self.sub_modules.values_mut() {
                    sub_module_sel.select();
                }
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub fn is_partial(&self) -> bool {
        let mut has_selected_children = false;
        let mut has_deselected_children = false;

        for module_sel in self.modules.values() {
            if module_sel.is_partial() {
                return true;
            }

            if module_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        for sub_module_sel in self.sub_modules.values() {
            if sub_module_sel.is_partial() {
                return true;
            }

            if sub_module_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        false
    }
}

#[derive(Default)]
pub struct SubModuleNodeSelection {
    pub metadata: NodeMetadata,
    pub sub_modules: HashMap<SubModuleSegment, SubModuleNodeSelection>,
}
impl SubModuleNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;

                for sub_module_sel in self.sub_modules.values_mut() {
                    sub_module_sel.deselect();
                }
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;

                for sub_module_sel in self.sub_modules.values_mut() {
                    sub_module_sel.select();
                }
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub fn is_partial(&self) -> bool {
        let mut has_selected_children = false;
        let mut has_deselected_children = false;

        for sub_module_sel in self.sub_modules.values() {
            if sub_module_sel.is_partial() {
                return true;
            }

            if sub_module_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        false
    }
}

// --- Physical ---

#[derive(Default)]
pub struct CrateFolderNodeSelection {
    pub metadata: NodeMetadata,
    pub folders: HashMap<FolderSegment, FolderNodeSelection>,
    pub files: HashMap<FileSegment, FileNodeSelection>,
}
impl CrateFolderNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;

                for folder_sel in self.folders.values_mut() {
                    folder_sel.deselect();
                }
                for file_sel in self.files.values_mut() {
                    file_sel.deselect();
                }
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;

                for folder_sel in self.folders.values_mut() {
                    folder_sel.select();
                }
                for file_sel in self.files.values_mut() {
                    file_sel.select();
                }
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub fn is_partial(&self) -> bool {
        let mut has_selected_children = false;
        let mut has_deselected_children = false;

        for folder_sel in self.folders.values() {
            if folder_sel.is_partial() {
                return true;
            }

            if folder_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        for file_sel in self.files.values() {
            if file_sel.is_partial() {
                return true;
            }

            if file_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        false
    }
}

#[derive(Default)]
pub struct FolderNodeSelection {
    pub metadata: NodeMetadata,
    pub folders: HashMap<FolderSegment, FolderNodeSelection>,
    pub files: HashMap<FileSegment, FileNodeSelection>,
}
impl FolderNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;

                for folder_sel in self.folders.values_mut() {
                    folder_sel.deselect();
                }
                for file_sel in self.files.values_mut() {
                    file_sel.deselect();
                }
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;

                for folder_sel in self.folders.values_mut() {
                    folder_sel.select();
                }
                for file_sel in self.files.values_mut() {
                    file_sel.select();
                }
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub fn is_partial(&self) -> bool {
        let mut has_selected_children = false;
        let mut has_deselected_children = false;

        for folder_sel in self.folders.values() {
            if folder_sel.is_partial() {
                return true;
            }

            if folder_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        for file_sel in self.files.values() {
            if file_sel.is_partial() {
                return true;
            }

            if file_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        false
    }
}

#[derive(Default)]
pub struct FileNodeSelection {
    pub metadata: NodeMetadata,
    pub lines: HashMap<LineSegment, LineNodeSelection>,
}
impl FileNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;

                for line_sel in self.lines.values_mut() {
                    line_sel.deselect();
                }
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;

                for line_sel in self.lines.values_mut() {
                    line_sel.select();
                }
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub fn is_partial(&self) -> bool {
        let mut has_selected_children = false;
        let mut has_deselected_children = false;

        for line_sel in self.lines.values() {
            if line_sel.metadata.explicit_selection_state.is_selected() {
                has_selected_children = true;
            } else {
                has_deselected_children = true;
            }

            if has_selected_children && has_deselected_children {
                return true;
            }
        }

        false
    }
}

#[derive(Default)]
pub struct LineNodeSelection {
    pub metadata: NodeMetadata,
}
impl LineNodeSelection {
    pub fn toggle_selection(&mut self) {
        match self.metadata.explicit_selection_state {
            ExplicitSelectionState::Selected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
            },
            ExplicitSelectionState::Deselected => {
                self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
            },
        };
    }

    pub fn select(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Selected;
    }

    pub fn deselect(&mut self) {
        self.metadata.explicit_selection_state = ExplicitSelectionState::Deselected;
    }

    pub const fn is_partial(&self) -> bool {
        false
    }
}
