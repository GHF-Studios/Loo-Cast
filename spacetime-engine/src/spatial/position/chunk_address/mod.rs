//! Canonical identity of one USF chunk at one spatial scale.

use bevy::prelude::IVec3;

use super::{
    SPATIAL_SCALE_COUNT, SPATIAL_SCALE_MAX, SpatialScale, UsfPosition, UsfPositionError,
};

/// Canonical identity of one USF Chunk at one spatial scale.
///
/// An address retains only the balanced-decimal digits that identify the chunk
/// at `scale`; finer digits and the leaf-local offset are intentionally ignored.
/// This makes the same type useful for sparse Phenomenon state, generation
/// caches and later representation attachments without allocating the hierarchy.
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
