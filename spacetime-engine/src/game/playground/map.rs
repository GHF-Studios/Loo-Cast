//! Bootstrap for the authored physics playground map.
//!
//! The authored map supplies generic marker kinds. This adapter interprets a
//! small built-in vocabulary once after the map loads; it does not retain
//! authority over the player or portals afterward. Tools/mods can therefore
//! move them without being overwritten every frame.

use avian3d::prelude::LinearVelocity;
use bevy::{camera::visibility::NoFrustumCulling, prelude::*};

use crate::{
    game::{
        GameSet,
        map_selection::GameMap,
        player::Player,
        portal::{PortalCommand, PortalEndpoint, PortalPair, PortalTraveler},
    },
    geometry::{AuthoredMap, AuthoredMapMarker, AuthoredMapScene},
    spatial::UsfSpatialFrame,
    voxel::{
        VoxelBase, VoxelChunkCoord, VoxelChunkOf, VoxelMaterialId, VoxelQueryPosition, VoxelWorld,
        empty_voxel_mesh,
    },
};

const CAMPUS_MAP: &str = "maps/physics_campus.spacemap";
const PLAYER_SPAWN_MARKER: &str = "player_spawn";
const PORTAL_A_MARKER: &str = "portal_a";
const PORTAL_B_MARKER: &str = "portal_b";

pub struct PlaygroundMapPlugin;

impl Plugin for PlaygroundMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(GameMap::Playground),
            (load_campus, spawn_voxel_test_rock),
        )
        .add_systems(
            Update,
            place_player_at_spawn_marker.run_if(in_state(GameMap::Playground)),
        )
        .add_systems(
            Update,
            initialize_demo_portals_from_markers
                .in_set(GameSet::Action)
                .run_if(in_state(GameMap::Playground)),
        );
    }
}

fn load_campus(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle: Handle<AuthoredMap> = asset_server.load(CAMPUS_MAP);

    commands.spawn((Name::new("Physics Campus"), AuthoredMapScene::new(handle)));
}

/// Keeps voxel mechanics immediately testable in the authored playground while
/// leaving ownership of world setup out of the voxel-hand item itself.
fn spawn_voxel_test_rock(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    frame: Res<UsfSpatialFrame>,
) {
    let center = Vec3::new(0.0, 4.0, -8.0);
    let world_origin = *frame.origin();
    let semantic_center = VoxelQueryPosition::new(world_origin)
        .translated(center)
        .expect("playground voxel center must be a valid canonical translation");
    let world_entity = commands
        .spawn((Name::new("Playground Voxel Rock"), Transform::IDENTITY))
        .id();
    let material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.34, 0.31, 0.27),
        perceptual_roughness: 1.0,
        ..default()
    });
    let mut world = VoxelWorld::new_at(
        VoxelBase::sphere(semantic_center, 6.0, VoxelMaterialId::ROCK),
        world_origin,
    );

    for z in -2..=-1 {
        for y in -1..=0 {
            for x in -1..=0 {
                let coord = VoxelChunkCoord::new(IVec3::new(x, y, z));
                let address = world
                    .chunk_address(coord)
                    .expect("playground voxel materialization must translate canonically");
                let chunk = world.materialize_chunk(address);
                let local_translation = address
                    .query_origin()
                    .relative_to(VoxelQueryPosition::new(*frame.origin()), 64.0)
                    .expect("playground voxel materialization must project locally");
                let chunk_entity = commands
                    .spawn((
                        Name::new(format!(
                            "Playground Voxel Materialization Chunk ({x}, {y}, {z})"
                        )),
                        VoxelChunkOf::new(world_entity),
                        address,
                        chunk,
                        Mesh3d(meshes.add(empty_voxel_mesh())),
                        MeshMaterial3d(material.clone()),
                        NoFrustumCulling,
                        Transform::from_translation(local_translation),
                    ))
                    .id();

                assert!(world.insert_chunk(address, chunk_entity).is_none());
            }
        }
    }

    commands.entity(world_entity).insert(world);
}

/// Applies the authored player spawn once. Hot-reloading the map never yanks
/// the player out of their current experiment.
fn place_player_at_spawn_marker(
    mut placed: Local<bool>,
    markers: Query<(&AuthoredMapMarker, &Transform), Without<Player>>,
    player: Single<(&mut Transform, &mut PortalTraveler, &mut LinearVelocity), With<Player>>,
) {
    if *placed {
        return;
    }

    let Some((_, spawn)) = markers
        .iter()
        .find(|(marker, _)| marker.kind == PLAYER_SPAWN_MARKER)
    else {
        return;
    };

    let (mut transform, mut traveler, mut velocity) = player.into_inner();
    *transform = *spawn;
    velocity.0 = Vec3::ZERO;
    traveler.commit_position(transform.translation);
    *placed = true;
}

/// Seeds the persistent demo pair from authored markers exactly once.
fn initialize_demo_portals_from_markers(
    mut initialized: Local<bool>,
    pair: Option<Res<PortalPair>>,
    markers: Query<(&AuthoredMapMarker, &Transform)>,
    mut portal_commands: MessageWriter<PortalCommand>,
) {
    if *initialized || pair.is_none() {
        return;
    }

    let mut first = None;
    let mut second = None;

    for (marker, transform) in &markers {
        match marker.kind.as_str() {
            PORTAL_A_MARKER => first = Some(*transform),
            PORTAL_B_MARKER => second = Some(*transform),
            _ => {}
        }
    }

    let (Some(first), Some(second)) = (first, second) else {
        return;
    };

    portal_commands.write(PortalCommand::Place {
        endpoint: PortalEndpoint::First,
        transform: first,
    });
    portal_commands.write(PortalCommand::Place {
        endpoint: PortalEndpoint::Second,
        transform: second,
    });
    *initialized = true;
}
