//! Typed layered runtime configuration.
//!
//! Configuration is applied in three layers: compiled defaults, the project RON
//! file, and typed runtime/developer overrides. Consumers read only the effective
//! [`EngineConfig`] resource.

mod model;
mod source;
mod voxel;

pub use model::{EngineConfig, EngineConfigOverrides};
pub use source::EngineConfigPlugin;
pub use voxel::{
    VoxelConfig, VoxelConfigOverrides, VoxelManifestationConfig,
    VoxelManifestationConfigOverrides, VoxelStreamingConfig, VoxelStreamingConfigOverrides,
};

#[cfg(test)]
mod tests;
