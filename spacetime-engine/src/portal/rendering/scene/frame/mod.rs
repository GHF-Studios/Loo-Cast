//! Visible frame geometry around the aperture.

use bevy::prelude::*;

use crate::portal::rendering::layout::{FRAME_BORDER, FRAME_DEPTH};

pub struct FrameMeshes {
    pub vertical: Handle<Mesh>,
    pub horizontal: Handle<Mesh>,
}

pub fn create_frame_meshes(meshes: &mut Assets<Mesh>, aperture_size: Vec2) -> FrameMeshes {
    FrameMeshes {
        // Side bars start at the aperture's lower edge and extend through the
        // top border. They never protrude below a floor-aligned portal.
        vertical: meshes.add(Cuboid::new(
            FRAME_BORDER,
            aperture_size.y + FRAME_BORDER,
            FRAME_DEPTH,
        )),

        horizontal: meshes.add(Cuboid::new(aperture_size.x, FRAME_BORDER, FRAME_DEPTH)),
    }
}

/// Spawns the temporary three-sided frame entirely outside the aperture.
///
/// There is deliberately no lower horizontal bar. For a wall portal whose
/// aperture touches the floor, the two side bars end exactly at floor level.
pub fn spawn_frame(
    commands: &mut Commands,
    portal: Entity,
    meshes: &FrameMeshes,
    material: Handle<StandardMaterial>,
    aperture_size: Vec2,
) {
    let x = aperture_size.x / 2.0 + FRAME_BORDER / 2.0;
    let side_y = FRAME_BORDER / 2.0;
    let top_y = aperture_size.y / 2.0 + FRAME_BORDER / 2.0;

    commands.entity(portal).with_children(|parent| {
        for x in [-x, x] {
            parent.spawn((
                Mesh3d(meshes.vertical.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x, side_y, 0.0),
            ));
        }

        parent.spawn((
            Mesh3d(meshes.horizontal.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, top_y, 0.0),
        ));
    });
}
