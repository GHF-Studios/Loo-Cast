//! Directed aperture surfaces.
//!
//! Each [`PortalSide`] gets an ordinary one-sided mesh surface. Front and back
//! may therefore use completely independent portal render targets.

use std::f32::consts::PI;

use bevy::{camera::visibility::RenderLayers, prelude::*};

use crate::game::portal::domain::PortalSide;

const SURFACE_OFFSET: f32 = 0.002;

/// Local presentation transform of a directed portal face.
fn surface_transform(side: PortalSide) -> Transform {
    match side {
        PortalSide::Front => Transform::from_xyz(0.0, 0.0, SURFACE_OFFSET),

        PortalSide::Back => {
            Transform::from_xyz(0.0, 0.0, -SURFACE_OFFSET).with_rotation(Quat::from_rotation_y(PI))
        }
    }
}

/// Spawns one independently rendered portal face.
pub fn spawn_portal_surface(
    commands: &mut Commands,
    portal: Entity,
    side: PortalSide,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<super::super::material::PortalMaterial>,
) {
    commands.entity(portal).with_children(|parent| {
        parent.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            surface_transform(side),
            RenderLayers::layer(layer),
        ));
    });
}

/// Spawns one recursion-terminal face.
pub fn spawn_terminal_surface(
    commands: &mut Commands,
    portal: Entity,
    side: PortalSide,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    commands.entity(portal).with_children(|parent| {
        parent.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            surface_transform(side),
            RenderLayers::layer(layer),
        ));
    });
}
