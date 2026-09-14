//! Editor interaction state and gizmo backends.
//!
//! The generic layer owns selection and tool intent. It does **not** own domain
//! authoring state. A gizmo backend may directly mutate a component only when a
//! domain has explicitly declared that component authoritative for that edit.
//!
//! The first backend uses Bevy's transform gizmo. Keeping the adapter here means
//! future portal, light, collider, USF, or other domain handles can share editor
//! selection/view/input policy without pretending every gizmo is a transform.

use bevy::{
    camera::visibility::RenderLayers,
    gizmos::transform_gizmo::{
        TransformGizmoCamera, TransformGizmoFocus, TransformGizmoMeshMarker,
        TransformGizmoMode, TransformGizmoPlugin, TransformGizmoRoot, TransformGizmoSettings,
        TransformGizmoSpace, TransformGizmoState, TransformGizmoSystems,
    },
    prelude::*,
    window::{CursorGrabMode, CursorOptions, PrimaryWindow},
};

use crate::{
    input_focus::{InputFocus, InputFocusSet},
    view::{PrimaryGameView, PrimaryViewPresentation, ViewportSpace},
};

use super::{DeveloperArtifact, DeveloperFocus, DeveloperSet};

const INPUT_FOCUS_OWNER: &str = "editor_gizmo";

// Bevy 0.19's mesh transform-gizmo renderer uses a private overlay-camera marker
// and this dedicated layer. The adapter keeps that implementation detail in one
// place so the rest of the editor never needs to know it. Re-check this constant
// when upgrading Bevy.
const BEVY_TRANSFORM_GIZMO_RENDER_LAYER: usize = 15;

/// Durable ECS selection owned by the editor shell.
///
/// This is intentionally different from [`DeveloperFocus`]: focus is the live
/// semantic thing under the developer pointer/look ray, while editor selection is
/// explicit UI state that survives pointer movement and drives authoring tools.
#[derive(Resource, Debug, Default, Clone)]
pub struct EditorSelection {
    entities: Vec<Entity>,
}

impl EditorSelection {
    pub fn as_slice(&self) -> &[Entity] {
        &self.entities
    }

    pub fn single(&self) -> Option<Entity> {
        match self.entities.as_slice() {
            [entity] => Some(*entity),
            _ => None,
        }
    }

    pub fn clear(&mut self) {
        self.entities.clear();
    }

    pub fn replace(&mut self, entity: Entity) {
        if self.entities.len() != 1 || self.entities[0] != entity {
            self.entities.clear();
            self.entities.push(entity);
        }
    }

    pub fn replace_many(&mut self, entities: impl IntoIterator<Item = Entity>) {
        let mut next = Vec::new();
        for entity in entities {
            if !next.contains(&entity) {
                next.push(entity);
            }
        }
        if self.entities != next {
            self.entities = next;
        }
    }
}

/// High-level editor manipulation intent.
///
/// Backends translate this into their own implementation-specific modes. Domain
/// gizmos may ignore transform-specific variants when they are not relevant.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EditorTool {
    #[default]
    Select,
    Translate,
    Rotate,
    Scale,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum EditorTransformSpace {
    #[default]
    World,
    Local,
}

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct EditorToolState {
    tool: EditorTool,
    transform_space: EditorTransformSpace,
}

impl EditorToolState {
    pub fn tool(&self) -> EditorTool {
        self.tool
    }

    pub fn set_tool(&mut self, tool: EditorTool) {
        self.tool = tool;
    }

    pub fn transform_space(&self) -> EditorTransformSpace {
        self.transform_space
    }

    pub fn set_transform_space(&mut self, space: EditorTransformSpace) {
        self.transform_space = space;
    }
}

/// Grants the generic transform-gizmo backend permission to mutate this entity's
/// [`Transform`] directly.
///
/// This marker is deliberately opt-in. A `Transform` can be presentation output,
/// physics state, generated map output, a portal manifestation, or other derived
/// state. Those entities must not become editor-authoritative just because they
/// happen to carry `Transform`.
///
/// Persistent/domain-authored objects should normally expose an adapter that
/// commits edits to their real source of truth (potentially through an editor-only
/// proxy entity) rather than attaching this marker to generated runtime output.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct EditorTransformWritable;

pub(super) fn configure(app: &mut App) {
    app.add_plugins(TransformGizmoPlugin)
        .init_resource::<InputFocus>()
        .init_resource::<PrimaryViewPresentation>()
        .init_resource::<EditorSelection>()
        .init_resource::<EditorToolState>()
        .configure_sets(
            PostUpdate,
            DeveloperSet::Interact.after(TransformGizmoSystems),
        )
        .add_systems(
            PreUpdate,
            claim_gizmo_input.before(InputFocusSet::Resolve),
        )
        .add_systems(
            Update,
            (
                prune_selection,
                sync_backend_camera,
                sync_backend_settings,
                sync_backend_target,
            )
                .chain(),
        )
        .add_systems(
            PostUpdate,
            (
                select_from_primary_view,
                sync_backend_overlay_camera,
                tag_backend_artifacts,
            )
                .chain()
                .in_set(DeveloperSet::Interact),
        );
}

fn claim_gizmo_input(
    presentation: Res<PrimaryViewPresentation>,
    state: Res<TransformGizmoState>,
    mut input_focus: ResMut<InputFocus>,
) {
    input_focus.set_modal_claim(
        INPUT_FOCUS_OWNER,
        presentation.is_embedded() && state.active,
    );
}


fn prune_selection(
    mut selection: ResMut<EditorSelection>,
    entities: Query<Entity>,
) {
    if selection
        .entities
        .iter()
        .any(|entity| !entities.contains(*entity))
    {
        selection.entities.retain(|entity| entities.contains(*entity));
    }
}

fn sync_backend_camera(
    mut commands: Commands,
    primary: Query<(Entity, Option<&TransformGizmoCamera>), With<PrimaryGameView>>,
    other_marked: Query<Entity, (With<TransformGizmoCamera>, Without<PrimaryGameView>)>,
) {
    let Ok((primary, marker)) = primary.single() else {
        return;
    };

    for entity in &other_marked {
        commands.entity(entity).remove::<TransformGizmoCamera>();
    }

    if marker.is_none() {
        commands.entity(primary).insert(TransformGizmoCamera);
    }
}

fn sync_backend_settings(
    tools: Res<EditorToolState>,
    mut settings: ResMut<TransformGizmoSettings>,
) {
    settings.mode = match tools.tool() {
        EditorTool::Select | EditorTool::Translate => TransformGizmoMode::Translate,
        EditorTool::Rotate => TransformGizmoMode::Rotate,
        EditorTool::Scale => TransformGizmoMode::Scale,
    };
    settings.space = match tools.transform_space() {
        EditorTransformSpace::World => TransformGizmoSpace::World,
        EditorTransformSpace::Local => TransformGizmoSpace::Local,
    };

    // The editor already owns pointer/capture policy. Letting the backend confine
    // the OS cursor would make a handle drag another hidden source of window-level
    // input state and works poorly with a sub-viewport.
    settings.confine_cursor = false;
}

fn sync_backend_target(
    mut commands: Commands,
    presentation: Res<PrimaryViewPresentation>,
    tools: Res<EditorToolState>,
    selection: Res<EditorSelection>,
    writable: Query<(), (With<EditorTransformWritable>, With<Transform>)>,
    focused: Query<Entity, With<TransformGizmoFocus>>,
) {
    let desired = if presentation.is_embedded() && tools.tool() != EditorTool::Select {
        selection.single().filter(|entity| writable.contains(*entity))
    } else {
        None
    };

    for entity in &focused {
        if Some(entity) != desired {
            commands.entity(entity).remove::<TransformGizmoFocus>();
        }
    }

    if let Some(entity) = desired {
        if !focused.contains(entity) {
            commands.entity(entity).insert(TransformGizmoFocus);
        }
    }
}

/// Turns a free-pointer click in the embedded Game view into editor selection.
///
/// The developer focus resolver already owns ray/world hit policy, including
/// portal apertures and semantic manifestation mapping. Reusing its hovered
/// *spatial* entity avoids a second editor-only picking implementation.
fn select_from_primary_view(
    mouse: Res<ButtonInput<MouseButton>>,
    presentation: Res<PrimaryViewPresentation>,
    window: Single<(&Window, &CursorOptions), With<PrimaryWindow>>,
    camera: Single<&Camera, With<PrimaryGameView>>,
    focus: Res<DeveloperFocus>,
    gizmo: Res<TransformGizmoState>,
    mut selection: ResMut<EditorSelection>,
) {
    if !presentation.is_embedded() || !mouse.just_pressed(MouseButton::Left) {
        return;
    }

    // A locked cursor means gameplay owns the pointer. Also avoid replacing the
    // selected object when the same click started a gizmo drag.
    let (window, cursor) = window.into_inner();
    if cursor.grab_mode != CursorGrabMode::None || gizmo.active || gizmo.hovered_axis.is_some() {
        return;
    }

    let Some(position) = window.cursor_position() else {
        return;
    };
    if !ViewportSpace::new(&camera).contains_target_position(position) {
        return;
    }

    if let Some(target) = focus.hovered() {
        selection.replace(target.spatial_entity);
    } else {
        selection.clear();
    }
}

/// Bevy's 0.19 gizmo renderer mirrors the selected camera transform/viewport but
/// its private overlay camera keeps a default projection. Our primary view has a
/// custom horizontal-FOV-derived perspective, so copy the projection here to keep
/// rendered handles and Bevy's own hit-testing in the same screen space.
fn sync_backend_overlay_camera(
    primary: Query<(&Projection, &Camera), With<PrimaryGameView>>,
    focused: Query<(), With<TransformGizmoFocus>>,
    mut cameras: Query<
        (&mut Projection, &mut Camera, &RenderLayers),
        (With<Camera3d>, Without<PrimaryGameView>),
    >,
) {
    let Ok((primary_projection, primary_camera)) = primary.single() else {
        return;
    };
    let layer = RenderLayers::layer(BEVY_TRANSFORM_GIZMO_RENDER_LAYER);
    let has_focus = focused.iter().next().is_some();

    for (mut projection, mut camera, layers) in &mut cameras {
        if camera.order != 1 || *layers != layer {
            continue;
        }

        *projection = primary_projection.clone();
        camera.viewport = primary_camera.viewport.clone();
        camera.is_active = primary_camera.is_active && has_focus;
    }
}

/// Keep implementation-detail entities out of hierarchy and structural runtime
/// diagnostics just like our own retained developer presentation artifacts.
fn tag_backend_artifacts(
    mut commands: Commands,
    gizmo_entities: Query<
        Entity,
        (
            Or<(With<TransformGizmoRoot>, With<TransformGizmoMeshMarker>)>,
            Without<DeveloperArtifact>,
        ),
    >,
    cameras: Query<
        (Entity, &Camera, &RenderLayers),
        (
            With<Camera3d>,
            Without<PrimaryGameView>,
            Without<DeveloperArtifact>,
        ),
    >,
) {
    for entity in &gizmo_entities {
        commands.entity(entity).insert(DeveloperArtifact);
    }

    let layer = RenderLayers::layer(BEVY_TRANSFORM_GIZMO_RENDER_LAYER);
    for (entity, camera, layers) in &cameras {
        if camera.order == 1 && *layers == layer {
            commands.entity(entity).insert(DeveloperArtifact);
        }
    }
}
