//! ECS realization of observer-relative USF presentation state.

use super::*;

pub(in crate::spatial) fn configure(app: &mut App) {
    app.init_resource::<UsfViewFrame>();
}

/// Keeps the view anchored to an ordinary bounded runtime transform while
/// deriving its semantic position through the current local physical frame.
pub(in crate::spatial) fn sync_view_anchor(
    frame: Res<UsfSpatialFrame>,
    anchors: Query<&Transform, With<UsfViewAnchor>>,
    mut view: ResMut<UsfViewFrame>,
) {
    let Some(anchor) = anchors.iter().next() else {
        return;
    };

    let Ok(canonical) = frame.origin().translated_native(anchor.translation) else {
        error!(
            local_anchor = ?anchor.translation,
            "USF view anchor could not project into canonical space"
        );
        return;
    };

    if view.anchor != canonical || view.runtime_anchor != anchor.translation {
        view.anchor = canonical;
        view.runtime_anchor = anchor.translation;
    }
}

/// Projects scale-authored presentation geometry around the observer without
/// modifying logical/physics transforms.
///
/// Current child presentations (voxel surfaces) have root parents with identity
/// rotation/scale, so child compensation is translation-only. General rotated
/// representation frames can later promote this to an explicit projection frame.
pub(in crate::spatial) fn project_local_scale_presentations(
    view: Res<UsfViewFrame>,
    active: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    parents: Query<
        (&Transform, &UsfScaleLayer, Option<&UsfFollowsActiveScale>),
        Without<UsfLocalScalePresentation>,
    >,
    mut presentations: Query<(&mut UsfLocalScalePresentation, &ChildOf, &mut Transform)>,
) {
    let active_scale = active.scale();
    let observer_absolute = frames.absolute(active_scale, view.runtime_anchor());

    for (mut presentation, parent, mut transform) in &mut presentations {
        let Ok((parent_transform, layer, follows_active)) = parents.get(parent.0) else {
            continue;
        };
        if presentation.scale() != layer.scale() {
            presentation.set_scale(layer.scale());
        }

        let observer_in_parent_chart = if follows_active.is_some() {
            view.runtime_anchor()
        } else {
            let converted = frames.convert_absolute(observer_absolute, active_scale, layer.scale());
            frames.runtime_from_absolute(layer.scale(), converted)
        };

        let factor = view.projection_factor(layer.scale());
        let desired_global = view.runtime_anchor()
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
    }
}

pub(in crate::spatial) fn project_scale_presentations(
    view: Res<UsfViewFrame>,
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
        let desired_global = view.runtime_anchor() + relative * factor;

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
