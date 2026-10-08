//! ECS realization of observer-relative USF presentation state.

use super::*;
use crate::{
    ecs::{UsfLogicalRealizationOf, UsfOwnershipQuery, UsfPresentationProjectionOf},
    spatial::{
        UsfCanonicalMotion, UsfCapabilityRealization, UsfPrimaryInteractionSlice,
        UsfScaleCoverageSnapshot, UsfScaleRoleMask,
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

fn fallback_should_render(
    fallback: UsfScaleFallbackPresentation,
    view_scale: SpatialScale,
    interaction_scale: SpatialScale,
    replacement_ready: bool,
) -> bool {
    if fallback.owns_view_scale(view_scale) {
        return true;
    }

    // Presentation may move finer before physical interaction. Keep a
    // fallback visible until interaction enters the voxel ladder and local
    // presentation is published.
    if interaction_scale > fallback.scale() {
        return true;
    }

    !replacement_ready
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

fn scenery_is_inside_near_field_exclusion(
    presentation: UsfSceneryPresentation,
    observer_relative_native: Vec3,
) -> bool {
    presentation
        .near_field_exclusion_radius_native()
        .is_some_and(|radius| f64::from(observer_relative_native.length()) <= radius)
}

/// Projects persistent multiscale scenery into one bounded render scene.
///
/// For a raw observer-relative distance `d`, the rendered radius is
/// `R * d / (R + d)`. The same compression is applied to object scale, preserving
/// angular size while keeping arbitrarily distant representations inside `R`.
pub(in crate::spatial) fn project_scenery_presentations(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    probe: Res<UsfPresentationDomainProbe>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    ownership: UsfOwnershipQuery,
    parents: Query<&Transform, Without<UsfSceneryPresentation>>,
    mut presentations: Query<(
        Entity,
        &UsfSceneryPresentation,
        Option<&ChildOf>,
        &mut Transform,
        &mut Visibility,
        Option<&RenderLayers>,
        Option<&NotShadowCaster>,
        Option<&NotShadowReceiver>,
        Option<&UsfScaleFallbackPresentation>,
        Option<&UsfPresentationProjectionOf>,
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
        fallback,
        projection,
    ) in &mut presentations
    {
        if !probe.context_enabled() {
            *visibility = Visibility::Hidden;
            continue;
        }

        if let Some(fallback) = fallback {
            let replacement_ready = projection
                .and_then(|projection| ownership.semantic_of(projection.0))
                .is_some_and(|semantic| {
                    coverage.has_near_for_authority(
                        semantic,
                        interaction.scale(),
                        view.anchor(),
                        UsfScaleRoleMask::PRESENTATION,
                        0.0,
                    )
                });

            if !fallback_should_render(
                *fallback,
                view.scale(),
                interaction.scale(),
                replacement_ready,
            ) {
                if !matches!(*visibility, Visibility::Hidden) {
                    *visibility = Visibility::Hidden;
                }
                continue;
            }
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

        let parent_translation = if let Some(parent) = parent {
            let Ok(parent_transform) = parents.get(parent.0) else {
                *visibility = Visibility::Hidden;
                continue;
            };
            Some(parent_transform.translation)
        } else {
            None
        };
        let Some((translation, scale)) =
            scenery_projection_pose(&view, presentation, parent_translation)
        else {
            *visibility = Visibility::Hidden;
            continue;
        };
        if transform.translation != translation {
            transform.translation = translation;
        }
        transform.scale = Vec3::splat(scale);
        *visibility = Visibility::Inherited;
    }
}

/// Numerical far-field projection has no authority over scenery identity or
/// coverage; failure only suppresses this disposable presentation.
fn scenery_projection_pose(
    view: &UsfViewContext,
    presentation: &UsfSceneryPresentation,
    parent_translation: Option<Vec3>,
) -> Option<(Vec3, f32)> {
    let relative = presentation
        .anchor()
        .relative_at_scale_bounded(view.anchor(), presentation.scale(), SCENERY_RELATIVE_BOUND)
        .ok()?;
    if scenery_is_inside_near_field_exclusion(*presentation, relative) {
        return None;
    }
    let native_to_view = view.projection_factor_f64(presentation.scale())?;
    let raw_relative = view.project_relative_native_from_eye(relative, presentation.scale())?;
    let raw_distance = raw_relative.length();
    if !raw_distance.is_finite() {
        return None;
    }
    let shell = presentation.render_shell_radius();
    let compression = if raw_distance > f64::EPSILON {
        shell / (shell + raw_distance)
    } else {
        1.0
    };
    let scale = (native_to_view * compression) as f32;
    if !scale.is_finite() || scale <= f32::EPSILON {
        return None;
    }
    let projected = raw_relative * compression;
    let projected = Vec3::new(projected.x as f32, projected.y as f32, projected.z as f32);
    if !projected.is_finite() {
        return None;
    }
    let global = view.presentation_origin() + projected;
    Some((
        local_translation_from_global(global, parent_translation),
        scale,
    ))
}

/// Contextual Scale presentation is view-owned; physical interaction Scale
/// selection remains outside this projection calculation.
fn contextual_scale_projection_pose(
    view: &UsfViewContext,
    presentation: &UsfScalePresentation,
    parent_translation: Option<Vec3>,
    require_context_scale: bool,
) -> Option<(Vec3, f32)> {
    if require_context_scale && !view.context_scale_eligible(presentation.scale()) {
        return None;
    }
    let relative = presentation
        .anchor()
        .relative_at_scale_bounded(
            view.anchor(),
            presentation.scale(),
            PRESENTATION_RELATIVE_BOUND,
        )
        .ok()?;
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
    if !global.is_finite() {
        return None;
    }
    Some((
        local_translation_from_global(global, parent_translation),
        factor,
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
        let Some((translation, scale)) = contextual_scale_projection_pose(
            &view,
            presentation,
            parent_translation,
            capability.is_none(),
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
