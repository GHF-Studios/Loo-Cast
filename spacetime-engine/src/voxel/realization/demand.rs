//! Resolve semantic authority/Scale intent to disposable voxel worlds.

use super::*;

/// Resolve authority+Scale intent to the disposable world entity consumed by
/// the existing dense materialization backend.
pub(in crate::voxel) fn resolve_voxel_realization_demand(
    intents: Res<VoxelRealizationIntentSnapshot>,
    registry: Res<CelestialVoxelRealizationRegistry>,
    mut output: ResMut<VoxelRealizationDemandSnapshot>,
) {
    let mut next = VoxelRealizationDemandSnapshot::default();

    for intent in intents.iter() {
        let target_world = match intent.target {
            VoxelRealizationIntentTarget::ExistingWorld(world) => Some(world),
            VoxelRealizationIntentTarget::Celestial(target) => registry.world_for(target),
        };
        let Some(target_world) = target_world else {
            continue;
        };

        next.push(
            target_world,
            intent.scope,
            intent.roles,
            intent.view_source,
            intent.residency_half_extent_native,
            intent.priority_focus,
        );
    }

    next.sort_for_publication();
    output.replace_if_changed(next);
}
