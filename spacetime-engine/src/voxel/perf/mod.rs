//! Voxel worker scheduling policy.

use std::num::NonZeroUsize;

/// Bounds each voxel worker stage so engine work cannot monopolize the process.
pub(crate) fn per_stage_in_flight_limit() -> usize {
    let threads = std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).unwrap())
        .get();
    (threads / 2).max(2)
}
