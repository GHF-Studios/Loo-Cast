use std::collections::HashSet;

use bevy::prelude::*;

/// Stable semantic ID for one world visualization.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VisualizationId(pub &'static str);

/// Deliberately small replacement for the generic debug-control graph.
///
/// Visualization-specific settings do not live here. A visualization that
/// genuinely needs configuration owns a typed settings resource in its domain.
#[derive(Resource, Debug)]
pub struct DeveloperTools {
    enabled: bool,
    visualizations: HashSet<VisualizationId>,
}

impl Default for DeveloperTools {
    fn default() -> Self {
        Self {
            enabled: true,
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

    pub fn visualization_enabled(&self, id: VisualizationId) -> bool {
        self.enabled && self.visualizations.contains(&id)
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
}
