//! Canonical identity and traversal of one USF chunk at one spatial scale.
//!
//! This is pure Scale Stack topology. It knows canonical containment,
//! parent ancestry, and same-scale neighborhood translation. It does not know
//! whether an address is loaded, resident, realized, rendered, or simulated.

use glam::IVec3;

use crate::{SpatialScale, USF_CHUNK_NATIVE_SIZE, UsfPosition, UsfPositionError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct UsfChunkAddress {
    scale: SpatialScale,
    center: UsfPosition,
}

impl UsfChunkAddress {
    /// Returns the canonical chunk at `scale` containing `position`.
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

    /// Returns this address's balanced chunk digit at `scale`.
    ///
    /// Scales finer than this address are not part of its canonical identity.
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

    /// Traverses the same-scale canonical chunk lattice by `delta` cells.
    pub fn translated_chunks(self, delta: IVec3) -> Result<Self, UsfPositionError> {
        let size = USF_CHUNK_NATIVE_SIZE as i64;
        let position = self.center.translated_whole_native([
            i64::from(delta.x) * size,
            i64::from(delta.y) * size,
            i64::from(delta.z) * size,
        ])?;
        Self::containing(position, self.scale)
    }

    /// Returns the immediately coarser canonical ancestor.
    pub fn parent(self) -> Option<Self> {
        if self.scale == SpatialScale::MAX {
            return None;
        }

        let scale = SpatialScale::new(self.scale.exponent() + 1)
            .expect("non-root scale always has a parent");

        Some(
            Self::containing(self.center, scale)
                .expect("canonical chunk center always has a coarser containing address"),
        )
    }

    /// Returns this address re-expressed as its ancestor at `scale`.
    ///
    /// Requesting a finer scale is invalid because a parent address does not
    /// identify which of its many descendants was intended.
    pub fn ancestor_at(self, scale: SpatialScale) -> Result<Self, UsfPositionError> {
        if scale < self.scale {
            return Err(UsfPositionError::IncompatibleLeafScale);
        }

        Self::containing(self.center, scale)
    }
}
