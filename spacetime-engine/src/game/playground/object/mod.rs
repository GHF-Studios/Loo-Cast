use bevy::prelude::*;

use crate::game::item::AimRay;

#[derive(Component, Debug)]
pub struct PlaygroundRoot;

#[derive(Component, Debug, Clone, Copy)]
pub struct PlaygroundPickable {
    pub root: Entity,
    pub half_extents: Vec3,
}

impl PlaygroundPickable {
    pub fn cuboid(root: Entity, half_extents: Vec3) -> Self {
        Self { root, half_extents }
    }

    pub fn cube(root: Entity, size: f32) -> Self {
        Self::cuboid(root, Vec3::splat(size / 2.0))
    }
}

/// Global sandbox deletion request. This intentionally remains separate from
/// item actions; the built-in mouse adapter maps it to middle-click.
#[derive(Message, Debug, Clone, Copy)]
pub struct ErasePlaygroundObject {
    pub aim: AimRay,
}
