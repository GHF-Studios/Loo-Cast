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

/// Rigid collision response of one voxel material.
///
/// This is deliberately orthogonal to optical opacity and medium interaction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VoxelCollisionMode {
    None,
    Rigid,
}

/// Bootstrap behavior descriptor for voxel matter.
///
/// The static lookup is intentionally temporary; rendering, collision and
/// medium systems depend on these independent semantic axes rather than on the
/// lookup mechanism itself.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelMaterialBehavior {
    pub opacity: f32,
    pub collision: VoxelCollisionMode,
    /// Exponential linear drag coefficient in inverse seconds.
    pub linear_drag: f32,
}

impl VoxelMaterialBehavior {
    pub const fn is_visible(self) -> bool {
        self.opacity > 0.0
    }

    pub const fn is_rigid(self) -> bool {
        matches!(self.collision, VoxelCollisionMode::Rigid)
    }

    pub const fn is_translucent(self) -> bool {
        self.opacity > 0.0 && self.opacity < 1.0
    }
}

/// Stable identifier for material carried by a voxel sample.
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct VoxelMaterialId(pub u16);

impl VoxelMaterialId {
    pub const VOID: Self = Self(0);
    pub const ROCK: Self = Self(1);
    pub const GLASS: Self = Self(2);
    pub const NEBULA: Self = Self(3);

    pub const fn behavior(self) -> VoxelMaterialBehavior {
        match self.0 {
            0 => VoxelMaterialBehavior {
                opacity: 0.0,
                collision: VoxelCollisionMode::None,
                linear_drag: 0.0,
            },
            2 => VoxelMaterialBehavior {
                opacity: 0.18,
                collision: VoxelCollisionMode::Rigid,
                linear_drag: 0.0,
            },
            3 => VoxelMaterialBehavior {
                opacity: 0.12,
                collision: VoxelCollisionMode::None,
                linear_drag: 0.85,
            },
            _ => VoxelMaterialBehavior {
                opacity: 1.0,
                collision: VoxelCollisionMode::Rigid,
                linear_drag: 0.0,
            },
        }
    }
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
