//! Body-local semantic coordinates for movable voxel authority.
//!
//! A [`VoxelFramePosition`] is deliberately distinct from a world-global
//! [`VoxelQueryPosition`]. Persistent authority edits use this coordinate;
//! disposable realizations project it through a current [`VoxelFrameSnapshot`].

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{SpatialScale, UsfPosition, UsfPositionError, UsfSemanticFrame};

use super::{
    VoxelBounds, VoxelBrush, VoxelEdit, VoxelMaterialId, VoxelQueryPosition, edit::VoxelLocalEdit,
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelFramePosition(UsfPosition);

impl VoxelFramePosition {
    pub const fn new(local: UsfPosition) -> Self {
        Self(local)
    }

    pub fn from_scale_native(
        local_native: DVec3,
        scale: SpatialScale,
    ) -> Result<Self, UsfPositionError> {
        UsfPosition::from_scale_native_f64(local_native, scale, scale).map(Self)
    }

    pub const fn usf(self) -> UsfPosition {
        self.0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoxelFrameSnapshot {
    origin: UsfPosition,
    frame: UsfSemanticFrame,
    scale: SpatialScale,
}

impl VoxelFrameSnapshot {
    pub const fn new(origin: UsfPosition, frame: UsfSemanticFrame, scale: SpatialScale) -> Self {
        Self {
            origin,
            frame,
            scale,
        }
    }

    pub const fn origin(self) -> UsfPosition {
        self.origin
    }
    pub const fn frame(self) -> UsfSemanticFrame {
        self.frame
    }
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub fn world_to_frame(
        self,
        world: VoxelQueryPosition,
    ) -> Result<VoxelFramePosition, UsfPositionError> {
        let local_metres =
            self.frame
                .world_to_local_metres(&self.origin, &world.usf(), self.scale, f64::MAX)?;
        let local_native = local_metres / self.scale.metres_per_native();
        UsfPosition::from_scale_native_f64(local_native, self.scale, self.scale)
            .map(VoxelFramePosition)
    }

    pub fn frame_to_world(
        self,
        local: VoxelFramePosition,
    ) -> Result<VoxelQueryPosition, UsfPositionError> {
        let local_native = local.usf().coordinate_at_scale_f64(self.scale)?;
        let local_metres = local_native * self.scale.metres_per_native();
        self.frame
            .local_metres_to_world(self.origin, local_metres)?
            .reexpressed_at(self.scale)
            .map(VoxelQueryPosition::new)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelFrameBrush {
    Sphere {
        center: VoxelFramePosition,
        radius_metres: f64,
    },
}

impl VoxelFrameBrush {
    pub fn sphere(center: VoxelFramePosition, radius_metres: f64) -> Self {
        Self::Sphere {
            center,
            radius_metres: radius_metres.max(0.0),
        }
    }

    pub const fn center(self) -> VoxelFramePosition {
        match self {
            Self::Sphere { center, .. } => center,
        }
    }

    pub const fn radius_metres(self) -> f64 {
        match self {
            Self::Sphere { radius_metres, .. } => radius_metres,
        }
    }

    fn projected_world(self, snapshot: VoxelFrameSnapshot) -> Result<VoxelBrush, UsfPositionError> {
        let center = snapshot.frame_to_world(self.center())?;
        let radius_native = self.radius_metres() / snapshot.scale.metres_per_native();
        if !radius_native.is_finite() || radius_native > f64::from(f32::MAX) {
            return Err(UsfPositionError::TranslationTooLarge);
        }
        Ok(VoxelBrush::sphere(center, radius_native as f32))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VoxelFrameEdit {
    Add {
        brush: VoxelFrameBrush,
        material: VoxelMaterialId,
    },
    Remove {
        brush: VoxelFrameBrush,
    },
    Paint {
        brush: VoxelFrameBrush,
        material: VoxelMaterialId,
    },
}

impl VoxelFrameEdit {
    pub fn from_world(
        edit: VoxelEdit,
        snapshot: VoxelFrameSnapshot,
    ) -> Result<Self, UsfPositionError> {
        let (brush, material, kind) = match edit {
            VoxelEdit::Add { brush, material } => (brush, Some(material), 0_u8),
            VoxelEdit::Remove { brush } => (brush, None, 1_u8),
            VoxelEdit::Paint { brush, material } => (brush, Some(material), 2_u8),
        };
        let center = snapshot.world_to_frame(brush.center())?;
        let radius_metres = f64::from(brush.radius()) * snapshot.scale.metres_per_native();
        let brush = VoxelFrameBrush::sphere(center, radius_metres);
        Ok(match kind {
            0 => Self::Add {
                brush,
                material: material.expect("add material"),
            },
            1 => Self::Remove { brush },
            _ => Self::Paint {
                brush,
                material: material.expect("paint material"),
            },
        })
    }

    pub(in crate::voxel) fn projected_world(
        self,
        snapshot: VoxelFrameSnapshot,
    ) -> Result<VoxelEdit, UsfPositionError> {
        Ok(match self {
            Self::Add { brush, material } => VoxelEdit::Add {
                brush: brush.projected_world(snapshot)?,
                material,
            },
            Self::Remove { brush } => VoxelEdit::Remove {
                brush: brush.projected_world(snapshot)?,
            },
            Self::Paint { brush, material } => VoxelEdit::Paint {
                brush: brush.projected_world(snapshot)?,
                material,
            },
        })
    }

    pub(in crate::voxel) fn localized_for(
        self,
        snapshot: VoxelFrameSnapshot,
        chunk_origin: VoxelQueryPosition,
        extra_extent: f32,
    ) -> Option<VoxelLocalEdit> {
        self.projected_world(snapshot)
            .ok()?
            .localized(chunk_origin, extra_extent)
    }

    pub(in crate::voxel) fn world_bounds(
        self,
        snapshot: VoxelFrameSnapshot,
    ) -> Result<VoxelBounds, UsfPositionError> {
        Ok(self.projected_world(snapshot)?.influence_bounds())
    }
}
