//! Spawn the root procedural world and hierarchical scale realization.

use bevy::prelude::*;

use crate::{
    procedural_assets::ProceduralAssetLibrary,
    spatial::UsfPosition,
    worldgen::{PhenomenonRegistry, TemporalScale, WorldgenEpoch, WorldgenStore},
};

use super::scale_stack::ProceduralScaleStack;

#[derive(Component)]
struct ProceduralWorldRoot;

pub(super) fn spawn_procedural_world(
    mut commands: Commands,
    procedural_assets: Res<ProceduralAssetLibrary>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = UsfPosition::default();
    let root = worldgen
        .bootstrap_root(target, TemporalScale::WORLDGEN_SNAPSHOT, epoch, &registry)
        .expect("present-day root context must be canonically addressable");
    // Voxels are a geological/local realizer, not the universal representation.
    // The semantic universe exists at every scale; voxel terrain begins only
    // when geology becomes meaningful at Scale +4.
    let base_material = procedural_assets.debug_grid.clone();

    let stack_entity = commands
        .spawn((
            Name::new("Procedural Hierarchical Scale Stack"),
            ProceduralWorldRoot,
            Transform::IDENTITY,
            Visibility::Inherited,
        ))
        .id();

    commands
        .entity(stack_entity)
        .insert(ProceduralScaleStack::new(root, base_material));

    debug!(
        epoch_gyr = epoch.age_gyr(),
        "bootstrapped semantic universe root; realizers activate in their domain scales"
    );
}
