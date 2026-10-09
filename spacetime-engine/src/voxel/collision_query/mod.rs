//! Canonical swept collision against semantic voxel authority.
//!
//! This provider bypasses dense materializations, Surface Nets and Avian
//! colliders. A conservative bounds pass finds possible contact intervals; a
//! bounded semantic-field trace resolves them. Candidate intervals remain
//! observations, while an incomplete or exhausted trace returns `Unknown`.
//! Local collider residency never decides whether high-speed collision exists.

use bevy::{ecs::system::SystemParam, math::DVec3, prelude::*};

use crate::{
    physics::collision_query::{
        UsfCanonicalSweep, UsfCollisionCandidate, UsfSweepInterval, UsfSweepResolution,
    },
    spatial::{SpatialScale, UsfPosition, UsfSemanticFrame},
};

use super::{
    CelestialVoxelField, VoxelBounds, VoxelFrameEdit, VoxelFrameSnapshot, VoxelSemanticAuthority,
};

mod resolve;

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
            Option<&'static VoxelSemanticAuthority>,
        ),
    >,
}

impl VoxelCollisionQuery<'_, '_> {
    /// Conservative voxel candidates for one canonical swept volume.
    ///
    /// The returned candidates are query evidence only. Neither this provider
    /// nor [`UsfCollisionCandidate`] owns collision response.
    pub fn candidates(&self, sweep: UsfCanonicalSweep) -> Vec<UsfCollisionCandidate> {
        self.collect_candidates(sweep).0
    }

    pub(super) fn collect_candidates(
        &self,
        sweep: UsfCanonicalSweep,
    ) -> (Vec<UsfCollisionCandidate>, bool) {
        let mut candidates = Vec::new();
        let mut complete = true;

        for (authority_entity, body_origin, body_frame, field, edits) in &self.authorities {
            // Candidate metadata reports the coarsest semantic detail slice,
            // while the bound itself includes every finer possible detail band.
            let query_scale = field.coarsest_detail_scale();
            let outer_radius =
                field.conservative_outer_radius_metres() + sweep.bounding_radius_metres();

            match sweep.start().relative_at_scale_bounded_f64(
                body_origin,
                SpatialScale::ZERO,
                f64::MAX,
            ) {
                Ok(start_from_center) if start_from_center.is_finite() => {
                    match segment_sphere_interval(
                        start_from_center,
                        sweep.displacement_metres(),
                        outer_radius,
                    ) {
                        Ok(Some(interval)) => candidates.push(candidate_for_interval(
                            authority_entity,
                            query_scale,
                            sweep,
                            interval,
                        )),
                        Ok(None) => {}
                        Err(()) => complete = false,
                    }
                }
                Ok(_) | Err(_) => complete = false,
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
                    complete = false;
                    continue;
                };
                match segment_bounds_interval(sweep, bounds) {
                    Ok(Some(interval)) => candidates.push(candidate_for_interval(
                        authority_entity,
                        query_scale,
                        sweep,
                        interval,
                    )),
                    Ok(None) => {}
                    Err(()) => complete = false,
                }
            }
        }

        candidates.sort_by(|a, b| {
            a.interval()
                .minimum()
                .total_cmp(&b.interval().minimum())
                .then_with(|| a.interval().maximum().total_cmp(&b.interval().maximum()))
                .then_with(|| a.authority().to_bits().cmp(&b.authority().to_bits()))
        });
        (candidates, complete)
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
) -> Result<Option<UsfSweepInterval>, ()> {
    if !start.is_finite() || !displacement.is_finite() || !radius.is_finite() || radius < 0.0 {
        return Err(());
    }

    let a = displacement.length_squared();
    let c = start.length_squared() - radius * radius;
    if !a.is_finite() || !c.is_finite() {
        return Err(());
    }

    if a <= f64::EPSILON {
        return Ok((c <= 0.0)
            .then(|| UsfSweepInterval::new(0.0, 0.0))
            .flatten());
    }

    let b = 2.0 * start.dot(displacement);
    let discriminant = b * b - 4.0 * a * c;
    if !b.is_finite() || !discriminant.is_finite() {
        return Err(());
    }
    if discriminant < 0.0 {
        return Ok(None);
    }

    let root = discriminant.sqrt();
    let denominator = 2.0 * a;
    let t0 = (-b - root) / denominator;
    let t1 = (-b + root) / denominator;
    if !t0.is_finite() || !t1.is_finite() {
        return Err(());
    }
    let minimum = t0.min(t1).max(0.0);
    let maximum = t0.max(t1).min(1.0);

    Ok(UsfSweepInterval::new(minimum, maximum))
}

fn segment_bounds_interval(
    sweep: UsfCanonicalSweep,
    bounds: VoxelBounds,
) -> Result<Option<UsfSweepInterval>, ()> {
    let anchor = bounds.anchor().usf();
    let scale = anchor.leaf_scale();

    let start = sweep
        .start()
        .relative_at_scale_bounded_f64(&anchor, scale, f64::MAX)
        .map_err(|_| ())?;
    let displacement = sweep.displacement_metres() / scale.metres_per_native();
    let expansion = scale.metres_to_native_f64(sweep.bounding_radius_metres());

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
) -> Result<Option<UsfSweepInterval>, ()> {
    if !start.is_finite()
        || !displacement.is_finite()
        || !minimum.is_finite()
        || !maximum.is_finite()
    {
        return Err(());
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
                return Ok(None);
            }
            continue;
        }

        let inverse = 1.0 / delta;
        let a = (low - origin) * inverse;
        let b = (high - origin) * inverse;
        if !a.is_finite() || !b.is_finite() {
            return Err(());
        }
        enter = enter.max(a.min(b));
        exit = exit.min(a.max(b));

        if enter > exit {
            return Ok(None);
        }
    }

    Ok(UsfSweepInterval::new(enter.max(0.0), exit.min(1.0)))
}

pub(in crate::voxel) fn publish_collision_query_candidates(
    provider: VoxelCollisionQuery,
    mut frame: ResMut<crate::physics::collision_query::UsfCollisionQueryFrame>,
) {
    let requests = frame.requests().collect::<Vec<_>>();

    for request in requests {
        for candidate in provider.candidates(request.sweep()) {
            frame.publish_candidate(request.id(), candidate);
        }
        frame.publish_resolution(
            request.id(),
            provider.resolve(request.sweep(), request.target_error_metres()),
        );
    }
}
