//! Temporary telemetry shell.
//!
//! Developer-tool controls, UI and World Draw have moved to `crate::devtools`.
//! This module remains only until telemetry moves to `diagnostics` in Stage 5.

mod telemetry;

pub use telemetry::{DebugMetricSpec, DebugMetrics, MetricUnit};

use bevy::prelude::*;

/// Stable temporary metric identifier. Stage 5 will move/rename this with the
/// telemetry subsystem rather than keeping it as a developer-control concept.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DebugId(pub &'static str);

impl std::fmt::Display for DebugId {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter.write_str(self.0)
    }
}

/// Presentation-only marker retained until telemetry learns about
/// `DeveloperArtifact` directly in Stage 5.
#[derive(Component, Debug, Default)]
pub struct DebugArtifact;

pub struct ObservabilityPlugin;

impl Plugin for ObservabilityPlugin {
    fn build(&self, app: &mut App) {
        telemetry::configure(app);
    }
}
