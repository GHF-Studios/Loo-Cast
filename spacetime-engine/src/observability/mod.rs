//! Legacy developer-tool control and telemetry shell.
//!
//! Spatial visualization and inspection have moved to `crate::devtools`. This
//! module now exists only while the old generic control graph/menu and telemetry
//! are being migrated in Stages 4 and 5.

mod control;
mod menu;
mod telemetry;

pub use control::{
    AppObservabilityExt, DebugChoiceOption, DebugCondition, DebugControlKind,
    DebugControlSpec, DebugControls, DebugId,
};
pub use menu::DebugMenuState;
pub use telemetry::{DebugMetricSpec, DebugMetrics, MetricUnit};

use bevy::prelude::*;

/// Presentation-only marker retained until telemetry learns about
/// `DeveloperArtifact` directly in Stage 5.
#[derive(Component, Debug, Default)]
pub struct DebugArtifact;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObservabilitySet {
    /// Keyboard/UI intent is applied before temporary legacy adapters consume controls.
    Control,
}

pub const CATEGORY_WORLD: DebugId = DebugId("category.world");
pub const CATEGORY_PHYSICS: DebugId = DebugId("category.physics");
pub const CATEGORY_DIAGNOSTICS: DebugId = DebugId("category.diagnostics");

/// Temporary legacy composition for the control graph/menu and telemetry.
pub struct ObservabilityPlugin;

impl Plugin for ObservabilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugControls>()
            .register_debug_control(
                DebugControlSpec::group(CATEGORY_WORLD, None, "World / Simulation", 0)
                    .described("Temporary controls for remaining developer visualizations."),
            )
            .register_debug_control(
                DebugControlSpec::group(CATEGORY_PHYSICS, None, "Physics", 1)
                    .described("Temporary controls for physics developer tools."),
            )
            .register_debug_control(
                DebugControlSpec::group(CATEGORY_DIAGNOSTICS, None, "Diagnostics", 2)
                    .described("Performance and instrumentation."),
            )
            .add_systems(PostStartup, validate_debug_control_graph);

        telemetry::configure(app);

        crate::ecs::observability::configure(app);
        crate::physics::observability::configure(app);
        crate::physics::character::observability::configure(app);

        menu::configure(app);
    }
}

fn validate_debug_control_graph(controls: Res<DebugControls>) {
    controls.validate();
}
