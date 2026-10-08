//! Capture active camera state after Bevy updates frusta.

use super::{UsfViewDemand, UsfViewDemandMode, UsfViewDemandPolicy, UsfViewDemandSnapshot};
use crate::spatial::{UsfViewContext, UsfViewRenderAnchor};
use bevy::{
    camera::{Projection, primitives::Frustum},
    prelude::*,
};

pub(super) fn capture_view_demand(
    policy: Res<UsfViewDemandPolicy>,
    views: Query<
        (
            Entity,
            &Frustum,
            &Transform,
            &Camera,
            &Projection,
            &UsfViewContext,
        ),
        With<UsfViewRenderAnchor>,
    >,
    mut snapshot: ResMut<UsfViewDemandSnapshot>,
) {
    match policy.mode() {
        UsfViewDemandMode::Frozen => return,
        UsfViewDemandMode::Disabled => {
            snapshot.scratch.clear();
            if !snapshot.entries.is_empty() {
                snapshot.entries.clear();
                snapshot.revision = snapshot.revision.wrapping_add(1).max(1);
            }
            return;
        }
        UsfViewDemandMode::Live => {}
    }

    // Bevy change ticks are intentionally not used as semantic invalidation.
    // Camera synchronization may perform idempotent mutable writes; observer
    // demand only changes when values that can alter culling actually differ.
    let snapshot = &mut *snapshot;
    snapshot.scratch.clear();
    let view_count = views.iter().len();
    if snapshot.scratch.capacity() < view_count {
        // `reserve` is relative to len (zero after clear), so request the full
        // desired cardinality rather than the capacity delta.
        snapshot.scratch.reserve(view_count);
    }

    for (source, frustum, transform, camera, projection, view) in &views {
        if !camera.is_active {
            continue;
        }

        let (perspective, perspective_fov, perspective_aspect_ratio, pixels_per_radian) =
            match projection {
                Projection::Perspective(perspective) => {
                    let pixels_per_radian = camera
                        .logical_viewport_size()
                        .filter(|size| size.y > 0.0 && perspective.fov > 0.0)
                        .map(|size| size.y / perspective.fov);
                    (
                        true,
                        Some(perspective.fov),
                        Some(perspective.aspect_ratio),
                        pixels_per_radian,
                    )
                }
                _ => (false, None, None, None),
            };

        snapshot.scratch.push(UsfViewDemand {
            source,
            anchor: *view.anchor(),
            velocity_metres_per_second: view.velocity_metres_per_second(),
            projection_eye_offset_metres: view.projection_eye_offset_metres(),
            finest_scale: view.scale(),
            camera_translation: transform.translation,
            camera_rotation: transform.rotation,
            frustum: frustum.clone(),
            perspective,
            perspective_fov,
            perspective_aspect_ratio,
            pixels_per_radian,
        });
    }

    snapshot.scratch.sort_by_key(|entry| entry.source.to_bits());

    let observer_changed = snapshot.scratch.len() != snapshot.entries.len()
        || snapshot
            .scratch
            .iter()
            .zip(snapshot.entries.iter())
            .any(|(next, current)| !next.same_observer_state(current));
    let motion_changed = snapshot.scratch.len() == snapshot.entries.len()
        && snapshot
            .scratch
            .iter()
            .zip(snapshot.entries.iter())
            .any(|(next, current)| {
                next.velocity_metres_per_second != current.velocity_metres_per_second
            });

    if !observer_changed && !motion_changed {
        return;
    }
    std::mem::swap(&mut snapshot.entries, &mut snapshot.scratch);
    if observer_changed {
        snapshot.revision = snapshot.revision.wrapping_add(1).max(1);
    }
}
