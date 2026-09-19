//! Constructive edits over the authoritative voxel field.

use bevy::prelude::Vec3;

use crate::spatial::{UsfPosition, UsfPositionError};

use super::{VoxelMaterialId, VoxelSample};

/// Number of leaf-native units around an analytic brush surface in which CSG
/// edits are allowed to update signed-distance values.
///
/// A true global SDF Boolean can change distance magnitudes arbitrarily far
/// from the edited surface. That is mathematically useful, but it defeats local
/// sparse editing. A narrow influence band preserves enough exterior/interior
/// distance data for smooth extraction while giving each edit finite bounds.
pub const EDIT_INFLUENCE_MARGIN: f32 = 2.0;

mod bounds;
mod brush;
mod operation;
mod position;

pub use bounds::VoxelBounds;
pub use brush::VoxelBrush;
pub use operation::VoxelEdit;
pub use position::VoxelQueryPosition;
pub(crate) use operation::{VoxelLocalBounds, VoxelLocalEdit};

#[cfg(test)]
mod tests;
