//! Tracy observation of scheduler pressure; never scheduling authority.

use super::{VoxelWorkExecutor, VoxelWorkLane};
use bevy::prelude::*;

#[cfg(feature = "profiling-tracy")]
pub(in crate::voxel) fn emit_worker_pressure(workers: Res<VoxelWorkExecutor>) {
    let Some(client) = tracy_client::Client::running() else {
        return;
    };

    let generation_outstanding = workers.admission.outstanding(VoxelWorkLane::Generation);
    let generation_running = workers.admission.running(VoxelWorkLane::Generation);
    let derivation_outstanding = workers.admission.outstanding(VoxelWorkLane::Derivation);
    let derivation_running = workers.admission.running(VoxelWorkLane::Derivation);
    let planning_outstanding = workers
        .admission
        .outstanding(VoxelWorkLane::PresentationPlanning);
    let planning_running = workers
        .admission
        .running(VoxelWorkLane::PresentationPlanning);

    client.plot(
        tracy_client::plot_name!("Voxel workers/capacity"),
        workers.capacity() as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/running"),
        workers.admission.total_running() as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/outstanding"),
        workers.admission.total_outstanding() as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/queued"),
        workers
            .admission
            .total_outstanding()
            .saturating_sub(workers.admission.total_running()) as f64,
    );

    client.plot(
        tracy_client::plot_name!("Voxel workers/Generation running"),
        generation_running as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Generation queued"),
        generation_outstanding.saturating_sub(generation_running) as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Generation avg ms"),
        workers
            .admission
            .average_job_seconds(VoxelWorkLane::Generation)
            .unwrap_or(0.0)
            * 1_000.0,
    );

    client.plot(
        tracy_client::plot_name!("Voxel workers/Derivation running"),
        derivation_running as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Derivation queued"),
        derivation_outstanding.saturating_sub(derivation_running) as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/Derivation avg ms"),
        workers
            .admission
            .average_job_seconds(VoxelWorkLane::Derivation)
            .unwrap_or(0.0)
            * 1_000.0,
    );

    client.plot(
        tracy_client::plot_name!("Voxel workers/PresentationPlanning running"),
        planning_running as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/PresentationPlanning queued"),
        planning_outstanding.saturating_sub(planning_running) as f64,
    );
    client.plot(
        tracy_client::plot_name!("Voxel workers/PresentationPlanning avg ms"),
        workers
            .admission
            .average_job_seconds(VoxelWorkLane::PresentationPlanning)
            .unwrap_or(0.0)
            * 1_000.0,
    );
}
