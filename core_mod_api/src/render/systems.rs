use crate::bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};
use crate::chunk::resources::{ChunkActionWorkflowState, ChunkLoadGate};
use crate::config::statics::CONFIG;
use crate::core::protocol::PlayerMotionIntent;
use crate::input::states::InputMode;
use crate::player::components::Player;
use crate::usf::scale::Scale;
use crate::render::{
    components::{MainCamera, RenderProxy, RenderProxyHandle, UiCamera},
    functions::draw_primary_window_ui,
    resources::{GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState, ViewScale, ZoomAuthority, ZoomFactor},
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
    let Ok(window) = windows.single() else {
        return;
    };
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
    // Keep render-space origin aligned with chunk detection/spawn origin.
    // Detection/spawn are driven by `chunk_loader.coord`, so using `origin_offset`
    // here causes visual space to drift from loaded space during scale pivots.
    let render_origin = chunk_loader.coord.clone();
    let max_scale_diff = Scale::MAX_DIFF_SCALE_EXP;

    let actor_updates = {
        let chunk_actor_query = params.p1();
        chunk_actor_query
            .iter()
            .filter_map(|(handle, chunk_actor)| {
                let scale_diff = chunk_actor.coord.scale as i8 - render_origin.scale as i8;
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
                let (pos, scale) = coord.to_native_visual(render_origin.clone());
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
                let scale_diff = chunk.coord.scale as i8 - render_origin.scale as i8;
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
                let (pos, scale) = coord.to_native_visual(render_origin.clone());
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
    mut zoom_authority: ResMut<ZoomAuthority>,
) {
    let min_zoom = CONFIG().get::<f32>("camera/min_zoom");
    let max_zoom = CONFIG().get::<f32>("camera/max_zoom");
    let base_zoom_speed = CONFIG().get::<f32>("camera/base_zoom_speed");
    let chunk_load_gate_enabled = CONFIG().get::<bool>("workflow/chunk_load_gate_enabled");
    let gate_locked = chunk_load_gate_enabled && chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked());
    zoom_authority.gate_locked = gate_locked;

    if !input_mode.is_game() || virtual_paused.0 {
        scroll_message_reader.clear();
        zoom_authority.local_zoom = zoom_authority.local_zoom.clamp(min_zoom, max_zoom);
        zoom_factor.0 = zoom_authority.local_zoom;
        for mut projection in projection_query.iter_mut() {
            match projection.as_mut() {
                Projection::Orthographic(ortho) => {
                    ortho.scale = zoom_factor.0;
                }
                _ => panic!("Main camera is not orthographic/2d!"),
            }
        }
        return;
    }

    let mut frame_zoom_delta = 0.0_f32;
    for message in scroll_message_reader.read() {
        let scroll_delta = match message.unit {
            MouseScrollUnit::Line => -message.y,
            MouseScrollUnit::Pixel => message.y * -0.01,
        };
        let zoom_speed = base_zoom_speed * zoom_authority.local_zoom;
        frame_zoom_delta += scroll_delta * zoom_speed * time.delta_secs();
    }

    if gate_locked {
        if frame_zoom_delta != 0.0 {
            zoom_authority.pending_zoom_delta += frame_zoom_delta;
        }
    } else {
        // Drain buffered zoom pressure gradually to avoid post-unlock jumps.
        let replay_cap = (zoom_authority.local_zoom.abs() * 0.25).max(0.01);
        let replay_delta = zoom_authority.pending_zoom_delta.clamp(-replay_cap, replay_cap);
        zoom_authority.pending_zoom_delta -= replay_delta;
        zoom_authority.local_zoom += frame_zoom_delta + replay_delta;
    }

    zoom_authority.local_zoom = zoom_authority.local_zoom.clamp(min_zoom, max_zoom);
    zoom_factor.0 = zoom_authority.local_zoom;

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
    mut chunk_load_gate: Option<ResMut<ChunkLoadGate>>,
    workflow_state: Option<Res<ChunkActionWorkflowState>>,
    mut player_motion_intent: ResMut<PlayerMotionIntent>,
    mut zoom_authority: ResMut<ZoomAuthority>,
) {
    let chunk_load_gate_enabled = CONFIG().get::<bool>("workflow/chunk_load_gate_enabled");
    zoom_authority.gate_locked = chunk_load_gate_enabled && chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked());

    let Ok((mut chunk_loader, mut player_transform)) = player_loader_query.single_mut() else {
        player_motion_intent.clear();
        return;
    };

    let intent_translation_delta = player_motion_intent.translation_delta;
    let intent_rotation_delta = player_motion_intent.rotation_delta;
    player_motion_intent.clear();

    let mut gate_locked = chunk_load_gate_enabled && chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked());
    let scale_policy = chunk_loader.usf_transform.scale.policy;
    let local_min = scale_policy.local_min as f32;
    let local_max = scale_policy.local_max as f32;
    let scale_commit_min = scale_policy.commit_min() as f32;
    let scale_commit_max = scale_policy.commit_max() as f32;
    let translation_policy = chunk_loader.usf_transform.translation.policy;
    let translation_local_min = translation_policy.local_min as f32;
    let translation_local_max = translation_policy.local_max as f32;
    let translation_commit_min = translation_policy.commit_min() as f32;
    let translation_commit_max = translation_policy.commit_max() as f32;
    let rotation_policy = chunk_loader.usf_transform.rotation.policy;
    let rotation_local_min = rotation_policy.local_min;
    let rotation_local_max = rotation_policy.local_max;
    let workflow_in_flight = chunk_load_gate_enabled && workflow_state.as_ref().is_some_and(|state| !state.is_idle());

    if gate_locked {
        // Hard freeze mode: do not process additional pivot transitions while input is locked.
        zoom_factor.0 = zoom_factor.0.clamp(local_min, local_max);
        chunk_loader.usf_transform.scale.uniform.local =
            chunk_loader.usf_transform.scale.uniform.local.clamp(scale_policy.local_min, scale_policy.local_max);
        player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
        player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
        chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
        chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
        chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
        chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
        chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
    } else {
        let candidate_translation = player_transform.translation + intent_translation_delta.extend(0.0);

        let would_cross_scale_boundary = zoom_factor.0 <= scale_commit_min || zoom_factor.0 >= scale_commit_max;
        let would_cross_translation_boundary = candidate_translation.x <= translation_commit_min
            || candidate_translation.x >= translation_commit_max
            || candidate_translation.y <= translation_commit_min
            || candidate_translation.y >= translation_commit_max;

        if workflow_in_flight && (would_cross_scale_boundary || would_cross_translation_boundary) {
            if let Some(gate) = chunk_load_gate.as_mut() {
                let changed = gate.lock_by_in_flight_boundary();
                if changed {
                    warn!("ChunkLoadGate preemptively locked due to boundary crossing attempt while chunk workflow is in flight");
                }
            }

            // Reject boundary commit while a previous boundary batch is still in flight.
            zoom_factor.0 = zoom_factor.0.clamp(local_min, local_max);
            chunk_loader.usf_transform.scale.uniform.local =
                chunk_loader.usf_transform.scale.uniform.local.clamp(scale_policy.local_min, scale_policy.local_max);
            player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
            player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
            chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
            chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
            chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
            chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
            chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
        } else {
            if intent_translation_delta != Vec2::ZERO {
                player_transform.translation += intent_translation_delta.extend(0.0);
            }
            if intent_rotation_delta != Vec3::ZERO {
                chunk_loader.rotate_world_local(intent_rotation_delta);
            }

            let (scale_pivot, translation_grid_delta) =
                chunk_loader.apply_player_anchor_pivots(&mut zoom_factor.0, &mut player_transform.translation);
            zoom_factor.0 = zoom_factor.0.clamp(scale_commit_min, scale_commit_max);

            let boundary_crossed = scale_pivot.lower_crossings > 0 || scale_pivot.upper_crossings > 0 || translation_grid_delta != IVec2::ZERO;
            if boundary_crossed && workflow_in_flight {
                if let Some(gate) = chunk_load_gate.as_mut() {
                    let changed = gate.lock_by_in_flight_boundary();
                    if changed {
                        warn!("ChunkLoadGate locked immediately during pivot due to in-flight boundary overlap");
                    }
                }
                gate_locked = true;
            }

            if gate_locked {
                zoom_factor.0 = zoom_factor.0.clamp(local_min, local_max);
                chunk_loader.usf_transform.scale.uniform.local =
                    chunk_loader.usf_transform.scale.uniform.local.clamp(scale_policy.local_min, scale_policy.local_max);
                player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
                player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
                chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
                chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
                chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
                chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
                chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
            }

            if boundary_crossed {
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
        }
    }

    // Keep commit-buffer accumulation internal. Rendering should never show values outside strict local bounds.
    let display_zoom = zoom_factor.0.clamp(local_min, local_max);

    // Player is a fine-scale phenomena: local mousewheel zoom also scales the player.
    player_transform.scale = Vec3::splat(display_zoom.max(f32::EPSILON));

    for mut projection in projection_query.iter_mut() {
        if let Projection::Orthographic(ortho) = projection.as_mut() {
            ortho.scale = display_zoom;
        }
    }

    zoom_authority.local_zoom = zoom_factor.0;
    zoom_authority.gate_locked = chunk_load_gate_enabled && chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked());
    zoom_authority.global_scale_exponent = chunk_loader.scale as i8;
    zoom_authority.global_scale_index_from_top = chunk_loader.scale.index_from_top();
    zoom_authority.global_scale_name = format!("{:?}", chunk_loader.scale);
}

#[tracing::instrument(skip_all)]
pub(super) fn update_view_scale_from_zoom(zoom_factor: Res<ZoomFactor>, mut view_scale: ResMut<ViewScale>) {
    let zoom = zoom_factor.0;
    let scale = -zoom.log10(); // Since zooming in decreases ortho scale
    view_scale.discrete = scale.floor() as i32;
    view_scale.offset = scale.fract();
}
