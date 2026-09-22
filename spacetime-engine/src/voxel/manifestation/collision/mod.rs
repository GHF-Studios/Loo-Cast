//! Collision residency and collider construction for one-to-one voxel manifestations.

use avian3d::prelude::{Collider, CollisionMargin};
use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    spatial::{UsfScaleLayer, UsfViewContext, UsfViewRenderAnchor},
};

use super::{VoxelManifestation, VoxelManifestationRegistry};
use super::super::{
    MATERIALIZATION_CHUNK_SIZE, VoxelCollisionDisabled, VoxelMaterializationChunkAddress, VoxelWorld, physics,
};

pub(in crate::voxel) fn sync_manifestation_collision_residency(
    config: Res<EngineConfig>,
    mut commands: Commands,
    view: Single<Ref<UsfViewContext>, With<UsfViewRenderAnchor>>,
    worlds: Query<(&VoxelWorld, &UsfScaleLayer, Option<&VoxelCollisionDisabled>)>,
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
        let Ok((world, layer, collision_disabled)) = worlds.get(key.world) else {
            continue;
        };

        let has_rigid_surface = world
            .materializations()
            .surface(key.address)
            .is_some_and(|cache| {
                cache.revision == expected_revision && cache.surface.has_rigid_triangles()
            });
        // Physics slices coexist. Collision residency is bounded by local
        // proximity and mechanism policy, not one global active-world scale.
        let wants_collider = collision_disabled.is_none()
            && has_rigid_surface
            && manifestation_collider_proximity_squared(
                &view,
                key.address,
                layer.scale(),
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
    // Physics residency controls only the derived collider representation;
    // body identity remains stable across residency transitions.
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

fn manifestation_collider_proximity_squared(
    view: &UsfViewContext,
    address: VoxelMaterializationChunkAddress,
    scale: crate::spatial::SpatialScale,
    interaction_radius_native: f32,
) -> Option<f32> {
    let extent = MATERIALIZATION_CHUNK_SIZE as f32;

    // The semantic observer may retain a much finer leaf (for example S-35)
    // than this concrete voxel realization. Collider residency is a slice-local
    // question, so compare canonically at the realization's Scale Slice rather
    // than requiring identical leaf scales.
    let minimum = address
        .origin()
        .relative_at_scale_bounded(
            view.anchor(),
            scale,
            interaction_radius_native + extent * 2.0,
        )
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
