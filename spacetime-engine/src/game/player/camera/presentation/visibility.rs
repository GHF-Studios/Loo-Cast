//! Primary-view self-presentation policy.

use super::*;

/// Self-visibility is primary-view policy, not model identity or portal policy.
///
/// All manifestations of the viewed semantic subject are hidden from the
/// primary first-person camera by moving body presentations onto the derived
/// view layer. Portal cameras intentionally include that layer. Presentations
/// of previous/unrelated view subjects are restored to ordinary world layers.
pub(in crate::game::player) fn sync_view_subject_presentations(
    mut commands: Commands,
    freecam: Res<DebugFreecam>,
    camera: Single<&PlayerCamera>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    target: Single<Entity, With<LocalViewTarget>>,
    presentations: Query<
        (Entity, &UsfPresentationProjectionOf, Option<&RenderLayers>),
        With<ViewSubjectPresentation>,
    >,
) {
    let viewed_semantic = runtime_ownership.semantic_of(target.into_inner());

    for (entity, projection, layers) in &presentations {
        let is_self = viewed_semantic.is_some()
            && runtime_ownership.semantic_of(projection.0) == viewed_semantic;

        let desired = if is_self && !freecam.enabled() && camera.mode == CameraMode::FirstPerson {
            RenderLayers::layer(DERIVED_VIEW_LAYER)
        } else {
            RenderLayers::default()
        };

        // View-subject self visibility owns this layer even when a model was
        // spawned without RenderLayers (the spacecraft used to be exactly that
        // case). Missing component state must not silently opt out of policy.
        if layers.is_none_or(|current| *current != desired) {
            commands.entity(entity).insert(desired);
        }
    }
}
