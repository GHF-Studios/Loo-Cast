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
    pub(super) fn apply_overrides(&mut self, overrides: &EngineConfigOverrides) {
        self.voxel.apply_overrides(&overrides.voxel);
    }

    pub(super) fn validate(&self) -> Result<(), String> {
        self.voxel.validate()
    }
}

/// Highest-priority typed runtime overrides.
#[derive(Resource, Debug, Clone, Default)]
pub struct EngineConfigOverrides {
    pub voxel: VoxelConfigOverrides,
}
