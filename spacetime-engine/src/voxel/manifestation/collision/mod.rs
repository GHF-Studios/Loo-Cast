//! Collision residency and collider construction for one-to-one voxel manifestations.

use avian3d::prelude::{Collider, CollisionMargin};
use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{UsfScaleLayer, UsfViewFrame},
};

use super::{VoxelManifestation, VoxelManifestationRegistry};
use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelMaterializationChunkAddress, VoxelWorld, physics,
};

pub(crate) fn sync_manifestation_collision_residency(
    config: Res<EngineConfig>,
    mut commands: Commands,
    view: Res<UsfViewFrame>,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer)>,
    manifestation_roots: Query<Option<&Collider>, With<VoxelManifestation>>,
    registry: Res<VoxelManifestationRegistry>,
) {
    if !config.is_changed() && !view.is_changed() && !registry.is_changed() {
        return;
    }

    for (&key, &entity) in &registry.entities {
        let Some(&expected_revision) = registry.revisions.get(&key) else {
            continue;
        };
        let Ok((world, layer)) = worlds.get(key.world) else {
            continue;
        };

        let wants_collider = manifestation_collider_proximity_squared(
            &view,
            key.address,
            layer,
            config.voxel.manifestation.physics_interaction_radius_native,
        )
        .is_some();
        let has_collider = manifestation_roots
            .get(entity)
            .ok()
            .flatten()
            .is_some();

        match (wants_collider, has_collider) {
            (true, false) => {
                let collider =
                    build_manifestation_collider(key.address, expected_revision, world);
                publish_collider_manifestation(&mut commands, entity, true, collider);
            }
            (false, true) => {
                publish_collider_manifestation(&mut commands, entity, false, None);
            }
            _ => {}
        }
    }
}

pub(super) fn publish_collider_manifestation(
    commands: &mut Commands,
    entity: Entity,
    requested: bool,
    collider: Option<Collider>,
) {
    let mut entity_commands = commands.entity(entity);

    // A voxel manifestation is static world geometry for its entire lifetime.
    // Physics residency controls only the derived collider representation.
    // Toggling RigidBody together with Collider can transiently detach ColliderOf,
    // move the collider through Avian's standalone tree, and leave invalid
    // contact pairs behind across body-classification changes.
    if requested {
        if let Some(collider) = collider {
            entity_commands.insert((
                collider,
                CollisionMargin(physics::VOXEL_COLLISION_MARGIN),
            ));
        } else {
            entity_commands.remove::<Collider>();
            entity_commands.remove::<CollisionMargin>();
        }
    } else {
        entity_commands.remove::<Collider>();
        entity_commands.remove::<CollisionMargin>();
    }
}

pub(crate) fn manifestation_collider_proximity_squared(
    view: &UsfViewFrame,
    address: VoxelMaterializationChunkAddress,
    layer: &UsfScaleLayer,
    interaction_radius_native: f32,
) -> Option<f32> {
    if layer.scale() != view.dominant_scale() {
        return None;
    }

    let extent = MATERIALIZATION_CHUNK_SIZE as f32;
    let minimum = address
        .origin()
        .relative_native_bounded(view.anchor(), interaction_radius_native + extent * 2.0)
        .ok()?;
    let maximum = minimum + Vec3::splat(extent);
    let nearest = Vec3::new(
        0.0_f32.clamp(minimum.x, maximum.x),
        0.0_f32.clamp(minimum.y, maximum.y),
        0.0_f32.clamp(minimum.z, maximum.z),
    );
    let distance_squared = nearest.length_squared();
    (distance_squared <= interaction_radius_native * interaction_radius_native)
        .then_some(distance_squared)
}

fn build_manifestation_collider(
    address: VoxelMaterializationChunkAddress,
    expected_revision: u64,
    world: &VoxelWorld,
) -> Option<Collider> {
    let cache = world.materializations().surface(address)?;
    if cache.revision != expected_revision {
        return None;
    }

    let vertices = cache
        .surface
        .positions
        .iter()
        .copied()
        .map(Vec3::from_array)
        .collect();
    let triangles = physics::owned_triangles(&cache.surface);

    physics::build_trimesh_collider(vertices, triangles, "voxel manifestation")
}
