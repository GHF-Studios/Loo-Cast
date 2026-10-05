//! Conservative swept-collision candidates from semantic voxel authority.
//!
//! This provider deliberately bypasses dense materializations, Surface Nets and
//! Avian colliders. It answers only the broad question "which canonical sweep
//! interval may contain rigid voxel matter?".
//!
//! False positives are valid and expected. A later refinement stage samples the
//! actual field and narrows the bracket. False negatives are not acceptable:
//! local collider residency must never decide whether high-speed collision
//! exists.

use bevy::{
    ecs::system::SystemParam,
    math::DVec3,
    prelude::*,
};

use crate::{
    physics::collision_query::{
        UsfCanonicalSweep, UsfCollisionCandidate, UsfSweepInterval,
    },
    spatial::{SpatialScale, UsfPosition, UsfSemanticFrame},
};

use super::{CelestialVoxelField, VoxelAuthority, VoxelBounds, VoxelFrameSnapshot};

#[derive(SystemParam)]
pub struct VoxelCollisionQuery<'w, 's> {
    authorities: Query<
        'w,
        's,
        (
            Entity,
            &'static UsfPosition,
            &'static UsfSemanticFrame,
            &'static CelestialVoxelField,
            Option<&'static VoxelAuthority>,
        ),
    >,
}

impl VoxelCollisionQuery<'_, '_> {
    /// Conservative voxel candidates for one canonical swept volume.
    ///
    /// The returned candidates are query evidence only. Neither this provider
    /// nor [`UsfCollisionCandidate`] owns collision response.
    pub fn candidates(
        &self,
        sweep: UsfCanonicalSweep,
    ) -> Vec<UsfCollisionCandidate> {
        let mut candidates = Vec::new();

        for (authority_entity, body_origin, body_frame, field, edits) in &self.authorities {
            // Candidate metadata reports the coarsest semantic detail slice,
            // while the bound itself includes every finer possible detail band.
            let query_scale = field.coarsest_detail_scale();
            let outer_radius =
                field.conservative_outer_radius_metres() + sweep.bounding_radius_metres();

            if let Ok(start_from_center) = sweep.start().relative_at_scale_bounded_f64(
                body_origin,
                SpatialScale::ZERO,
                f64::MAX,
            ) {
                if let Some(interval) = segment_sphere_interval(
                    start_from_center,
                    sweep.displacement_metres(),
                    outer_radius,
                ) {
                    candidates.push(candidate_for_interval(
                        authority_entity,
                        query_scale,
                        sweep,
                        interval,
                    ));
                }
            }

            let Some(edits) = edits else {
                continue;
            };

            // Every semantic edit influence is included independently. Add
            // edits may create rigid matter outside the body's procedural
            // envelope. Remove/paint edits can over-report here; refinement is
            // responsible for proving whether matter actually remains.
            let snapshot = VoxelFrameSnapshot::new(*body_origin, *body_frame, SpatialScale::ZERO);
            for edit in edits.edits() {
                let Ok(bounds) = edit.world_bounds(snapshot) else {
                    continue;
                };
                if let Some(interval) =
                    segment_bounds_interval(sweep, bounds)
                {
                    candidates.push(candidate_for_interval(
                        authority_entity,
                        query_scale,
                        sweep,
                        interval,
                    ));
                }
            }
        }

        candidates.sort_by(|a, b| {
            a.interval()
                .minimum()
                .total_cmp(&b.interval().minimum())
                .then_with(|| {
                    a.interval()
                        .maximum()
                        .total_cmp(&b.interval().maximum())
                })
                .then_with(|| a.authority().to_bits().cmp(&b.authority().to_bits()))
        });
        candidates
    }
}

fn candidate_for_interval(
    authority: Entity,
    scale: SpatialScale,
    sweep: UsfCanonicalSweep,
    interval: UsfSweepInterval,
) -> UsfCollisionCandidate {
    let swept_metres = sweep.displacement_metres().length();
    let interval_uncertainty = swept_metres * interval.width();

    UsfCollisionCandidate::new(
        authority,
        scale,
        interval,
        interval_uncertainty.max(scale.metres_per_native()),
    )
}

fn segment_sphere_interval(
    start: DVec3,
    displacement: DVec3,
    radius: f64,
) -> Option<UsfSweepInterval> {
    if !start.is_finite()
        || !displacement.is_finite()
        || !radius.is_finite()
        || radius < 0.0
    {
        return None;
    }

    let a = displacement.length_squared();
    let c = start.length_squared() - radius * radius;

    if a <= f64::EPSILON {
        return (c <= 0.0)
            .then(|| UsfSweepInterval::new(0.0, 0.0))
            .flatten();
    }

    let b = 2.0 * start.dot(displacement);
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return None;
    }

    let root = discriminant.sqrt();
    let denominator = 2.0 * a;
    let t0 = (-b - root) / denominator;
    let t1 = (-b + root) / denominator;
    let minimum = t0.min(t1).max(0.0);
    let maximum = t0.max(t1).min(1.0);

    UsfSweepInterval::new(minimum, maximum)
}

fn segment_bounds_interval(
    sweep: UsfCanonicalSweep,
    bounds: VoxelBounds,
) -> Option<UsfSweepInterval> {
    let anchor = bounds.anchor().usf();
    let scale = anchor.leaf_scale();

    let start = sweep
        .start()
        .relative_at_scale_bounded_f64(
            &anchor,
            scale,
            f64::MAX,
        )
        .ok()?;
    let displacement =
        sweep.displacement_metres() / scale.metres_per_native();
    let expansion =
        scale.metres_to_native_f64(sweep.bounding_radius_metres());

    let minimum = DVec3::new(
        f64::from(bounds.min_offset().x) - expansion,
        f64::from(bounds.min_offset().y) - expansion,
        f64::from(bounds.min_offset().z) - expansion,
    );
    let maximum = DVec3::new(
        f64::from(bounds.max_offset().x) + expansion,
        f64::from(bounds.max_offset().y) + expansion,
        f64::from(bounds.max_offset().z) + expansion,
    );

    segment_aabb_interval(start, displacement, minimum, maximum)
}

fn segment_aabb_interval(
    start: DVec3,
    displacement: DVec3,
    minimum: DVec3,
    maximum: DVec3,
) -> Option<UsfSweepInterval> {
    if !start.is_finite()
        || !displacement.is_finite()
        || !minimum.is_finite()
        || !maximum.is_finite()
    {
        return None;
    }

    let mut enter = 0.0_f64;
    let mut exit = 1.0_f64;

    for (origin, delta, low, high) in [
        (start.x, displacement.x, minimum.x, maximum.x),
        (start.y, displacement.y, minimum.y, maximum.y),
        (start.z, displacement.z, minimum.z, maximum.z),
    ] {
        if delta.abs() <= f64::EPSILON {
            if origin < low || origin > high {
                return None;
            }
            continue;
        }

        let inverse = 1.0 / delta;
        let a = (low - origin) * inverse;
        let b = (high - origin) * inverse;
        enter = enter.max(a.min(b));
        exit = exit.min(a.max(b));

        if enter > exit {
            return None;
        }
    }

    UsfSweepInterval::new(enter.max(0.0), exit.min(1.0))
}

pub(in crate::voxel) fn publish_collision_query_candidates(
    provider: VoxelCollisionQuery,
    mut frame: ResMut<crate::physics::collision_query::UsfCollisionQueryFrame>,
) {
    let requests = frame.requests().collect::<Vec<_>>();

    for request in requests {
        for candidate in provider.candidates(request.sweep()) {
            frame.push_candidate(request.id(), candidate);
        }
    }
}
