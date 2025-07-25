use std::collections::HashMap;

use bevy::prelude::*;

use crate::log::{types::*, ui::types::SelectionMode};

#[derive(Resource, Default)]
pub struct LogRegistry {
    pub logs: HashMap<LogId, LogEntry>,
    pub span_registry: SpanRegistry,
    pub module_registry: ModuleRegistry,
    pub physical_registry: PhysicalRegistry,
    pub span_selections: SpanPathSelections,
    pub module_selections: ModulePathSelections,
    pub physical_selections: PhysicalPathSelections,
    pub selection_mode: SelectionMode,
}
impl LogRegistry {
    pub fn insert_without_log(
        &mut self,
        span_path: &SpanPath,
    ) {
        self.span_registry.insert_without_log(span_path);
        self.span_selections.insert(span_path);
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
        self.span_selections.insert(&span_path);
        self.module_selections.insert(&module_path);
        self.physical_selections.insert(&physical_path);
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