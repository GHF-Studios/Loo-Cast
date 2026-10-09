//! Bounded canonical narrow phase over the semantic sphere and ordered edits.
//!
//! The field is sampled in body-local SI coordinates. Dense voxel chunks and
//! their Avian colliders do not determine whether this path is clear.

use super::*;

const MAX_TRACE_STEPS: usize = 256;

#[derive(Clone, Copy)]
enum LocalEdit {
    Add { center: DVec3, radius: f64 },
    Remove { center: DVec3, radius: f64 },
}

fn local_edits(edits: Option<&VoxelSemanticAuthority>) -> Option<Vec<LocalEdit>> {
    let mut result = Vec::new();
    for edit in edits.into_iter().flat_map(VoxelSemanticAuthority::edits) {
        let (brush, add) = match *edit {
            VoxelFrameEdit::Add { brush, .. } => (brush, true),
            VoxelFrameEdit::Remove { brush } => (brush, false),
            VoxelFrameEdit::Paint { .. } => continue,
        };
        let center = brush
            .center()
            .usf()
            .coordinate_at_scale_f64(SpatialScale::ZERO)
            .ok()?;
        let radius = brush.radius_metres();
        if !center.is_finite() || !radius.is_finite() {
            return None;
        }
        result.push(if add {
            LocalEdit::Add { center, radius }
        } else {
            LocalEdit::Remove { center, radius }
        });
    }
    Some(result)
}

fn field_distance(field: CelestialVoxelField, edits: &[LocalEdit], point: DVec3) -> Option<f64> {
    let mut distance = field.signed_distance_local_metres(point)?;
    for edit in edits {
        match *edit {
            LocalEdit::Add { center, radius } => {
                distance = distance.min(point.distance(center) - radius);
            }
            LocalEdit::Remove { center, radius } => {
                distance = distance.max(radius - point.distance(center));
            }
        }
    }
    distance.is_finite().then_some(distance)
}

fn contact_normal(
    field: CelestialVoxelField,
    edits: &[LocalEdit],
    point: DVec3,
    step_metres: f64,
) -> Option<DVec3> {
    let step = step_metres.max(0.01);
    let x = field_distance(field, edits, point + DVec3::X * step)?
        - field_distance(field, edits, point - DVec3::X * step)?;
    let y = field_distance(field, edits, point + DVec3::Y * step)?
        - field_distance(field, edits, point - DVec3::Y * step)?;
    let z = field_distance(field, edits, point + DVec3::Z * step)?
        - field_distance(field, edits, point - DVec3::Z * step)?;
    let normal = DVec3::new(x, y, z).normalize_or_zero();
    (normal != DVec3::ZERO && normal.is_finite()).then_some(normal)
}

fn trace_candidate(
    sweep: UsfCanonicalSweep,
    interval: UsfSweepInterval,
    field: CelestialVoxelField,
    edits: &[LocalEdit],
    start_local: DVec3,
    displacement_local: DVec3,
    target_error_metres: f64,
    authority: Entity,
) -> UsfSweepResolution {
    let length = displacement_local.length();
    if !length.is_finite() {
        return UsfSweepResolution::Unknown { safe_fraction: 0.0 };
    }
    let tolerance = target_error_metres.max(0.001);
    let mut fraction = interval.minimum();
    for _ in 0..MAX_TRACE_STEPS {
        let point = start_local + displacement_local * fraction;
        let Some(distance) = field_distance(field, edits, point) else {
            return UsfSweepResolution::Unknown {
                safe_fraction: fraction,
            };
        };
        let clearance = distance - sweep.bounding_radius_metres();
        if clearance <= tolerance {
            let normal = contact_normal(field, edits, point, tolerance).unwrap_or(DVec3::ZERO);
            return UsfSweepResolution::Contact {
                safe_fraction: (fraction - tolerance / length.max(1.0)).max(0.0),
                authority,
                normal,
            };
        }
        if length <= f64::EPSILON {
            return UsfSweepResolution::Clear;
        }
        let next = fraction + (clearance - tolerance).max(tolerance) / length;
        if next > interval.maximum() || next >= 1.0 {
            return UsfSweepResolution::Clear;
        }
        if next <= fraction {
            return UsfSweepResolution::Unknown {
                safe_fraction: fraction,
            };
        }
        fraction = next;
    }
    UsfSweepResolution::Unknown {
        safe_fraction: fraction,
    }
}

impl VoxelCollisionQuery<'_, '_> {
    /// Resolve voxel contact for one proposed physical step. Every provider
    /// candidate is narrowed against the semantic field; an exhausted or
    /// unrepresentable query stays unknown rather than becoming clear.
    pub fn resolve(
        &self,
        sweep: UsfCanonicalSweep,
        target_error_metres: f64,
    ) -> UsfSweepResolution {
        let (candidates, complete) = self.collect_candidates(sweep);
        if !complete {
            return UsfSweepResolution::Unknown { safe_fraction: 0.0 };
        }
        let mut result = UsfSweepResolution::Clear;
        for candidate in candidates {
            let authority = candidate.authority();
            let Ok((_, origin, frame, field, edits)) = self.authorities.get(authority) else {
                return UsfSweepResolution::Unknown { safe_fraction: 0.0 };
            };
            let Ok(start_local) =
                frame.world_to_local_metres(origin, &sweep.start(), SpatialScale::ZERO, f64::MAX)
            else {
                return UsfSweepResolution::Unknown { safe_fraction: 0.0 };
            };
            let displacement_local =
                frame.world_direction_to_local_f64(sweep.displacement_metres());
            let Some(edits) = local_edits(edits) else {
                return UsfSweepResolution::Unknown { safe_fraction: 0.0 };
            };
            let narrowed = trace_candidate(
                sweep,
                candidate.interval(),
                *field,
                &edits,
                start_local,
                displacement_local,
                target_error_metres,
                authority,
            );
            if narrowed.safe_fraction() < result.safe_fraction()
                || (narrowed.safe_fraction() == result.safe_fraction()
                    && narrowed.tie_priority() < result.tie_priority())
            {
                result = narrowed;
            }
        }
        result
    }
}
