use bevy::prelude::*;

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
