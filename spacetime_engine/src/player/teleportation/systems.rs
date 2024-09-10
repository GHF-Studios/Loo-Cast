use bevy::{prelude::*, window::PrimaryWindow};
use crate::player::{components::Player, constants::PLAYER_Z_INDEX};

// teleports the player tranmsform exactly to where the mouse points (when pressing down middle mouse button/scroll wheel)
pub(in crate) fn update(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
    mut query: Query<&mut Transform, With<Player>>,
    mouse_button_input: Res<ButtonInput<MouseButton>>,
) {
    if let Ok(window) = window_query.get_single() {
        if let Some(cursor_position) = window.cursor_position() {
            let window_size = Vec2::new(window.width(), window.height());
            let cursor_position_ndc = Vec2::new(
                (cursor_position.x / window_size.x) * 2.0 - 1.0, 
                1.0 - (cursor_position.y / window_size.y) * 2.0
            );

            if let Ok((camera, camera_transform)) = camera_query.get_single() {
                let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
                let world_position = ndc_to_world.project_point3(cursor_position_ndc.extend(-1.0)).truncate();

                if mouse_button_input.just_pressed(MouseButton::Middle) {
                    for mut transform in query.iter_mut() {
                        transform.translation = world_position.extend(PLAYER_Z_INDEX);
                    }
                }
            }
        }
    }
}