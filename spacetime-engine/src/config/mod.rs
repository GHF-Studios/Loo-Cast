//! Typed layered runtime configuration.
//!
//! Configuration is applied in three layers: compiled defaults, the project RON
//! file, and typed runtime/developer overrides. Consumers read only the effective
//! [`EngineConfig`] resource.
//!
//! ## Module map
//!
//! - `model`: Root effective configuration model.
//! - `source`: Project-file loading and hot reload.
//! - `voxel`: Voxel-owned runtime policy.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod model;
mod source;
mod voxel;

pub use model::{EngineConfig, EngineConfigOverrides};
pub use source::EngineConfigPlugin;
pub use voxel::{
    VoxelConfig, VoxelConfigOverrides, VoxelManifestationConfig, VoxelManifestationConfigOverrides,
    VoxelStreamingConfig, VoxelStreamingConfigOverrides,
};
