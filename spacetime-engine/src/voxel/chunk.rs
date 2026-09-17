//! Dense chunk-local working representation used by the editable voxel world.

use bevy::prelude::{Component, IVec3, UVec3, Vec3};

use super::{
    SignedDistance, VoxelEdit, VoxelMaterialId, VoxelMaterializationChunkAddress, VoxelSample,
};

/// Logical cell extent of one base voxel materialization chunk axis.
///
/// This decimal `10³` extent is representation structure, not a USF Chunk.
pub const MATERIALIZATION_CHUNK_SIZE: u32 = 10;

/// Transitional compatibility name for code that has not yet adopted the
/// materialization-specific terminology. New code should use
/// [`MATERIALIZATION_CHUNK_SIZE`].
#[doc(hidden)]
pub const CHUNK_SIZE: u32 = MATERIALIZATION_CHUNK_SIZE;

/// Neighbor samples retained around the logical chunk for seamless extraction.
/// Padding is private representation storage and is not part of chunk identity.
pub(crate) const SAMPLE_PADDING: u32 = 1;
pub(crate) const SAMPLE_SIZE: u32 = MATERIALIZATION_CHUNK_SIZE + SAMPLE_PADDING * 2;
const SAMPLE_COUNT: usize = (SAMPLE_SIZE * SAMPLE_SIZE * SAMPLE_SIZE) as usize;

const RAY_STEP: f32 = 0.25;
const RAY_REFINEMENT_STEPS: usize = 6;

/// Result of applying one semantic edit to a chunk.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VoxelChunkEditResult {
    pub changed_samples: usize,
    pub revision: u64,
}

impl VoxelChunkEditResult {
    pub const fn changed(self) -> bool {
        self.changed_samples != 0
    }
}

/// Ray hit expressed in the dense chunk's bounded local chart.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelRayHit {
    pub position: Vec3,
    pub distance: f32,
}

/// Dense sampled volume used as the active working representation.
///
/// Every coordinate stored here is chunk-local. Samples cover `[-1, 10]` on
/// each axis because Surface Nets keeps one copied neighbor sample around the
/// logical half-open `[0, 10)³` ownership extent. Canonical semantic location
/// lives on the materialization entity's address component, never in this data.
#[derive(Component, Debug, Clone)]
pub struct VoxelChunk {
    distances: Box<[f32]>,
    materials: Box<[VoxelMaterialId]>,
    revision: u64,
    meshed_revision: Option<u64>,
}

impl VoxelChunk {
    pub fn generate(mut generator: impl FnMut(Vec3) -> VoxelSample) -> Self {
        let mut distances = Vec::with_capacity(SAMPLE_COUNT);
        let mut materials = Vec::with_capacity(SAMPLE_COUNT);
        let padding = IVec3::splat(SAMPLE_PADDING as i32);

        for z in 0..SAMPLE_SIZE {
            for y in 0..SAMPLE_SIZE {
                for x in 0..SAMPLE_SIZE {
                    let storage = IVec3::new(x as i32, y as i32, z as i32);
                    let local = storage - padding;
                    let sample = generator(local.as_vec3());
                    distances.push(sample.distance.0);
                    materials.push(sample.material);
                }
            }
        }

        Self {
            distances: distances.into_boxed_slice(),
            materials: materials.into_boxed_slice(),
            revision: 0,
            meshed_revision: None,
        }
    }

    pub fn filled(sample: VoxelSample) -> Self {
        Self::generate(|_| sample)
    }

    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub const fn meshed_revision(&self) -> Option<u64> {
        self.meshed_revision
    }

    pub fn needs_remesh(&self) -> bool {
        self.meshed_revision != Some(self.revision)
    }

    pub fn mark_meshed(&mut self) {
        self.meshed_revision = Some(self.revision);
    }

    pub fn distances(&self) -> &[f32] {
        &self.distances
    }

    pub fn materials(&self) -> &[VoxelMaterialId] {
        &self.materials
    }

    pub fn sample(&self, local: IVec3) -> Option<VoxelSample> {
        let index = Self::index(self.storage_coord(local)?);
        Some(self.sample_at_index(index))
    }

    /// Trilinearly samples the scalar field at an arbitrary chunk-local point.
    pub fn sample_distance(&self, local: Vec3) -> Option<f32> {
        let base = local.floor().as_ivec3();
        let fraction = local - base.as_vec3();

        let d000 = self.sample(base)?.distance.0;
        let d100 = self.sample(base + IVec3::X)?.distance.0;
        let d010 = self.sample(base + IVec3::Y)?.distance.0;
        let d110 = self.sample(base + IVec3::X + IVec3::Y)?.distance.0;
        let d001 = self.sample(base + IVec3::Z)?.distance.0;
        let d101 = self.sample(base + IVec3::X + IVec3::Z)?.distance.0;
        let d011 = self.sample(base + IVec3::Y + IVec3::Z)?.distance.0;
        let d111 = self
            .sample(base + IVec3::X + IVec3::Y + IVec3::Z)?
            .distance
            .0;

        let x00 = d000 + (d100 - d000) * fraction.x;
        let x10 = d010 + (d110 - d010) * fraction.x;
        let x01 = d001 + (d101 - d001) * fraction.x;
        let x11 = d011 + (d111 - d011) * fraction.x;
        let y0 = x00 + (x10 - x00) * fraction.y;
        let y1 = x01 + (x11 - x01) * fraction.y;
        Some(y0 + (y1 - y0) * fraction.z)
    }

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

    /// Applies one canonical semantic edit by projecting it into this chunk's
    /// bounded local chart. The canonical edit remains the authoritative state.
    pub fn apply_edit(
        &mut self,
        address: VoxelMaterializationChunkAddress,
        edit: VoxelEdit,
    ) -> VoxelChunkEditResult {
        let extra_extent = MATERIALIZATION_CHUNK_SIZE as f32 + SAMPLE_PADDING as f32;
        let Some(edit) = edit.localized(address.query_origin(), extra_extent) else {
            return VoxelChunkEditResult {
                changed_samples: 0,
                revision: self.revision,
            };
        };

        let padding = IVec3::splat(SAMPLE_PADDING as i32);
        let stored_min = -padding;
        let stored_max = IVec3::splat(MATERIALIZATION_CHUNK_SIZE as i32);
        let bounds = edit.influence_bounds();
        let edit_min = bounds.min.ceil().as_ivec3().max(stored_min);
        let edit_max = bounds.max.floor().as_ivec3().min(stored_max);

        if edit_min.cmpgt(edit_max).any() {
            return VoxelChunkEditResult {
                changed_samples: 0,
                revision: self.revision,
            };
        }

        let storage_min = (edit_min + padding).as_uvec3();
        let storage_max = (edit_max + padding).as_uvec3();
        let mut changed_samples = 0;

        for z in storage_min.z..=storage_max.z {
            for y in storage_min.y..=storage_max.y {
                for x in storage_min.x..=storage_max.x {
                    let storage = UVec3::new(x, y, z);
                    let local = storage.as_ivec3() - padding;
                    let index = Self::index(storage);
                    let before = self.sample_at_index(index);
                    let after = edit.apply_to_sample(local.as_vec3(), before);

                    if after != before {
                        self.distances[index] = after.distance.0;
                        self.materials[index] = after.material;
                        changed_samples += 1;
                    }
                }
            }
        }

        if changed_samples != 0 {
            self.revision = self.revision.wrapping_add(1);
        }

        VoxelChunkEditResult {
            changed_samples,
            revision: self.revision,
        }
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

    fn sample_at_index(&self, index: usize) -> VoxelSample {
        VoxelSample {
            distance: SignedDistance(self.distances[index]),
            material: self.materials[index],
        }
    }

    fn storage_coord(&self, local: IVec3) -> Option<UVec3> {
        let storage = local + IVec3::splat(SAMPLE_PADDING as i32);
        let upper = IVec3::splat(SAMPLE_SIZE as i32);

        if storage.cmplt(IVec3::ZERO).any() || storage.cmpge(upper).any() {
            return None;
        }

        Some(storage.as_uvec3())
    }

    #[inline]
    const fn index(storage: UVec3) -> usize {
        (storage.x + SAMPLE_SIZE * (storage.y + SAMPLE_SIZE * storage.z)) as usize
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec3;

    use super::*;
    use crate::voxel::{VoxelBrush, VoxelQueryPosition};

    fn address() -> VoxelMaterializationChunkAddress {
        VoxelMaterializationChunkAddress::new(crate::spatial::UsfPosition::default())
    }

    fn query(local: Vec3) -> VoxelQueryPosition {
        address().query_origin().translated(local).unwrap()
    }

    #[test]
    fn new_chunk_needs_initial_mesh() {
        let chunk = VoxelChunk::filled(VoxelSample::empty(100.0));
        assert_eq!(chunk.revision(), 0);
        assert_eq!(chunk.meshed_revision(), None);
        assert!(chunk.needs_remesh());
    }

    #[test]
    fn meshed_revision_tracks_authoritative_revision() {
        let mut chunk = VoxelChunk::filled(VoxelSample::empty(100.0));
        chunk.mark_meshed();
        assert!(!chunk.needs_remesh());

        let result = chunk.apply_edit(
            address(),
            VoxelEdit::Add {
                brush: VoxelBrush::sphere(query(Vec3::splat(8.0)), 2.0),
                material: VoxelMaterialId::ROCK,
            },
        );

        assert!(result.changed());
        assert_eq!(chunk.revision(), 1);
        assert!(chunk.needs_remesh());
    }

    #[test]
    fn add_then_remove_changes_the_authoritative_field() {
        let center = IVec3::splat(8);
        let mut chunk = VoxelChunk::filled(VoxelSample::empty(100.0));

        chunk.apply_edit(
            address(),
            VoxelEdit::Add {
                brush: VoxelBrush::sphere(query(center.as_vec3()), 2.0),
                material: VoxelMaterialId::ROCK,
            },
        );
        let added = chunk.sample(center).expect("center sample must exist");
        assert!(added.distance.is_solid());
        assert_eq!(added.material, VoxelMaterialId::ROCK);

        chunk.apply_edit(
            address(),
            VoxelEdit::Remove {
                brush: VoxelBrush::sphere(query(center.as_vec3()), 2.0),
            },
        );
        let removed = chunk.sample(center).expect("center sample must exist");
        assert!(removed.distance.is_empty());
        assert_eq!(removed.material, VoxelMaterialId::VOID);
    }

    #[test]
    fn padded_neighbor_samples_are_addressable() {
        let chunk = VoxelChunk::generate(|point| {
            VoxelSample::empty(point.x + point.y * 100.0 + point.z * 10_000.0)
        });

        assert!(chunk.sample(IVec3::splat(-1)).is_some());
        assert!(
            chunk
                .sample(IVec3::splat(MATERIALIZATION_CHUNK_SIZE as i32))
                .is_some()
        );
        assert!(chunk.sample(IVec3::splat(-2)).is_none());
        assert!(
            chunk
                .sample(IVec3::splat(MATERIALIZATION_CHUNK_SIZE as i32 + 1))
                .is_none()
        );
    }

    #[test]
    fn raycast_finds_a_generated_sphere() {
        // Keep the analytic surface inside this chunk's sampled/interpolatable
        // domain. With the decimal 10³ base chunk, a radius-3 sphere centered
        // at 8 would have its front surface at z=11, outside local storage.
        let center = Vec3::splat(5.0);
        let chunk = VoxelChunk::generate(|point| {
            let distance = point.distance(center) - 3.0;
            VoxelSample::new(
                distance,
                if distance < 0.0 {
                    VoxelMaterialId::ROCK
                } else {
                    VoxelMaterialId::VOID
                },
            )
        });

        let hit = chunk
            .raycast(Vec3::new(5.0, 5.0, 9.0), Vec3::NEG_Z, 20.0)
            .expect("ray should hit sphere");
        assert!((hit.position.z - 8.0).abs() < 0.1);
    }
}
