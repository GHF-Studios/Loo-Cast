//! Developer-facing inspection, UI, editor interaction and spatial visualization.
//!
//! The editor is one consumer of semantic tooling rather than the owner of domain
//! meaning. Raw ECS reflection, semantic inspection, contextual gizmos and World
//! Draw deliberately remain separate mechanisms that can be composed together.
//!
//! ## Integration
//!
//! Inspection, structure, focus, and World Draw consume published domain state. Editing and actions
//! remain requests to the owning domain.
//!
//! ## Module map
//!
//! - `draw`: Text-free world-space developer visualization.
//! - `editor`: Runtime editor/composer shell around the already-running game.
//! - `focus`: Track the developer focus target and prune stale focus.
//! - `gizmo`: Contextual editor gizmos.
//! - `inspect`: Compose inspection frames, metadata, typed models, and visitor traits.
//! - `inspect_ui`: Reusable egui presentation for the UI-agnostic inspection model.
//! - `script_workbench`: Host-managed developer scripting workspace.
//! - `structure`: Publish semantic structure rows and preserve valid selection.
//! - `tools`: Register developer visualizations and expose their application extension.
//! - `ui`: Screen-space developer UI.
//! - `view`: Store the developer view state used by tooling and inspection.
//!
//! The plugin or configure entrypoint here wires the child systems and resources.
//!

mod draw;
mod editor;
mod focus;
mod gizmo;
mod inspect;
pub mod inspect_ui;
mod script_workbench;
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
    AppInspectExt, Inspect, InspectAccess, InspectAction, InspectActionId, InspectActionRequest,
    InspectEditRequest, InspectField, InspectFieldId, InspectFieldMetadata, InspectFieldVisitor,
    InspectFieldVisitorMut, InspectNumberFormat, InspectNumberInput, InspectSection,
    InspectSectionId, InspectTypeMetadata, InspectTypeRegistration, InspectTypeRegistry,
    InspectUnit, InspectValue, InspectWidgetId, InspectionFrame,
};
pub use inspect_ui::{AppInspectorWidgetsExt, InspectorWidgetRegistry};
pub(crate) use script_workbench::{DeveloperScriptWorkbench, ScriptTarget, draw_script_workspace};
pub use structure::{StructureFrame, StructureItem, StructureItemId, StructureSelection};
pub use tools::{AppDeveloperToolsExt, DeveloperTools, VisualizationId, VisualizationSpec};
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
                    structure::begin_structure_frame,
                    inspect::begin_inspection_frame,
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

        inspect_ui::configure(app);
        gizmo::configure(app);
        draw::configure(app);
        ui::configure(app);
        script_workbench::configure(app);
        editor::configure(app);
    }
}

fn toggle_developer_tools(keyboard: Res<ButtonInput<KeyCode>>, mut tools: ResMut<DeveloperTools>) {
    if keyboard.just_pressed(KeyCode::F3) {
        tools.toggle_enabled();
    }
}
