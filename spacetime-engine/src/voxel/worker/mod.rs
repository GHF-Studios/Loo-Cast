//! Bounded execution facility for reconstructible voxel work.
//!
//! The queue selects durable closures; admission bounds compute pressure;
//! tickets govern publication; profiling observes without choosing work.

mod admission;
mod executor;
#[cfg(feature = "profiling-tracy")]
mod profiling;
mod queue;
mod ticket;

pub(super) use executor::{VoxelWorkExecutor, VoxelWorkTask};
#[cfg(feature = "profiling-tracy")]
pub(super) use profiling::emit_worker_pressure;
pub(super) use queue::VoxelWorkLane;
pub(super) use ticket::VoxelWorkTicket;
