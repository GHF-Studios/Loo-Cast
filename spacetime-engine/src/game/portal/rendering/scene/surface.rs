//! Explicit front/back aperture surfaces.

use std::f32::consts::PI;

use bevy::{
    camera::visibility::RenderLayers,
    prelude::*,
};

use crate::game::portal::domain::{
    PortalSide,
    PortalSidedness,
};

fn local_surface_transform(
    side: PortalSide,
) -> Transform {
    match side {
        PortalSide::Front => Transform::IDENTITY,

        // A physically separate, oppositely oriented face. We do not encode
        // portal topology by disabling GPU culling.
        PortalSide::Back => {
            Transform::from_rotation(
                Quat::from_rotation_y(PI),
            )
        }
    }
}

fn sides(
    sidedness: PortalSidedness,
) -> &'static [PortalSide] {
    if sidedness.is_two_sided() {
        &[
            PortalSide::Front,
            PortalSide::Back,
        ]
    } else {
        &[PortalSide::Front]
    }
}

pub fn spawn_portal_surfaces(
    commands: &mut Commands,
    portal: Entity,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<super::super::material::PortalMaterial>,
    sidedness: PortalSidedness,
) {
    commands
        .entity(portal)
        .with_children(|parent| {
            for side in sides(sidedness) {
                parent.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(
                        material.clone(),
                    ),
                    local_surface_transform(
                        *side,
                    ),
                    RenderLayers::layer(layer),
                ));
            }
        });
}

pub fn spawn_terminal_surfaces(
    commands: &mut Commands,
    portal: Entity,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    sidedness: PortalSidedness,
) {
    commands
        .entity(portal)
        .with_children(|parent| {
            for side in sides(sidedness) {
                parent.spawn((
                    Mesh3d(mesh.clone()),
                    MeshMaterial3d(
                        material.clone(),
                    ),
                    local_surface_transform(
                        *side,
                    ),
                    RenderLayers::layer(layer),
                ));
            }
        });
}
