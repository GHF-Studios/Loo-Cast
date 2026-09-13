//! Deliberate developer observability architecture.
//!
//! The subsystem separates:
//! - runtime control/tool intent (`control`),
//! - world-space semantic observations (`frame`),
//! - rendering backends (`render`),
//! - instrumentation (`telemetry`),
//! - and the nested configuration surface (`menu`).
//!
//! Domains publish meaning. Renderers decide presentation. The simulation does not
//! acquire debug components or debug-camera knowledge.

mod color;
mod context;
mod control;
mod frame;
mod menu;
mod render;
mod telemetry;

pub use color::{DebugColorRamp, DebugColorStop, DebugScalarRange};
pub use context::DebugContext;
pub use control::{
    AppObservabilityExt, DebugChoiceOption, DebugCondition, DebugControlKind,
    DebugControlSpec, DebugControls, DebugId,
};
pub use frame::{
    DebugDepth, DebugFrame, DebugFrameBatch, DebugLabel, DebugPrimitive, DebugScalarField,
    DebugScalarFieldMode, DebugTextFacing, DebugVectorField, DebugVectorSpace,
};
pub use menu::DebugMenuState;
pub use telemetry::{DebugMetricSpec, DebugMetrics, MetricUnit};

use bevy::{prelude::*, transform::TransformSystems};

#[derive(Component, Debug, Default)]
pub struct DebugArtifact;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ObservabilitySet {
    /// Keyboard/UI intent is applied before backend adapters consume controls.
    Control,
    Prepare,
    Collect,
    Render,
}

pub const CATEGORY_WORLD: DebugId = DebugId("category.world");
pub const CATEGORY_PHYSICS: DebugId = DebugId("category.physics");
pub const CATEGORY_DIAGNOSTICS: DebugId = DebugId("category.diagnostics");

/// Core/engine developer observability. Test-game-specific tools are composed
/// separately by [`crate::game::TestGameObservabilityPlugin`].
pub struct ObservabilityPlugin;

impl Plugin for ObservabilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DebugControls>()
            .init_resource::<DebugContext>()
            .init_resource::<DebugFrame>()
            .register_debug_control(
                DebugControlSpec::group(CATEGORY_WORLD, None, "World / Simulation", 0)
                    .described("Semantic entities and simulation-domain tools."),
            )
            .register_debug_control(
                DebugControlSpec::group(CATEGORY_PHYSICS, None, "Physics", 1)
                    .described("Backend physics and engine-owned movement semantics."),
            )
            .register_debug_control(
                DebugControlSpec::group(CATEGORY_DIAGNOSTICS, None, "Diagnostics", 2)
                    .described("Performance and instrumentation."),
            )
            .configure_sets(
                PostUpdate,
                ObservabilitySet::Prepare.after(TransformSystems::Propagate),
            )
            .configure_sets(
                PostUpdate,
                (
                    ObservabilitySet::Prepare,
                    ObservabilitySet::Collect,
                    ObservabilitySet::Render,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                frame::clear_debug_frame.in_set(ObservabilitySet::Prepare),
            )
            .add_systems(PostStartup, validate_debug_control_graph);

        render::configure(app);
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
