//! Voxel-owned runtime policy.

use serde::Deserialize;

mod manifestation;
mod overrides;
mod streaming;

pub use manifestation::VoxelManifestationConfig;
pub use overrides::{
    VoxelConfigOverrides, VoxelManifestationConfigOverrides, VoxelStreamingConfigOverrides,
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

fn validate_generation_group_edge(value: i32, field: &str) -> Result<(), String> {
    const MATERIALIZATION_CHUNKS_PER_USF_DIGIT: i32 = 100;

    if value <= 0
        || value > MATERIALIZATION_CHUNKS_PER_USF_DIGIT
        || MATERIALIZATION_CHUNKS_PER_USF_DIGIT % value != 0
    {
        return Err(format!(
            "{field} must be a positive divisor of {MATERIALIZATION_CHUNKS_PER_USF_DIGIT}; got {value}"
        ));
    }
    Ok(())
}
