use crate::bevy::camera::RenderTarget;
use crate::bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};
use crate::bevy_rapier3d::parry::shape::Capsule as RapierCapsule;
use crate::bevy_rapier3d::prelude::{QueryFilter as RapierQueryFilter, ReadRapierContext};

use crate::chunk::components::{Chunk, ChunkActor, ChunkDebugWireframe, ChunkLoader};
use crate::chunk::resources::{ChunkActionWorkflowState, ChunkLoadGate};
use crate::config::statics::CONFIG;
use crate::core::protocol::PlayerMotionIntent;
use crate::input::states::InputMode;
use crate::player::components::Player;
use crate::player::resources::{PlayerCameraMode, PlayerCameraRigSettings, PlayerControlSettings};
use crate::render::{
    camera_contract,
    components::{EguiCamera, EntityProxyLink, LogicProxy, MainCamera, ProxySyncRevision, RenderProxy, RenderProxyWindowMode, UiCamera, WorldPresentationRoot},
    functions::draw_primary_window_ui,
    resources::{DevZoomFactor, GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState, RenderPrecisionAnchor, RuntimeDebugToggles, ViewScale},
};
use crate::time::resources::VirtualPaused;
use crate::tracing::{error, info};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::pos::unit::types::UnitVec;
use crate::usf::scale::Scale;
use std::collections::{BTreeMap, HashMap, HashSet};

const MIN_WINDOW_SIZE_LOCAL: f32 = f32::MIN_POSITIVE;
const CHUNK_SPAN_UNITS_F64: f64 = 1_000.0;
const ROOT_AXIS_CELL_COUNT_F64: f64 = 10.0;
const ROOT_AXIS_PERIOD_UNITS_F64: f64 = CHUNK_SPAN_UNITS_F64 * ROOT_AXIS_CELL_COUNT_F64;
#[cfg(test)]
const CAMERA_EFFECTIVE_ZOOM_MIN: f32 = 0.1;
#[cfg(test)]
const CAMERA_EFFECTIVE_ZOOM_MAX: f32 = 10.0;
#[cfg(test)]
const CAMERA_REFERENCE_HALF_VIEW_SPAN: f32 = 1_200.0;
#[cfg(test)]
const CAMERA_DISTANCE_MIN: f32 = 80.0;
#[cfg(test)]
const CAMERA_DISTANCE_MAX: f32 = 25_000.0;

#[inline]
fn player_local_zoom_for_presentation(chunk_loader: &ChunkLoader) -> f32 {
    let local_min = chunk_loader.usf_transform.scale.policy.local_min as f32;
    let local_max = chunk_loader.usf_transform.scale.policy.local_max as f32;
    chunk_loader
        .usf_transform
        .scale
        .local_f32()
        .clamp(local_min.max(f32::MIN_POSITIVE), local_max.max(local_min * 1.001))
}

#[inline]
fn world_presentation_scale_from_local_zoom(local_zoom: f32) -> f32 {
    local_zoom.max(f32::MIN_POSITIVE).recip()
}

#[inline]
fn canonical_grid_coord(coord: &GridVec) -> GridVec {
    let mut canonical = coord.clone();
    canonical.normalize();
    canonical
}

#[inline]
fn sample_root_native_position(canonical_coord: &GridVec, local_offset: Vec3) -> [f64; 3] {
    let mut sample = UnitVec::new(canonical_coord.clone(), local_offset);
    while sample.grid_offset.scale != Scale::MAX {
        sample.zoom_out();
    }
    let root = sample.grid_offset.xyz;
    [
        root.x as f64 * CHUNK_SPAN_UNITS_F64 + sample.unit_offset.x as f64,
        root.y as f64 * CHUNK_SPAN_UNITS_F64 + sample.unit_offset.y as f64,
        root.z as f64 * CHUNK_SPAN_UNITS_F64 + sample.unit_offset.z as f64,
    ]
}

#[inline]
fn wrap_root_native_delta_axis(delta: f64) -> f64 {
    if !delta.is_finite() || ROOT_AXIS_PERIOD_UNITS_F64 <= f64::EPSILON {
        return 0.0;
    }
    let half_period = ROOT_AXIS_PERIOD_UNITS_F64 * 0.5;
    ((delta + half_period).rem_euclid(ROOT_AXIS_PERIOD_UNITS_F64)) - half_period
}

#[inline]
fn saturating_f64_to_f32(value: f64) -> f32 {
    if value.is_nan() {
        return 0.0;
    }
    if value.is_infinite() {
        return if value.is_sign_negative() { f32::MIN } else { f32::MAX };
    }
    if value > f32::MAX as f64 {
        return f32::MAX;
    }
    if value < f32::MIN as f64 {
        return f32::MIN;
    }
    value as f32
}

#[inline]
fn saturating_scale_factor_from_scale_diff(scale_diff: i16) -> f32 {
    let scale = 10.0_f64.powi(scale_diff as i32);
    if !scale.is_finite() {
        return f32::MAX;
    }
    if scale <= 0.0 {
        return f32::MIN_POSITIVE;
    }
    scale.clamp(f32::MIN_POSITIVE as f64, f32::MAX as f64) as f32
}

#[inline]
fn player_relative_chunk_center_native_from_anchor(coord: &GridVec, anchor: &RenderPrecisionAnchor) -> (Vec3, f32, i8) {
    let canonical_coord = canonical_grid_coord(coord);
    let chunk_root_native = sample_root_native_position(&canonical_coord, Vec3::ZERO);
    let root_to_active = 10.0_f64.powi(anchor.active_scale_index as i32);

    let dx = wrap_root_native_delta_axis(chunk_root_native[0] - anchor.player_root_native[0]) * root_to_active;
    let dy = wrap_root_native_delta_axis(chunk_root_native[1] - anchor.player_root_native[1]) * root_to_active;
    let dz = wrap_root_native_delta_axis(chunk_root_native[2] - anchor.player_root_native[2]) * root_to_active;

    let scale_diff = coord.scale.index_from_top() as i16 - anchor.active_scale_index;
    let relative_scale_to_player = scale_diff.clamp(i8::MIN as i16, i8::MAX as i16) as i8;
    let scale = saturating_scale_factor_from_scale_diff(scale_diff);

    (
        Vec3::new(saturating_f64_to_f32(dx), saturating_f64_to_f32(dy), saturating_f64_to_f32(dz)),
        scale,
        relative_scale_to_player,
    )
}

#[inline]
#[cfg(test)]
fn camera_distance_from_zoom_and_fov(zoom: f32, fov_radians: f32) -> f32 {
    let zoom = zoom.clamp(CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX);
    let tan_half = (fov_radians * 0.5).tan().abs().max(1e-4);
    let half_view_span = CAMERA_REFERENCE_HALF_VIEW_SPAN / zoom;
    (half_view_span / tan_half).clamp(CAMERA_DISTANCE_MIN, CAMERA_DISTANCE_MAX)
}

fn create_game_view_render_image(size_uvec2: UVec2) -> Image {
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
    image
}

pub(super) fn pre_setup_phase_0(mut commands: Commands, mut images: ResMut<Assets<Image>>, windows: Query<&Window>) {
    // Reserve camera entities
    let egui_camera = commands.spawn(EguiCamera).id();
    let ui_camera = commands.spawn(UiCamera).id();
    let main_camera = commands.spawn(MainCamera).id();
    commands.spawn((
        Name::new("world_presentation_root"),
        WorldPresentationRoot,
        Transform::default(),
        GlobalTransform::default(),
    ));
    super::functions::reserve_camera_entities(egui_camera, ui_camera, main_camera);

    // Reserve game view render target handle
    let window = windows.single().unwrap();
    let size_uvec2 = window.physical_size();
    let image_handle = images.add(create_game_view_render_image(size_uvec2));
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
    mut resize_cooldown_frames: Local<u8>,
    mut previous_window_size_uvec2: Local<UVec2>,
    mut images: ResMut<Assets<Image>>,
    mut egui_textures: ResMut<bevy_egui::EguiUserTextures>,
    mut game_view_render_target: ResMut<GameViewRenderTarget>,
    mut game_view_cameras: Query<&mut Camera, Or<(With<MainCamera>, With<UiCamera>)>>,
    mut game_view_camera_targets: Query<&mut RenderTarget, Or<(With<MainCamera>, With<UiCamera>)>>,
    windows: Query<&Window>,
) {
    let Ok(window) = windows.single() else {
        return;
    };
    let size_uvec2 = window.physical_size();

    if *resize_cooldown_frames > 0 {
        *resize_cooldown_frames -= 1;
        if *resize_cooldown_frames == 0 {
            for mut camera in game_view_cameras.iter_mut() {
                camera.is_active = true;
            }
        }
    }

    if size_uvec2 == *previous_window_size_uvec2 {
        return;
    }

    *previous_window_size_uvec2 = size_uvec2;
    let old_handle = game_view_render_target.handle.clone();
    let new_handle = images.add(create_game_view_render_image(size_uvec2));

    for mut camera in game_view_cameras.iter_mut() {
        camera.is_active = false;
    }

    for mut camera_target in game_view_camera_targets.iter_mut() {
        *camera_target = camera_contract::game_view_render_target(&new_handle);
    }

    let _ = egui_textures.remove_image(old_handle.id());
    let new_texture_id = egui_textures.add_image(bevy_egui::EguiTextureHandle::Weak(new_handle.id()));

    game_view_render_target.handle = new_handle;
    game_view_render_target.size = size_uvec2;
    game_view_render_target.id = new_texture_id;
    *resize_cooldown_frames = 2;
}

pub(super) fn validate_camera_contract_system(
    active_cameras: Query<(Entity, &Camera, &RenderTarget, Option<&Name>)>,
    main_cameras: Query<Entity, With<MainCamera>>,
    ui_cameras: Query<Entity, With<UiCamera>>,
    egui_cameras: Query<Entity, With<EguiCamera>>,
    mut previous_report: Local<Option<String>>,
) {
    let mut violations = Vec::new();

    let main_count = main_cameras.iter().count();
    let ui_count = ui_cameras.iter().count();
    let egui_count = egui_cameras.iter().count();
    if main_count != 1 {
        violations.push(format!("expected exactly 1 MainCamera marker, found {}", main_count));
    }
    if ui_count != 1 {
        violations.push(format!("expected exactly 1 UiCamera marker, found {}", ui_count));
    }
    if egui_count != 1 {
        violations.push(format!("expected exactly 1 EguiCamera marker, found {}", egui_count));
    }

    let mut active_by_order_target: HashMap<(isize, String), Vec<String>> = HashMap::new();
    for (entity, camera, render_target, name) in active_cameras.iter() {
        if !camera.is_active {
            continue;
        }
        let key = (camera.order, format!("{:?}", render_target));
        let label = name.map(|n| n.as_str().to_string()).unwrap_or_else(|| format!("{:?}", entity));
        active_by_order_target.entry(key).or_default().push(label);
    }
    for ((order, target), names) in active_by_order_target {
        if names.len() > 1 {
            violations.push(format!("active camera order collision for order={} target={}: {:?}", order, target, names));
        }
    }

    let mut report_key = format!("main={main_count};ui={ui_count};egui={egui_count}");
    if !violations.is_empty() {
        report_key.push('|');
        report_key.push_str(&violations.join("|"));
    }
    if previous_report.as_deref() == Some(report_key.as_str()) {
        return;
    }
    *previous_report = Some(report_key);

    if violations.is_empty() {
        info!(
            "Camera contract validated: orders main={} ui={} egui={}, layers main={} ui={}",
            camera_contract::MAIN_CAMERA_ORDER,
            camera_contract::UI_CAMERA_ORDER,
            camera_contract::EGUI_CAMERA_ORDER,
            camera_contract::MAIN_CAMERA_RENDER_LAYER,
            camera_contract::UI_CAMERA_RENDER_LAYER
        );
    } else {
        error!("Camera contract violation detected:");
        for violation in violations {
            error!("  {violation}");
        }
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn update_render_precision_anchor_system(
    player_loader_query: Single<(&ChunkLoader, &Transform), (With<Player>, Without<WorldPresentationRoot>)>,
    mut render_precision_anchor: ResMut<RenderPrecisionAnchor>,
) {
    let (chunk_loader, player_transform) = *player_loader_query;
    let canonical_coord = canonical_grid_coord(&chunk_loader.coord);

    render_precision_anchor.active_scale = chunk_loader.scale;
    render_precision_anchor.active_scale_index = chunk_loader.scale.index_from_top() as i16;
    render_precision_anchor.player_root_native = sample_root_native_position(&canonical_coord, player_transform.translation);
}

#[tracing::instrument(skip_all)]
pub(super) fn update_world_presentation_root_transform_system(
    player_loader_query: Single<(&ChunkLoader, &Transform), (With<Player>, Without<WorldPresentationRoot>, Without<MainCamera>)>,
    root_query: Single<&mut Transform, (With<WorldPresentationRoot>, Without<Player>)>,
) {
    let (chunk_loader, player_transform) = *player_loader_query;
    let local_zoom = player_local_zoom_for_presentation(chunk_loader);
    let world_presentation_scale = world_presentation_scale_from_local_zoom(local_zoom);

    let mut root_transform = root_query.into_inner();
    root_transform.translation = player_transform.translation;
    root_transform.rotation = chunk_loader.world_rotation_quat();
    root_transform.scale = Vec3::splat(world_presentation_scale);
}

#[tracing::instrument(skip_all)]
pub(super) fn bind_render_proxies_to_world_presentation_root_system(
    mut commands: Commands,
    root_query: Single<Entity, With<WorldPresentationRoot>>,
    proxy_query: Query<(Entity, Option<&ChildOf>), With<RenderProxy>>,
) {
    let root = *root_query;
    for (entity, child_of) in proxy_query.iter() {
        if child_of.is_some_and(|relation| relation.parent() == root) {
            continue;
        }
        commands.entity(entity).insert(ChildOf(root));
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn update_render_proxies(
    render_precision_anchor: Res<RenderPrecisionAnchor>,
    mut params: ParamSet<(
        Single<(&ChunkLoader, &Transform), (With<Player>, Without<MainCamera>)>,
        Query<(&EntityProxyLink, &ChunkActor), Without<RenderProxy>>,
        Query<(Entity, &mut Transform, &mut ProxySyncRevision, &mut RenderProxy, &mut Visibility), With<RenderProxy>>,
    )>,
) {
    let (world_rotation_origin, view_pos_native, player_local_zoom, world_presentation_scale, precision_anchor) = {
        let (chunk_loader, chunk_loader_transform) = *params.p0();
        let local_zoom = player_local_zoom_for_presentation(chunk_loader);
        let presentation_scale = world_presentation_scale_from_local_zoom(local_zoom);
        let world_presentation_origin = chunk_loader_transform.translation;
        let view_anchor_native = chunk_loader_transform.translation;
        (
            world_presentation_origin,
            view_anchor_native,
            local_zoom,
            presentation_scale,
            *render_precision_anchor,
        )
    };
    let load_radius = CONFIG().get::<u32>("chunk_loader/load_radius") as usize;
    let active_budget = (load_radius.saturating_mul(12)).clamp(16, 128);

    let actor_updates = {
        let chunk_actor_query = params.p1();
        chunk_actor_query
            .iter()
            .filter_map(|(link, chunk_actor)| Some((link.render_entity, link.revision, chunk_actor.coord.clone())))
            .collect::<Vec<_>>()
    };

    {
        let mut proxy_transforms = params.p2();
        for (proxy_entity, incoming_revision, coord) in actor_updates {
            if let Ok((_entity, mut proxy_transform, mut proxy_revision, mut proxy_state, _visibility)) = proxy_transforms.get_mut(proxy_entity) {
                if incoming_revision.0 < proxy_revision.0 {
                    continue;
                }
                let (player_relative_center, scale, relative_scale_to_player) = player_relative_chunk_center_native_from_anchor(&coord, &precision_anchor);
                let z_bias = proxy_state.depth_bias;
                let world_center = view_pos_native + player_relative_center;
                let world_pos = Vec3::new(world_center.x, world_center.y, world_center.z + z_bias);
                proxy_transform.translation = world_pos - world_rotation_origin;
                proxy_transform.scale = Vec3::splat(scale);
                proxy_transform.rotation = Quat::IDENTITY;
                proxy_state.layer_index = coord.scale.render_layer_index();
                let (window_mode, window_center_local, window_size_local) =
                    compute_render_proxy_windowing(relative_scale_to_player, world_center, view_pos_native);
                proxy_state.window_mode = window_mode;
                proxy_state.window_center_local = window_center_local;
                proxy_state.window_size_local = window_size_local;
                proxy_state.relative_scale_to_player = relative_scale_to_player;
                proxy_state.player_local_zoom = player_local_zoom;
                proxy_state.player_world_presentation_scale = world_presentation_scale;
                proxy_state.coarse_context_persistent = true;
                proxy_revision.0 = incoming_revision.0;
            }
        }

        let mut cull_candidates = Vec::new();
        for (entity, proxy_transform, _proxy_revision, proxy_state, _visibility) in proxy_transforms.iter_mut() {
            if proxy_state.relative_scale_to_player > 0 {
                continue;
            }

            let world_pos = world_rotation_origin + proxy_transform.translation;
            cull_candidates.push(RenderProxyCullCandidate {
                entity,
                relative_scale_to_player: proxy_state.relative_scale_to_player,
                distance_sq: world_pos.distance_squared(view_pos_native),
            });
        }

        let visible_entities = select_visible_render_proxy_entities(&cull_candidates, active_budget);
        for (entity, _proxy_transform, _proxy_revision, _proxy_state, mut visibility) in proxy_transforms.iter_mut() {
            *visibility = if visible_entities.contains(&entity) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct RenderProxyCullCandidate {
    entity: Entity,
    relative_scale_to_player: i8,
    distance_sq: f32,
}

#[inline]
fn render_proxy_band_budget(relative_scale_to_player: i8, active_budget: usize) -> usize {
    if relative_scale_to_player >= 0 {
        return active_budget.max(1);
    }

    match (-relative_scale_to_player) as u8 {
        1 => (active_budget / 3).max(6),
        2 => (active_budget / 6).max(3),
        3 => (active_budget / 12).max(1),
        _ => 1,
    }
}

fn select_visible_render_proxy_entities(candidates: &[RenderProxyCullCandidate], active_budget: usize) -> HashSet<Entity> {
    let mut grouped = BTreeMap::<i8, Vec<RenderProxyCullCandidate>>::new();
    for candidate in candidates.iter().copied() {
        grouped.entry(candidate.relative_scale_to_player).or_default().push(candidate);
    }

    let mut selected = HashSet::<Entity>::new();
    for (band, mut entries) in grouped {
        entries.sort_by(|a, b| {
            a.distance_sq
                .total_cmp(&b.distance_sq)
                .then_with(|| a.entity.to_bits().cmp(&b.entity.to_bits()))
        });
        let budget = render_proxy_band_budget(band, active_budget).min(entries.len());
        for candidate in entries.into_iter().take(budget) {
            selected.insert(candidate.entity);
        }
    }

    selected
}

#[tracing::instrument(skip_all)]
pub(super) fn draw_chunk_locator_gizmos_system(
    mut gizmos: Gizmos,
    player_loader_query: Single<(&ChunkLoader, &Transform), (With<Player>, Without<MainCamera>)>,
    loaded_chunks: Query<&Chunk, With<ChunkDebugWireframe>>,
    runtime_debug_toggles: Option<Res<RuntimeDebugToggles>>,
) {
    let chunk_locator_enabled = runtime_debug_toggles.as_ref().map(|toggles| toggles.chunk_locator_enabled).unwrap_or(true);
    if !CONFIG().get::<bool>("debug/chunk_locator/enabled") || !chunk_locator_enabled {
        return;
    }

    let base_extent = CONFIG().get::<f32>("debug/chunk_locator/base_extent").max(1.0);
    let z_scale = CONFIG().get::<f32>("debug/chunk_locator/z_scale").max(0.01);
    let alpha = CONFIG().get::<f32>("debug/chunk_locator/alpha").clamp(0.01, 1.0);
    let player_alpha = CONFIG().get::<f32>("debug/chunk_locator/player_alpha").clamp(alpha, 1.0);
    let load_radius = CONFIG().get::<u32>("chunk_loader/load_radius");

    let (chunk_loader, player_transform) = *player_loader_query;
    let world_rotation = chunk_loader.world_rotation_quat();
    let world_rotation_origin = player_transform.translation;
    let local_zoom = player_local_zoom_for_presentation(chunk_loader);
    let world_presentation_scale = world_presentation_scale_from_local_zoom(local_zoom);
    let origin_offset = chunk_loader.origin_offset.clone();
    let player_coord = chunk_loader.coord.clone();

    let loaded_coords: HashSet<GridVec> = loaded_chunks.iter().map(|chunk| chunk.coord.clone()).collect();
    let target_coords = collect_target_chunk_frontier(chunk_loader, load_radius);

    let loaded_color = Color::linear_rgba(0.35, 0.65, 1.0, alpha);
    let unloaded_color = Color::linear_rgba(0.96, 0.35, 0.24, alpha);
    let player_color = Color::linear_rgba(1.0, 0.96, 0.45, player_alpha);

    for coord in target_coords.difference(&loaded_coords) {
        if *coord == player_coord {
            continue;
        }
        let marker = chunk_wire_transform(
            coord,
            &origin_offset,
            world_rotation,
            world_rotation_origin,
            world_presentation_scale,
            base_extent,
            z_scale,
        );
        gizmos.cube(marker, unloaded_color);
    }

    for coord in &loaded_coords {
        if *coord == player_coord {
            continue;
        }
        let marker = chunk_wire_transform(
            coord,
            &origin_offset,
            world_rotation,
            world_rotation_origin,
            world_presentation_scale,
            base_extent,
            z_scale,
        );
        gizmos.cube(marker, loaded_color);
    }

    let player_marker = chunk_wire_transform(
        &player_coord,
        &origin_offset,
        world_rotation,
        world_rotation_origin,
        world_presentation_scale,
        base_extent,
        z_scale,
    );
    gizmos.cube(player_marker, player_color);
}

fn collect_target_chunk_frontier(chunk_loader: &ChunkLoader, load_radius: u32) -> HashSet<GridVec> {
    let mut target_coords = chunk_loader.coord.query_grid_radius(load_radius).into_iter().collect::<HashSet<_>>();
    let mut frontier = target_coords.clone();

    loop {
        let parent_coords = frontier
            .iter()
            .filter_map(|coord| coord.parent.as_ref().map(|parent| parent.as_ref().clone()))
            .collect::<HashSet<_>>();
        if parent_coords.is_empty() {
            break;
        }
        frontier = parent_coords.difference(&target_coords).cloned().collect::<HashSet<_>>();
        if frontier.is_empty() {
            break;
        }
        target_coords.extend(frontier.iter().cloned());
    }

    target_coords
}

fn chunk_wire_transform(
    coord: &GridVec,
    origin_offset: &GridVec,
    world_rotation: Quat,
    world_rotation_origin: Vec3,
    world_presentation_scale: f32,
    base_extent: f32,
    z_scale: f32,
) -> Transform {
    let (pos, scale) = coord.clone().to_native_visual(origin_offset.clone());
    let world_pos = Vec3::new(pos.x, pos.y, pos.z);
    let marker_scale_xy = (scale * base_extent).max(1.0);
    let marker_scale_z = (marker_scale_xy * z_scale).max(1.0);

    let world_delta = (world_pos - world_rotation_origin) * world_presentation_scale;
    Transform {
        translation: world_rotation_origin + world_rotation * world_delta,
        rotation: world_rotation,
        scale: Vec3::new(marker_scale_xy, marker_scale_xy, marker_scale_z) * world_presentation_scale,
    }
}

#[inline]
fn compute_render_proxy_windowing(scale_diff: i8, chunk_center_native: Vec3, view_pos_native: Vec3) -> (RenderProxyWindowMode, Vec3, Vec3) {
    #[inline]
    fn continuous_axis_center(center_native: f64, view_native: f64, scale_diff: i8) -> f64 {
        let coarse_factor = 10.0_f64.powi(scale_diff as i32);
        if !coarse_factor.is_finite() || coarse_factor <= 0.0 {
            return 0.0;
        }
        let chunk_span = 1000.0_f64 * coarse_factor;
        if !chunk_span.is_finite() || chunk_span <= 0.0 {
            return 0.0;
        }

        let chunk_min = center_native - chunk_span * 0.5;
        ((view_native - chunk_min) / chunk_span).clamp(0.0, 1.0) - 0.5
    }

    if scale_diff <= 0 {
        // Even full-entity mode tracks local center for future nested windowing composition.
        let center = Vec3::new(
            continuous_axis_center(chunk_center_native.x as f64, view_pos_native.x as f64, scale_diff) as f32,
            continuous_axis_center(chunk_center_native.y as f64, view_pos_native.y as f64, scale_diff) as f32,
            continuous_axis_center(chunk_center_native.z as f64, view_pos_native.z as f64, scale_diff) as f32,
        );
        return (RenderProxyWindowMode::FullEntity, center, Vec3::ONE);
    }

    #[inline]
    fn quantized_axis_center_and_size(center_native: f64, view_native: f64, scale_diff: i8) -> (f64, f64) {
        let coarse_factor = 10.0_f64.powi(scale_diff as i32);
        if !coarse_factor.is_finite() || coarse_factor <= 0.0 {
            return (0.5, MIN_WINDOW_SIZE_LOCAL as f64);
        }
        let chunk_span = 1000.0_f64 * coarse_factor;
        if !chunk_span.is_finite() || chunk_span <= 0.0 {
            return (0.5, MIN_WINDOW_SIZE_LOCAL as f64);
        }

        let chunk_min = center_native - chunk_span * 0.5;
        let mut normalized = ((view_native - chunk_min) / chunk_span).clamp(0.0, 1.0 - f64::EPSILON);
        let mut min = 0.0_f64;
        let mut size = 1.0_f64;
        for _ in 0..scale_diff {
            let scaled = normalized * 10.0;
            let digit = scaled.floor().clamp(0.0, 9.0);
            size *= 0.1;
            min += digit * size;
            normalized = (scaled - digit).clamp(0.0, 1.0 - f64::EPSILON);
        }
        let center = (min + size * 0.5).clamp(0.0, 1.0);
        (center, size.max(MIN_WINDOW_SIZE_LOCAL as f64))
    }

    let (center_x, size_x) = quantized_axis_center_and_size(chunk_center_native.x as f64, view_pos_native.x as f64, scale_diff);
    let (center_y, size_y) = quantized_axis_center_and_size(chunk_center_native.y as f64, view_pos_native.y as f64, scale_diff);
    let (center_z, size_z) = quantized_axis_center_and_size(chunk_center_native.z as f64, view_pos_native.z as f64, scale_diff);
    (
        RenderProxyWindowMode::WindowedSubsection,
        Vec3::new(center_x as f32, center_y as f32, center_z as f32) - Vec3::splat(0.5),
        Vec3::new(size_x as f32, size_y as f32, size_z as f32),
    )
}

#[tracing::instrument(skip_all)]
pub(super) fn enforce_main_camera_depth_contract_system(
    mut main_camera_query: Query<(&mut Transform, &mut Projection), (With<MainCamera>, Without<Player>)>,
    player_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    player_camera_mode: Res<PlayerCameraMode>,
    player_camera_rig_settings: Res<PlayerCameraRigSettings>,
) {
    let Ok((mut camera_transform, mut projection)) = main_camera_query.single_mut() else {
        return;
    };

    let first_person_camera_height = player_camera_rig_settings.first_person_camera_height.max(1.0);

    if matches!(*player_camera_mode, PlayerCameraMode::FirstPerson) {
        camera_transform.translation.z = player_query
            .single()
            .map(|player_transform| player_transform.translation.z + first_person_camera_height)
            .unwrap_or(Scale::CANONICAL_CAMERA_Z);
    }

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
    mut scroll_message_reader: MessageReader<MouseWheel>,
    keys: Res<ButtonInput<KeyCode>>,
    input_mode: Res<State<InputMode>>,
    virtual_paused: Res<VirtualPaused>,
    chunk_load_gate: Option<Res<ChunkLoadGate>>,
    mut player_loader_query: Query<(Entity, &mut ChunkLoader, &Transform), With<Player>>,
    rapier_context: ReadRapierContext,
    mut dev_zoom_factor: ResMut<DevZoomFactor>,
    mut zoom_initialized: Local<bool>,
) {
    const ZOOM_SCROLL_DEADZONE: f32 = 0.05;

    let base_zoom_speed = CONFIG().get::<f32>("camera/base_zoom_speed").abs();
    let min_dev_zoom = CONFIG().get::<f32>("camera/min_dev_zoom").max(f32::EPSILON);
    let max_dev_zoom = CONFIG().get::<f32>("camera/max_dev_zoom").max(min_dev_zoom * 1.001);
    let dev_zoom_speed = CONFIG().get::<f32>("camera/dev_zoom_speed").abs();
    let chunk_load_gate_enabled = CONFIG().get::<bool>("workflow/chunk_load_gate_enabled");

    let Ok((player_entity, mut chunk_loader, player_transform)) = player_loader_query.single_mut() else {
        scroll_message_reader.clear();
        return;
    };
    let rapier_context = rapier_context.single().ok();
    let capsule_radius = CONFIG().get::<f32>("player/capsule_radius").max(1.0);
    let capsule_half_height = CONFIG().get::<f32>("player/capsule_half_height").max(capsule_radius);
    let scale_policy = chunk_loader.usf_transform.scale.policy;
    let commit_min_zoom = (scale_policy.commit_min() as f32).max(f32::EPSILON);
    let commit_max_zoom = (scale_policy.commit_max() as f32).max(commit_min_zoom * 1.001);
    // Input bounds intentionally straddle commit bounds to avoid f32/f64 boundary misses.
    let (input_local_zoom_min, input_local_zoom_max) = zoom_input_bounds_from_commit_bounds(commit_min_zoom, commit_max_zoom);
    let configured_default_zoom = CONFIG().get::<f32>("camera/default_zoom").max(f32::MIN_POSITIVE);

    if !*zoom_initialized {
        let startup_zoom = configured_default_zoom.clamp(input_local_zoom_min, input_local_zoom_max);
        chunk_loader.usf_transform.scale.set_local(startup_zoom as f64);
        *zoom_initialized = true;
    }

    if !input_mode.is_game() || virtual_paused.0 || (chunk_load_gate_enabled && chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked())) {
        scroll_message_reader.clear();
        return;
    }

    let shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let base_zoom_step = (1.0 + base_zoom_speed * 0.01).max(1.001);
    let dev_zoom_step = (1.0 + dev_zoom_speed * 0.01).max(1.001);
    let mut local_zoom = chunk_loader.usf_transform.scale.local_f32();
    let mut physics_blocked = false;

    for message in scroll_message_reader.read() {
        let scroll_delta = match message.unit {
            MouseScrollUnit::Line => -message.y,
            MouseScrollUnit::Pixel => message.y * -0.01,
        };
        if scroll_delta.abs() < ZOOM_SCROLL_DEADZONE {
            continue;
        }
        if shift_pressed {
            let zoom_multiplier = dev_zoom_step.powf(scroll_delta.clamp(-6.0, 6.0));
            dev_zoom_factor.0 = (dev_zoom_factor.0 * zoom_multiplier).clamp(min_dev_zoom, max_dev_zoom);
        } else {
            let zoom_multiplier = base_zoom_step.powf(scroll_delta.clamp(-6.0, 6.0));
            let candidate_zoom = (local_zoom * zoom_multiplier).clamp(input_local_zoom_min, input_local_zoom_max);
            if zoom_candidate_respects_physics(
                rapier_context.as_ref(),
                player_transform.translation,
                player_entity,
                capsule_half_height,
                capsule_radius,
                local_zoom,
                candidate_zoom,
            ) {
                local_zoom = candidate_zoom;
            } else {
                physics_blocked = true;
            }
        }
    }
    chunk_loader.usf_transform.scale.set_local(local_zoom as f64);
    if physics_blocked {
        info!("Zoom blocked by physics overlap guard.");
    }
}

#[inline]
fn world_space_planar_delta_from_intent(intent_translation_delta: Vec3, yaw_radians: f32) -> Vec3 {
    let local_planar = Vec3::new(intent_translation_delta.x, intent_translation_delta.y, 0.0);
    if local_planar.length_squared() <= f32::EPSILON && intent_translation_delta.z.abs() <= f32::EPSILON {
        return Vec3::ZERO;
    }

    let yaw_rotation = Quat::from_rotation_z(yaw_radians);
    let mut world_planar = yaw_rotation * local_planar;
    world_planar.z = intent_translation_delta.z;
    world_planar
}

#[inline]
fn zoom_input_bounds_from_commit_bounds(commit_min_zoom: f32, commit_max_zoom: f32) -> (f32, f32) {
    let lower_epsilon = commit_min_zoom.abs().max(1.0) * 1e-4;
    let upper_epsilon = commit_max_zoom.abs().max(1.0) * 1e-4;
    let input_min = (commit_min_zoom - lower_epsilon).max(f32::MIN_POSITIVE);
    let input_max = (commit_max_zoom + upper_epsilon).max(input_min * 1.001);
    (input_min, input_max)
}

#[inline]
fn zoom_candidate_respects_physics(
    rapier_context: Option<&crate::bevy_rapier3d::prelude::RapierContext<'_>>,
    player_translation: Vec3,
    player_entity: Entity,
    capsule_half_height: f32,
    capsule_radius: f32,
    current_local_zoom: f32,
    candidate_local_zoom: f32,
) -> bool {
    let current = current_local_zoom.max(f32::MIN_POSITIVE);
    let candidate = candidate_local_zoom.max(f32::MIN_POSITIVE);
    if candidate <= current {
        // Zoom-in shrinks the effective player footprint relative to world presentation.
        return true;
    }

    let Some(rapier_context) = rapier_context else {
        return true;
    };

    let relative_player_scale = (candidate / current).max(1.0);
    let probe_half_height = (capsule_half_height * relative_player_scale) as f32;
    let probe_radius = (capsule_radius * relative_player_scale) as f32;
    let probe_capsule = RapierCapsule::new_z(probe_half_height, probe_radius);

    let filter = RapierQueryFilter::new().exclude_sensors().exclude_collider(player_entity);

    let mut intersects_world = false;
    rapier_context.intersect_shape(player_translation, Quat::IDENTITY, &probe_capsule, filter, |_entity| {
        intersects_world = true;
        false
    });
    !intersects_world
}

fn clamp_local_zoom_accumulator(local_zoom: f32, local_min: f32, local_max: f32) -> f32 {
    let min = local_min.max(f32::MIN_POSITIVE);
    let max = local_max.max(min * 1.001);
    let max_exclusive = (max - max.abs().max(1.0) * 1e-6).max(min * 1.001);
    local_zoom.clamp(min, max_exclusive)
}

#[tracing::instrument(skip_all)]
pub(super) fn apply_usf_player_pivots_system(
    mut projection_query: Query<&mut Projection, With<MainCamera>>,
    mut player_loader_query: Query<(&mut ChunkLoader, &mut ChunkActor, &mut Transform), With<Player>>,
    mut chunk_load_gate: Option<ResMut<ChunkLoadGate>>,
    workflow_state: Option<Res<ChunkActionWorkflowState>>,
    mut player_motion_intent: ResMut<PlayerMotionIntent>,
    player_camera_mode: Res<PlayerCameraMode>,
    player_control_settings: Res<PlayerControlSettings>,
) {
    let Ok((mut chunk_loader, mut chunk_actor, mut player_transform)) = player_loader_query.single_mut() else {
        player_motion_intent.clear();
        return;
    };

    let intent_translation_delta = player_motion_intent.translation_delta;
    let intent_rotation_delta = player_motion_intent.rotation_delta;
    player_motion_intent.clear();
    let yaw_radians = chunk_loader.usf_transform.rotation.z.local as f32;
    let world_space_translation_delta = world_space_planar_delta_from_intent(intent_translation_delta, yaw_radians);

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
    let rotation_policy = chunk_loader.usf_transform.rotation.policy;
    let rotation_local_min = rotation_policy.local_min;
    let rotation_local_max = rotation_policy.local_max;
    let mut local_zoom = chunk_loader.usf_transform.scale.local_f32();
    let workflow_in_flight = chunk_load_gate_enabled && workflow_state.as_ref().is_some_and(|state| !state.is_idle());

    if gate_locked {
        // Hard freeze mode: do not process additional pivot transitions while input is locked.
        local_zoom = clamp_local_zoom_accumulator(local_zoom, local_min, local_max);
        chunk_loader.usf_transform.scale.set_local(local_zoom as f64);
        player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
        player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
        player_transform.translation.z = player_transform.translation.z.clamp(translation_local_min, translation_local_max);
        chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
        chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
        chunk_loader.usf_transform.translation.z.set_local(player_transform.translation.z as f64);
        chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
        chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
        chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
    } else {
        let candidate_translation = player_transform.translation + world_space_translation_delta;

        let can_cross_lower_scale_boundary = chunk_loader.scale != Scale::MIN;
        let can_cross_upper_scale_boundary = chunk_loader.scale != Scale::MAX;
        let would_cross_scale_boundary =
            (can_cross_lower_scale_boundary && local_zoom <= scale_commit_min) || (can_cross_upper_scale_boundary && local_zoom >= scale_commit_max);
        let would_cross_translation_boundary = candidate_translation.x < translation_local_min
            || candidate_translation.x >= translation_local_max
            || candidate_translation.y < translation_local_min
            || candidate_translation.y >= translation_local_max
            || candidate_translation.z < translation_local_min
            || candidate_translation.z >= translation_local_max;

        if workflow_in_flight && (would_cross_scale_boundary || would_cross_translation_boundary) {
            if let Some(gate) = chunk_load_gate.as_mut() {
                let changed = gate.lock_by_in_flight_boundary();
                if changed {
                    warn!("ChunkLoadGate preemptively locked due to boundary crossing attempt while chunk workflow is in flight");
                }
            }

            // Reject boundary commit while a previous boundary batch is still in flight.
            local_zoom = clamp_local_zoom_accumulator(local_zoom, local_min, local_max);
            chunk_loader.usf_transform.scale.set_local(local_zoom as f64);
            player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
            player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
            player_transform.translation.z = player_transform.translation.z.clamp(translation_local_min, translation_local_max);
            chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
            chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
            chunk_loader.usf_transform.translation.z.set_local(player_transform.translation.z as f64);
            chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
            chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
            chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
        } else {
            if world_space_translation_delta != Vec3::ZERO {
                player_transform.translation += world_space_translation_delta;
            }
            if intent_rotation_delta != Vec3::ZERO {
                chunk_loader.rotate_world_local(intent_rotation_delta);
            }

            let (scale_pivot, translation_grid_delta) = chunk_loader.apply_player_anchor_pivots(&mut local_zoom, &mut player_transform.translation);
            chunk_loader.usf_transform.scale.set_local(local_zoom as f64);
            chunk_actor.coord = chunk_loader.coord.clone();

            let zoom_in_events = scale_pivot.lower_crossings.max(0) as u32;
            let zoom_out_events = scale_pivot.upper_crossings.max(0) as u32;
            let boundary_crossed = zoom_in_events > 0 || zoom_out_events > 0 || translation_grid_delta != IVec3::ZERO;
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
                local_zoom = clamp_local_zoom_accumulator(local_zoom, local_min, local_max);
                chunk_loader.usf_transform.scale.set_local(local_zoom as f64);
                player_transform.translation.x = player_transform.translation.x.clamp(translation_local_min, translation_local_max);
                player_transform.translation.y = player_transform.translation.y.clamp(translation_local_min, translation_local_max);
                player_transform.translation.z = player_transform.translation.z.clamp(translation_local_min, translation_local_max);
                chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
                chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
                chunk_loader.usf_transform.translation.z.set_local(player_transform.translation.z as f64);
                chunk_loader.usf_transform.rotation.x.local = chunk_loader.usf_transform.rotation.x.local.clamp(rotation_local_min, rotation_local_max);
                chunk_loader.usf_transform.rotation.y.local = chunk_loader.usf_transform.rotation.y.local.clamp(rotation_local_min, rotation_local_max);
                chunk_loader.usf_transform.rotation.z.local = chunk_loader.usf_transform.rotation.z.local.clamp(rotation_local_min, rotation_local_max);
            }

            if boundary_crossed {
                warn!(
                    "USF player pivot event: scale={:?}, zoom={:.6}, scale_rebase_events(in={},out={}), translation_grid_delta={:?}, player_pos={:?}",
                    chunk_loader.scale, local_zoom, zoom_in_events, zoom_out_events, translation_grid_delta, player_transform.translation
                );
            }
        }
    }

    chunk_loader.usf_transform.translation.x.set_local(player_transform.translation.x as f64);
    chunk_loader.usf_transform.translation.y.set_local(player_transform.translation.y as f64);
    chunk_loader.usf_transform.translation.z.set_local(player_transform.translation.z as f64);

    // Keep player visual scale stable; zoom authority comes from world/scale transitions.
    player_transform.scale = Vec3::ONE;
    let first_person_fov = player_control_settings.first_person_fov_degrees.to_radians().clamp(0.35, 3.12);
    let third_person_fov = CONFIG().get::<f32>("camera/default_fov_degrees").to_radians().clamp(0.35, 3.12);

    for mut projection in projection_query.iter_mut() {
        if let Projection::Perspective(perspective) = projection.as_mut() {
            perspective.fov = match *player_camera_mode {
                PlayerCameraMode::FirstPerson => first_person_fov,
                PlayerCameraMode::ThirdPerson => third_person_fov,
            };
        }
    }
}

#[inline]
#[cfg(test)]
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
pub(super) fn update_view_scale_from_zoom(player_loader_query: Query<&ChunkLoader, With<Player>>, mut view_scale: ResMut<ViewScale>) {
    let Ok(chunk_loader) = player_loader_query.single() else {
        view_scale.discrete = 0;
        view_scale.offset = 0.0;
        return;
    };

    // Authoritative scale state comes from USF/ChunkLoader, not projection zoom math.
    view_scale.discrete = chunk_loader.scale.index_from_top() as i32;

    // Keep only local intra-scale zoom as a fractional hint in [-1, 1].
    let local_zoom = player_local_zoom_for_presentation(chunk_loader);
    view_scale.offset = (-local_zoom.log10()).clamp(-1.0, 1.0);
}
