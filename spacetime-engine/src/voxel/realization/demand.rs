//! Resolve semantic authority/Scale intent to disposable voxel scale realizations.

use super::*;

/// Resolve authority+Scale intent to the disposable realization entity consumed by
/// the existing dense materialization backend.
pub(in crate::voxel) fn resolve_voxel_realization_demands(
    intents: Res<VoxelRealizationIntentSnapshot>,
    registry: Res<CelestialVoxelRealizations>,
    mut output: ResMut<VoxelRealizationDemandSnapshot>,
) {
    let mut next = VoxelRealizationDemandSnapshot::default();

    for intent in intents.iter() {
        let target_realization = match intent.target {
            VoxelRealizationIntentTarget::ExistingRealization(world) => Some(world),
            VoxelRealizationIntentTarget::Celestial(target) => registry.realization_for(target),
        };
        let Some(target_realization) = target_realization else {
            continue;
        };

        next.push(
            target_realization,
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
