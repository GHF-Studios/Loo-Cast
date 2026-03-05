use crate::bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};
use crate::chunk::resources::ChunkLoadGate;
use crate::config::statics::CONFIG;
use crate::input::states::InputMode;
use crate::player::components::Player;
use crate::usf::scale::Scale;
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
    let texture_id = egui_textures.add_image(bevy_egui::EguiTextureHandle::Weak(image_handle.id()));

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
    mut params: ParamSet<(
        Single<(&ChunkLoader, &Transform), With<Player>>,
        Query<(&RenderProxyHandle, &ChunkActor), Without<RenderProxy>>,
        Query<(&RenderProxyHandle, &Chunk), Without<RenderProxy>>,
        Query<&mut Transform, With<RenderProxy>>,
    )>,
) {
    let (chunk_loader, chunk_loader_transform) = *params.p0();
    let world_rotation = chunk_loader.world_rotation_quat();
    let world_rotation_origin = chunk_loader_transform.translation;
    let origin_offset = chunk_loader.origin_offset.clone();
    let max_scale_diff = Scale::MAX_DIFF_SCALE_EXP;

    let actor_updates = {
        let chunk_actor_query = params.p1();
        chunk_actor_query
            .iter()
            .filter_map(|(handle, chunk_actor)| {
                let scale_diff = chunk_actor.coord.scale as i8 - origin_offset.scale as i8;
                if scale_diff < 0 || scale_diff > max_scale_diff {
                    return None;
                }
                Some((handle.proxy_entity, chunk_actor.coord.clone()))
            })
            .collect::<Vec<_>>()
    };

    {
        let mut proxy_transforms = params.p3();
        for (proxy_entity, coord) in actor_updates {
            if let Ok(mut proxy_transform) = proxy_transforms.get_mut(proxy_entity) {
                let (pos, scale) = coord.to_native_visual(origin_offset.clone());
                let world_pos = pos.extend(proxy_transform.translation.z);
                proxy_transform.translation = world_rotation_origin + world_rotation * (world_pos - world_rotation_origin);
                proxy_transform.scale = Vec3::splat(scale);
                proxy_transform.rotation = world_rotation;
            }
        }
    }

    let chunk_updates = {
        let chunk_query = params.p2();
        chunk_query
            .iter()
            .filter_map(|(handle, chunk)| {
                let scale_diff = chunk.coord.scale as i8 - origin_offset.scale as i8;
                if scale_diff < 0 || scale_diff > max_scale_diff {
                    return None;
                }
                Some((handle.proxy_entity, chunk.coord.clone()))
            })
            .collect::<Vec<_>>()
    };

    {
        let mut proxy_transforms = params.p3();
        for (proxy_entity, coord) in chunk_updates {
            if let Ok(mut proxy_transform) = proxy_transforms.get_mut(proxy_entity) {
                let (pos, scale) = coord.to_native_visual(origin_offset.clone());
                let world_pos = pos.extend(proxy_transform.translation.z);
                proxy_transform.translation = world_rotation_origin + world_rotation * (world_pos - world_rotation_origin);
                proxy_transform.scale = Vec3::splat(scale);
                proxy_transform.rotation = world_rotation;
            }
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
    mut scroll_message_reader: MessageReader<MouseWheel>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Real>>,
    virtual_paused: Res<VirtualPaused>,
    chunk_load_gate: Option<Res<ChunkLoadGate>>,
    mut zoom_factor: ResMut<ZoomFactor>,
) {
    let min_zoom = CONFIG().get::<f32>("camera/min_zoom");
    let max_zoom = CONFIG().get::<f32>("camera/max_zoom");
    let base_zoom_speed = CONFIG().get::<f32>("camera/base_zoom_speed");

    if !input_mode.is_game() || virtual_paused.0 || chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked()) {
        scroll_message_reader.clear();
        return;
    }

    for message in scroll_message_reader.read() {
        let scroll_delta = match message.unit {
            MouseScrollUnit::Line => -message.y,
            MouseScrollUnit::Pixel => message.y * -0.01,
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
pub(super) fn apply_usf_player_pivots_system(
    mut zoom_factor: ResMut<ZoomFactor>,
    mut projection_query: Query<&mut Projection, With<Camera>>,
    mut player_loader_query: Query<(&mut ChunkLoader, &mut Transform), With<Player>>,
) {
    let Ok((mut chunk_loader, mut player_transform)) = player_loader_query.single_mut() else {
        return;
    };

    let (scale_pivot, translation_grid_delta) = chunk_loader.apply_player_anchor_pivots(&mut zoom_factor.0, &mut player_transform.translation);
    if chunk_loader.scale == Scale::MAX {
        let top_level_zoom_cap = chunk_loader.usf_transform.scale.policy.local_max as f32;
        if zoom_factor.0 > top_level_zoom_cap {
            zoom_factor.0 = top_level_zoom_cap;
        }
    }

    if scale_pivot.lower_crossings > 0 || scale_pivot.upper_crossings > 0 || translation_grid_delta != IVec2::ZERO {
        warn!(
            "USF player pivot event: scale={:?}, zoom={:.6}, scale_crossings(l={},u={}), translation_grid_delta={:?}, player_pos={:?}",
            chunk_loader.scale,
            zoom_factor.0,
            scale_pivot.lower_crossings,
            scale_pivot.upper_crossings,
            translation_grid_delta,
            player_transform.translation
        );
        player_transform.translation.z = chunk_loader.scale.compute_z() + CONFIG().get::<f32>("player/z_offset");
    }

    // Player is a fine-scale phenomena: local mousewheel zoom also scales the player.
    player_transform.scale = Vec3::splat(zoom_factor.0.max(f32::EPSILON));

    for mut projection in projection_query.iter_mut() {
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scale = zoom_factor.0;
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
