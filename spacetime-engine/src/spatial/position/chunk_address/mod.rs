//! Canonical identity of one USF chunk at one spatial scale.
//!
//! Topology remains engine-owned for now and is extracted by #30. This module
//! intentionally consumes only the public canonical-position algebra from #29.

use bevy::prelude::IVec3;

use super::{SpatialScale, USF_CHUNK_NATIVE_SIZE, UsfPosition, UsfPositionError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfChunkAddress {
    scale: SpatialScale,
    center: UsfPosition,
}

impl UsfChunkAddress {
    pub fn containing(
        position: UsfPosition,
        scale: SpatialScale,
    ) -> Result<Self, UsfPositionError> {
        if scale < position.leaf_scale() {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        let expressed = position.reexpressed_at(scale)?;
        let center = expressed.translated_native(-expressed.offset())?;
        Ok(Self { scale, center })
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub fn digit(self, scale: SpatialScale) -> Option<IVec3> {
        if scale < self.scale {
            None
        } else {
            Some(self.center.digit(scale))
        }
    }

    pub const fn center(self) -> UsfPosition {
        self.center
    }

    pub fn translated_chunks(self, delta: IVec3) -> Result<Self, UsfPositionError> {
        let size = USF_CHUNK_NATIVE_SIZE as i64;
        let position = self.center.translated_whole_native([
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

        let scale = SpatialScale::new(self.scale.exponent() + 1)
            .expect("non-root scale always has a parent");
        let expressed = self
            .center
            .reexpressed_at(scale)
            .expect("canonical chunk center can always coarsen to its parent");
        let center = expressed
            .translated_native(-expressed.offset())
            .expect("subtracting the canonical local offset is always bounded");

        Some(Self { scale, center })
    }
}
