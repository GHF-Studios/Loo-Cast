//! Movable playground probe for generic spatial demand.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    ecs::{
        UsfEntity, UsfLogicalProjection, UsfManifestationAuthority, UsfManifestationOf,
        UsfManifestations, UsfPresentationProjectionOf,
    },
    game::{
        GameAssets, GameSet,
        combat::{Health, Hitbox},
    },
    spatial::SpatialDemandSource,
    voxel::VoxelMaterializationDemand,
};

use super::super::{
    PlaygroundCatalog, PlaygroundItem, PlaygroundItemAction, PlaygroundItemId, PlaygroundPickable,
    PlaygroundRoot, UsePlaygroundItem,
};

pub const CHUNKLOADING_CUBE: PlaygroundItemId = PlaygroundItemId::new("chunkloading_cube");

const CUBE_SIZE: f32 = 1.0;
const CUBE_MASS_KG: f32 = 20.0;
const MAXIMUM_HEALTH: f32 = 100.0;
const PLACEMENT_DISTANCE: f32 = 100.0;
const SURFACE_CLEARANCE: f32 = 0.08;
const DEMAND_HALF_EXTENT: Vec3 = Vec3::new(20.0, 10.0, 20.0);
const DEMAND_PRIORITY: i32 = 50;

#[derive(Resource, Default)]
struct ChunkloadingCubeCounter(u64);

pub struct ChunkloadingCubeItemPlugin;

impl Plugin for ChunkloadingCubeItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkloadingCubeCounter>()
            .add_systems(PreStartup, register_item)
            .add_systems(Update, use_chunkloading_cube.in_set(GameSet::Action));
    }
}

fn register_item(mut catalog: ResMut<PlaygroundCatalog>) {
    catalog.register(PlaygroundItem {
        id: CHUNKLOADING_CUBE,
        name: "Chunkloading Cube",
        description: "Movable spatial-demand source that keeps nearby voxel materialization realized.",
    });
}

fn use_chunkloading_cube(
    mut commands: Commands,
    mut uses: MessageReader<UsePlaygroundItem>,
    assets: Res<GameAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    manifestations: Query<&UsfManifestationOf>,
    semantic_entities: Query<&UsfManifestations>,
    spatial_query: SpatialQuery,
    mut counter: ResMut<ChunkloadingCubeCounter>,
) {
    for request in uses.read() {
        if request.item != CHUNKLOADING_CUBE || request.action != PlaygroundItemAction::PRIMARY {
            continue;
        }

        let Ok(direction) = Dir3::new(request.aim.direction) else {
            continue;
        };
        let filter = manifestations
            .get(request.actor)
            .ok()
            .and_then(|manifestation| semantic_entities.get(manifestation.0).ok())
            .map(|manifestations| SpatialQueryFilter::from_excluded_entities(manifestations.iter()))
            .unwrap_or_else(|| SpatialQueryFilter::from_excluded_entities([request.actor]));
        let Some(hit) = spatial_query.cast_ray(
            request.aim.origin,
            direction,
            PLACEMENT_DISTANCE,
            false,
            &filter,
        ) else {
            continue;
        };

        let normal = hit.normal.normalize_or_zero();
        if normal == Vec3::ZERO {
            continue;
        }
        let hit_point = request.aim.origin + request.aim.direction * hit.distance;
        let half = CUBE_SIZE * 0.5;
        let support_radius = half * (normal.x.abs() + normal.y.abs() + normal.z.abs());
        let position = hit_point + normal * (support_radius + SURFACE_CLEARANCE);

        counter.0 += 1;
        let root = commands
            .spawn((
                Name::new(format!("Chunkloading Cube {}", counter.0)),
                PlaygroundRoot,
                UsfEntity,
                Health::new(MAXIMUM_HEALTH),
            ))
            .id();

        let collider = Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE);
        let inertia = AngularInertia::from_shape(&collider, CUBE_MASS_KG);
        let manifestation = commands
            .spawn((
            (
                Name::new(format!("Chunkloading Cube Manifestation {}", counter.0)),
                UsfManifestationOf(root),
                UsfManifestationAuthority,
                UsfLogicalProjection,
                PlaygroundPickable::cube(root, CUBE_SIZE),
                Hitbox::cube(CUBE_SIZE),
                SpatialDemandSource::cuboid(DEMAND_HALF_EXTENT).with_priority(DEMAND_PRIORITY),
                VoxelMaterializationDemand,
                RigidBody::Dynamic,
                SleepingDisabled,
                Mass(CUBE_MASS_KG),
                inertia,
                CenterOfMass::ZERO,
            ), (
                LinearVelocity::ZERO,
                AngularVelocity::ZERO,
                collider,
                Transform::from_translation(position),
            )
        ))
            .id();

        commands.entity(manifestation).with_children(|parent| {
            parent.spawn((
                Name::new("Chunkloading Cube Presentation"),
                UsfPresentationProjectionOf(manifestation),
                Mesh3d(meshes.add(Cuboid::from_length(CUBE_SIZE))),
                MeshMaterial3d(assets.damageable_cube_material.clone()),
                Transform::IDENTITY,
            ));
        });
    }
}
