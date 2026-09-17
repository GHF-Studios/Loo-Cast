//! Built-in playground item plugins.
//!
//! Each item is an ordinary Bevy plugin. This is intentionally the same shape
//! expected from statically composed Vapor/mod content: register metadata, then
//! consume semantic playground actions and/or emit domain messages.

mod chunkloading_cube;
mod damageable_cube;
mod heat_ray;
mod portal_gun;
mod projectile_gun;
mod voxel_hand;

use bevy::prelude::*;

pub struct PlaygroundItemsPlugin;

impl Plugin for PlaygroundItemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            portal_gun::PortalGunItemPlugin,
            projectile_gun::ProjectileGunItemPlugin,
            damageable_cube::DamageableCubeItemPlugin,
            chunkloading_cube::ChunkloadingCubeItemPlugin,
            heat_ray::HeatRayItemPlugin,
            voxel_hand::VoxelHandItemPlugin,
        ));
    }
}
