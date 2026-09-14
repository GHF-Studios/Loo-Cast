//! Developer-facing inspection, UI and spatial visualization.
//!
//! Developer tools deliberately separate structured inspection, screen-space UI,
//! and text-free World Draw. Domain-specific adapters live beside the domains they
//! inspect or visualize rather than under a second cross-cutting framework.

mod draw;
mod editor;
mod focus;
mod gizmo;
mod inspect;
mod tools;
mod ui;
mod view;

pub use draw::{
    ColorRamp, ColorStop, DrawDepth, DrawId, ScalarFieldMode, ScalarRange, WorldDrawBatch,
    WorldDrawFrame, WorldPrimitive, WorldScalarField,
};
pub use focus::{DeveloperFocus, FocusHit, FocusTarget};
pub use gizmo::{
    EditorSelection, EditorTool, EditorToolState, EditorTransformSpace, EditorTransformWritable,
};
pub use inspect::{
    InspectField, InspectNumberFormat, InspectSection, InspectSectionId, InspectUnit,
    InspectValue, InspectionFrame,
};
pub use tools::{
    AppDeveloperToolsExt, DeveloperTools, VisualizationId, VisualizationSpec,
};
pub use view::DeveloperView;

use bevy::{prelude::*, transform::TransformSystems};

/// Presentation-only entities owned by developer tooling.
///
/// Runtime diagnostics exclude these entities from world/ECS structural counts.
#[derive(Component, Debug, Default)]
pub struct DeveloperArtifact;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum DeveloperSet {
    ResolveFocus,
    Interact,
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
                    DeveloperSet::Interact,
                    DeveloperSet::CollectInspection,
                    DeveloperSet::CollectWorldDraw,
                    DeveloperSet::RenderUi,
                    DeveloperSet::RenderWorldDraw,
                )
                    .chain()
                    .before(bevy_egui::EguiPostUpdateSet::EndPass),
            )
            .add_systems(
                PostUpdate,
                inspect::clear_inspection_frame.in_set(DeveloperSet::ResolveFocus),
            )
            .add_systems(PreUpdate, toggle_developer_tools);

        crate::ecs::devtools::configure(app);
        crate::physics::character::devtools::configure(app);

        gizmo::configure(app);
        draw::configure(app);
        ui::configure(app);
        editor::configure(app);
    }
}

fn toggle_developer_tools(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut tools: ResMut<DeveloperTools>,
) {
    if keyboard.just_pressed(KeyCode::F3) {
        tools.toggle_enabled();
    }
}
