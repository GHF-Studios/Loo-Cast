//! Visible frame geometry around the aperture.

use bevy::prelude::*;

use crate::game::portal::rendering::layout::{FRAME_BORDER, FRAME_DEPTH};

pub struct FrameMeshes {
    pub vertical: Handle<Mesh>,
    pub horizontal: Handle<Mesh>,
}

pub fn create_frame_meshes(meshes: &mut Assets<Mesh>, aperture_size: Vec2) -> FrameMeshes {
    FrameMeshes {
        // Vertical bars also cover the corners.
        vertical: meshes.add(Cuboid::new(
            FRAME_BORDER,
            aperture_size.y + 2.0 * FRAME_BORDER,
            FRAME_DEPTH,
        )),

        horizontal: meshes.add(Cuboid::new(aperture_size.x, FRAME_BORDER, FRAME_DEPTH)),
    }
}

/// Spawns the frame entirely *outside* the configured aperture.
///
/// The old implementation centered bars directly on the aperture boundary,
/// which caused half of each bar to overlap the portal plane.
pub fn spawn_frame(
    commands: &mut Commands,
    portal: Entity,
    meshes: &FrameMeshes,
    material: Handle<StandardMaterial>,
    aperture_size: Vec2,
) {
    let x = aperture_size.x / 2.0 + FRAME_BORDER / 2.0;

    let y = aperture_size.y / 2.0 + FRAME_BORDER / 2.0;

    commands.entity(portal).with_children(|parent| {
        for x in [-x, x] {
            parent.spawn((
                Mesh3d(meshes.vertical.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x, 0.0, 0.0),
            ));
        }

        for y in [-y, y] {
            parent.spawn((
                Mesh3d(meshes.horizontal.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(0.0, y, 0.0),
            ));
        }
    });
}
