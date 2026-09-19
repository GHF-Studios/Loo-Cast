//! Spawn the root procedural world and hierarchical scale realization.

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfPosition, UsfScaleLayer},
    voxel::{VoxelBase, VoxelPresentationMaterial, VoxelStreaming, VoxelWorld},
    worldgen::{PhenomenonRegistry, TemporalScale, WorldgenEpoch, WorldgenStore},
};

use super::scale_stack::{ProceduralScaleStack, volume_for_scale_context};

#[derive(Component)]
struct ProceduralWorldRoot;

pub(super) fn spawn_procedural_world(
    config: Res<EngineConfig>,
    mut commands: Commands,
    procedural_assets: Res<ProceduralAssetLibrary>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let target = UsfPosition::default();
    let root = worldgen
        .bootstrap_root(target, TemporalScale::WORLDGEN_SNAPSHOT, epoch, &registry)
        .expect("present-day root context must be canonically addressable");
    let volume = volume_for_scale_context(&worldgen, root);

    // Keep scale-compositor depth policy local to these terrain realizers rather
    // than mutating the shared procedural asset-library material.
    let base_material = procedural_assets.debug_grid.clone();
    let root_material = materials
        .get(&base_material)
        .cloned()
        .map(|mut material| {
            material.depth_bias = 0.0;
            materials.add(material)
        })
        .expect("procedural debug-grid material must exist before world bootstrap");

    let stack_entity = commands
        .spawn((
            Name::new("Procedural Hierarchical Scale Stack"),
            ProceduralWorldRoot,
            Transform::IDENTITY,
            Visibility::Inherited,
        ))
        .id();

    let root_world = commands
        .spawn((
            Name::new("USF Scale +35 Root Voxel World"),
            ChildOf(stack_entity),
            UsfScaleLayer::new(SpatialScale::MAX),
            VoxelWorld::new_at(
                VoxelBase::Volume(volume),
                UsfPosition::zero(SpatialScale::MAX),
            ),
            VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
            VoxelPresentationMaterial::new(root_material.clone()),
            Transform::IDENTITY,
            Visibility::Inherited,
        ))
        .id();

    commands
        .entity(stack_entity)
        .insert(ProceduralScaleStack::new(
            root,
            base_material,
            root_material,
            root_world,
        ));

    debug!(
        epoch_gyr = epoch.age_gyr(),
        "bootstrapped visible Scale +35 voxel root; finer scales remain contextual refinements"
    );
}
