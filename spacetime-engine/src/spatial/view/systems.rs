//! ECS realization of observer-relative USF presentation state.

use super::*;
use crate::spatial::UsfSemanticBounds;
use crate::{
    ecs::{UsfLogicalRealizationOf, UsfOwnershipQuery},
    spatial::{
        UsfCanonicalMotion, UsfCapabilityRealization, UsfPrimaryInteractionSlice, UsfScaleRoleMask,
    },
    view::USF_PRESENTATION_LAYER,
};
use bevy::{
    camera::visibility::RenderLayers,
    light::{NotShadowCaster, NotShadowReceiver},
    math::DVec3,
};

/// Keeps the view anchored to an ordinary bounded runtime transform while
/// deriving its semantic position through the current local physical frame.
pub(in crate::spatial) fn sync_view_context(
    observation_override: Res<UsfViewObservationOverride>,
    ownership: UsfOwnershipQuery,
    semantic_anchors: Query<(&Transform, &UsfLogicalRealizationOf), With<UsfViewAnchor>>,
    semantic_positions: Query<&UsfPosition>,
    semantic_motions: Query<&UsfCanonicalMotion>,
    observer: Single<(&Transform, &mut UsfViewContext), With<UsfViewRenderAnchor>>,
) {
    let (canonical, runtime_translation, velocity_metres_per_second) = if let Some((
        anchor,
        runtime_anchor,
    )) =
        observation_override.current()
    {
        (anchor, runtime_anchor, DVec3::ZERO)
    } else {
        let mut semantic_anchors = semantic_anchors.iter();
        let Some((runtime_anchor, realization)) = semantic_anchors.next() else {
            return;
        };
        if semantic_anchors.next().is_some() {
            error!("primary USF view has multiple semantic anchors");
            return;
        }
        let Some(subject) = ownership.semantic_for(realization) else {
            error!(partition = ?realization.0, "USF semantic view anchor has no semantic owner");
            return;
        };
        let Ok(&canonical) = semantic_positions.get(subject) else {
            error!(subject = ?subject, "USF semantic view anchor has no canonical position");
            return;
        };
        let velocity = semantic_motions
            .get(subject)
            .map(|motion| motion.velocity_metres_per_second())
            .unwrap_or(DVec3::ZERO);
        (canonical, runtime_anchor.translation, velocity)
    };

    let (render_anchor, mut view) = observer.into_inner();
    view.sync_observer(
        canonical,
        runtime_translation,
        render_anchor.translation,
        velocity_metres_per_second,
    );
}

/// Projects scale-authored presentation geometry around the observer without
/// modifying logical/physics transforms.
///
/// Current child presentations (voxel surfaces) have root parents with identity
/// rotation/scale, so child compensation is translation-only. General rotated
/// representation frames can later promote this to an explicit projection frame.
pub(in crate::spatial) fn project_local_scale_presentations(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    parents: Query<
        (
            &UsfScaleLayer,
            Option<&UsfInteractionProjection>,
            Option<&UsfCapabilityRealization>,
        ),
        Without<UsfLocalScalePresentation>,
    >,
    mut presentations: Query<(
        Entity,
        &mut UsfLocalScalePresentation,
        &ChildOf,
        &mut Transform,
        &mut Visibility,
        Option<&RenderLayers>,
        Option<&NotShadowCaster>,
        Option<&NotShadowReceiver>,
    )>,
) {
    for (
        entity,
        mut presentation,
        parent,
        mut transform,
        mut visibility,
        render_layers,
        not_shadow_caster,
        not_shadow_receiver,
    ) in &mut presentations
    {
        let Ok((layer, follows_active, capability)) = parents.get(parent.0) else {
            continue;
        };

        if presentation.scale() != layer.scale() {
            presentation.set_scale(layer.scale());
        }

        // Controlled-subject presentation remains an explicit interaction/view
        // adapter. Capability-local presentations instead follow observer demand
        // and realized PRESENTATION readiness; interaction scale is not a global
        // rendering owner.
        let view_requested = view.contribution(layer.scale()) > CONTRIBUTION_EPSILON;
        let presentation_ready = capability
            .is_none_or(|realization| realization.roles().contains(UsfScaleRoleMask::PRESENTATION));
        let should_render = follows_active.is_some() || (view_requested && presentation_ready);

        if !should_render {
            if !matches!(*visibility, Visibility::Hidden) {
                *visibility = Visibility::Hidden;
            }
            continue;
        }

        // Subject self-visibility is owned by sync_view_subject_presentations.
        // Physical voxel terrain belongs to the ordinary local render layer.
        if follows_active.is_none() {
            let desired_layers = RenderLayers::default();
            if render_layers.is_none_or(|current| *current != desired_layers) {
                commands.entity(entity).insert(desired_layers);
            }
        }

        if not_shadow_caster.is_some() {
            commands.entity(entity).remove::<NotShadowCaster>();
        }
        if not_shadow_receiver.is_some() {
            commands.entity(entity).remove::<NotShadowReceiver>();
        }

        if transform.translation != Vec3::ZERO {
            transform.translation = Vec3::ZERO;
        }

        let desired_scale = Vec3::splat(presentation.authored_to_native_scale());
        if transform.scale != desired_scale {
            transform.scale = desired_scale;
        }

        if !matches!(*visibility, Visibility::Inherited) {
            *visibility = Visibility::Inherited;
        }
    }
}

/// Converts one desired runtime-global translation into a child-local
/// translation while preserving the desired global pose.
///
/// Scenery projections are presentation state. Their parent hierarchy may move
/// for runtime-chart reasons, but that movement must not be applied a second
/// time to a projection that was already computed from canonical/view state.
fn local_translation_from_global(desired_global: Vec3, parent_translation: Option<Vec3>) -> Vec3 {
    parent_translation.map_or(desired_global, |parent| desired_global - parent)
}

/// Contextual representation remains view-owned, not interaction-owned.
///
/// Local/near geometry is projected into the bounded physical view chart.
/// Distant bounded phenomena can instead use one conservative, uniform
/// camera-eye similarity, shared by every realization of their semantic owner.
/// This does not change semantic position, geometry, or simulation authority.
fn contextual_scale_projection_pose(
    view: &UsfViewContext,
    presentation: &UsfScalePresentation,
    parent_translation: Option<Vec3>,
    require_context_scale: bool,
    semantic_observation: Option<(&UsfPosition, &UsfSemanticBounds)>,
) -> Option<(Vec3, f32)> {
    if require_context_scale && !view.context_scale_eligible(presentation.scale()) {
        return None;
    }

    // Preserve the existing direct bounded projection whenever it works.
    let direct = (|| -> Option<(Vec3, f32)> {
        let relative = presentation.anchor().relative_at_scale_bounded(
            view.anchor(),
            presentation.scale(),
            PRESENTATION_RELATIVE_BOUND,
        ).ok()?;
        let factor = view.direct_projection_factor(presentation.scale())?;
        let projected = view.project_relative_native_from_eye(relative, presentation.scale())?;
        if projected.abs().max_element() > f64::from(PRESENTATION_RELATIVE_BOUND) {
            return None;
        }
        let projected = Vec3::new(projected.x as f32, projected.y as f32, projected.z as f32);
        if !projected.is_finite() {
            return None;
        }
        let global = view.presentation_origin() + projected;
        global.is_finite().then_some((
            local_translation_from_global(global, parent_translation),
            factor,
        ))
    })();
    if let Some(pose) = direct {
        return Some(pose);
    }

    // No implicit global fallback: only a *known semantic owner* with an
    // explicitly published conservative bound may enter the far-field path.
    let (semantic_center, bound) = semantic_observation?;
    let ratio = view.distant_presentation_compression(semantic_center, bound.radius_metres())?;
    let relative_metres = presentation.anchor()
        .relative_at_scale_bounded_f64(view.anchor(), SpatialScale::ZERO, f64::MAX)
        .ok()?;
    let projected = view.project_relative_metres_from_eye(relative_metres)? * ratio;
    if !projected.is_finite()
        || projected.abs().max_element() > f64::from(PRESENTATION_RELATIVE_BOUND)
    {
        return None;
    }
    let factor = view.projection_factor_f64(presentation.scale())? * ratio;
    if !factor.is_finite() || factor <= 0.0 || factor > f64::from(f32::MAX) {
        return None;
    }
    let projected = Vec3::new(projected.x as f32, projected.y as f32, projected.z as f32);
    let global = view.presentation_origin() + projected;
    global.is_finite().then_some((
        local_translation_from_global(global, parent_translation),
        factor as f32,
    ))
}

fn capability_terrain_uses_physical_projection(
    presentation_scale: SpatialScale,
    interaction_scale: SpatialScale,
    view_scale: SpatialScale,
) -> bool {
    presentation_scale == interaction_scale && view_scale == interaction_scale
}

pub(in crate::spatial) fn project_scale_presentations(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    probe: Res<UsfPresentationDomainProbe>,
    parents: Query<(&Transform, Option<&UsfCapabilityRealization>), Without<UsfScalePresentation>>,
    semantic_observations: Query<(&UsfPosition, &UsfSemanticBounds)>,
    mut presentations: Query<(
        Entity,
        &UsfScalePresentation,
        Option<&ChildOf>,
        &mut Transform,
        &mut Visibility,
        Option<&RenderLayers>,
        Option<&NotShadowCaster>,
        Option<&NotShadowReceiver>,
    )>,
) {
    for (
        entity,
        presentation,
        parent,
        mut transform,
        mut visibility,
        render_layers,
        not_shadow_caster,
        not_shadow_receiver,
    ) in &mut presentations
    {
        let parent_state = if let Some(parent) = parent {
            let Ok(state) = parents.get(parent.0) else {
                if !matches!(*visibility, Visibility::Hidden) {
                    *visibility = Visibility::Hidden;
                }
                continue;
            };
            Some(state)
        } else {
            None
        };
        let capability = parent_state.and_then(|(_, capability)| capability);

        if capability.is_some_and(|realization| {
            !realization.roles().contains(UsfScaleRoleMask::PRESENTATION)
        }) {
            if !matches!(*visibility, Visibility::Hidden) {
                *visibility = Visibility::Hidden;
            }
            continue;
        }

        // Physical presentation is used only while the view itself is in the
        // interaction Scale Slice. Presentation may refine ahead of interaction:
        // once the view moves finer, this same capability realization becomes a
        // contextual ancestor while collision/edit authority remains unchanged.
        let physical_local = capability.is_some()
            && capability_terrain_uses_physical_projection(
                presentation.scale(),
                interaction.scale(),
                view.scale(),
            );
        if physical_local {
            if !probe.physical_enabled() {
                *visibility = Visibility::Hidden;
                continue;
            }

            let desired_layers = RenderLayers::default();
            if render_layers.is_none_or(|current| *current != desired_layers) {
                commands.entity(entity).insert(desired_layers);
            }
            if not_shadow_caster.is_some() {
                commands.entity(entity).remove::<NotShadowCaster>();
            }
            if not_shadow_receiver.is_some() {
                commands.entity(entity).remove::<NotShadowReceiver>();
            }

            if transform.translation != Vec3::ZERO {
                transform.translation = Vec3::ZERO;
            }
            if transform.scale != Vec3::ONE {
                transform.scale = Vec3::ONE;
            }
            if !matches!(*visibility, Visibility::Inherited) {
                *visibility = Visibility::Inherited;
            }
            continue;
        }

        // Contextual branch: authored scale presentations plus every realized
        // terrain Scale participating in the observer's coarse->fine branch.
        // This is deliberately view-owned, not interaction-owned. The refinement
        // clip material performs spatial child-over-parent aperture ownership.
        if !probe.context_enabled() {
            *visibility = Visibility::Hidden;
            continue;
        }

        let desired_layers = RenderLayers::layer(USF_PRESENTATION_LAYER);
        if render_layers.is_none_or(|current| *current != desired_layers) {
            commands.entity(entity).insert(desired_layers);
        }
        if not_shadow_caster.is_none() {
            commands.entity(entity).insert(NotShadowCaster);
        }
        if not_shadow_receiver.is_none() {
            commands.entity(entity).insert(NotShadowReceiver);
        }

        let parent_translation = parent_state.map(|(transform, _)| transform.translation);
        let semantic_observation = capability
            .and_then(|realization| semantic_observations.get(realization.authority()).ok());
        let Some((translation, scale)) = contextual_scale_projection_pose(
            &view,
            presentation,
            parent_translation,
            capability.is_none(),
            semantic_observation,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };
        if transform.translation != translation {
            transform.translation = translation;
        }
        if transform.scale != Vec3::splat(scale) {
            transform.scale = Vec3::splat(scale);
        }
        if !matches!(*visibility, Visibility::Inherited) {
            *visibility = Visibility::Inherited;
        }
    }
}
