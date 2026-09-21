//! Damageable, thermal, dynamic rigid cubes exposed as playground items.

use avian3d::prelude::*;
use bevy::prelude::*;

use crate::{
    ecs::{
        UsfEntity, UsfLogicalProjection, UsfManifestationOf, UsfManifestations,
        UsfPresentationProjectionOf,
    },
    game::{
        GameSet,
        health::{Health, DamageableBounds},
        item::{ItemAction, ItemActionHint, ItemCatalog, ItemDefinition, ItemId, UseItem},
    },
    physics::topology::{SpatialSplitBox, SpatialSplitPeer},
    portal::{PortalRigidSplitBody, PortalSplitTraveler, PortalSplitVisual, PortalTraveler},
    thermal::{
        CombustibleMaterial, Fuel, ThermalBody, ThermalField, ThermalMaterial,
        ThermalSpatialSample,
    },
};

use super::assets::PlaygroundItemPresentationAssets;
use super::super::{PlaygroundPickable, PlaygroundRoot};

pub const DAMAGEABLE_CUBE: ItemId = ItemId::new("damageable_cube");
pub const SPLIT_DAMAGEABLE_CUBE: ItemId = ItemId::new("split_damageable_cube");

const CUBE_SIZE: f32 = 1.0;
const CUBE_MASS_KG: f32 = 20.0;
const MAXIMUM_HEALTH: f32 = 100.0;
const PLACEMENT_DISTANCE: f32 = 100.0;
const SURFACE_CLEARANCE: f32 = 0.08;

#[derive(Resource, Default)]
struct CubeCounter(u64);

pub struct DamageableCubeItemPlugin;

impl Plugin for DamageableCubeItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CubeCounter>()
            .add_systems(PreStartup, register_items)
            .add_systems(Update, use_cube_items.in_set(GameSet::Action));
    }
}

fn register_items(mut catalog: ResMut<ItemCatalog>) {
    catalog.register(ItemDefinition {
        id: DAMAGEABLE_CUBE,
        name: "Damageable Cube",
        description: "Dynamic rigid cube with Health, finite fuel and an internal thermal-energy field.",
        action_hints: vec![
            ItemActionHint::new(ItemAction::PRIMARY, "Place cube"),
        ],
    });

    catalog.register(ItemDefinition {
        id: SPLIT_DAMAGEABLE_CUBE,
        name: "Split Damageable Cube",
        description: "One semantic state with two independently dynamic spatial manifestations.",
        action_hints: vec![
            ItemActionHint::new(ItemAction::PRIMARY, "Place split cube"),
        ],
    });
}

fn use_cube_items(
    mut commands: Commands,
    mut uses: MessageReader<UseItem>,
    assets: Res<PlaygroundItemPresentationAssets>,
    mut meshes: ResMut<Assets<Mesh>>,
    manifestations: Query<&UsfManifestationOf>,
    semantic_entities: Query<&UsfManifestations>,
    spatial_query: SpatialQuery,
    mut counter: ResMut<CubeCounter>,
) {
    for request in uses.read() {
        if request.action != ItemAction::PRIMARY {
            continue;
        }

        let manifestation_count = match request.item {
            DAMAGEABLE_CUBE => 1,
            SPLIT_DAMAGEABLE_CUBE => 2,
            _ => continue,
        };

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
        // The cube is world-axis aligned when spawned. Its support radius along
        // an arbitrary surface normal is larger than half its side length on
        // sloped faces, so use the projected half-extents rather than assuming
        // a horizontal plane.
        let half = CUBE_SIZE * 0.5;
        let support_radius = half * (normal.x.abs() + normal.y.abs() + normal.z.abs());
        let spawn_center = hit_point + normal * (support_radius + SURFACE_CLEARANCE);

        counter.0 += 1;
        let logical_name = if request.item == DAMAGEABLE_CUBE {
            format!("Cube {}", counter.0)
        } else {
            format!("Split Cube {}", counter.0)
        };

        let thermal_material = ThermalMaterial::dry_wood();
        let thermal_field =
            ThermalField::ambient_box(Vec3::splat(CUBE_SIZE), UVec3::splat(6), &thermal_material);
        let thermal_capacity =
            thermal_field.total_heat_capacity_joules_per_kelvin(&thermal_material);

        let root = commands
            .spawn((
                Name::new(logical_name),
                PlaygroundRoot,
                UsfEntity,
                Health::new(MAXIMUM_HEALTH),
                ThermalBody::ambient(thermal_capacity, 0.0),
                thermal_material,
                thermal_field,
                CombustibleMaterial::wood_like(),
                Fuel::new(1_200_000.0),
            ))
            .id();

        let tangent = surface_tangent(normal);
        for index in 0..manifestation_count {
            let offset = if manifestation_count == 1 {
                Vec3::ZERO
            } else {
                tangent * if index == 0 { -1.25 } else { 1.25 }
            };
            spawn_dynamic_manifestation(
                &mut commands,
                &mut meshes,
                &assets,
                root,
                index,
                spawn_center + offset,
            );
        }
    }
}

fn surface_tangent(normal: Vec3) -> Vec3 {
    [Vec3::X, Vec3::Z, Vec3::Y]
        .into_iter()
        .map(|axis| (axis - normal * axis.dot(normal)).normalize_or_zero())
        .find(|axis| *axis != Vec3::ZERO)
        .unwrap_or(Vec3::X)
}

fn spawn_dynamic_manifestation(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    assets: &PlaygroundItemPresentationAssets,
    semantic: Entity,
    index: usize,
    position: Vec3,
) {
    let transform = Transform::from_translation(position);
    let full_collider = Collider::cuboid(CUBE_SIZE, CUBE_SIZE, CUBE_SIZE);
    let inertia = AngularInertia::from_shape(&full_collider, CUBE_MASS_KG);

    let authority = commands
        .spawn((
            Name::new(format!("Cube Manifestation {index}")),
            UsfManifestationOf(semantic),
            UsfLogicalProjection,
            ThermalSpatialSample,
            PlaygroundPickable::cube(semantic, CUBE_SIZE),
            DamageableBounds::cube(CUBE_SIZE),
            SpatialSplitBox::from_size(Vec3::splat(CUBE_SIZE)),
            PortalTraveler::new(position),
            PortalRigidSplitBody::default(),
            transform,
        ))
        .insert((
            RigidBody::Dynamic,
            SleepingDisabled,
            Mass(CUBE_MASS_KG),
            inertia,
            CenterOfMass::ZERO,
            LinearVelocity::ZERO,
            AngularVelocity::ZERO,
            full_collider.clone(),
        ))
        .id();

    let peer = commands
        .spawn((
            Name::new(format!("Cube Portal Peer {index}")),
            UsfManifestationOf(semantic),
            UsfLogicalProjection,
            SpatialSplitPeer { authority },
            ThermalSpatialSample,
            PlaygroundPickable::cube(semantic, CUBE_SIZE),
            DamageableBounds::cube(CUBE_SIZE),
            transform,
        ))
        .insert((
            ActiveCollisionHooks::FILTER_PAIRS,
            RigidBody::Dynamic,
            SleepingDisabled,
            CustomVelocityIntegration,
            Mass(CUBE_MASS_KG),
            inertia,
            CenterOfMass::ZERO,
            LinearVelocity::ZERO,
            AngularVelocity::ZERO,
            full_collider,
            CollisionLayers::NONE,
        ))
        .id();

    commands
        .entity(authority)
        .insert(PortalSplitTraveler::new(transform, peer));

    for body in [authority, peer] {
        commands.entity(body).with_children(|parent| {
            parent.spawn((
                Name::new("Cube Model"),
                UsfPresentationProjectionOf(body),
                PortalSplitVisual,
                Mesh3d(meshes.add(Cuboid::from_length(CUBE_SIZE))),
                MeshMaterial3d(assets.cube_material.clone()),
                Transform::IDENTITY,
            ));
        });
    }
}
