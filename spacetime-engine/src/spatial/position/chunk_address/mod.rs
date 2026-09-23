//! Canonical identity of one USF chunk at one spatial scale.

use bevy::prelude::{IVec3, Vec3};

use super::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SpatialScale, USF_CHUNK_NATIVE_SIZE, UsfPosition,
    UsfPositionError,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfChunkAddress {
    scale: SpatialScale,
    digits: [IVec3; SPATIAL_SCALE_COUNT],
}

impl UsfChunkAddress {
    pub fn containing(
        position: UsfPosition,
        scale: SpatialScale,
    ) -> Result<Self, UsfPositionError> {
        if scale < position.leaf_scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let mut digits = [IVec3::ZERO; SPATIAL_SCALE_COUNT];
        for raw_scale in scale.exponent()..=SPATIAL_SCALE_MAX {
            let digit_scale = SpatialScale::new(raw_scale).expect("validated spatial scale range");
            digits[digit_scale.index_from_top()] = position.digit(digit_scale);
        }

        Ok(Self { scale, digits })
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub fn digit(self, scale: SpatialScale) -> Option<IVec3> {
        if scale < self.scale {
            None
        } else {
            Some(self.digits[scale.index_from_top()])
        }
    }

    pub fn center(self) -> UsfPosition {
        UsfPosition {
            digits: self.digits,
            leaf_scale: self.scale,
            offset: Vec3::ZERO,
        }
    }

    pub fn translated_chunks(self, delta: IVec3) -> Result<Self, UsfPositionError> {
        let size = USF_CHUNK_NATIVE_SIZE as i64;
        let position = self.center().translated_whole_native([
            i64::from(delta.x) * size,
            i64::from(delta.y) * size,
            i64::from(delta.z) * size,
        ])?;
        Self::containing(position, self.scale)
    }

    pub fn parent(self) -> Option<Self> {
        if self.scale == SpatialScale::MAX {
            return None;
        }

        let mut parent = self;
        parent.digits[self.scale.index_from_top()] = IVec3::ZERO;
        parent.scale = SpatialScale::new(self.scale.exponent() + 1)
            .expect("non-root scale always has a parent");
        Some(parent)
    }
}
