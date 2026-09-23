//! ECS realization of observer-relative USF presentation state.

use super::*;
use bevy::camera::visibility::RenderLayers;
use crate::{
    ecs::UsfManifestationOf,
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
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    interaction: Res<UsfPrimaryInteractionSlice>,
    frame: Res<UsfSpatialFrame>,
    parents: Query<
        (
            &Transform,
            &UsfScaleLayer,
            Option<&UsfInteractionProjection>,
            Option<&UsfScaleFallbackPresentation>,
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
    )>,
) {
    for (entity, mut presentation, parent, mut transform, mut visibility, render_layers) in
        &mut presentations
    {
        let Ok((parent_transform, layer, follows_active, fallback)) = parents.get(parent.0) else {
            continue;
        };
        if presentation.scale() != layer.scale() {
            presentation.set_scale(layer.scale());
        }

        // Subject-local presentations keep their dedicated self-visibility
        // policy. Persistent scale-world geometry is split by responsibility:
        // the active physical interaction slice is rendered by the local camera;
        // non-active/coarser stack members are rendered by the semantic-origin
        // USF projection camera.
        if follows_active.is_none() {
            let desired_layers = if layer.scale() == interaction.scale() {
                RenderLayers::default()
            } else {
                RenderLayers::layer(USF_PRESENTATION_LAYER)
            };
            if render_layers.is_none_or(|current| *current != desired_layers) {
                commands.entity(entity).insert(desired_layers);
            }
        }

        // Active-chart followers (player model, etc.) remain visible.
        //
        // Persistent scale-local worlds are fundamentally NOT mutually-exclusive
        // LOD levels. The visible world is an additive nested stack:
        //
        //   coarse domain
        //     minus finer refinement aperture
        //       plus finer domain
        //
        // Until the aperture compositor is in place, keep every requested stack
        // layer visible so refinement can never create a global terrain void.
        // Coarse/fine overlap is preferable to deleting the parent world.
        let far_fallback = fallback.is_some_and(|fallback| {
            layer.scale() == fallback.scale()
                && view.continuous_exponent() >= f32::from(fallback.scale().exponent())
        });
        let presentation_demands_scale = view
            .active_scale_demands()
            .into_iter()
            .flatten()
            .any(|demand| {
                demand.scale() == layer.scale()
                    && demand.contribution() > CONTRIBUTION_EPSILON
            });
        let participates_in_stack = layer.scale() >= interaction.target_scale()
            || presentation_demands_scale
            || far_fallback;

        if follows_active.is_none() && !participates_in_stack {
            if !matches!(*visibility, Visibility::Hidden) {
                *visibility = Visibility::Hidden;
            }
            continue;
        }

        let observer_in_parent_chart = if follows_active.is_some() {
            view.runtime_anchor()
        } else {
            // Canonical subtraction first, float projection last.
            let Ok(relative) = view.anchor().relative_at_scale_bounded(
                frame.origin(),
                layer.scale(),
                PRESENTATION_RELATIVE_BOUND,
            ) else {
                *visibility = Visibility::Hidden;
                continue;
            };
            relative
        };

        let factor = view.projection_factor(layer.scale());

        // Semantic distance is measured and rendered around the semantic view
        // anchor. Camera eye/boom offsets remain presentation-only because the
        // camera moves independently through this projected scene.
        let desired_global = view.presentation_origin()
            + (parent_transform.translation - observer_in_parent_chart) * factor;
        let delta = desired_global - parent_transform.translation;
        let desired_translation = parent_transform.rotation.inverse() * delta;
        let desired_scale = Vec3::splat(factor);

        if transform.translation != desired_translation {
            transform.translation = desired_translation;
        }
        if transform.scale != desired_scale {
            transform.scale = desired_scale;
        }
        if !matches!(*visibility, Visibility::Inherited) {
            *visibility = Visibility::Inherited;
        }
    }
}

/// Projects persistent multiscale scenery into one bounded render scene.
///
/// For a raw observer-relative distance `d`, the rendered radius is
/// `R * d / (R + d)`. The same compression is applied to object scale, preserving
/// angular size while keeping arbitrarily distant representations inside `R`.
pub(in crate::spatial) fn project_scenery_presentations(
    mut commands: Commands,
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    mut presentations: Query<(
        Entity,
        &UsfSceneryPresentation,
        &mut Transform,
        &mut Visibility,
        Option<&RenderLayers>,
    )>,
) {
    for (entity, presentation, mut transform, mut visibility, render_layers) in
        &mut presentations
    {
        let desired_layers = RenderLayers::layer(USF_PRESENTATION_LAYER);
        if render_layers.is_none_or(|current| *current != desired_layers) {
            commands.entity(entity).insert(desired_layers);
        }
        let Ok(relative) = presentation.anchor().relative_at_scale_bounded(
            view.anchor(),
            presentation.scale(),
            SCENERY_RELATIVE_BOUND,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };

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

        transform.translation = view.presentation_origin() + projected;
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
    )>,
) {
    for (entity, presentation, parent, mut transform, mut visibility, render_layers) in
        &mut presentations
    {
        let desired_layers = RenderLayers::layer(USF_PRESENTATION_LAYER);
        if render_layers.is_none_or(|current| *current != desired_layers) {
            commands.entity(entity).insert(desired_layers);
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
