use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy::input::mouse::MouseScrollUnit;

use crate::config::statics::CONFIG;

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
    let min_zoom = CONFIG.get::<f32>("camera/min_zoom");
    let max_zoom = CONFIG.get::<f32>("camera/max_zoom");
    let base_zoom_speed = CONFIG.get::<f32>("camera/base_zoom_speed");

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