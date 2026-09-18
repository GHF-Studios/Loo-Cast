//! Scalar field samples used by the voxel world.

/// Signed distance to the material surface.
///
/// Negative values are inside matter, positive values are outside, and zero is
/// the surface. Keeping this convention explicit lets meshing and constructive
/// edits share the same semantics.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
pub struct SignedDistance(pub f32);

impl SignedDistance {
    pub const SURFACE: Self = Self(0.0);

    #[inline]
    pub const fn is_solid(self) -> bool {
        self.0 < 0.0
    }

    #[inline]
    pub const fn is_empty(self) -> bool {
        !self.is_solid()
    }
}

/// Stable identifier for material carried by a voxel sample.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VoxelMaterialId(pub u16);

impl VoxelMaterialId {
    pub const VOID: Self = Self(0);
    pub const ROCK: Self = Self(1);
}

/// Authoritative sampled state of the volumetric field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelSample {
    pub distance: SignedDistance,
    pub material: VoxelMaterialId,
}

impl VoxelSample {
    pub const fn new(distance: f32, material: VoxelMaterialId) -> Self {
        Self {
            distance: SignedDistance(distance),
            material,
        }
    }

    pub const fn empty(distance: f32) -> Self {
        Self::new(distance, VoxelMaterialId::VOID)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn signed_distance_sign_is_the_authoritative_solidity_test() {
        assert!(SignedDistance(-0.1).is_solid());
        assert!(SignedDistance::SURFACE.is_empty());
        assert!(SignedDistance(0.1).is_empty());
    }
}
