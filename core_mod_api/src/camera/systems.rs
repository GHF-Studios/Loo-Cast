use bevy::input::mouse::MouseScrollUnit;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::config::statics::CONFIG;
use crate::input::states::InputMode;
use crate::time::resources::VirtualPaused;

use super::resources::{GameViewRenderTarget, ZoomFactor, ViewScale};

pub(super) fn setup_main_render_target(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut egui_textures: ResMut<bevy_egui::EguiUserTextures>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let size_uvec2 = window.physical_size();
    let size_extent3d = Extent3d {
        width: size_uvec2.x,
        height: size_uvec2.y,
        depth_or_array_layers: 1,
    };

    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: Some("Game View Render Target"),
            size: size_extent3d,
            dimension: TextureDimension::D2,
            format: TextureFormat::Bgra8UnormSrgb,
            usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
            mip_level_count: 1,
            sample_count: 1,
            view_formats: &[],
        },
        ..default()
    };
    image.resize(size_extent3d);

    let image_handle = images.add(image);
    let texture_id = egui_textures.add_image(image_handle.clone());

    commands.insert_resource(GameViewRenderTarget {
        handle: image_handle,
        size: size_uvec2,
        id: texture_id,
    });
}

#[tracing::instrument(skip_all)]
pub(super) fn main_camera_zoom_system(
    mut projection_query: Query<&mut Projection, With<Camera>>,
    mut scroll_event_reader: EventReader<MouseWheel>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Real>>,
    virtual_paused: Res<VirtualPaused>,
    mut zoom_factor: ResMut<ZoomFactor>,
) {
    let min_zoom = CONFIG().get::<f32>("camera/min_zoom");
    let max_zoom = CONFIG().get::<f32>("camera/max_zoom");
    let base_zoom_speed = CONFIG().get::<f32>("camera/base_zoom_speed");

    if !input_mode.is_game() || virtual_paused.0 {
        scroll_event_reader.clear();
        return;
    }

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

pub(super) fn update_view_scale_from_zoom(
    zoom_factor: Res<ZoomFactor>,
    mut view_scale: ResMut<ViewScale>,
) {
    let zoom = zoom_factor.0;
    let scale = -zoom.log10(); // Since zooming in decreases ortho scale
    view_scale.discrete = scale.floor() as i32;
    view_scale.offset = scale.fract();
}

