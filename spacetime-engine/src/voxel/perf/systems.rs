//! Fixed-step cadence sampling for voxel performance diagnostics.

use super::*;

pub(crate) fn count_fixed_step(mut probe: ResMut<FixedStepProbe>) {
    probe.steps_this_frame = probe.steps_this_frame.saturating_add(1);
}

pub(crate) fn sample_fixed_steps(
    mut probe: ResMut<FixedStepProbe>,
    mut stats: ResMut<VoxelPerfStats>,
) {
    let steps = probe.steps_this_frame;
    probe.steps_this_frame = 0;
    stats.fixed_step_samples += 1;
    stats.fixed_steps_total = stats.fixed_steps_total.saturating_add(u64::from(steps));
    stats.fixed_steps_max = stats.fixed_steps_max.max(steps);
}
