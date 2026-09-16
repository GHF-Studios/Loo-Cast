//! Sparse authoritative modifications layered over a reconstructible base.

use super::{VoxelBounds, VoxelEdit};

/// Ordered edit log representing all non-procedural voxel state.
///
/// M2 keeps this deliberately simple: untouched space costs nothing, while each
/// edit costs one compact operation. A spatial index can replace the linear
/// lookup later without changing the base + modification model or persistence
/// format semantics.
#[derive(Debug, Clone, Default)]
pub struct VoxelModificationLayer {
    edits: Vec<VoxelEdit>,
}

impl VoxelModificationLayer {
    pub fn push(&mut self, edit: VoxelEdit) {
        self.edits.push(edit);
    }

    pub fn len(&self) -> usize {
        self.edits.len()
    }

    pub fn is_empty(&self) -> bool {
        self.edits.is_empty()
    }

    pub fn edits(&self) -> &[VoxelEdit] {
        &self.edits
    }

    pub fn intersecting(&self, bounds: VoxelBounds) -> impl Iterator<Item = VoxelEdit> + '_ {
        self.edits
            .iter()
            .copied()
            .filter(move |edit| edit.influence_bounds().intersects(bounds))
    }
}

#[cfg(test)]
mod tests {
    use bevy::prelude::Vec3;

    use super::*;
    use crate::voxel::{VoxelBrush, VoxelMaterialId};

    #[test]
    fn only_spatially_relevant_edits_are_replayed_for_a_chunk() {
        let mut layer = VoxelModificationLayer::default();
        layer.push(VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::ZERO, 2.0),
            material: VoxelMaterialId::ROCK,
        });
        layer.push(VoxelEdit::Add {
            brush: VoxelBrush::sphere(Vec3::splat(1000.0), 2.0),
            material: VoxelMaterialId::ROCK,
        });

        let local = VoxelBounds::new(Vec3::splat(-4.0), Vec3::splat(4.0));
        assert_eq!(layer.intersecting(local).count(), 1);
    }
}
