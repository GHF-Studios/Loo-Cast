use bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use crate::chunk::components::{ChunkActor, ChunkLoader};
use crate::config::statics::CONFIG;
use crate::input::states::InputMode;
use crate::render::{
    components::{MainCamera, RenderProxy, RenderProxyHandle, UiCamera},
    functions::draw_primary_window_ui,
    resources::{GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState, ViewScale, ZoomFactor},
};
use crate::time::resources::VirtualPaused;

pub(super) fn pre_setup_phase_0(mut commands: Commands, mut images: ResMut<Assets<Image>>, windows: Query<&Window>) {
    // Reserve camera entities
    let egui_camera = commands.spawn(()).id();
    let ui_camera = commands.spawn(UiCamera).id();
    let main_camera = commands.spawn(MainCamera).id();
    super::functions::reserve_camera_entities(egui_camera, ui_camera, main_camera);

    // Reserve game view render target handle
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
    super::functions::reserve_game_view_render_target(image_handle, size_uvec2);
}

pub(super) fn pre_setup_phase_1(mut commands: Commands, mut egui_textures: ResMut<bevy_egui::EguiUserTextures>) {
    let (image_handle, size) = super::functions::get_reserved_game_view_render_target();
    let texture_id = egui_textures.add_image(image_handle.clone_weak());

    commands.insert_resource(GameViewRenderTarget {
        handle: image_handle,
        size,
        id: texture_id,
    });
}

pub(super) fn resize_render_texture(
    mut previous_window_size_uvec2: Local<UVec2>,
    mut images: ResMut<Assets<Image>>,
    mut game_view_render_target: ResMut<GameViewRenderTarget>,
    windows: Query<&Window>,
) {
    let window = windows.single().unwrap();
    let size_uvec2 = window.physical_size();

    if size_uvec2 == *previous_window_size_uvec2 {
        return;
    }

    *previous_window_size_uvec2 = size_uvec2;
    game_view_render_target.size = size_uvec2;

    let image = images.get_mut(&game_view_render_target.handle).unwrap();
    image.resize(Extent3d {
        width: size_uvec2.x,
        height: size_uvec2.y,
        depth_or_array_layers: 1,
    });
}

#[tracing::instrument(skip_all)]
pub(super) fn update_render_proxies(
    chunk_loader_query: Query<&ChunkLoader>,
    sources: Query<(&RenderProxyHandle, &ChunkActor), Without<RenderProxy>>,
    mut proxy_transforms: Query<&mut Transform, With<RenderProxy>>,
) {
    let chunk_loader = match chunk_loader_query.single() {
        Ok(loader) => loader,
        Err(_) => return,
    };

    for (handle, actor) in &sources {
        if let Ok(mut proxy_transform) = proxy_transforms.get_mut(handle.proxy_entity) {
            let (pos, scale) = actor.coord.clone().to_native_visual(chunk_loader.coord.clone());
            proxy_transform.translation = pos.extend(proxy_transform.translation.z); // preserve Z
            proxy_transform.scale = Vec3::splat(scale);
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn despawn_orphaned_render_proxies(
    mut removed: RemovedComponents<RenderProxyHandle>,
    proxies: Query<(Entity, &RenderProxy)>,
    mut commands: Commands,
) {
    for removed_source in removed.read() {
        for (proxy_entity, proxy) in &proxies {
            if proxy.source == removed_source {
                commands.entity(proxy_entity).despawn();
            }
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn primary_window_ui_system(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut bevy_egui::EguiContext, With<bevy_egui::PrimaryEguiContext>>()
        .single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    world.resource_scope::<PrimaryWindowUiState, _>(|world, mut state| {
        world.resource_scope::<PrimaryWindowUiDockState, _>(|world, mut dock_state| {
            world.resource_scope::<GameViewRenderTarget, _>(|world, target| {
                draw_primary_window_ui(&mut state, &mut dock_state, &target, world, egui_context.get_mut());
            });
        });
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

#[tracing::instrument(skip_all)]
pub(super) fn update_view_scale_from_zoom(zoom_factor: Res<ZoomFactor>, mut view_scale: ResMut<ViewScale>) {
    let zoom = zoom_factor.0;
    let scale = -zoom.log10(); // Since zooming in decreases ortho scale
    view_scale.discrete = scale.floor() as i32;
    view_scale.offset = scale.fract();
}
