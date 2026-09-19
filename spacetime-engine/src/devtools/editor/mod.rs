//! Runtime editor/composer shell around the already-running game.
//!
//! Hierarchy is the raw ECS entity chooser. Structure refines that canonical
//! entity focus into semantic parts. Semantic Inspector and contextual Gizmos
//! consume the refinement; ECS Inspector remains a deliberately raw whole-entity
//! reflection surface.

use bevy::{
    camera::visibility::RenderLayers,
    camera::{CameraOutputMode, ClearColorConfig, Viewport},
    prelude::*,
    render::render_resource::BlendState,
    window::PrimaryWindow,
};
use bevy_egui::{
    EguiContext, EguiGlobalSettings, EguiPlugin, EguiPrimaryContextPass, PrimaryEguiContext, egui,
};
use bevy_inspector_egui::{
    DefaultInspectorConfigPlugin,
    bevy_inspector::{
        self,
        hierarchy::{SelectedEntities, hierarchy_ui_filtered},
    },
};
use egui_dock::{DockArea, DockState, NodeIndex, Style, TabViewer};

use crate::{
    ecs::{UsfManifestationAuthority, UsfManifestationOf, UsfManifestations},
    view::{PrimaryGameView, PrimaryViewPresentation},
};

use super::{
    DeveloperArtifact, DeveloperFocus, DeveloperTools, EditorTransformGizmoSettings,
    EditorTransformWritable, FocusTarget, InspectEditRequest, InspectTypeRegistry, InspectValue,
    InspectionFrame, StructureFrame, StructureSelection,
    inspect_ui::{
        self, InspectWidgetContext, InspectorWidget, InspectorWidgetRegistry, TransformWidget,
    },
};

mod hierarchy;
mod inspector;
mod shell;
mod visualization;

use hierarchy::{
    apply_hierarchy_selection, draw_structure, focus_name, sync_hierarchy_selection,
};
use inspector::{draw_gizmos, draw_semantic_inspector};
use shell::draw_transform_space_control;
use visualization::{draw_legacy_slot, draw_visualizations};

pub(super) fn configure(app: &mut App) {
    shell::configure(app);
}
