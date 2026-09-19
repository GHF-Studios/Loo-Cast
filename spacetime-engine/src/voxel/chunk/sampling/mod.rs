//! Dense chunk-local sample lookup, interpolation and storage addressing.

use super::*;

impl VoxelChunk {
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
    pub(super) const fn index(storage: UVec3) -> usize {
        (storage.x + SAMPLE_SIZE * (storage.y + SAMPLE_SIZE * storage.z)) as usize
    }
}
