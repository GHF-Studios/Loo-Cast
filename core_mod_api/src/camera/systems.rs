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
    const ZOOM_IN_THRESHOLD: f32 = 0.095;
    const ZOOM_OUT_THRESHOLD: f32 = 10.5;

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

    // Get global zoom as continuous value
    let mut global_zoom = zoom_factor.0 * scale_factor;

    if total_scroll_delta != 0.0 {
        // Apply zoom delta in global space
        let zoom_speed = base_zoom_speed * global_zoom;
        let zoom_delta = total_scroll_delta * zoom_speed * time.delta_secs();
        global_zoom = (global_zoom + zoom_delta)
            .clamp(min_zoom * 10f32.powi(MIN_SCALE_EXP as i32), max_zoom * 10f32.powi(MAX_SCALE_EXP as i32));

        // Decompose new global zoom
        let raw_scale_exp = global_zoom.log10().floor() as i8;
        let clamped_exp = raw_scale_exp.clamp(MIN_SCALE_EXP, MAX_SCALE_EXP);
        let new_scale_factor = 10f32.powi(clamped_exp as i32);
        let new_zoom_factor = (global_zoom / new_scale_factor).clamp(min_zoom, max_zoom);

        // Trigger scale change if needed (but don't apply scale yet)
        if clamped_exp < scale_exp && zoom_factor.0 < ZOOM_IN_THRESHOLD && chunk_loader.zoom_state == ZoomState::None {
            chunk_loader.suggest_zoom_in(); 
            println!("Zooming in: scale_exp {} → {}", scale_exp, clamped_exp);
        } else if clamped_exp > scale_exp && zoom_factor.0 > ZOOM_OUT_THRESHOLD && chunk_loader.zoom_state == ZoomState::None {
            chunk_loader.suggest_zoom_out(); 
            println!("Zooming out: scale_exp {} → {}", scale_exp, clamped_exp);
        }

        // Only update zoom_factor if no scale transition is currently pending
        if chunk_loader.zoom_state == ZoomState::None {
            zoom_factor.0 = new_zoom_factor;
        }

        println!(
            "global_zoom: {:.6}, zoom_factor: {:.6}, raw_exp: {}, clamped_exp: {}, scale_exp: {}",
            global_zoom, zoom_factor.0, raw_scale_exp, clamped_exp, scale_exp
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
    let scale_factor = 10f32.powi(scale_exp as i32);
    let global_zoom = zoom_factor.0 * scale_factor;
    let offset = (global_zoom / scale_factor).log10();

    view_scale.discrete = scale_exp as i32;
    view_scale.offset = offset;

    // Optional log
    // println!("View scale updated → level {}, offset {:.2}", scale_exp, offset);
}
