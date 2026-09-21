//! ECS realization of observer-relative USF presentation state.

use super::*;

/// Keeps the view anchored to an ordinary bounded runtime transform while
/// deriving its semantic position through the current local physical frame.
pub(in crate::spatial) fn sync_view_context(
    active: Res<UsfActiveScaleLayer>,
    frame: Res<UsfSpatialFrame>,
    semantic_anchors: Query<&Transform, With<UsfViewAnchor>>,
    observer: Single<
        (&Transform, &mut UsfViewContext),
        With<UsfViewRenderAnchor>,
    >,
) {
    let mut semantic_anchors = semantic_anchors.iter();
    let Some(semantic_anchor) = semantic_anchors.next() else {
        return;
    };
    if semantic_anchors.next().is_some() {
        error!("primary USF view has multiple semantic anchors");
        return;
    }

    let (render_anchor, mut view) = observer.into_inner();

    let Ok(canonical) = frame
        .origin()
        .translated_at_scale(active.scale(), semantic_anchor.translation)
    else {
        error!(
            local_anchor = ?semantic_anchor.translation,
            "USF semantic view anchor could not project into canonical space"
        );
        return;
    };

    if view.anchor != canonical
        || view.runtime_anchor != semantic_anchor.translation
        || view.render_anchor != render_anchor.translation
    {
        view.anchor = canonical;
        view.runtime_anchor = semantic_anchor.translation;
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
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    frame: Res<UsfSpatialFrame>,
    parents: Query<
        (&Transform, &UsfScaleLayer, Option<&UsfFollowsActiveScale>),
        Without<UsfLocalScalePresentation>,
    >,
    mut presentations: Query<(
        &mut UsfLocalScalePresentation,
        &ChildOf,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    let render_scale = view.render_scale();

    for (mut presentation, parent, mut transform, mut visibility) in &mut presentations {
        let Ok((parent_transform, layer, follows_active)) = parents.get(parent.0) else {
            continue;
        };
        if presentation.scale() != layer.scale() {
            presentation.set_scale(layer.scale());
        }

        // Active-chart followers (player model, etc.) remain visible. Persistent
        // scale-local worlds get exactly one opaque depth owner. Resident
        // adjacent worlds stay hot for handoff but are not drawn simultaneously.
        if follows_active.is_none() && layer.scale() != render_scale {
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

        // Semantic distance is measured from the semantic observer, but visual
        // placement is centered around the actual render camera. Camera boom
        // offsets therefore remain presentation-only.
        let desired_global = view.render_anchor()
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
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    active: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    mut presentations: Query<(
        &UsfSceneryPresentation,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    let active_scale = active.scale();
    let observer_absolute = frames.absolute(active_scale, view.runtime_anchor());

    for (presentation, mut transform, mut visibility) in &mut presentations {
        let observer_in_scale =
            frames.convert_absolute(observer_absolute, active_scale, presentation.scale());
        let relative = presentation.absolute() - observer_in_scale;

        let exponent_delta =
            f64::from(presentation.scale().exponent()) - f64::from(view.continuous_exponent());
        let native_to_view = 10.0_f64.powf(exponent_delta);
        let raw_relative = relative * native_to_view;
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

        transform.translation = view.render_anchor() + projected;
        transform.scale = Vec3::splat(projected_scale);
        *visibility = Visibility::Inherited;
    }
}

pub(in crate::spatial) fn project_scale_presentations(
    view: Single<&UsfViewContext, With<UsfViewRenderAnchor>>,
    parents: Query<&Transform, Without<UsfScalePresentation>>,
    mut presentations: Query<(
        &UsfScalePresentation,
        Option<&ChildOf>,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    for (presentation, parent, mut transform, mut visibility) in &mut presentations {
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
        let desired_global = view.render_anchor() + relative * factor;

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
