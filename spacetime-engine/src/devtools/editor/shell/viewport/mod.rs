//! Embedded game-camera viewport projection between egui and Bevy camera space.

use super::*;

pub(super) fn set_primary_game_viewport(world: &mut World, viewport: Option<Viewport>, active: bool) {
    let mut cameras = world.query_filtered::<&mut Camera, With<PrimaryGameView>>();
    let Ok(mut camera) = cameras.single_mut(world) else {
        return;
    };

    camera.viewport = viewport;
    camera.is_active = active;
}

pub(super) fn egui_rect_to_viewport(
    rect: egui::Rect,
    scale_factor: f32,
    target_size: UVec2,
) -> Option<Viewport> {
    let logical_min = Vec2::new(rect.min.x, rect.min.y).max(Vec2::ZERO);
    let logical_max = Vec2::new(rect.max.x, rect.max.y).max(logical_min);

    let mut min = (logical_min * scale_factor).floor().as_uvec2();
    let mut max = (logical_max * scale_factor).ceil().as_uvec2();

    min.x = min.x.min(target_size.x);
    min.y = min.y.min(target_size.y);
    max.x = max.x.min(target_size.x);
    max.y = max.y.min(target_size.y);

    let size = UVec2::new(max.x.saturating_sub(min.x), max.y.saturating_sub(min.y));
    if size.x < 2 || size.y < 2 {
        return None;
    }

    Some(Viewport {
        physical_position: min,
        physical_size: size,
        ..default()
    })
}
