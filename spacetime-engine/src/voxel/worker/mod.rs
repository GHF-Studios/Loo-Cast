//! Shared background worker policy for voxel realization.

use std::num::NonZeroUsize;

use bevy::prelude::*;

/// Marks any long-running voxel background job, regardless of pipeline stage.
#[derive(Component, Debug, Default, Clone, Copy)]
pub(super) struct VoxelWorkerTask;

/// Remaining shared voxel worker capacity.
///
/// Generation and surface derivation deliberately consume the same budget so
/// they cannot each reserve half the machine and collectively saturate it.
pub(super) fn available_slots(in_flight: usize) -> usize {
    let threads = std::thread::available_parallelism()
        .unwrap_or(NonZeroUsize::new(4).unwrap())
        .get();

    // Leave substantial CPU headroom for gameplay, physics, rendering and the
    // rest of Bevy's schedules.
    let limit = (threads / 2).max(2);
    limit.saturating_sub(in_flight)
}
