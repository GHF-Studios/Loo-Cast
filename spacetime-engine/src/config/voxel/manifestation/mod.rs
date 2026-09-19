//! Runtime rebuild and physics policy for voxel manifestations.

use serde::Deserialize;

use super::overrides::VoxelManifestationConfigOverrides;
use super::require_positive;

/// Runtime manifestation policy for derived voxel representations.
///
/// One active materialization surface maps to one runtime manifestation.
/// There is deliberately no cross-materialization render or collision grouping.
#[derive(Debug, Clone, Copy, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoxelManifestationConfig {
    pub rebuild_budget_per_frame: usize,
    pub physics_interaction_radius_native: f32,
}

impl Default for VoxelManifestationConfig {
    fn default() -> Self {
        Self {
            rebuild_budget_per_frame: 8,
            physics_interaction_radius_native: 32.0,
        }
    }
}

impl VoxelManifestationConfig {
    pub(super) fn apply_overrides(&mut self, overrides: &VoxelManifestationConfigOverrides) {
        if let Some(value) = overrides.rebuild_budget_per_frame {
            self.rebuild_budget_per_frame = value;
        }
        if let Some(value) = overrides.physics_interaction_radius_native {
            self.physics_interaction_radius_native = value;
        }
    }

    pub(super) fn validate(self) -> Result<(), String> {
        require_positive(
            self.rebuild_budget_per_frame,
            "voxel.manifestation.rebuild_budget_per_frame",
        )?;
        if !self.physics_interaction_radius_native.is_finite()
            || self.physics_interaction_radius_native <= 0.0
        {
            return Err(
                "voxel.manifestation.physics_interaction_radius_native must be finite and > 0"
                    .into(),
            );
        }
        Ok(())
    }
}
