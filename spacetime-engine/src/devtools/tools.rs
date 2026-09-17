use std::collections::HashSet;

use bevy::prelude::*;

/// Stable semantic ID for one developer visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct VisualizationId(pub &'static str);

/// One flat palette entry.
///
/// This is intentionally not a generic control node: there is no hierarchy,
/// dependency graph, condition language, or arbitrary value payload. Domains
/// register only visualizations that currently earn a place in the palette.
#[derive(Debug, Clone, Copy)]
pub struct VisualizationSpec {
    pub id: VisualizationId,
    pub label: &'static str,
    pub order: i32,
    pub default_enabled: bool,
}

impl VisualizationSpec {
    pub const fn new(
        id: VisualizationId,
        label: &'static str,
        order: i32,
        default_enabled: bool,
    ) -> Self {
        Self {
            id,
            label,
            order,
            default_enabled,
        }
    }
}

/// Deliberately small replacement for the generic debug-control graph.
///
/// Visualization-specific settings do not live here. A visualization that
/// genuinely needs configuration owns a typed settings resource in its domain.
#[derive(Resource, Debug)]
pub struct DeveloperTools {
    enabled: bool,
    specs: Vec<VisualizationSpec>,
    visualizations: HashSet<VisualizationId>,
}

impl Default for DeveloperTools {
    fn default() -> Self {
        Self {
            enabled: true,
            specs: Vec::new(),
            visualizations: HashSet::new(),
        }
    }
}

impl DeveloperTools {
    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub fn toggle_enabled(&mut self) -> bool {
        self.enabled = !self.enabled;
        self.enabled
    }

    pub fn visualizations(&self) -> impl Iterator<Item = &VisualizationSpec> {
        self.specs.iter()
    }

    pub fn visualization_selected(&self, id: VisualizationId) -> bool {
        self.visualizations.contains(&id)
    }

    pub fn visualization_enabled(&self, id: VisualizationId) -> bool {
        self.enabled && self.visualization_selected(id)
    }

    pub fn set_visualization_enabled(&mut self, id: VisualizationId, enabled: bool) {
        if enabled {
            self.visualizations.insert(id);
        } else {
            self.visualizations.remove(&id);
        }
    }

    pub fn toggle_visualization(&mut self, id: VisualizationId) -> bool {
        if !self.visualizations.remove(&id) {
            self.visualizations.insert(id);
            true
        } else {
            false
        }
    }

    fn register(&mut self, spec: VisualizationSpec) {
        assert!(
            !self.specs.iter().any(|existing| existing.id == spec.id),
            "duplicate developer visualization id {}",
            spec.id.0,
        );
        if spec.default_enabled {
            self.visualizations.insert(spec.id);
        }
        self.specs.push(spec);
    }
}

pub trait AppDeveloperToolsExt {
    fn register_developer_visualization(&mut self, spec: VisualizationSpec) -> &mut Self;
}

impl AppDeveloperToolsExt for App {
    fn register_developer_visualization(&mut self, spec: VisualizationSpec) -> &mut Self {
        self.init_resource::<DeveloperTools>();
        self.world_mut()
            .resource_mut::<DeveloperTools>()
            .register(spec);
        self
    }
}
