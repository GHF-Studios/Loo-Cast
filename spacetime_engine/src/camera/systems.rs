use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy::input::mouse::MouseScrollUnit;

use crate::player::components::PlayerComponent;

use super::components::MainCameraFollow;

pub(in crate) struct ZoomFactor(pub f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(1.0)
    }
}

/// Updates the camera's FOV based on mouse wheel scroll.
pub(in crate) fn main_camera_zoom_system(
    mut projection_query: Query<&mut OrthographicProjection, With<Camera>>,
    mut scroll_event_reader: EventReader<MouseWheel>,
    time: Res<Time>,
    mut zoom_factor: Local<ZoomFactor>,
) {
    let min_zoom = 0.03125;
    let max_zoom = 32.0;
    let base_zoom_speed = 1.0;

    for event in scroll_event_reader.read() {
        let scroll_delta = match event.unit {
            MouseScrollUnit::Line => event.y * -1.0, // Adjust sensitivity for lines
            MouseScrollUnit::Pixel => event.y * -0.01, // Adjust sensitivity for pixels
        };

        // Proportional zoom speed based on current zoom factor
        let zoom_speed = base_zoom_speed * zoom_factor.0;

        // Calculate the new zoom factor
        zoom_factor.0 = (zoom_factor.0 + scroll_delta * zoom_speed * time.delta_seconds())
            .clamp(min_zoom, max_zoom);
    }

    // Adjust the camera's FOV based on the zoom factor
    for mut projection in projection_query.iter_mut() {
        projection.scale = zoom_factor.0;
    }
}

pub(in crate) fn main_camera_follow_system(
    mut camera_query: Query<(&mut Transform, &mut MainCameraFollow), Without<PlayerComponent>>,
    target_query: Query<(Entity, &Transform), (With<PlayerComponent>, Without<MainCameraFollow>)>,
    time: Res<Time>,
) {
    let (mut camera_transform, mut camera_follow) = match camera_query.get_single_mut() {
        Ok(value) => value,
        Err(_) => {
            return;
        }
    };

    let follow_target = match camera_follow.target {
        Some(target) => target,
        None => {
            match target_query.get_single() {
                Ok((target, _)) => {
                    camera_follow.target = Some(target);
                    target
                },
                Err(_) => {
                    return;
                }
            }
        }
    };

    if let Ok((_, target_transform)) = target_query.get(follow_target) {
        // Interpolate the camera's position towards the target
        let target_position = target_transform.translation;
        let current_position = camera_transform.translation;

        // Linear interpolation towards the target position
        let new_position = current_position.lerp(target_position, camera_follow.speed * time.delta_seconds());

        // Update the camera's position
        camera_transform.translation = new_position;
    }
}