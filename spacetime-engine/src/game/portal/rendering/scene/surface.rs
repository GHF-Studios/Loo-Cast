//! Portal aperture surfaces.
//!
//! This file currently preserves the original known-working one-sided renderer.
//! Two-sided rendering will be reintroduced only after this baseline is
//! visibly correct again.

use bevy::{
    camera::visibility::RenderLayers,
    prelude::*,
};

use crate::game::portal::domain::PortalSidedness;

/// Spawns the ordinary render-texture aperture.
///
/// The surface sits a tiny distance in front of the mathematical portal plane,
/// matching the original working implementation.
pub fn spawn_portal_surfaces(
    commands: &mut Commands,
    portal: Entity,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<
        super::super::material::PortalMaterial,
    >,
    _sidedness: PortalSidedness,
) {
    commands
        .entity(portal)
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(
                    material.clone(),
                ),
                Transform::from_xyz(
                    0.0,
                    0.0,
                    0.002,
                ),
                RenderLayers::layer(layer),
            ));
        });
}

/// Spawns the recursion-terminal aperture.
///
/// Like the live surface, this intentionally reproduces the original
/// one-sided renderer exactly.
pub fn spawn_terminal_surfaces(
    commands: &mut Commands,
    portal: Entity,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    _sidedness: PortalSidedness,
) {
    commands
        .entity(portal)
        .with_children(|parent| {
            parent.spawn((
                Mesh3d(mesh.clone()),
                MeshMaterial3d(
                    material.clone(),
                ),
                Transform::from_xyz(
                    0.0,
                    0.0,
                    0.002,
                ),
                RenderLayers::layer(layer),
            ));
        });
}