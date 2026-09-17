//! Bootstrap for the actual top-down procedural game-world path.

use avian3d::prelude::LinearVelocity;
use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::NoFrustumCulling,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    game::{
        player::{Player, PlayerNoclip},
        portal::PortalTraveler,
    },
    physics::character::{CharacterDimensions, CharacterMotor},
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfPosition, UsfScalePresentation, UsfSpatialFrame},
    voxel::{ProceduralTerrain, VoxelBase, VoxelQueryPosition, VoxelStreaming, VoxelWorld},
    worldgen::{
        GEOLOGY_CLIMATE_HYDROLOGY, GeologyClimateHydrologyState, PhenomenonRegistry, TemporalScale,
        WorldgenEpoch, WorldgenEvaluationKey, WorldgenStore,
    },
};

use super::map_selection::GameMap;

#[derive(Component)]
struct ProceduralWorldRoot;

pub struct ProceduralWorldPlugin;

impl Plugin for ProceduralWorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMap::ProceduralWorld),
            (spawn_procedural_world, prepare_player).chain(),
        );
    }
}

fn spawn_procedural_world(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    frame: Res<UsfSpatialFrame>,
    procedural_assets: Res<ProceduralAssetLibrary>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let before = worldgen.len();
    let leaf = worldgen
        .ensure_branch(
            *frame.origin(),
            SpatialScale::ZERO,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
            &registry,
        )
        .expect("present-day worldgen branch must be canonically addressable");
    let generated = worldgen.len() - before;
    let terrain = terrain_for_branch(&worldgen, leaf);
    let lineage = worldgen.lineage(leaf);

    info!(
        generated_scopes = generated,
        lineage_depth = lineage.len(),
        epoch_gyr = epoch.age_gyr(),
        "generated sparse USF world branch from Scale +35 to Scale 0"
    );
    for node in lineage.iter().rev() {
        let phenomena = node
            .phenomena()
            .map(|snapshot| format!("{}: {}", snapshot.id().name(), snapshot.summary()))
            .collect::<Vec<_>>()
            .join(" | ");
        debug!(
            scale = %node.context().spatial_scale(),
            temporal_scale = node.context().temporal_scale().label(),
            %phenomena,
            "USF worldgen lineage"
        );
    }

    let world_entity = commands
        .spawn((
            Name::new("Procedural World"),
            ProceduralWorldRoot,
            VoxelWorld::new_at(VoxelBase::Terrain(terrain), *frame.origin()),
            VoxelStreaming::new(24, procedural_assets.cracked_clay.material.clone()),
            Transform::IDENTITY,
        ))
        .id();

    let s1 = SpatialScale::new(1).expect("S+1 exists");
    let s1_key = worldgen
        .key_for(*frame.origin(), s1, TemporalScale::WORLDGEN_SNAPSHOT, epoch)
        .expect("S+1 worldgen scope must be addressable");
    let s1_terrain = terrain_for_branch(&worldgen, s1_key);
    let landscape_mesh = coarse_landscape_mesh(s1_terrain, *frame.origin());
    let landscape_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.24, 0.29, 0.18),
        perceptual_roughness: 1.0,
        ..default()
    });

    commands.spawn((
        Name::new("Scale +1 Landscape Presentation"),
        ChildOf(world_entity),
        UsfScalePresentation::new(*frame.origin(), s1),
        Mesh3d(meshes.add(landscape_mesh)),
        MeshMaterial3d(landscape_material),
        NoFrustumCulling,
        Transform::IDENTITY,
        Visibility::Hidden,
    ));

    super::scale_stack::spawn_higher_scale_stack(
        &mut commands,
        &mut meshes,
        &mut materials,
        world_entity,
        *frame.origin(),
        terrain,
    );
}

fn terrain_for_branch(worldgen: &WorldgenStore, leaf: WorldgenEvaluationKey) -> ProceduralTerrain {
    let geology = worldgen
        .state::<GeologyClimateHydrologyState>(leaf, GEOLOGY_CLIMATE_HYDROLOGY)
        .expect("worldgen branch must carry geology/climate/hydrology state");

    ProceduralTerrain::configured(
        geology.terrain_seed,
        -4.0,
        geology.local_relief_m,
        geology.terrain_frequency,
    )
}

fn coarse_landscape_mesh(terrain: ProceduralTerrain, anchor: UsfPosition) -> Mesh {
    const CELLS: u32 = 128;
    const HALF_EXTENT_S1: f32 = 2_000.0;
    const METERS_PER_S1_UNIT: f32 = 10.0;

    let side = CELLS + 1;
    let step = HALF_EXTENT_S1 * 2.0 / CELLS as f32;
    let world_origin = VoxelQueryPosition::new(anchor);
    let mut positions = Vec::with_capacity((side * side) as usize);
    let mut heights = Vec::with_capacity((side * side) as usize);
    let mut uvs = Vec::with_capacity((side * side) as usize);

    for z in 0..side {
        for x in 0..side {
            let sx = -HALF_EXTENT_S1 + x as f32 * step;
            let sz = -HALF_EXTENT_S1 + z as f32 * step;
            let query = world_origin
                .translated(Vec3::new(
                    sx * METERS_PER_S1_UNIT,
                    0.0,
                    sz * METERS_PER_S1_UNIT,
                ))
                .expect("bounded S+1 landscape sample must translate canonically");
            let sy = terrain.height_at(world_origin, query) / METERS_PER_S1_UNIT;
            heights.push(sy);
            positions.push([sx, sy, sz]);
            uvs.push([
                (sx + HALF_EXTENT_S1) / (HALF_EXTENT_S1 * 2.0),
                (sz + HALF_EXTENT_S1) / (HALF_EXTENT_S1 * 2.0),
            ]);
        }
    }

    let index = |x: u32, z: u32| -> usize { (z * side + x) as usize };
    let mut normals = Vec::with_capacity(positions.len());
    for z in 0..side {
        for x in 0..side {
            let left = heights[index(x.saturating_sub(1), z)];
            let right = heights[index((x + 1).min(CELLS), z)];
            let down = heights[index(x, z.saturating_sub(1))];
            let up = heights[index(x, (z + 1).min(CELLS))];
            let dx = (right - left)
                / if x == 0 || x == CELLS {
                    step
                } else {
                    step * 2.0
                };
            let dz = (up - down)
                / if z == 0 || z == CELLS {
                    step
                } else {
                    step * 2.0
                };
            normals.push(Vec3::new(-dx, 1.0, -dz).normalize().to_array());
        }
    }

    let mut indices = Vec::with_capacity((CELLS * CELLS * 6) as usize);
    for z in 0..CELLS {
        for x in 0..CELLS {
            let a = z * side + x;
            let b = a + 1;
            let c = a + side;
            let d = c + 1;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
    .with_inserted_indices(Indices::U32(indices))
}

/// Places the player above the first streamed terrain window with ordinary
/// character physics enabled. The terrain parameters come from the exact same
/// Scale-0 semantic branch used to create the voxel realization.
fn prepare_player(
    mut commands: Commands,
    frame: Res<UsfSpatialFrame>,
    worldgen: Res<WorldgenStore>,
    player: Single<
        (
            Entity,
            &mut Transform,
            &mut PortalTraveler,
            &mut LinearVelocity,
            &mut PlayerNoclip,
        ),
        With<Player>,
    >,
) {
    let epoch = WorldgenEpoch::present_day_bootstrap();
    let leaf = worldgen
        .key_for(
            *frame.origin(),
            SpatialScale::ZERO,
            TemporalScale::WORLDGEN_SNAPSHOT,
            epoch,
        )
        .expect("player worldgen branch must be canonically addressable");
    let terrain = terrain_for_branch(&worldgen, leaf);

    let (entity, mut transform, mut traveler, mut velocity, mut noclip) = player.into_inner();
    let x = 0.0;
    let z = 8.0;
    let world_origin = VoxelQueryPosition::new(*frame.origin());
    let terrain_query = world_origin
        .translated(Vec3::new(x, 0.0, z))
        .expect("procedural spawn query must translate canonically");
    let ground = terrain.height_at(world_origin, terrain_query);
    let position = Vec3::new(x, ground + CharacterDimensions::HALF_HEIGHT + 0.20, z);

    transform.translation = position;
    traveler.commit_position(position);
    velocity.0 = Vec3::ZERO;
    noclip.active = false;
    commands.entity(entity).insert(CharacterMotor);
}
