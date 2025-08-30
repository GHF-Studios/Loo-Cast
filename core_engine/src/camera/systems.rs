use bevy::input::mouse::MouseScrollUnit;
use bevy::{input::mouse::MouseWheel, prelude::*};
use bevy::render::render_resource::{
    TextureDescriptor, Extent3d,
    TextureDimension, TextureFormat, TextureUsages,
};

use crate::config::statics::CONFIG;

use super::resources::GameViewRenderTarget;
use super::types::ZoomFactor;

pub(crate) fn setup_main_render_target(mut commands: Commands, mut images: ResMut<Assets<Image>>, windows: Query<&Window>) {
    let window = windows.single().unwrap();
    let size = window.physical_size();

    let image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("Game View Render Target"),
            size: Extent3d {
                width: size.x,
                height: size.y,
                depth_or_array_layers: 1,
            },
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        },
        ..default()
    };

    let image_handle = images.add(image);
    commands.insert_resource(GameViewRenderTarget { image_handle });
}

#[tracing::instrument(skip_all)]
pub(crate) fn main_camera_zoom_system(
    mut projection_query: Query<&mut Projection, With<Camera>>,
    mut scroll_event_reader: EventReader<MouseWheel>,
    time: Res<Time<Virtual>>,
    mut zoom_factor: Local<ZoomFactor>,
) {
    let min_zoom = CONFIG.get::<f32>("camera/min_zoom");
    let max_zoom = CONFIG.get::<f32>("camera/max_zoom");
    let base_zoom_speed = CONFIG.get::<f32>("camera/base_zoom_speed");

    for event in scroll_event_reader.read() {
        let scroll_delta = match event.unit {
            MouseScrollUnit::Line => -event.y,
            MouseScrollUnit::Pixel => event.y * -0.01,
        };

        let zoom_speed = base_zoom_speed * zoom_factor.0;

        zoom_factor.0 = (zoom_factor.0 + scroll_delta * zoom_speed * time.delta_secs()).clamp(min_zoom, max_zoom);
    }

    for mut projection in projection_query.iter_mut() {
        match projection.as_mut() {
            Projection::Orthographic(ortho) => {
                ortho.scale = zoom_factor.0;
            }
            _ => panic!("Main camera is not orthographic/2d!"),
        }
    }
}
