//! Reusable spatial-topology primitives.
//!
//! This subsystem is independent from portals. Portal traversal is one adapter
//! that consumes these primitives; other topology mechanics may reuse them
//! without depending on game code.
//!
//! ## Module map
//!
//! - `hooks`: Collision-pipeline integration for USF charts and split manifestations.
//! - `query`: Manual spatial-query exclusions used by topology-aware manifestations.
//! - `split`: Reusable chart-local box/plane partition geometry.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod hooks;
mod query;
mod split;

pub(super) use hooks::SpatialTopologyCollisionHooks;
pub use hooks::{SpatialSplitPeer, SpatialSplitPeerActive};
pub use query::{KinematicQueryExclusions, UsfRuntimeOwnershipQuery, runtime_semantic_of_world};
pub use split::{BoxPlanePartition, SpatialSplitBox, SplitPlane, partition_box_by_plane};
