//! Sparse capability-local region addressing over voxel materialization leaves.
//!
//! A region is not a chunk, entity, allocation, residency claim, or semantic
//! world object. It is an aligned span over the existing 10-native-unit
//! materialization lattice. Consumers may traverse/split it for planning, or
//! eventually satisfy it directly with a coarser representation. Leaf chunks
//! deliberately have no parent/back-pointer to these transient management regions.

use super::VoxelMaterializationKey;
use crate::spatial::UsfPositionError;
use bevy::prelude::IVec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::voxel) struct VoxelRegionSpan {
    origin: VoxelMaterializationKey,
    extent_chunks: IVec3,
}

impl VoxelRegionSpan {
    pub(in crate::voxel) fn from_relative_bounds(
        reference: VoxelMaterializationKey,
        minimum: IVec3,
        maximum: IVec3,
    ) -> Result<Self, UsfPositionError> {
        if minimum.cmpgt(maximum).any() {
            return Err(UsfPositionError::TranslationTooLarge);
        }
        let extent_chunks = IVec3::new(
            checked_extent(minimum.x, maximum.x)?,
            checked_extent(minimum.y, maximum.y)?,
            checked_extent(minimum.z, maximum.z)?,
        );
        Ok(Self {
            origin: reference.translated_chunks(minimum)?,
            extent_chunks,
        })
    }

    pub(in crate::voxel) fn aligned_containing(
        key: VoxelMaterializationKey,
        edge_chunks: i32,
    ) -> Result<Self, UsfPositionError> {
        if edge_chunks <= 0 {
            return Err(UsfPositionError::TranslationTooLarge);
        }
        let [x, y, z] = key.components();
        let edge = i64::from(edge_chunks);
        let remainder = IVec3::new(
            x.rem_euclid(edge) as i32,
            y.rem_euclid(edge) as i32,
            z.rem_euclid(edge) as i32,
        );
        Ok(Self {
            origin: key.translated_chunks(-remainder)?,
            extent_chunks: IVec3::splat(edge_chunks),
        })
    }

    pub(in crate::voxel) const fn origin(self) -> VoxelMaterializationKey {
        self.origin
    }
    pub(in crate::voxel) const fn extent_chunks(self) -> IVec3 {
        self.extent_chunks
    }
    pub(in crate::voxel) const fn is_leaf(self) -> bool {
        self.extent_chunks.x == 1 && self.extent_chunks.y == 1 && self.extent_chunks.z == 1
    }

    pub(in crate::voxel) fn relative_origin_chunks(
        self,
        reference: VoxelMaterializationKey,
    ) -> Result<IVec3, UsfPositionError> {
        let o = self.origin.components();
        let r = reference.components();
        Ok(IVec3::new(
            checked_i32(o[0].checked_sub(r[0]))?,
            checked_i32(o[1].checked_sub(r[1]))?,
            checked_i32(o[2].checked_sub(r[2]))?,
        ))
    }

    pub(in crate::voxel) fn split_longest(self) -> Result<Option<(Self, Self)>, UsfPositionError> {
        if self.is_leaf() {
            return Ok(None);
        }
        let e = self.extent_chunks;
        let axis = if e.x >= e.y && e.x >= e.z {
            0
        } else if e.y >= e.z {
            1
        } else {
            2
        };
        let n = match axis {
            0 => e.x,
            1 => e.y,
            2 => e.z,
            _ => unreachable!(),
        };
        let left_n = n / 2;
        let right_n = n - left_n;
        let mut le = e;
        let mut re = e;
        let mut delta = IVec3::ZERO;
        match axis {
            0 => {
                le.x = left_n;
                re.x = right_n;
                delta.x = left_n;
            }
            1 => {
                le.y = left_n;
                re.y = right_n;
                delta.y = left_n;
            }
            2 => {
                le.z = left_n;
                re.z = right_n;
                delta.z = left_n;
            }
            _ => unreachable!(),
        }
        Ok(Some((
            Self {
                origin: self.origin,
                extent_chunks: le,
            },
            Self {
                origin: self.origin.translated_chunks(delta)?,
                extent_chunks: re,
            },
        )))
    }
}

fn checked_extent(minimum: i32, maximum: i32) -> Result<i32, UsfPositionError> {
    let v = i64::from(maximum)
        .checked_sub(i64::from(minimum))
        .and_then(|v| v.checked_add(1))
        .ok_or(UsfPositionError::TranslationTooLarge)?;
    i32::try_from(v).map_err(|_| UsfPositionError::TranslationTooLarge)
}
fn checked_i32(value: Option<i64>) -> Result<i32, UsfPositionError> {
    i32::try_from(value.ok_or(UsfPositionError::TranslationTooLarge)?)
        .map_err(|_| UsfPositionError::TranslationTooLarge)
}
