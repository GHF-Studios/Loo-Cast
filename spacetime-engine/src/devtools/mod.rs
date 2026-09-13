//! Developer-facing inspection and visualization foundations.
//!
//! This is the replacement path for the current broad `observability` umbrella.
//! Migration is staged; legacy observability remains active until individual
//! responsibilities have working replacements.

mod draw;
mod focus;
mod inspect;
mod tools;
mod ui;
mod view;

pub use draw::{
    ColorRamp, ColorStop, DrawDepth, DrawId, ScalarFieldMode, ScalarRange, VectorSpace,
    WorldDrawBatch, WorldDrawFrame, WorldPrimitive, WorldScalarField, WorldVectorField,
};
pub use focus::{DeveloperFocus, FocusHit, FocusTarget};
pub use inspect::{
    InspectField, InspectNumberFormat, InspectSection, InspectSectionId, InspectUnit,
    InspectValue, InspectionFrame,
};
pub use tools::{DeveloperTools, VisualizationId};
pub use view::DeveloperView;

use bevy::{prelude::*, transform::TransformSystems};

/// Presentation-only entities owned by developer tooling.
///
/// During migration the new UI also carries the legacy `DebugArtifact` marker
/// so old telemetry keeps excluding it. Stage 5 removes that bridge.
#[derive(Component, Debug, Default)]
pub struct DeveloperArtifact;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeveloperSet {
    ResolveFocus,
    CollectInspection,
    CollectWorldDraw,
    RenderUi,
    RenderWorldDraw,
}

pub struct DeveloperToolsPlugin;

impl Plugin for DeveloperToolsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DeveloperFocus>()
            .init_resource::<DeveloperView>()
            .init_resource::<InspectionFrame>()
            .init_resource::<DeveloperTools>()
            .configure_sets(
                PostUpdate,
                DeveloperSet::ResolveFocus.after(TransformSystems::Propagate),
            )
            .configure_sets(
                PostUpdate,
                (
                    DeveloperSet::ResolveFocus,
                    DeveloperSet::CollectInspection,
                    DeveloperSet::CollectWorldDraw,
                    DeveloperSet::RenderUi,
                    DeveloperSet::RenderWorldDraw,
                )
                    .chain(),
            )
            .add_systems(
                PostUpdate,
                inspect::clear_inspection_frame.in_set(DeveloperSet::ResolveFocus),
            )
            .add_systems(PreUpdate, toggle_developer_tools);

        draw::configure(app);
        ui::configure(app);
    }
}

fn toggle_developer_tools(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tools: ResMut<DeveloperTools>,
) {
    // F3 is also the legacy observability master during migration. Both start
    // enabled and intentionally follow the same key until Stage 4 deletes the
    // old control graph.
    if keyboard.just_pressed(KeyCode::F3) {
        tools.toggle_enabled();
    }
}
