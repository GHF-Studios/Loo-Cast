use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::log::arena::{Level};
use crate::log_NEW::traits::PathSegment;

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

pub struct LogRegistry {
    pub logs: HashMap<LogId, LogEntry>,
    pub span_index: SpanPathIndex,
    pub module_index: ModulePathIndex,
    pub physical_index: PhysicalPathIndex,
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
    pub _crate_: ModuleCratePathSegment,        // First third of the full path
    pub modules: Vec<ModulePathSegment>,        // Second third of the full path
    pub sub_modules: Vec<SubModulePathSegment>  // Third third of the full path
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
    pub crate_name: PhysicalCratePathSegment,       // First fourth of the full path
    pub folders: Vec<FolderPathSegment>,    // Second fourth of the full path
    pub file: FilePathSegment,              // Third fourth of the full path
    pub leaf: LinePathSegment,                      // Fourth fourth of the full path
}
impl std::fmt::Display for PhysicalPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "PhysicalPath({}/{}/{}:{})", 
            self.crate_name.name,
            self.folders.iter().map(|s| s.name.as_str()).collect::<Vec<_>>().join("/"),
            self.file.name,
            self.leaf.number
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






pub struct LogStore {
    pub logs: HashMap<LogId, LogEntry>,
    pub span_index: SpanPathIndex,
    pub module_index: ModulePathIndex,
    pub physical_index: PhysicalPathIndex,
}
impl LogStore {
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







































/*
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum CratePathMode {
    Module,
    File,
}
impl std::fmt::Display for CratePathMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CratePathMode::Module => write!(f, "CratePathMode::Module"),
            CratePathMode::File => write!(f, "CratePathMode::File"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModulePathMode {
    Module,
    File
}
impl std::fmt::Display for ModulePathMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModulePathMode::Module => write!(f, "ModulePathMode::Module"),
            ModulePathMode::File => write!(f, "ModulePathMode::File"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FilePathMode {
    Line,
    SubModule
}
impl std::fmt::Display for FilePathMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FilePathMode::Line => write!(f, "FilePathMode::Line"),
            FilePathMode::SubModule => write!(f, "FilePathMode::SubModule"),
        }
    }
}







#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct SpanPath(Vec<SpanPathSegment>);
impl std::fmt::Display for SpanPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SpanPath({})", self.0.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" > "))
    }
}
impl SpanPath {
    pub fn new(segments: Vec<SpanPathSegment>) -> Self {
        Self(segments)
    }

    pub fn get(&self) -> &[SpanPathSegment] {
        &self.0
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct LocationPath(Vec<LocationPathSegment>);
impl std::fmt::Display for LocationPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LocationPath({})", self.0.iter().map(|s| s.to_string()).collect::<Vec<_>>().join(" > "))
    }
}
impl LocationPath {
    /// Creates a new LocationPath and verifies it's validty,
    /// A.K.A whether the path follows these rules:
    /// A.: `Root -> Crates`
    /// B.: `Crate -> Files/Modules`
    /// C.: `Module -> Modules/Files`
    /// D.: `File -> Lines/SubModules`
    /// E.: `Line -> Logs`
    /// F.: `SubModule -> SubModules/Logs`
    /// Panics if any one or more of these rules are broken
    pub fn new(segments: Vec<LocationPathSegment>) -> Option<Self> {
        if segments.is_empty() {
            return None;
        }

        let mut current_segment = &segments[0];

        // Check if the first segment is a crate
        if !matches!(current_segment, LocationPathSegment::Crate(_)) {
            return None; // The first segment must be a crate
        }
        // Iterate through the segments to validate the path structure
        for segment in &segments[1..] {
            use LocationPathSegment::*;

            let next_segment = segment;

            match (current_segment, next_segment) {
                (Crate(_), Module(_)) | (Crate(_), File(_)) => {
                    // Crates can have modules and/or files
                }
                (Module(_), Module(_)) | (Module(_), File(_)) => {
                    // Modules can have modules and/or files
                }
                (File(_), Line(_)) | (File(_), SubModule(_)) => {
                    // Files can have lines and/or submodules
                }
                (SubModule(_), SubModule(_)) => {
                    // Submodules can have submodules
                }
                (Line(line), next) => {
                    // Lines can only have logs, which are not a part of paths
                    panic!("Invalid path segment type transitions from Line to anywhere, because Lines are Leafs.");
                    return None;
                }
                (current, next) => {
                    // Fallback case for invalid transitions
                    panic!("Invalid path segment type transitions from {:?} to {:?}. Valid transitions are: 
                        Crate -> Module/File, Module -> Module/File, File -> Line/SubModule, SubModule -> SubModule
                        where only lines and submodules are direct log storages.",
                        current, next);
                    return None;
                }
            }

            current_segment = segment;
        }

        Some(Self(segments))
    }

    pub fn get(&self) -> &[LocationPathSegment] {
        &self.0
    }

    // Gets different slices that cover the path 100% without overlap or leftovers: For the Crate, the File or Set of nested Modules inside it, the file within the last module of that module chain, and the Line or Set of nested SubModules inside it.
    pub fn splice(self) -> &[&[LocationPathSegment]] {
        let mut segments = self.0.as_slice();
        let mut result = Vec::new();
    }

    /// Traverses both paths and determines whether both paths use the same set of mutually exclusive modes.
    /// For now only 3 sets of mutually exclusive modes exists: 
    /// 1. Inside a full Path, a `Crate` may only contain either: a `Module`, or a `File`.
    /// 2. Inside a full Path, a `Module` may only contain either: a `Module`, or a `File`.
    /// 3. Inside a full Path, a `File` may only contain either: a `SubModules`, or a `Line`.
    pub fn eq_modes(&self, other: &Self) -> bool {
        // Different lengths DONT matter! Only the effective mode cannot be equal
        // We need to collapse each path(aka a collection of segments, many of which have the same type, aka a module in a module in a module, but we just wanna know whether we branch from a crate directly to a file, or continue with modules, and whether we branch from a file to a specific line, or continue with submodules) into a semantic MVR (Minimum Viable Representation, basically just some collection of the same path, but semantically collapsed), and then we can easily match over those two semantic modes

        let mut self_crate_mode = None;
        let mut other_crate_mode = None;

        let self_iter = self.0.iter();
        let other_iter = other.0.iter();

        let iter = self_iter.zip(other_iter);
        
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum LocationPathMode {
    FileLine,
    FileSubModule,
    ModuleFileLine,
    ModuleFileSubModule,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum SlicedLocationPath {
    FileLine(pub Vec<LocationPathSegment>),
    FileSubModule(pub Vec<LocationPathSegment>),
    ModuleFileLine(pub Vec<LocationPathSegment>),
    ModuleFileSubModule(pub Vec<LocationPathSegment>),
}
impl SlicedLocationPath {
    pub fn new(full_location_path: LocationPath) -> Self {
        let segments = full_location_path.get();
        if segments.is_empty() {
            panic!("Cannot create SlicedLocationPath from an empty LocationPath");
        }

        match segments.last().unwrap() {
            LocationPathSegment::Line(_) => Self::FileLine(segments.to_vec()),
            LocationPathSegment::SubModule(_) => Self::FileSubModule(segments.to_vec()),
            LocationPathSegment::Module(_) => {
                if segments.len() > 1 && matches!(segments[segments.len() - 2], LocationPathSegment::File(_)) {
                    Self::ModuleFileLine(segments.to_vec())
                } else {
                    Self::ModuleFileSubModule(segments.to_vec())
                }
            }
            _ => panic!("Invalid last segment in LocationPath for slicing"),
        }
    }

    pub fn mode(&self) -> LocationPathMode {
        match self {
            Self::FileLine(_) => LocationPathMode::FileLine,
            Self::FileSubModule(_) => LocationPathMode::FileSubModule,
            Self::ModuleFileLine(_) => LocationPathMode::ModuleFileLine,
            Self::ModuleFileSubModule(_) => LocationPathMode::ModuleFileSubModule,
        }
    }
}














#[derive(Default)]
pub struct LogStorage {
    log_id_counter: std::sync::atomic::AtomicU64,
    logs_map: DashMap<LogId, LogEntry>,
    span_tree: SpanStorage,
    location_tree: LocationStorage,
}
impl LogStorage {
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a given span into the span tree, recursively creating any necessary parent spans.
    pub fn insert_span(&self, span_path: SpanPath) {
        let mut current = self.span_tree.spans.write().unwrap();
        for segment in span_path {
            current = current.entry(segment).or_insert_with(|| Arc::new(RwLock::new(SpanStorage::default()))).write().unwrap();
        }
    }

    /// Inserts a given location into the location tree, recursively creating any necessary parent locations.
    pub fn insert_location(&self, location_path: LocationPath) {
        let mut current = self.location_tree.crates.write().unwrap();
        for segment in location_path {
            match segment {
                LocationPathSegment::Crate(name) => {
                    current = current.entry(name).or_insert_with(|| Arc::new(RwLock::new(CrateStorage::default()))).write().unwrap();
                }
                LocationPathSegment::Module(name) => {
                    let modules = current.modules.write().unwrap();
                    current = modules.entry(name).or_insert_with(|| Arc::new(RwLock::new(ModuleStorage::default()))).write().unwrap();
                }
                LocationPathSegment::File(name) => {
                    let files = current.files.write().unwrap();
                    current = files.entry(name).or_insert_with(|| Arc::new(RwLock::new(FileStorage::default()))).write().unwrap();
                }
                LocationPathSegment::Line(line) => {
                    let lines = current.lines.write().unwrap();
                    current = lines.entry(line).or_insert_with(|| Arc::new(RwLock::new(LineStorage::default()))).write().unwrap();
                }
                LocationPathSegment::SubModule(name) => {
                    let submodules = current.submodules.write().unwrap();
                    current = submodules.entry(name).or_insert_with(|| Arc::new(RwLock::new(SubModuleStorage::default()))).write().unwrap();
                }
            }
        }
    }

    /// Inserts a log entry into the storage, associating it with a span path and/or location path.
    pub fn insert_log(
        &self,
        associated_span_path: Option<Vec<String>>,
        associated_location_path: Option<Vec<LocationPathSegment>>,
        lvl: Level,
        msg: Arc<str>
    ) -> LogId {
        // Create a new log entry and insert it into the logs map
        let ts = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_nanos() as u64;
        let id = LogId(self.log_id_counter.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        let entry = LogEntry { ts, lvl, msg };
        self.logs.insert(id, entry);

        // Insert into span tree if a span path is provided
        if let Some(span_path) = associated_span_path {
            let span_path_segments = span_path.into_iter().map(SpanPathSegment).collect();
            self.insert_span(SpanPath::new(span_path_segments));

        }

        // Insert into location tree if a location path is provided
        if let Some(location_path) = associated_location_path {
            let location_path_segments = location_path.into_iter().collect();
            self.insert_location(LocationPath::new(location_path_segments).unwrap());

        }

        id
    }

    /// Retrieves all log IDs associated with a given span path.
    /// If the span path does not exist, it returns an empty vector.
    /// Relatively simple function, because traversing the span tree is simple, because every level is of the same type, just a boring simple span, which is just a boring simple string without any further semantic rules.
    /// Has to do a BFS to collect all logs associated with multiple span paths in an efficient manner from all spans, as all of them can hold logs. 
    pub fn get_log_ids_by_span_paths(&self, span_paths: Vec<&SpanPath>) -> Vec<Vec<LogId>> {
        let mut all_log_ids = Vec::new();
        for span_path in span_paths {
            let mut current = self.span_tree.spans.read().unwrap();
            let mut my_log_ids = Vec::new();

            // Traverse the span tree according to the path
            for segment in span_path.get_path() {
                if let Some(span) = current.get(segment) {
                    let span_storage = span.read().unwrap();
                    my_log_ids.extend(span_storage.logs.read().unwrap().clone());
                    current = &span_storage.spans;
                } else {
                    my_log_ids.clear(); // If any segment is not found, clear the log IDs
                    eprintln!("Span path segment {:?} not found in the span tree.", segment);
                    break;
                }
            }

            all_log_ids.push(my_log_ids);
        }
        all_log_ids
    }

    /// Retrieves all log IDs associated with a given location path.
    /// If the location path does not exist, it returns an empty vector.
    /// Very complex function, because it has to traverse the location tree, which has multiple types of levels, which can even have mutually exclusive types for their children, aka the children are not all of the same type, aka one path = either one mode or the other (assuming two modes)
    /// Has to do some sort of custom BFS with a multi-modal "let mut current" kinda thing to collect all logs associated with multiple span paths in an efficient manner from all nodes that can possibly holds logs, which are: Lines and SubModules.
    /// To differentiate between the different types of modes the `current` path segment can be in, we use the macros implemented on the path segment types to make `current` a trait-object with the required functionality.
    pub fn get_log_ids_by_location_paths(&self, location_path: &LocationPath) -> Vec<LogId> {}

    pub fn get_log(&self, id: &LogId) -> Option<LogEntry> {
        self.logs.get(id).map(|e| e.clone())
    }
}

#[derive(Default)]
pub struct SpanStorage {
    pub spans: RwLock<HashMap<String, Arc<RwLock<SpanStorage>>>>,
    pub logs: RwLock<Vec<LogId>>,
}

#[derive(Default)]
pub struct LocationStorage {
    pub crates: RwLock<HashMap<String, Arc<RwLock<CrateStorage>>>>,
}

#[derive(Default)]
pub struct CrateStorage {
    pub modules: RwLock<HashMap<String, Arc<RwLock<ModuleStorage>>>>,
    pub files: RwLock<HashMap<String, Arc<RwLock<FileStorage>>>>,
}

#[derive(Default)]
pub struct ModuleStorage {
    pub modules: RwLock<HashMap<String, Arc<RwLock<ModuleStorage>>>>,
    pub files: RwLock<HashMap<String, Arc<RwLock<FileStorage>>>>,
}

#[derive(Default)]
pub struct FileStorage {
    pub lines: RwLock<HashMap<u32, Arc<RwLock<LineStorage>>>>,
    pub submodules: RwLock<HashMap<String, Arc<RwLock<SubModuleStorage>>>>,
}

#[derive(Default)]
pub struct LineStorage {
    pub logs: RwLock<Vec<LogId>>,
}

#[derive(Default)]
pub struct SubModuleStorage {
    pub submodules: RwLock<HashMap<String, Arc<RwLock<SubModuleStorage>>>>,
    pub logs: RwLock<Vec<LogId>>,
}









// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE
// UNCANNY VALLEY STARTS HERE






#[derive(Clone, Debug)]
pub enum TreeRootViewMode {
    Span(Arc<RwLock<SpanTreeView>>),
    Location(Arc<RwLock<LocationTreeView>>),
}

#[derive(Default)]
pub struct SpanTreeView {
    pub spans: RwLock<HashMap<String, Arc<RwLock<SpanTreeView>>>>,
}

#[derive(Default)]
pub struct LocationTreeView {
    pub locations: RwLock<HashMap<String, Arc<RwLock<SpanTreeView>>>>,
}

#[derive(Clone, Debug)]
pub enum SpanViewMode {
    Spans,
    Logs,
}

#[derive(Clone, Debug)]
pub enum CrateViewMode {
    Modules,
    Files,
}

#[derive(Clone, Debug)]
pub enum ModuleViewMode {
    Modules,
    Files,
}

#[derive(Clone, Debug)]
pub enum FileViewMode {
    Lines,
    SubModules,
}

#[derive(Clone, Debug)]
pub enum SubModuleViewMode {
    SubModules,
    Logs,
}

#[derive(Clone, Debug)]
pub enum LineViewMode {
    Logs,
}
*/