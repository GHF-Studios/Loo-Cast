//! Reusable spatial-topology primitives.
//!
//! This subsystem is independent from portals. Portal traversal is one adapter
//! that consumes these primitives; other topology mechanics may reuse them
//! without depending on game code.

mod hooks;
mod query;
mod split;

pub(super) use hooks::SpatialTopologyCollisionHooks;
pub use hooks::{SpatialSplitPeer, SpatialSplitPeerActive};
pub use query::KinematicQueryExclusions;
pub use split::{BoxPlanePartition, SpatialSplitBox, SplitPlane, partition_box_by_plane};
