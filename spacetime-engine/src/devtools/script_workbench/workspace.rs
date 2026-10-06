//! Script workspace, live roots, open tabs, and document operations.

use super::{
    document::{
        DEFAULT_SCALAR_SOURCE, DEFAULT_SCRATCH_PATH, DEFAULT_SCRATCH_SOURCE, FREECAM_SCRIPT_PATH,
        ScriptDocument, ScriptTarget, call_f64,
    },
    host::bounded_engine,
    storage::{ScriptWorkspaceRoots, collect_rhai_sources, prepare_workspace_roots},
};
use bevy::prelude::*;
use rhai::Engine;
use std::collections::BTreeMap;

#[derive(Resource)]
pub(crate) struct DeveloperScriptWorkbench {
    pub(super) engine: Engine,
    pub(super) documents: BTreeMap<String, ScriptDocument>,
    pub(super) open_tabs: Vec<String>,
    pub(super) active_path: String,
    pub(super) roots: ScriptWorkspaceRoots,
    pub(super) bootstrap_warning: Option<String>,
    pub(super) scratch_counter: u32,
}

impl Default for DeveloperScriptWorkbench {
    fn default() -> Self {
        let engine = bounded_engine();
        let bootstrap = prepare_workspace_roots();

        let mut sources = BTreeMap::<String, String>::new();
        collect_rhai_sources(
            &bootstrap.roots.live_root,
            &bootstrap.roots.live_root,
            &mut sources,
        );

        sources
            .entry(FREECAM_SCRIPT_PATH.to_string())
            .or_insert_with(|| DEFAULT_SCALAR_SOURCE.to_string());
        sources
            .entry(DEFAULT_SCRATCH_PATH.to_string())
            .or_insert_with(|| DEFAULT_SCRATCH_SOURCE.to_string());

        let documents = sources
            .into_iter()
            .map(|(path, source)| {
                let document = ScriptDocument::from_source(&engine, path.clone(), source);
                (path, document)
            })
            .collect::<BTreeMap<_, _>>();

        let active_path = if documents.contains_key(FREECAM_SCRIPT_PATH) {
            FREECAM_SCRIPT_PATH.to_string()
        } else {
            documents
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| DEFAULT_SCRATCH_PATH.to_string())
        };

        let mut open_tabs = Vec::new();
        for path in [FREECAM_SCRIPT_PATH, DEFAULT_SCRATCH_PATH] {
            if documents.contains_key(path) {
                open_tabs.push(path.to_string());
            }
        }
        if open_tabs.is_empty() {
            open_tabs.push(active_path.clone());
        }

        Self {
            engine,
            documents,
            open_tabs,
            active_path,
            roots: bootstrap.roots,
            bootstrap_warning: bootstrap.warning,
            scratch_counter: 1,
        }
    }
}

impl DeveloperScriptWorkbench {
    pub(super) fn active(&self) -> Option<&ScriptDocument> {
        self.documents.get(&self.active_path)
    }

    pub(super) fn active_mut(&mut self) -> Option<&mut ScriptDocument> {
        self.documents.get_mut(&self.active_path)
    }

    pub(crate) fn transform_live_scalar(&self, target: ScriptTarget, value: f64) -> f64 {
        let Some(document) = self.document_for_target(target) else {
            return value;
        };
        if !document.live_enabled {
            return value;
        }
        call_f64(&self.engine, &document.committed_ast, "transform", (value,)).unwrap_or(value)
    }

    pub(super) fn document_for_target(&self, target: ScriptTarget) -> Option<&ScriptDocument> {
        self.documents
            .values()
            .find(|document| document.target == target)
    }

    pub(super) fn open_document(&mut self, path: &str) {
        if !self.documents.contains_key(path) {
            return;
        }
        if !self.open_tabs.iter().any(|open| open == path) {
            self.open_tabs.push(path.to_string());
        }
        self.active_path = path.to_string();
    }

    pub(super) fn close_tab(&mut self, path: &str) {
        if self.open_tabs.len() <= 1 {
            return;
        }
        let Some(index) = self.open_tabs.iter().position(|open| open == path) else {
            return;
        };
        self.open_tabs.remove(index);
        if self.active_path == path {
            let next = index.min(self.open_tabs.len().saturating_sub(1));
            self.active_path.clone_from(&self.open_tabs[next]);
        }
    }

    pub(super) fn new_scratch(&mut self) -> String {
        loop {
            let path = format!("scratch/untitled_{}.rhai", self.scratch_counter);
            self.scratch_counter = self.scratch_counter.saturating_add(1);
            if self.documents.contains_key(&path) {
                continue;
            }
            self.documents.insert(
                path.clone(),
                ScriptDocument::from_source(
                    &self.engine,
                    path.clone(),
                    DEFAULT_SCRATCH_SOURCE.to_string(),
                ),
            );
            self.open_document(&path);
            return path;
        }
    }

    pub(super) fn compile_active(&mut self) -> Result<Option<f64>, String> {
        let path = self.active_path.clone();
        let document = self
            .documents
            .get_mut(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        document.compile_candidate(&self.engine)
    }

    pub(super) fn commit_active(&mut self) -> Result<u64, String> {
        let path = self.active_path.clone();
        let document = self
            .documents
            .get_mut(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        document.commit(&self.engine)
    }

    pub(super) fn revert_active_to_committed(&mut self) {
        if let Some(document) = self.active_mut() {
            document.revert_to_committed();
        }
    }

    pub(super) fn reload_active_from_saved(&mut self) {
        let path = self.active_path.clone();
        match self.roots.read_live(&path) {
            Ok((source, location)) => {
                if let Some(document) = self.active_mut() {
                    document.replace_with_saved(source, &location);
                }
            }
            Err(error) => {
                if let Some(document) = self.active_mut() {
                    document.diagnostic = error;
                }
            }
        }
    }

    pub(super) fn save_active(&mut self) -> Result<(), String> {
        let path = self.active_path.clone();
        let document = self
            .documents
            .get(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        let source = document.source.clone();
        let location = self.roots.write_live(&path, &source)?;
        let document = self
            .documents
            .get_mut(&path)
            .expect("active document exists");
        document.saved_source = source;
        document.diagnostic = format!("Saved live copy {}.", location.display());
        Ok(())
    }

    pub(super) fn reset_active_to_default(&mut self) -> Result<(), String> {
        let path = self.active_path.clone();
        let source = self.roots.reset_live_to_default(&path)?;
        let document = self
            .documents
            .get_mut(&path)
            .ok_or_else(|| "no active script document".to_string())?;
        document.source = source.clone();
        document.saved_source = source;
        document.invalidate_candidate();
        document.diagnostic =
            "Reset LIVE file to shipped default. Commit separately to activate it.".to_string();
        Ok(())
    }

    pub(super) fn active_has_default(&self) -> bool {
        self.roots.has_default(&self.active_path)
    }
}
