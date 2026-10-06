//! Root effective configuration model.

use bevy::prelude::Resource;
use serde::Deserialize;

use super::voxel::{VoxelConfig, VoxelConfigOverrides};

/// Effective engine configuration consumed by runtime systems.
#[derive(Resource, Debug, Clone, PartialEq, Deserialize)]
#[serde(default)]
pub struct EngineConfig {
    pub voxel: VoxelConfig,
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            voxel: VoxelConfig::default(),
        }
    }
}

impl EngineConfig {
    fn apply_overrides(&mut self, overrides: &EngineConfigOverrides) {
        self.voxel.apply_overrides(&overrides.voxel);
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        self.voxel.validate()
    }

    pub(super) fn resolved(&self, overrides: &EngineConfigOverrides) -> Result<Self, String> {
        let mut effective = self.clone();
        effective.apply_overrides(overrides);
        effective.validate()?;
        Ok(effective)
    }
}

/// Highest-priority typed runtime overrides.
#[derive(Resource, Debug, Clone, Default)]
pub struct EngineConfigOverrides {
    pub voxel: VoxelConfigOverrides,
}
