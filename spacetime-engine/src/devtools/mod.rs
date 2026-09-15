//! Developer-facing inspection, UI, editor interaction and spatial visualization.
//!
//! The editor is one consumer of semantic tooling rather than the owner of domain
//! meaning. Raw ECS reflection, semantic inspection, contextual gizmos and World
//! Draw deliberately remain separate mechanisms that can be composed together.

mod draw;
mod editor;
mod focus;
mod gizmo;
mod inspect;
pub mod inspect_ui;
mod structure;
mod tools;
mod ui;
mod view;

pub use draw::{
    ColorRamp, ColorStop, DrawDepth, DrawId, ScalarFieldMode, ScalarRange, WorldDrawBatch,
    WorldDrawFrame, WorldPrimitive, WorldScalarField,
};
pub use focus::{DeveloperFocus, FocusHit, FocusTarget};
pub use gizmo::{EditorTransformGizmoSettings, EditorTransformSpace, EditorTransformWritable};
pub use inspect::{
    InspectAccess, InspectAction, InspectActionId, InspectActionRequest, InspectEditRequest,
    InspectField, InspectFieldId, InspectNumberFormat, InspectNumberInput, InspectSection,
    InspectSectionId, InspectUnit, InspectValue, InspectionFrame,
};
pub use structure::{StructureFrame, StructureItem, StructureItemId, StructureSelection};
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
    CollectStructure,
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
            .init_resource::<StructureFrame>()
            .init_resource::<StructureSelection>()
            .init_resource::<InspectionFrame>()
            .init_resource::<DeveloperTools>()
            .add_message::<InspectEditRequest>()
            .add_message::<InspectActionRequest>()
            .configure_sets(
                PostUpdate,
                DeveloperSet::ResolveFocus.after(TransformSystems::Propagate),
            )
            .configure_sets(
                PostUpdate,
                (
                    DeveloperSet::ResolveFocus,
                    DeveloperSet::Interact,
                    DeveloperSet::CollectStructure,
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
                (
                    structure::clear_structure_frame,
                    inspect::clear_inspection_frame,
                )
                    .in_set(DeveloperSet::ResolveFocus),
            )
            .add_systems(
                PostUpdate,
                structure::prune_structure_selection.in_set(DeveloperSet::CollectInspection),
            )
            .add_systems(PreUpdate, (toggle_developer_tools, focus::prune_focus));

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
