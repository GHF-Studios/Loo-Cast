//! Voxel-owned runtime policy.

use serde::Deserialize;

mod manifestation;
mod overrides;
mod streaming;

pub use manifestation::{
    VoxelGroupingStrategy, VoxelManifestationConfig, VoxelManifestationGroupingConfig,
};
pub use overrides::{
    VoxelConfigOverrides, VoxelManifestationConfigOverrides,
    VoxelManifestationGroupingConfigOverrides, VoxelStreamingConfigOverrides,
};
pub use streaming::VoxelStreamingConfig;

#[derive(Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct VoxelConfig {
    pub streaming: VoxelStreamingConfig,
    pub manifestation: VoxelManifestationConfig,
}

impl Default for VoxelConfig {
    fn default() -> Self {
        Self {
            streaming: VoxelStreamingConfig::default(),
            manifestation: VoxelManifestationConfig::default(),
        }
    }
}

impl VoxelConfig {
    pub(super) fn apply_overrides(&mut self, overrides: &VoxelConfigOverrides) {
        self.streaming.apply_overrides(&overrides.streaming);
        self.manifestation.apply_overrides(&overrides.manifestation);
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        self.streaming.validate()?;
        self.manifestation.validate()
    }
}

fn require_positive(value: usize, field: &str) -> Result<(), String> {
    if value == 0 {
        Err(format!("{field} must be > 0"))
    } else {
        Ok(())
    }
}

fn validate_aligned_group_edge(value: i32, field: &str) -> Result<(), String> {
    const MATERIALIZATION_ATOMS_PER_USF_DIGIT: i32 = 100;

    if value <= 0
        || value > MATERIALIZATION_ATOMS_PER_USF_DIGIT
        || MATERIALIZATION_ATOMS_PER_USF_DIGIT % value != 0
    {
        return Err(format!(
            "{field} must be one of the positive divisors of 100 \
             (for example 1, 2, 4, 5, 10, 20, 25, 50, 100); got {value}"
        ));
    }
    Ok(())
}
