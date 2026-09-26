//! ECS realization of observer-relative USF presentation state.

use super::*;
use bevy::{
    camera::visibility::RenderLayers,
    light::{NotShadowCaster, NotShadowReceiver},
};
use crate::{
    ecs::{UsfManifestationOf, UsfPresentationProjectionOf},
    spatial::{UsfPrimaryInteractionSlice, UsfScaleCoverageSnapshot, UsfScaleRoleMask},
    view::USF_PRESENTATION_LAYER,
};

/// Keeps the view anchored to an ordinary bounded runtime transform while
/// deriving its semantic position through the current local physical frame.
pub(in crate::spatial) fn sync_view_context(
    semantic_anchors: Query<(&Transform, &UsfManifestationOf), With<UsfViewAnchor>>,
    semantic_positions: Query<&UsfPosition>,
    observer: Single<
        (&Transform, &mut UsfViewContext),
        With<UsfViewRenderAnchor>,
    >,
) {
    let mut semantic_anchors = semantic_anchors.iter();
    let Some((runtime_anchor, manifestation)) = semantic_anchors.next() else {
        return;
    };
    if semantic_anchors.next().is_some() {
        error!("primary USF view has multiple semantic anchors");
        return;
    }

    let Ok(&canonical) = semantic_positions.get(manifestation.0) else {
        error!(
            subject = ?manifestation.0,
            "USF semantic view anchor has no canonical position"
        );
        return;
    };

    let (render_anchor, mut view) = observer.into_inner();
    if view.anchor != canonical
        || view.runtime_anchor != runtime_anchor.translation
        || view.render_anchor != render_anchor.translation
    {
        view.anchor = canonical;
        view.runtime_anchor = runtime_anchor.translation;
        view.render_anchor = render_anchor.translation;
    }
}

/// Projects scale-authored presentation geometry around the observer without
/// modifying logical/physics transforms.
///
/// Current child presentations (voxel surfaces) have root parents with identity
/// rotation/scale, so child compensation is translation-only. General rotated
/// representation frames can later promote this to an explicit projection frame.
pub(in crate::spatial) fn project_local_scale_presentations(
    mut commands: Commands,
    interaction: Res<UsfPrimaryInteractionSlice>,
    parents: Query<
        (&UsfScaleLayer, Option<&UsfInteractionProjection>),
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
        let Ok((layer, follows_active)) = parents.get(parent.0) else {
            continue;
        };

        if presentation.scale() != layer.scale() {
            presentation.set_scale(layer.scale());
        }

        let physical_local =
            follows_active.is_some() || layer.scale() == interaction.scale();

        if !physical_local {
            // Non-active voxel realizations may remain resident for coverage,
            // handoff readiness and cache locality, but they are not a second
            // visual surface. Whole-body far appearance has a separate realizer.
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

    // Presentation may move finer before physical interaction. Keep a visible
    // whole-body representation until interaction itself enters the voxel
    // ladder and its local presentation is actually published.
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
fn local_translation_from_global(
    desired_global: Vec3,
    parent_translation: Option<Vec3>,
) -> Vec3 {
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
    coverage: Res<UsfScaleCoverageSnapshot>,
    manifestations: Query<&UsfManifestationOf>,
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
        if let Some(fallback) = fallback {
            let replacement_ready = projection
                .and_then(|projection| manifestations.get(projection.0).ok())
                .is_some_and(|manifestation| {
                    coverage.has_near_for_authority(
                        manifestation.0,
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

        let Ok(relative) = presentation.anchor().relative_at_scale_bounded(
            view.anchor(),
            presentation.scale(),
            SCENERY_RELATIVE_BOUND,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };

        // Radial-distance compression is a far-field illusion. Letting a huge
        // body's compressed proxy surround the observer turns a macro visual
        // approximation into fake local terrain. Inside its declared invalid
        // region, expose missing fine presentation rather than drawing a
        // counterfeit surface.
        if scenery_is_inside_near_field_exclusion(*presentation, relative) {
            *visibility = Visibility::Hidden;
            continue;
        }

        let exponent_delta =
            f64::from(presentation.scale().exponent()) - f64::from(view.continuous_exponent());
        let native_to_view = 10.0_f64.powf(exponent_delta);
        let raw_relative = DVec3::new(
            f64::from(relative.x) * native_to_view,
            f64::from(relative.y) * native_to_view,
            f64::from(relative.z) * native_to_view,
        );
        let raw_distance = raw_relative.length();

        if !raw_distance.is_finite() || !native_to_view.is_finite() {
            *visibility = Visibility::Hidden;
            continue;
        }

        let shell = presentation.render_shell_radius();
        let compression = if raw_distance > f64::EPSILON {
            shell / (shell + raw_distance)
        } else {
            1.0
        };
        let projected_scale = (native_to_view * compression) as f32;

        if !projected_scale.is_finite() || projected_scale <= f32::EPSILON {
            *visibility = Visibility::Hidden;
            continue;
        }

        let projected = raw_relative * compression;
        let projected = Vec3::new(projected.x as f32, projected.y as f32, projected.z as f32);
        if !projected.is_finite() {
            *visibility = Visibility::Hidden;
            continue;
        }

        let desired_global = view.presentation_origin() + projected;
        let parent_translation = if let Some(parent) = parent {
            let Ok(parent_transform) = parents.get(parent.0) else {
                *visibility = Visibility::Hidden;
                continue;
            };
            Some(parent_transform.translation)
        } else {
            None
        };
        let desired_translation =
            local_translation_from_global(desired_global, parent_translation);

        if transform.translation != desired_translation {
            transform.translation = desired_translation;
        }
        transform.scale = Vec3::splat(projected_scale);
        *visibility = Visibility::Inherited;
    }
}

pub(in crate::spatial) fn project_scale_presentations(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    parents: Query<&Transform, Without<UsfScalePresentation>>,
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

        let contribution = view.contribution(presentation.scale());
        if contribution <= CONTRIBUTION_EPSILON {
            if !matches!(*visibility, Visibility::Hidden) {
                *visibility = Visibility::Hidden;
            }
            continue;
        }

        let Ok(relative) = presentation.anchor().relative_at_scale_bounded(
            view.anchor(),
            presentation.scale(),
            PRESENTATION_RELATIVE_BOUND,
        ) else {
            if !matches!(*visibility, Visibility::Hidden) {
                *visibility = Visibility::Hidden;
            }
            continue;
        };

        let factor = view.projection_factor(presentation.scale());
        let desired_global = view.presentation_origin() + relative * factor;

        let desired_translation = if let Some(parent) = parent {
            let Ok(parent_transform) = parents.get(parent.0) else {
                if !matches!(*visibility, Visibility::Hidden) {
                    *visibility = Visibility::Hidden;
                }
                continue;
            };
            desired_global - parent_transform.translation
        } else {
            desired_global
        };
        if transform.translation != desired_translation {
            transform.translation = desired_translation;
        }
        let desired_scale = Vec3::splat(factor);
        if transform.scale != desired_scale {
            transform.scale = desired_scale;
        }
        if !matches!(*visibility, Visibility::Inherited) {
            *visibility = Visibility::Inherited;
        }
    }
}
#[cfg(test)]
mod fallback_handoff_tests {
    use super::*;

    #[test]
    fn fallback_survives_fine_view_until_replacement_is_ready() {
        let fallback = UsfScaleFallbackPresentation::new(SpatialScale::new(5).unwrap());
        let fine = SpatialScale::ZERO;
        assert!(fallback_should_render(fallback, fine, fine, false));
        assert!(!fallback_should_render(fallback, fine, fine, true));
    }

    #[test]
    fn fallback_survives_when_interaction_has_not_entered_voxel_ladder() {
        let fallback = UsfScaleFallbackPresentation::new(SpatialScale::new(5).unwrap());
        assert!(fallback_should_render(
            fallback,
            SpatialScale::ZERO,
            SpatialScale::new(35).unwrap(),
            false,
        ));
    }

    #[test]
    fn compressed_scenery_respects_near_field_validity_domain() {
        let scale = SpatialScale::ZERO;
        let presentation =
            UsfSceneryPresentation::from_anchor(UsfPosition::zero(scale), scale)
                .with_near_field_exclusion_radius_native(100.0);

        assert!(scenery_is_inside_near_field_exclusion(
            presentation,
            Vec3::new(99.0, 0.0, 0.0),
        ));
        assert!(!scenery_is_inside_near_field_exclusion(
            presentation,
            Vec3::new(101.0, 0.0, 0.0),
        ));
    }

    #[test]
    fn child_scenery_preserves_global_pose_across_parent_rebase() {
        let desired_global = Vec3::new(12.5, -33.0, 4.25);
        let shift = Vec3::new(0.0, 256.0, 0.0);

        let parent_before = Vec3::ZERO;
        let local_before =
            local_translation_from_global(desired_global, Some(parent_before));

        // rebase_local_frame applies `translation -= shift` to this top-level,
        // layerless parent. The child projection must compensate in local space
        // so its runtime-global pose remains unchanged.
        let parent_after = parent_before - shift;
        let local_after =
            local_translation_from_global(desired_global, Some(parent_after));

        assert_eq!(parent_before + local_before, desired_global);
        assert_eq!(parent_after + local_after, desired_global);
        assert_eq!(local_after - local_before, shift);
    }
}
