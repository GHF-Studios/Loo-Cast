use bevy::input::mouse::MouseScrollUnit;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use bevy::{input::mouse::MouseWheel, prelude::*};

use crate::chunk_loader::enums::ZoomState;
use crate::config::statics::CONFIG;
use crate::input::states::InputMode;
use crate::time::resources::VirtualPaused;
use crate::chunk_loader::components::ChunkLoader;
use crate::usf::scale::DynScale;

use super::resources::{GameViewRenderTarget, ZoomFactor, ViewScale};

pub(crate) fn setup_main_render_target(
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
pub(crate) fn main_camera_zoom_system(
    mut projection_query: Query<&mut Projection, With<Camera>>,
    mut scroll_event_reader: EventReader<MouseWheel>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Real>>,
    virtual_paused: Res<VirtualPaused>,
    mut zoom_factor: ResMut<ZoomFactor>,
    mut chunk_loader: Single<'_, &mut ChunkLoader>,
) {
    let min_zoom = CONFIG().get::<f32>("camera/min_zoom"); // e.g. 0.1
    let max_zoom = CONFIG().get::<f32>("camera/max_zoom"); // e.g. 10.0
    let base_zoom_speed = CONFIG().get::<f32>("camera/base_zoom_speed");
    let base_zoom = CONFIG().get::<f32>("camera/base_zoom");

    const MIN_SCALE_EXP: i8 = -35;
    const MAX_SCALE_EXP: i8 = 35;

    if !input_mode.is_game() || virtual_paused.0 {
        scroll_event_reader.clear();
        return;
    }

    // Aggregate scroll input
    let mut total_scroll_delta = 0.0;
    for event in scroll_event_reader.read() {
        let scroll_delta = match event.unit {
            MouseScrollUnit::Line => -event.y,
            MouseScrollUnit::Pixel => event.y * -0.01,
        };
        total_scroll_delta += scroll_delta;
    }

    let scale_exp = *chunk_loader.id().scale() as i8;
    let scale_exp = match chunk_loader.zoom_state {
        ZoomState::None => scale_exp,
        ZoomState::ZoomIn => scale_exp - 1,
        ZoomState::ZoomOut => scale_exp + 1,
    };
    let scale_factor = chunk_loader.id().scale().scale_factor() as f32;

    if total_scroll_delta != 0.0 {
        let zoom_speed = base_zoom_speed * zoom_factor.0;
        let zoom_delta = total_scroll_delta * zoom_speed * time.delta_secs();
        
        println!(
            "zoom_state: {:?}, scale_exp: {}, zoom_factor: {:.3}",
            chunk_loader.zoom_state,
            *chunk_loader.id().scale() as i8,
            zoom_factor.0,
        );

        let new_zoom = zoom_factor.0 + zoom_delta;

        // Only trigger transition *before* clamping — the zoom tries to escape
        if chunk_loader.zoom_state == ZoomState::None {
            if new_zoom >= max_zoom && scale_exp < MAX_SCALE_EXP {
                chunk_loader.suggest_zoom_out();
                println!("→ zoom_state is now: {:?}", chunk_loader.zoom_state);
                zoom_factor.0 = max_zoom; // freeze at edge
                return;
            } else if new_zoom <= min_zoom && scale_exp > MIN_SCALE_EXP {
                chunk_loader.suggest_zoom_in();
                println!("→ zoom_state is now: {:?}", chunk_loader.zoom_state);
                zoom_factor.0 = min_zoom;
                return;
            }
        }

        // Otherwise, just clamp normally
        zoom_factor.0 = new_zoom.clamp(min_zoom, max_zoom);

        println!(
            "[Zoom Debug] scale_exp: {}, zoom_factor: {:.3}, zoom_state: {:?}",
            scale_exp,
            zoom_factor.0,
            chunk_loader.zoom_state,
        );
    }

    // Apply zoom to camera
    for mut projection in projection_query.iter_mut() {
        match projection.as_mut() {
            Projection::Orthographic(ortho) => {
                ortho.scale = zoom_factor.0 * base_zoom;
            }
            _ => panic!("Main camera is not orthographic/2d!"),
        }
    }
}

pub fn update_view_scale_from_zoom(
    chunk_loader: Single<&ChunkLoader>,
    zoom_factor: Res<ZoomFactor>,
    mut view_scale: ResMut<ViewScale>,
) {
    let scale_exp = *chunk_loader.id().scale() as i8;
    let offset = zoom_factor.0.log10();

    view_scale.discrete = scale_exp as i32;
    view_scale.offset = offset;

    // Optional log
    // println!("View scale updated → level {}, offset {:.2}", scale_exp, offset);
}
