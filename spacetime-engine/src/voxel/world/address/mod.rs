//! Canonical voxel materialization addressing and compatibility coordinates.

use super::*;

/// Transitional coordinate in the original `VoxelWorld`-local sampling lattice.
///
/// Pass B no longer uses this as semantic identity, edit/query authority,
/// streaming identity, or generation input. It remains only as a compact
/// compatibility adapter for authored/tests that still describe a nearby grid
/// offset from [`VoxelWorld::origin`].
#[doc(hidden)]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelChunkCoord(pub IVec3);

impl VoxelChunkCoord {
    pub const fn new(value: IVec3) -> Self {
        Self(value)
    }

    pub fn origin(self) -> IVec3 {
        self.0 * MATERIALIZATION_CHUNK_SIZE as i32
    }

    /// Exact displacement from this voxel world's canonical origin in whole
    /// leaf-native units. This compatibility path never squeezes the semantic
    /// address through one large floating-point vector.
    pub fn native_offset(self) -> [i64; 3] {
        let size = i64::from(MATERIALIZATION_CHUNK_SIZE);
        [
            i64::from(self.0.x) * size,
            i64::from(self.0.y) * size,
            i64::from(self.0.z) * size,
        ]
    }

    /// Local compatibility chunk containing one nearby scale-0 point.
    pub fn containing(point: Vec3) -> Self {
        Self(
            (point / MATERIALIZATION_CHUNK_SIZE as f32)
                .floor()
                .as_ivec3(),
        )
    }
}

/// Canonical semantic address of one decimal `10³` voxel materialization chunk.
///
/// The address is sparse/virtual: constructing it allocates no dense voxel data,
/// entity, mesh, collider, or hierarchy node. It is representation identity, not
/// a USF Chunk and not a runtime Bevy/Avian coordinate.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct VoxelMaterializationChunkAddress {
    origin: UsfPosition,
}

impl VoxelMaterializationChunkAddress {
    pub const fn new(origin: UsfPosition) -> Self {
        Self { origin }
    }

    pub const fn origin(&self) -> &UsfPosition {
        &self.origin
    }

    pub const fn query_origin(self) -> VoxelQueryPosition {
        VoxelQueryPosition::new(self.origin)
    }

    /// Canonical center of this materialization region.
    pub fn center(self) -> Result<UsfPosition, UsfPositionError> {
        self.origin
            .translated_native(Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5))
    }

    /// Squared scale-local gap between this chunk AABB and a canonical region.
    ///
    /// Returns `None` when the region is farther away than the supplied padding
    /// can possibly matter.
    pub fn distance_squared_to_region(
        self,
        center: &UsfPosition,
        half_extent_native: Vec3,
        padding_native: f32,
    ) -> Option<f32> {
        let extent = MATERIALIZATION_CHUNK_SIZE as f32;
        let half = half_extent_native.abs();
        let padding = padding_native.max(0.0);
        let bound = extent * 2.0 + half.length() + padding + 1.0;
        let minimum = self
            .origin
            .relative_at_scale_bounded(center, self.origin.leaf_scale(), bound)
            .ok()?;
        let maximum = minimum + Vec3::splat(extent);
        let region_min = -half;
        let region_max = half;

        let separation = Vec3::new(
            if maximum.x < region_min.x {
                region_min.x - maximum.x
            } else if minimum.x > region_max.x {
                minimum.x - region_max.x
            } else {
                0.0
            },
            if maximum.y < region_min.y {
                region_min.y - maximum.y
            } else if minimum.y > region_max.y {
                minimum.y - region_max.y
            } else {
                0.0
            },
            if maximum.z < region_min.z {
                region_min.z - maximum.z
            } else if minimum.z > region_max.z {
                minimum.z - region_max.z
            } else {
                0.0
            },
        );
        Some(separation.length_squared())
    }

    pub fn translated_chunks(self, delta: IVec3) -> Result<Self, UsfPositionError> {
        let size = i64::from(MATERIALIZATION_CHUNK_SIZE);
        self.origin
            .translated_whole_native([
                i64::from(delta.x) * size,
                i64::from(delta.y) * size,
                i64::from(delta.z) * size,
            ])
            .map(Self::new)
    }

    /// Canonical scope of every lattice sample physically stored by this base
    /// materialization, including private Surface Nets neighbor padding.
    pub fn sample_bounds(self) -> VoxelBounds {
        VoxelBounds::new(
            self.query_origin(),
            Vec3::splat(-(SAMPLE_PADDING as f32)),
            Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32),
        )
    }
}

/// Compatibility alias for the M7.1a name.
#[doc(hidden)]
pub type VoxelChunkAddress = VoxelMaterializationChunkAddress;
