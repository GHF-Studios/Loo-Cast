//! Built-in playground item plugins.
//!
//! Each item is an ordinary Bevy plugin. This is intentionally the same shape
//! expected from statically composed Vapor/mod content: register metadata, then
//! consume semantic playground actions and/or emit domain messages.

mod assets;
mod chunkloading_cube;
mod damageable_cube;
mod heat_ray;
mod portal_gun;
mod projectile_gun;
mod voxel_hand;

use bevy::prelude::*;

use crate::game::inventory::Hotbar;

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
        ))
        .add_systems(PreStartup, initialize_hotbar)
        .add_systems(Startup, assets::setup_item_assets);
    }
}

fn initialize_hotbar(mut hotbar: ResMut<Hotbar>) {
    hotbar.slots = [None; crate::game::inventory::HOTBAR_SIZE];
    hotbar.selected = 0;
    hotbar.slots[0] = Some(portal_gun::PORTAL_GUN);
    hotbar.slots[1] = Some(projectile_gun::PROJECTILE_GUN);
    hotbar.slots[2] = Some(voxel_hand::VOXEL_HAND);
}
