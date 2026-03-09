use crate::bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};
use crate::chunk::resources::{ChunkActionWorkflowState, ChunkLoadGate};
use crate::config::statics::CONFIG;
use crate::core::protocol::PlayerMotionIntent;
use crate::input::states::InputMode;
use crate::player::components::Player;
use crate::render::{
    components::{ChunkCubeCamera, EntityProxyLink, LogicProxy, MainCamera, ProxySyncRevision, RenderProxy, RenderProxyWindowMode, UiCamera},
    functions::draw_primary_window_ui,
    resources::{DevZoomFactor, GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState, ViewScale, ZoomFactor},
};
use crate::time::resources::VirtualPaused;
use crate::usf::scale::Scale;

pub(super) fn pre_setup_phase_0(mut commands: Commands, mut images: ResMut<Assets<Image>>, windows: Query<&Window>) {
    // Reserve camera entities
    let egui_camera = commands.spawn(()).id();
    let ui_camera = commands.spawn(UiCamera).id();
    let main_camera = commands.spawn(MainCamera).id();
    let chunk_cube_camera = commands.spawn(ChunkCubeCamera).id();
    super::functions::reserve_camera_entities(egui_camera, ui_camera, main_camera, chunk_cube_camera);

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
    zoom_factor: Res<ZoomFactor>,
    dev_zoom_factor: Res<DevZoomFactor>,
    mut params: ParamSet<(
        Single<(&ChunkLoader, &Transform), With<Player>>,
        Query<(&EntityProxyLink, &ChunkActor), Without<RenderProxy>>,
        Query<(&EntityProxyLink, &Chunk), Without<RenderProxy>>,
        Query<(&mut Transform, &mut ProxySyncRevision, &mut RenderProxy), With<RenderProxy>>,
    )>,
) {
    let (chunk_loader, chunk_loader_transform) = *params.p0();
    let world_rotation = chunk_loader.world_rotation_quat();
    let world_rotation_origin = chunk_loader_transform.translation;
    let origin_offset = chunk_loader.origin_offset.clone();
    let view_pos_native = chunk_loader_transform.translation.truncate();
    let camera_zoom = (zoom_factor.0 * dev_zoom_factor.0).max(f32::EPSILON);
    let max_scale_diff = Scale::MAX_DIFF_SCALE_EXP;

    let actor_updates = {
        let chunk_actor_query = params.p1();
        chunk_actor_query
            .iter()
            .filter_map(|(link, chunk_actor)| {
                let scale_diff = chunk_actor.coord.scale as i8 - origin_offset.scale as i8;
                if scale_diff < 0 || scale_diff > max_scale_diff {
                    return None;
                }
                Some((link.render_entity, link.revision, chunk_actor.coord.clone()))
            })
            .collect::<Vec<_>>()
    };

    {
        let mut proxy_transforms = params.p3();
        for (proxy_entity, incoming_revision, coord) in actor_updates {
            if let Ok((mut proxy_transform, mut proxy_revision, mut proxy_state)) = proxy_transforms.get_mut(proxy_entity) {
                if incoming_revision.0 < proxy_revision.0 {
                    continue;
                }
                let coord_scale = coord.scale;
                let scale_diff = coord_scale as i8 - origin_offset.scale as i8;
                let z = coord_scale.compute_z() + proxy_state.depth_bias;
                let (pos, scale) = coord.to_native_visual(origin_offset.clone());
                let world_pos = pos.extend(z);
                proxy_transform.translation = world_rotation_origin + world_rotation * (world_pos - world_rotation_origin);
                proxy_transform.scale = Vec3::splat(scale);
                proxy_transform.rotation = world_rotation;
                proxy_state.layer_index = coord_scale.render_layer_index();
                let (window_mode, window_center_local, window_size_local) = compute_render_proxy_windowing(scale_diff, camera_zoom, pos, view_pos_native);
                proxy_state.window_mode = window_mode;
                proxy_state.window_center_local = window_center_local;
                proxy_state.window_size_local = window_size_local;
                proxy_state.coarse_context_persistent = true;
                proxy_revision.0 = incoming_revision.0;
            }
        }
    }

    let chunk_updates = {
        let chunk_query = params.p2();
        chunk_query
            .iter()
            .filter_map(|(link, chunk)| {
                let scale_diff = chunk.coord.scale as i8 - origin_offset.scale as i8;
                if scale_diff < 0 || scale_diff > max_scale_diff {
                    return None;
                }
                Some((link.render_entity, link.revision, chunk.coord.clone()))
            })
            .collect::<Vec<_>>()
    };

    {
        let mut proxy_transforms = params.p3();
        for (proxy_entity, incoming_revision, coord) in chunk_updates {
            if let Ok((mut proxy_transform, mut proxy_revision, mut proxy_state)) = proxy_transforms.get_mut(proxy_entity) {
                if incoming_revision.0 < proxy_revision.0 {
                    continue;
                }
                let coord_scale = coord.scale;
                let scale_diff = coord_scale as i8 - origin_offset.scale as i8;
                let z = coord_scale.compute_z() + proxy_state.depth_bias;
                let (pos, scale) = coord.to_native_visual(origin_offset.clone());
                let world_pos = pos.extend(z);
                proxy_transform.translation = world_rotation_origin + world_rotation * (world_pos - world_rotation_origin);
                proxy_transform.scale = Vec3::splat(scale);
                proxy_transform.rotation = world_rotation;
                proxy_state.layer_index = coord_scale.render_layer_index();
                let (window_mode, window_center_local, window_size_local) = compute_render_proxy_windowing(scale_diff, camera_zoom, pos, view_pos_native);
                proxy_state.window_mode = window_mode;
                proxy_state.window_center_local = window_center_local;
                proxy_state.window_size_local = window_size_local;
                proxy_state.coarse_context_persistent = true;
                proxy_revision.0 = incoming_revision.0;
            }
        }
    }
}

#[inline]
fn compute_render_proxy_windowing(scale_diff: i8, camera_zoom: f32, chunk_center_native: Vec2, view_pos_native: Vec2) -> (RenderProxyWindowMode, Vec2, Vec2) {
    if scale_diff <= 0 {
        return (RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);
    }

    let coarse_factor = 10.0_f32.powi(scale_diff as i32);
    if !coarse_factor.is_finite() || coarse_factor <= 0.0 {
        return (RenderProxyWindowMode::WindowedSubsection, Vec2::ZERO, Vec2::splat(0.001));
    }

    let chunk_span = 1000.0 * coarse_factor;
    let chunk_min = chunk_center_native - Vec2::splat(chunk_span * 0.5);
    let center01 = ((view_pos_native - chunk_min) / chunk_span).clamp(Vec2::ZERO, Vec2::ONE);
    let center_local = center01 - Vec2::splat(0.5);

    let zoom_term = camera_zoom.clamp(0.0001, 1.0);
    let window_size = (zoom_term / coarse_factor).clamp(0.0001, 1.0);

    (RenderProxyWindowMode::WindowedSubsection, center_local, Vec2::splat(window_size))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn full_entity_mode_for_same_or_finer_scale() {
        let (mode, center, size) = compute_render_proxy_windowing(0, 0.25, Vec2::ZERO, Vec2::new(123.0, -45.0));
        assert_eq!(mode, RenderProxyWindowMode::FullEntity);
        assert_eq!(center, Vec2::ZERO);
        assert_eq!(size, Vec2::ONE);
    }

    #[test]
    fn windowed_mode_scales_down_with_coarser_level() {
        let (mode, center, size) = compute_render_proxy_windowing(1, 1.0, Vec2::ZERO, Vec2::ZERO);
        assert_eq!(mode, RenderProxyWindowMode::WindowedSubsection);
        assert_eq!(center, Vec2::ZERO);
        assert!((size.x - 0.1).abs() < 1e-6);
        assert!((size.y - 0.1).abs() < 1e-6);
    }

    #[test]
    fn window_center_tracks_viewpoint_inside_chunk() {
        // scale_diff=1 => chunk span is 10,000 native units.
        let (mode, center, size) = compute_render_proxy_windowing(1, 0.5, Vec2::ZERO, Vec2::new(2_500.0, 2_500.0));
        assert_eq!(mode, RenderProxyWindowMode::WindowedSubsection);
        assert!(center.x > 0.0 && center.y > 0.0);
        assert!((size.x - 0.05).abs() < 1e-6);
        assert!((size.y - 0.05).abs() < 1e-6);
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn enforce_main_camera_depth_contract_system(mut main_camera_query: Query<(&mut Transform, &mut Projection), With<MainCamera>>) {
    let Ok((mut camera_transform, mut projection)) = main_camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.z = Scale::CANONICAL_CAMERA_Z;

    match projection.as_mut() {
        Projection::Orthographic(ortho) => {
            ortho.near = Scale::CANONICAL_CAMERA_NEAR;
            ortho.far = Scale::CANONICAL_CAMERA_FAR;
        }
        Projection::Perspective(perspective) => {
            perspective.near = 0.1;
            perspective.far = Scale::CANONICAL_CAMERA_FAR;
        }
        _ => {}
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn enforce_chunk_cube_camera_depth_contract_system(mut chunk_cube_camera_query: Query<(&mut Transform, &mut Projection), With<ChunkCubeCamera>>) {
    let Ok((mut camera_transform, mut projection)) = chunk_cube_camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.z = Scale::CANONICAL_CAMERA_Z;

    if let Projection::Perspective(perspective) = projection.as_mut() {
        perspective.near = 0.1;
        perspective.far = Scale::CANONICAL_CAMERA_FAR;
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn despawn_orphaned_render_proxies(
    mut removed: RemovedComponents<EntityProxyLink>,
    render_proxies: Query<(Entity, &RenderProxy)>,
    logic_proxies: Query<(Entity, &LogicProxy)>,
    mut commands: Commands,
) {
    for removed_source in removed.read() {
        for (proxy_entity, proxy) in &render_proxies {
            if proxy.source == removed_source {
                commands.entity(proxy_entity).despawn();
            }
        }
        for (proxy_entity, proxy) in &logic_proxies {
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
    mut projection_query: Query<&mut Projection, With<MainCamera>>,
    mut scroll_message_reader: MessageReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<State<InputMode>>,
    time: Res<Time<Real>>,
    virtual_paused: Res<VirtualPaused>,
    chunk_load_gate: Option<Res<ChunkLoadGate>>,
    mut zoom_factor: ResMut<ZoomFactor>,
    mut dev_zoom_factor: ResMut<DevZoomFactor>,
) {
    let min_zoom = CONFIG().get::<f32>("camera/min_zoom").max(f32::EPSILON);
    let max_zoom = CONFIG().get::<f32>("camera/max_zoom").max(min_zoom * 1.001);
    let base_zoom_speed = CONFIG().get::<f32>("camera/base_zoom_speed");
    let min_dev_zoom = CONFIG().get::<f32>("camera/min_dev_zoom").max(f32::EPSILON);
    let max_dev_zoom = CONFIG().get::<f32>("camera/max_dev_zoom").max(min_dev_zoom * 1.001);
    let dev_zoom_speed = CONFIG().get::<f32>("camera/dev_zoom_speed");
    let local_zoom_min = CONFIG().get::<f32>("usf/scale/local_min").max(f32::EPSILON);
    let local_zoom_max = CONFIG().get::<f32>("usf/scale/local_max").max(local_zoom_min * 1.001);
    let perspective_fov_min_deg = CONFIG().get::<f32>("camera/min_fov_degrees");
    let perspective_fov_max_deg = CONFIG().get::<f32>("camera/max_fov_degrees");
    let chunk_load_gate_enabled = CONFIG().get::<bool>("workflow/chunk_load_gate_enabled");

    if !input_mode.is_game() || virtual_paused.0 || (chunk_load_gate_enabled && chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked())) {
        scroll_message_reader.clear();
        return;
    }

    let shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);

    for message in scroll_message_reader.read() {
        let scroll_delta = match message.unit {
            MouseScrollUnit::Line => -message.y,
            MouseScrollUnit::Pixel => message.y * -0.01,
        };
        if shift_pressed {
            let zoom_speed = dev_zoom_speed * dev_zoom_factor.0;
            dev_zoom_factor.0 = (dev_zoom_factor.0 + scroll_delta * zoom_speed * time.delta_secs()).clamp(min_dev_zoom, max_dev_zoom);
        } else {
            let zoom_speed = base_zoom_speed * zoom_factor.0;
            zoom_factor.0 = (zoom_factor.0 + scroll_delta * zoom_speed * time.delta_secs()).clamp(min_zoom, max_zoom);
        }
    }
    let camera_zoom = (zoom_factor.0 * dev_zoom_factor.0).max(f32::EPSILON);
    let effective_zoom_min = local_zoom_min * min_dev_zoom;
    let effective_zoom_max = local_zoom_max * max_dev_zoom;
    for mut projection in projection_query.iter_mut() {
        apply_camera_zoom_to_projection(
            projection.as_mut(),
            camera_zoom,
            effective_zoom_min,
            effective_zoom_max,
            perspective_fov_min_deg,
            perspective_fov_max_deg,
        );
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn apply_usf_player_pivots_system(
    mut zoom_factor: ResMut<ZoomFactor>,
    dev_zoom_factor: Res<DevZoomFactor>,
    mut projection_query: Query<&mut Projection, With<MainCamera>>,
    mut player_loader_query: Query<(&mut ChunkLoader, &mut ChunkActor, &mut Transform), With<Player>>,
    mut chunk_load_gate: Option<ResMut<ChunkLoadGate>>,
    workflow_state: Option<Res<ChunkActionWorkflowState>>,
    mut player_motion_intent: ResMut<PlayerMotionIntent>,
) {
    let Ok((mut chunk_loader, mut chunk_actor, mut player_transform)) = player_loader_query.single_mut() else {
        player_motion_intent.clear();
        return;
    };

    let intent_translation_delta = player_motion_intent.translation_delta;
    let intent_rotation_delta = player_motion_intent.rotation_delta;
    player_motion_intent.clear();
    let world_space_translation_delta = if intent_translation_delta == Vec2::ZERO {
        Vec2::ZERO
    } else {
        // Input is authored in player-local XY; convert to world-space using current heading.
        (chunk_loader.world_rotation_quat().inverse() * intent_translation_delta.extend(0.0)).truncate()
    };

    let chunk_load_gate_enabled = CONFIG().get::<bool>("workflow/chunk_load_gate_enabled");
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
    let min_dev_zoom = CONFIG().get::<f32>("camera/min_dev_zoom").max(f32::EPSILON);
    let max_dev_zoom = CONFIG().get::<f32>("camera/max_dev_zoom").max(min_dev_zoom * 1.001);
    let perspective_fov_min_deg = CONFIG().get::<f32>("camera/min_fov_degrees");
    let perspective_fov_max_deg = CONFIG().get::<f32>("camera/max_fov_degrees");
    let workflow_in_flight = chunk_load_gate_enabled && workflow_state.as_ref().is_some_and(|state| !state.is_idle());

    if gate_locked {
        // Hard freeze mode: do not process additional pivot transitions while input is locked.
        zoom_factor.0 = zoom_factor.0.clamp(local_min, local_max);
        chunk_loader.usf_transform.scale.uniform.local = chunk_loader
            .usf_transform
            .scale
            .uniform
            .local
            .clamp(scale_policy.local_min, scale_policy.local_max);
        player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
        player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
        chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
        chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
        chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
        chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
        chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
    } else {
        let candidate_translation = player_transform.translation + world_space_translation_delta.extend(0.0);

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
            chunk_loader.usf_transform.scale.uniform.local = chunk_loader
                .usf_transform
                .scale
                .uniform
                .local
                .clamp(scale_policy.local_min, scale_policy.local_max);
            player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
            player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
            chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
            chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
            chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
            chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
            chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
        } else {
            if world_space_translation_delta != Vec2::ZERO {
                player_transform.translation += world_space_translation_delta.extend(0.0);
            }
            if intent_rotation_delta != Vec3::ZERO {
                chunk_loader.rotate_world_local(intent_rotation_delta);
            }

            let (scale_pivot, translation_grid_delta) = chunk_loader.apply_player_anchor_pivots(&mut zoom_factor.0, &mut player_transform.translation);
            chunk_actor.coord = chunk_loader.coord.clone();
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
                chunk_loader.usf_transform.scale.uniform.local = chunk_loader
                    .usf_transform
                    .scale
                    .uniform
                    .local
                    .clamp(scale_policy.local_min, scale_policy.local_max);
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
    let camera_zoom = (display_zoom * dev_zoom_factor.0).max(f32::EPSILON);

    // Player is a fine-scale phenomena: local mousewheel zoom also scales the player.
    player_transform.scale = Vec3::splat(display_zoom.max(f32::EPSILON));

    for mut projection in projection_query.iter_mut() {
        apply_camera_zoom_to_projection(
            projection.as_mut(),
            camera_zoom,
            local_min * min_dev_zoom,
            local_max * max_dev_zoom,
            perspective_fov_min_deg,
            perspective_fov_max_deg,
        );
    }
}

#[inline]
fn apply_camera_zoom_to_projection(
    projection: &mut Projection,
    camera_zoom: f32,
    effective_zoom_min: f32,
    effective_zoom_max: f32,
    perspective_fov_min_deg: f32,
    perspective_fov_max_deg: f32,
) {
    match projection {
        Projection::Orthographic(ortho) => {
            ortho.scale = camera_zoom.max(f32::EPSILON);
        }
        Projection::Perspective(perspective) => {
            perspective.fov = zoom_to_fov_radians(
                camera_zoom,
                effective_zoom_min.max(f32::EPSILON),
                effective_zoom_max.max(effective_zoom_min * 1.001),
                perspective_fov_min_deg,
                perspective_fov_max_deg,
            );
        }
        _ => {}
    }
}

#[inline]
fn zoom_to_fov_radians(camera_zoom: f32, zoom_min: f32, zoom_max: f32, fov_min_deg: f32, fov_max_deg: f32) -> f32 {
    let zoom = camera_zoom.clamp(zoom_min, zoom_max);
    let min_ln = zoom_min.ln();
    let max_ln = zoom_max.ln();
    let t = if (max_ln - min_ln).abs() <= f32::EPSILON {
        0.0
    } else {
        ((zoom.ln() - min_ln) / (max_ln - min_ln)).clamp(0.0, 1.0)
    };

    let min_fov = fov_min_deg.max(1.0).to_radians();
    let max_fov = fov_max_deg.max(fov_min_deg + 1.0).to_radians();
    (min_fov + (max_fov - min_fov) * t).clamp(0.01, std::f32::consts::PI - 0.01)
}

#[tracing::instrument(skip_all)]
pub(super) fn update_view_scale_from_zoom(zoom_factor: Res<ZoomFactor>, mut view_scale: ResMut<ViewScale>) {
    let zoom = zoom_factor.0;
    let scale = -zoom.log10(); // Since zooming in decreases ortho scale
    view_scale.discrete = scale.floor() as i32;
    view_scale.offset = scale.fract();
}
