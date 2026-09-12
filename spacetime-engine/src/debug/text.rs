use bevy::prelude::*;

use super::DebugOverlayGizmos;

/// Marks the camera whose orientation defines billboard-style debug text.
///
/// A game chooses the camera; the debug substrate does not assume a player,
/// editor, spectator, or portal implementation.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct DebugCamera;

pub fn billboard_text(
    gizmos: &mut Gizmos<DebugOverlayGizmos>,
    camera: &Transform,
    position: Vec3,
    text: &str,
    font_size: f32,
    anchor: Vec2,
    color: impl Into<Color>,
) {
    // Bevy cameras look down local -Z while 3D stroke text faces local +Z, so
    // reusing the camera rotation points the text front back toward the camera.
    gizmos.text(
        Isometry3d::new(position, camera.rotation),
        text,
        font_size,
        anchor,
        color,
    );
}
