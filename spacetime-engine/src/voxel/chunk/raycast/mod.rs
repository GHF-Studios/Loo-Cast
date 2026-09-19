//! Chunk-local signed-distance ray traversal and surface-crossing refinement.

use super::*;

impl VoxelChunk {
    /// Finds the first empty-to-solid crossing along a chunk-local ray.
    pub fn raycast(&self, origin: Vec3, direction: Vec3, max_distance: f32) -> Option<VoxelRayHit> {
        let direction = direction.normalize_or_zero();
        if direction == Vec3::ZERO || max_distance <= 0.0 {
            return None;
        }

        let mut previous: Option<(f32, f32)> = None;
        let mut distance = 0.0;

        while distance <= max_distance {
            let point = origin + direction * distance;
            if let Some(value) = self.sample_distance(point) {
                if value < 0.0 {
                    let refined = match previous {
                        Some((previous_distance, previous_value)) if previous_value >= 0.0 => self
                            .refine_surface_crossing(
                                origin,
                                direction,
                                previous_distance,
                                distance,
                            ),
                        _ => distance,
                    };
                    return Some(VoxelRayHit {
                        position: origin + direction * refined,
                        distance: refined,
                    });
                }
                previous = Some((distance, value));
            } else {
                previous = None;
            }

            distance += RAY_STEP;
        }

        None
    }

    fn refine_surface_crossing(
        &self,
        origin: Vec3,
        direction: Vec3,
        mut outside: f32,
        mut inside: f32,
    ) -> f32 {
        for _ in 0..RAY_REFINEMENT_STEPS {
            let middle = (outside + inside) * 0.5;
            let value = self
                .sample_distance(origin + direction * middle)
                .unwrap_or(f32::INFINITY);
            if value < 0.0 {
                inside = middle;
            } else {
                outside = middle;
            }
        }
        (outside + inside) * 0.5
    }
}
