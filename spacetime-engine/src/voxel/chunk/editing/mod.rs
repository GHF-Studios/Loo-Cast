//! Projection/application of canonical semantic edits into dense chunk-local storage.

use super::*;

impl VoxelChunk {
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

        {
            let distances = Arc::make_mut(&mut self.distances);
            let materials = Arc::make_mut(&mut self.materials);

            for z in storage_min.z..=storage_max.z {
                for y in storage_min.y..=storage_max.y {
                    for x in storage_min.x..=storage_max.x {
                        let storage = UVec3::new(x, y, z);
                        let local = storage.as_ivec3() - padding;
                        let index = Self::index(storage);
                        let before = VoxelSample {
                            distance: SignedDistance(distances[index]),
                            material: materials[index],
                        };
                        let after = edit.apply_to_sample(local.as_vec3(), before);

                        if after != before {
                            distances[index] = after.distance.0;
                            materials[index] = after.material;
                            changed_samples += 1;
                        }
                    }
                }
            }
        }

        if changed_samples != 0 {
            self.revision = self.revision.wrapping_add(1);
            self.surface_transition = detect_surface_transition(&self.distances);
        }

        VoxelChunkEditResult {
            changed_samples,
            revision: self.revision,
        }
    }
}
