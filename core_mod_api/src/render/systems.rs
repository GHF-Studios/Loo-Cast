use crate::bevy::camera::RenderTarget;
use crate::bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{Extent3d, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

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
    resources::{DevZoomFactor, GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState, RuntimeDebugToggles, ViewScale, ZoomFactor},
};
use crate::time::resources::VirtualPaused;
use crate::tracing::{error, info};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use std::collections::{HashMap, HashSet};

const MIN_WINDOW_SIZE_LOCAL: f32 = f32::MIN_POSITIVE;
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
fn world_presentation_origin_from_camera(
    main_camera_query: &Query<&Transform, (With<MainCamera>, Without<Player>, Without<WorldPresentationRoot>, Without<RenderProxy>)>,
    fallback: Vec3,
) -> Vec3 {
    main_camera_query.single().map(|transform| transform.translation).unwrap_or(fallback)
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
pub(super) fn update_world_presentation_root_transform_system(
    player_loader_query: Single<(&ChunkLoader, &Transform), (With<Player>, Without<WorldPresentationRoot>, Without<MainCamera>)>,
    main_camera_query: Query<&Transform, (With<MainCamera>, Without<Player>, Without<WorldPresentationRoot>, Without<RenderProxy>)>,
    root_query: Single<&mut Transform, (With<WorldPresentationRoot>, Without<Player>)>,
) {
    let (chunk_loader, player_transform) = *player_loader_query;
    let local_zoom = player_local_zoom_for_presentation(chunk_loader);
    let world_presentation_scale = world_presentation_scale_from_local_zoom(local_zoom);
    let world_presentation_origin = world_presentation_origin_from_camera(&main_camera_query, player_transform.translation);

    let mut root_transform = root_query.into_inner();
    root_transform.translation = world_presentation_origin;
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
    mut params: ParamSet<(
        Single<(&ChunkLoader, &Transform), (With<Player>, Without<MainCamera>)>,
        Query<(&EntityProxyLink, &ChunkActor), Without<RenderProxy>>,
        Query<(&mut Transform, &mut ProxySyncRevision, &mut RenderProxy), With<RenderProxy>>,
    )>,
    main_camera_query: Query<&Transform, (With<MainCamera>, Without<Player>, Without<WorldPresentationRoot>, Without<RenderProxy>)>,
) {
    let (world_rotation_origin, origin_offset, view_pos_native, player_local_zoom, world_presentation_scale) = {
        let (chunk_loader, chunk_loader_transform) = *params.p0();
        let local_zoom = player_local_zoom_for_presentation(chunk_loader);
        let presentation_scale = world_presentation_scale_from_local_zoom(local_zoom);
        let world_presentation_origin = world_presentation_origin_from_camera(&main_camera_query, chunk_loader_transform.translation);
        let view_anchor_native = chunk_loader_transform.translation;
        (
            world_presentation_origin,
            chunk_loader.origin_offset.clone(),
            view_anchor_native,
            local_zoom,
            presentation_scale,
        )
    };

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
            if let Ok((mut proxy_transform, mut proxy_revision, mut proxy_state)) = proxy_transforms.get_mut(proxy_entity) {
                if incoming_revision.0 < proxy_revision.0 {
                    continue;
                }
                let coord_scale = coord.scale;
                let scale_diff = coord_scale as i8 - origin_offset.scale as i8;
                let relative_scale_to_player = scale_diff;
                let layer_z = coord_scale.compute_z() + proxy_state.depth_bias;
                let (pos, scale) = coord.to_native_visual(origin_offset.clone());
                let world_pos = Vec3::new(pos.x, pos.y, pos.z + layer_z);
                proxy_transform.translation = world_pos - world_rotation_origin;
                proxy_transform.scale = Vec3::splat(scale);
                proxy_transform.rotation = Quat::IDENTITY;
                proxy_state.layer_index = coord_scale.render_layer_index();
                let (window_mode, window_center_local, window_size_local) = compute_render_proxy_windowing(scale_diff, pos, view_pos_native, player_local_zoom);
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
    }
}

#[tracing::instrument(skip_all)]
pub(super) fn draw_chunk_locator_gizmos_system(
    mut gizmos: Gizmos,
    player_loader_query: Single<(&ChunkLoader, &Transform), (With<Player>, Without<MainCamera>)>,
    main_camera_query: Query<&Transform, (With<MainCamera>, Without<Player>, Without<WorldPresentationRoot>, Without<RenderProxy>)>,
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
    let world_rotation_origin = world_presentation_origin_from_camera(&main_camera_query, player_transform.translation);
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
    let layer_z = coord.scale.compute_z();
    let world_pos = Vec3::new(pos.x, pos.y, pos.z + layer_z);
    let marker_scale_xy = (scale * base_extent).max(1.0);
    let marker_scale_z = (marker_scale_xy * z_scale).max(1.0);

    let world_delta = (world_pos - world_rotation_origin) * world_presentation_scale;
    Transform {
        translation: world_rotation_origin + world_rotation * world_delta,
        rotation: world_rotation,
        scale: Vec3::new(marker_scale_xy, marker_scale_xy, marker_scale_z) * world_presentation_scale,
    }
}

#[cfg(any())]
mod legacy_phenomenon_surface {
    use super::*;

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct PhenomenonModelWindowBounds {
        min: Vec3,
        max: Vec3,
        span: Vec3,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct SurfaceLodTuning {
        visibility_threshold: f32,
        scale_boost: f32,
        z_displacement: f32,
        mesh_resolution: u32,
        iso_level: f32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MandelbulbTuning {
        power: f32,
        iterations: u32,
        bailout: f32,
        z_span: f32,
        lod: SurfaceLodTuning,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct SierpinskiSpongeTuning {
        iterations: u32,
        domain_span: f32,
        hole_bias: f32,
        lod: SurfaceLodTuning,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum PhenomenonGeometryModel {
        Mandelbulb(MandelbulbTuning),
        SierpinskiSponge(SierpinskiSpongeTuning),
    }

    impl PhenomenonGeometryModel {
        fn from_kind(kind: PhenomenonKind) -> Option<Self> {
            match kind {
                PhenomenonKind::Mandelbulb => load_mandelbulb_tuning().map(Self::Mandelbulb),
                PhenomenonKind::SierpinskiSponge => load_sierpinski_sponge_tuning().map(Self::SierpinskiSponge),
            }
        }

        fn lod_tuning(self) -> SurfaceLodTuning {
            match self {
                Self::Mandelbulb(tuning) => tuning.lod,
                Self::SierpinskiSponge(tuning) => tuning.lod,
            }
        }

        fn surface_transform(self, proxy: &RenderProxy) -> Transform {
            phenomenon_surface_transform(proxy, self.lod_tuning())
        }

        fn update_surface_material(self, surface_material: &mut PhenomenonSurfaceMaterial, proxy: &RenderProxy, time_seconds: f32) {
            let lod = self.lod_tuning();
            let layer_t = layer_norm(proxy.layer_index);
            let window_scale = proxy_window_scale(proxy);
            let shimmer_speed = match self {
                Self::Mandelbulb(_) => 0.22,
                Self::SierpinskiSponge(_) => 0.35,
            };
            let shimmer = (time_seconds * shimmer_speed + layer_t * std::f32::consts::TAU).sin() * 0.5 + 0.5;

            let (primary_a, primary_b, secondary_a, secondary_b, glow_a, glow_b) = match self {
                Self::Mandelbulb(_) => (
                    Vec3::new(0.14, 0.48, 0.95),
                    Vec3::new(0.22, 0.74, 0.98),
                    Vec3::new(0.98, 0.58, 0.28),
                    Vec3::new(0.96, 0.84, 0.52),
                    Vec3::new(0.26, 0.86, 1.0),
                    Vec3::new(0.58, 1.0, 0.78),
                ),
                Self::SierpinskiSponge(_) => (
                    Vec3::new(0.75, 0.86, 0.97),
                    Vec3::new(0.62, 0.82, 0.96),
                    Vec3::new(0.34, 0.48, 0.7),
                    Vec3::new(0.42, 0.64, 0.82),
                    Vec3::new(0.84, 0.98, 1.0),
                    Vec3::new(0.66, 0.88, 0.96),
                ),
            };

            let primary = primary_a.lerp(primary_b, layer_t);
            let secondary = secondary_a.lerp(secondary_b, 1.0 - layer_t);
            let glow = glow_a.lerp(glow_b, shimmer);
            let emissive_strength = (0.25 + lod.visibility_threshold * 1.2).clamp(0.0, 2.0);

            surface_material.params.primary = primary.extend(1.0);
            surface_material.params.secondary = secondary.extend(1.0);
            surface_material.params.glow = glow.extend(1.0);
            surface_material.params.meta = Vec4::new(layer_t, window_scale, time_seconds, emissive_strength);
        }

        fn surface_signature(self, proxy: &RenderProxy) -> u64 {
            match self {
                Self::Mandelbulb(tuning) => compute_mandelbulb_surface_signature(proxy, tuning),
                Self::SierpinskiSponge(tuning) => compute_sierpinski_sponge_surface_signature(proxy, tuning),
            }
        }

        fn build_mesh(self, proxy: &RenderProxy, frontier_state: Option<&PhenomenonStateSnapshot>, generator: &LayerEchoGenerator) -> Option<Mesh> {
            let snapshot = frontier_state?;
            build_windowed_generator_mesh(proxy, self.lod_tuning(), snapshot, generator)
        }
    }

    fn load_surface_lod_tuning(root_key: &str) -> SurfaceLodTuning {
        SurfaceLodTuning {
            visibility_threshold: CONFIG().get::<f32>(&format!("{root_key}/visibility_threshold")).clamp(0.0, 1.0),
            scale_boost: CONFIG().get::<f32>(&format!("{root_key}/scale_boost")).max(0.0),
            z_displacement: CONFIG().get::<f32>(&format!("{root_key}/z_displacement")),
            mesh_resolution: CONFIG().get::<u32>(&format!("{root_key}/mesh_resolution")).clamp(6, 64),
            iso_level: CONFIG().get::<f32>(&format!("{root_key}/iso_level")).clamp(-1.0, 1.0),
        }
    }

    fn load_mandelbulb_tuning() -> Option<MandelbulbTuning> {
        if !CONFIG().get::<bool>("render/phenomenon_mandelbulb/enabled") {
            return None;
        }

        Some(MandelbulbTuning {
            power: CONFIG().get::<f32>("render/phenomenon_mandelbulb/power").max(1.1),
            iterations: CONFIG().get::<u32>("render/phenomenon_mandelbulb/iterations").max(1),
            bailout: CONFIG().get::<f32>("render/phenomenon_mandelbulb/bailout").max(1.1),
            z_span: CONFIG().get::<f32>("render/phenomenon_mandelbulb/z_span").abs().max(0.01),
            lod: load_surface_lod_tuning("render/phenomenon_mandelbulb"),
        })
    }

    fn load_sierpinski_sponge_tuning() -> Option<SierpinskiSpongeTuning> {
        if !CONFIG().get::<bool>("render/phenomenon_sierpinski_sponge/enabled") {
            return None;
        }

        Some(SierpinskiSpongeTuning {
            iterations: CONFIG().get::<u32>("render/phenomenon_sierpinski_sponge/iterations").clamp(1, 7),
            // Keep a minimum outer margin so the enclosing cube shell is sampled with outside context
            // and does not get clipped by the meshing volume boundary.
            domain_span: CONFIG().get::<f32>("render/phenomenon_sierpinski_sponge/domain_span").abs().max(1.05),
            hole_bias: CONFIG().get::<f32>("render/phenomenon_sierpinski_sponge/hole_bias").clamp(-0.2, 0.2),
            lod: load_surface_lod_tuning("render/phenomenon_sierpinski_sponge"),
        })
    }

    #[tracing::instrument(skip_all)]
    pub(super) fn update_phenomenon_model_surfaces_system(
        time: Res<Time>,
        mut meshes: ResMut<Assets<Mesh>>,
        mut surface_materials: ResMut<Assets<PhenomenonSurfaceMaterial>>,
        meshing_budget: Res<PhenomenonSurfaceMeshingBudget>,
        mut mesh_cache: ResMut<PhenomenonSurfaceMeshCache>,
        mut phenomenon_stats: Option<ResMut<PhenomenonDebugStats>>,
        generator_state: Res<PhenomenonGeneratorState>,
        proxy_query: Query<(&Children, &RenderProxy, &PhenomenonModel), With<PhenomenonZoneProxy>>,
        phenomenon_node_state_query: Query<(&PhenomenonNode, &PhenomenonNodeState)>,
        phenomenon_query: Query<&Phenomenon>,
        mut surface_query: Query<(
            &mut Mesh3d,
            &MeshMaterial3d<PhenomenonSurfaceMaterial>,
            &mut Transform,
            &mut Visibility,
            &mut PhenomenonModelSurface,
        )>,
    ) {
        let snapshot_cache = phenomenon_node_state_query
            .iter()
            .map(|(node, node_state)| ((node.phenomenon_id.0, node.seed.0), node_state.snapshot.clone()))
            .collect::<HashMap<_, _>>();

        if let Some(stats) = phenomenon_stats.as_mut() {
            stats.active_frontier_proxies = proxy_query.iter().count() as u32;
            stats.generated_meshes_frame = 0;
            stats.mesh_cache_hits_frame = 0;
        }
        let mut remaining_build_budget = meshing_budget.max_builds_per_frame;

        for (children, proxy, phenomenon_model) in proxy_query.iter() {
            let phenomenon = phenomenon_query.get(phenomenon_model.phenomenon_entity).ok();
            let model = phenomenon.and_then(|phenomenon| PhenomenonGeometryModel::from_kind(phenomenon.kind));
            let phenomenon_id = phenomenon.map(|phenomenon| phenomenon.id.0);

            for child in children.iter() {
                let Ok((mut mesh3d, material3d, mut transform, mut visibility, mut surface_state)) = surface_query.get_mut(child) else {
                    continue;
                };

                let Some(model) = model else {
                    surface_state.last_signature = 0;
                    *visibility = Visibility::Hidden;
                    continue;
                };

                *transform = model.surface_transform(proxy);
                if let Some(surface_material) = surface_materials.get_mut(&material3d.0) {
                    model.update_surface_material(surface_material, proxy, time.elapsed_secs());
                }

                let signature = model.surface_signature(proxy);
                if signature != surface_state.last_signature {
                    if let Some(cached_mesh) = mesh_cache.get(signature) {
                        surface_state.last_signature = signature;
                        mesh3d.0 = cached_mesh;
                        *visibility = Visibility::Visible;
                        if let Some(stats) = phenomenon_stats.as_mut() {
                            stats.mesh_cache_hits_total = stats.mesh_cache_hits_total.saturating_add(1);
                            stats.mesh_cache_hits_frame = stats.mesh_cache_hits_frame.saturating_add(1);
                        }
                        continue;
                    }
                    if remaining_build_budget == 0 {
                        *visibility = Visibility::Hidden;
                        continue;
                    }
                    let frontier_state = phenomenon_id.and_then(|id| snapshot_cache.get(&(id, proxy.frontier_node_seed)));
                    if let Some(mesh) = model.build_mesh(proxy, frontier_state, &generator_state.layer_echo) {
                        surface_state.last_signature = signature;
                        remaining_build_budget = remaining_build_budget.saturating_sub(1);
                        if let Some(stats) = phenomenon_stats.as_mut() {
                            stats.generated_meshes_total = stats.generated_meshes_total.saturating_add(1);
                            stats.generated_meshes_frame = stats.generated_meshes_frame.saturating_add(1);
                        }
                        let handle = meshes.add(mesh);
                        mesh_cache.insert(signature, handle.clone());
                        mesh3d.0 = handle;
                        *visibility = Visibility::Visible;
                    } else {
                        surface_state.last_signature = signature;
                        *visibility = Visibility::Hidden;
                    }
                } else if let Some(stats) = phenomenon_stats.as_mut() {
                    stats.mesh_cache_hits_total = stats.mesh_cache_hits_total.saturating_add(1);
                    stats.mesh_cache_hits_frame = stats.mesh_cache_hits_frame.saturating_add(1);
                }
            }
        }
    }

    #[inline]
    fn layer_norm(layer_index: u8) -> f32 {
        let max_layer = (Scale::SCALE_LEVEL_COUNT.saturating_sub(1)) as f32;
        if max_layer <= f32::EPSILON {
            0.5
        } else {
            (layer_index as f32 / max_layer).clamp(0.0, 1.0)
        }
    }

    #[inline]
    fn proxy_presentation_window_scale(proxy: &RenderProxy) -> f32 {
        if proxy.relative_scale_to_player < 0 {
            return 1.0;
        }
        proxy.player_local_zoom.clamp(MIN_WINDOW_SIZE_LOCAL, 1.0)
    }

    #[inline]
    fn proxy_window_scale(proxy: &RenderProxy) -> f32 {
        let base_window_scale = proxy.window_size_local.abs().max_element().clamp(MIN_WINDOW_SIZE_LOCAL, 1.0);
        (base_window_scale * proxy_presentation_window_scale(proxy)).clamp(MIN_WINDOW_SIZE_LOCAL, 1.0)
    }

    #[inline]
    fn compute_effective_mesh_resolution(proxy: &RenderProxy, base_mesh_resolution: u32) -> usize {
        let base_resolution = base_mesh_resolution as f32;
        let window_scale = proxy_window_scale(proxy);
        // Keep detail stable across scale layers; only window size should drive dynamic tessellation.
        let window_boost = 1.0 + (1.0 - window_scale) * 1.5;
        let mut resolution = (base_resolution * window_boost).round().clamp(8.0, 40.0) as usize;
        let full_entity_unit_window = matches!(proxy.window_mode, RenderProxyWindowMode::FullEntity) && window_scale >= 0.999_999;
        if full_entity_unit_window {
            resolution = resolution.max(10);
            resolution = (resolution / 10).max(1) * 10;
        }
        resolution
    }

    #[inline]
    fn phenomenon_surface_transform(proxy: &RenderProxy, tuning: SurfaceLodTuning) -> Transform {
        let window_scale = proxy_window_scale(proxy);
        let layer_t = layer_norm(proxy.layer_index);
        let local_scale = 1.0 + (1.0 - window_scale) * tuning.scale_boost;
        let z_offset = (layer_t - 0.5) * tuning.z_displacement;

        Transform {
            translation: Vec3::new(0.0, 0.0, z_offset),
            scale: Vec3::splat(local_scale),
            ..Default::default()
        }
    }

    #[inline]
    fn compute_window_bounds_from_local(center_local: Vec3, size_local: Vec3) -> PhenomenonModelWindowBounds {
        let center01 = center_local.clamp(Vec3::splat(-0.5), Vec3::splat(0.5)) + Vec3::splat(0.5);
        let size01 = size_local.abs().clamp(Vec3::splat(MIN_WINDOW_SIZE_LOCAL), Vec3::ONE);
        let mut window_min = (center01 - size01 * 0.5).clamp(Vec3::ZERO, Vec3::ONE);
        let mut window_max = (center01 + size01 * 0.5).clamp(Vec3::ZERO, Vec3::ONE);
        if window_max.x < window_min.x {
            std::mem::swap(&mut window_max.x, &mut window_min.x);
        }
        if window_max.y < window_min.y {
            std::mem::swap(&mut window_max.y, &mut window_min.y);
        }
        if window_max.z < window_min.z {
            std::mem::swap(&mut window_max.z, &mut window_min.z);
        }
        let span = (window_max - window_min).max(Vec3::splat(MIN_WINDOW_SIZE_LOCAL));

        PhenomenonModelWindowBounds {
            min: window_min,
            max: window_max,
            span,
        }
    }

    #[inline]
    fn compose_nested_window(base_center_local: Vec3, base_size_local: Vec3, nested_center_local: Vec3, nested_size_local: Vec3) -> (Vec3, Vec3) {
        let base_bounds = compute_window_bounds_from_local(base_center_local, base_size_local);
        let nested_bounds = compute_window_bounds_from_local(nested_center_local, nested_size_local);

        let base_span = base_bounds.span.max(Vec3::splat(MIN_WINDOW_SIZE_LOCAL));
        let mut composed_min = base_bounds.min + nested_bounds.min * base_span;
        let mut composed_max = base_bounds.min + nested_bounds.max * base_span;
        composed_min = composed_min.clamp(Vec3::ZERO, Vec3::ONE);
        composed_max = composed_max.clamp(Vec3::ZERO, Vec3::ONE);
        if composed_max.x < composed_min.x {
            std::mem::swap(&mut composed_max.x, &mut composed_min.x);
        }
        if composed_max.y < composed_min.y {
            std::mem::swap(&mut composed_max.y, &mut composed_min.y);
        }
        if composed_max.z < composed_min.z {
            std::mem::swap(&mut composed_max.z, &mut composed_min.z);
        }
        let size_local = (composed_max - composed_min).max(Vec3::splat(MIN_WINDOW_SIZE_LOCAL));
        let center_local = ((composed_min + composed_max) * 0.5) - Vec3::splat(0.5);
        (center_local, size_local)
    }

    #[inline]
    fn compute_phenomenon_window_bounds(window_mode: RenderProxyWindowMode, window_center_local: Vec3, window_size_local: Vec3) -> PhenomenonModelWindowBounds {
        let clamped_size = window_size_local.abs().clamp(Vec3::splat(MIN_WINDOW_SIZE_LOCAL), Vec3::ONE);
        let size_is_unit = clamped_size.min_element() >= 0.999_999;
        if matches!(window_mode, RenderProxyWindowMode::FullEntity) && size_is_unit {
            return PhenomenonModelWindowBounds {
                min: Vec3::ZERO,
                max: Vec3::ONE,
                span: Vec3::ONE,
            };
        }

        compute_window_bounds_from_local(window_center_local, clamped_size)
    }

    #[inline]
    fn effective_lattice_cells_for_bounds(bounds: PhenomenonModelWindowBounds, requested_cells: usize) -> usize {
        let requested = requested_cells.max(1);
        let quantized = seam_safe_lattice_window(bounds.min, bounds.span, requested);
        let span_x = (quantized.max.x - quantized.min.x).max(1) as usize;
        let span_y = (quantized.max.y - quantized.min.y).max(1) as usize;
        let span_z = (quantized.max.z - quantized.min.z).max(1) as usize;
        requested.min(span_x.min(span_y).min(span_z)).max(1)
    }

    #[inline]
    fn mandelbulb_density_from_model_space(local_uvw: Vec3, layer_index: u8, tuning: MandelbulbTuning) -> f32 {
        let point = map_model_space_to_mandelbulb_point(local_uvw, layer_index, tuning.z_span);
        sample_mandelbulb_signed_distance(point, tuning.power, tuning.iterations, tuning.bailout)
    }

    #[inline]
    fn map_model_space_to_mandelbulb_point(local_uvw: Vec3, layer_index: u8, z_span: f32) -> Vec3 {
        let uvw = local_uvw.clamp(Vec3::ZERO, Vec3::ONE);
        let x = (uvw.x - 0.5) * 3.0;
        let y = (uvw.y - 0.5) * 3.0;
        let local_z = (uvw.z - 0.5) * 2.0 * z_span;

        // Keep one coherent global fractal across all scales; do not offset the sampled slice per layer.
        let _ = layer_index;
        let layer_bias = 0.0;

        Vec3::new(x, y, local_z + layer_bias)
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct MeshingWindow {
        center_local: Vec3,
        size_local: Vec3,
        resolution: usize,
    }

    #[inline]
    fn compute_meshing_window(proxy: &RenderProxy, base_mesh_resolution: u32) -> MeshingWindow {
        let resolution = compute_effective_mesh_resolution(proxy, base_mesh_resolution);
        let presentation_window_scale = proxy_presentation_window_scale(proxy);

        let base_center_local = proxy.window_center_local.clamp(Vec3::splat(-0.5), Vec3::splat(0.5));
        let base_size_local = if matches!(proxy.window_mode, RenderProxyWindowMode::FullEntity) {
            Vec3::ONE
        } else {
            proxy.window_size_local.abs().clamp(Vec3::splat(MIN_WINDOW_SIZE_LOCAL), Vec3::ONE)
        };

        if matches!(proxy.window_mode, RenderProxyWindowMode::FullEntity) && presentation_window_scale >= 0.999_999 {
            return MeshingWindow {
                center_local: Vec3::ZERO,
                size_local: Vec3::ONE,
                resolution,
            };
        }

        let (mut center_local, size_local) = if presentation_window_scale < 0.999_999 {
            compose_nested_window(
                base_center_local,
                base_size_local,
                proxy.window_center_local,
                Vec3::splat(presentation_window_scale),
            )
        } else {
            (base_center_local, base_size_local)
        };
        let step_local = (size_local / resolution as f32).max(Vec3::splat(MIN_WINDOW_SIZE_LOCAL));

        // Snap subsection center to marching-grid voxels, so tiny camera deltas don't trigger full remesh every frame.
        center_local = (center_local / step_local).round() * step_local;
        let min_center = Vec3::splat(-0.5) + size_local * 0.5;
        let max_center = Vec3::splat(0.5) - size_local * 0.5;
        center_local = center_local.clamp(min_center, max_center);

        MeshingWindow {
            center_local,
            size_local,
            resolution,
        }
    }

    #[inline]
    fn sample_mandelbulb_signed_distance(c: Vec3, power: f32, iterations: u32, bailout: f32) -> f32 {
        let mut z = c;
        let mut dr = 1.0f32;
        let mut r = z.length();
        let mut escaped = false;

        for _ in 0..iterations {
            r = z.length();
            if r > bailout {
                escaped = true;
                break;
            }

            let safe_r = r.max(1e-6);
            let theta = (z.z / safe_r).clamp(-1.0, 1.0).acos();
            let phi = z.y.atan2(z.x);
            let zr = safe_r.powf(power);
            let theta_p = theta * power;
            let phi_p = phi * power;
            dr = safe_r.powf(power - 1.0) * power * dr + 1.0;

            z = Vec3::new(theta_p.sin() * phi_p.cos(), theta_p.sin() * phi_p.sin(), theta_p.cos()) * zr + c;
        }

        if escaped {
            let safe_r = r.max(1e-6);
            let safe_dr = dr.abs().max(1e-6);
            0.5 * safe_r.ln() * safe_r / safe_dr
        } else {
            // Interior points never escaped within the iteration budget; keep them strictly negative.
            let interior_depth = ((bailout - r).max(0.0) / bailout.max(1e-6)).clamp(0.0, 1.0);
            -0.001 - interior_depth * 0.5
        }
    }

    #[cfg(test)]
    #[inline]
    fn sample_sierpinski_sponge_signed_distance(point: Vec3, iterations: u32, hole_bias: f32) -> f32 {
        #[inline]
        fn sd_box(p: Vec3, half_extents: Vec3) -> f32 {
            let q = p.abs() - half_extents;
            q.max(Vec3::ZERO).length() + q.max_element().min(0.0)
        }

        let mut distance = sd_box(point, Vec3::ONE);
        let mut scale = 1.0f32;
        let hole_adjust = hole_bias.clamp(-0.2, 0.2) * 0.75;

        // Menger-style recursive cross cutouts; stable SDF-ish estimator for marching isosurface.
        for _ in 0..iterations {
            let p = point * scale;
            let cell = Vec3::new(p.x.rem_euclid(2.0) - 1.0, p.y.rem_euclid(2.0) - 1.0, p.z.rem_euclid(2.0) - 1.0);
            scale *= 3.0;

            let r = (Vec3::ONE - cell.abs() * 3.0).abs();
            let da = r.x.max(r.y);
            let db = r.y.max(r.z);
            let dc = r.z.max(r.x);
            let cross_cut = (da.min(db).min(dc) - (1.0 + hole_adjust)) / scale;
            distance = distance.max(cross_cut);
        }

        distance
    }

    #[inline]
    fn quantized_signature_value(value: f32) -> i32 {
        (value * 10_000.0).round() as i32
    }

    #[inline]
    fn compute_mandelbulb_surface_signature(proxy: &RenderProxy, tuning: MandelbulbTuning) -> u64 {
        compute_model_surface_signature(proxy, PhenomenonKind::Mandelbulb, tuning.lod, |hasher| {
            quantized_signature_value(tuning.power).hash(hasher);
            tuning.iterations.hash(hasher);
            quantized_signature_value(tuning.bailout).hash(hasher);
            quantized_signature_value(tuning.z_span).hash(hasher);
        })
    }

    #[inline]
    fn compute_sierpinski_sponge_surface_signature(proxy: &RenderProxy, tuning: SierpinskiSpongeTuning) -> u64 {
        compute_model_surface_signature(proxy, PhenomenonKind::SierpinskiSponge, tuning.lod, |hasher| {
            tuning.iterations.hash(hasher);
            quantized_signature_value(tuning.domain_span).hash(hasher);
            quantized_signature_value(tuning.hole_bias).hash(hasher);
        })
    }

    #[inline]
    fn compute_model_surface_signature<F>(proxy: &RenderProxy, kind: PhenomenonKind, lod: SurfaceLodTuning, mut hash_model: F) -> u64
    where
        F: FnMut(&mut std::collections::hash_map::DefaultHasher),
    {
        let meshing_window = compute_meshing_window(proxy, lod.mesh_resolution);
        let bounds = compute_phenomenon_window_bounds(proxy.window_mode, meshing_window.center_local, meshing_window.size_local);
        let effective_cells = effective_lattice_cells_for_bounds(bounds, meshing_window.resolution);
        let lattice_window = seam_safe_lattice_window(bounds.min, bounds.span, effective_cells);
        #[inline]
        fn hash_lod_fields(hasher: &mut std::collections::hash_map::DefaultHasher, lod: SurfaceLodTuning) {
            quantized_signature_value(lod.iso_level).hash(hasher);
            quantized_signature_value(lod.visibility_threshold).hash(hasher);
            quantized_signature_value(lod.scale_boost).hash(hasher);
            quantized_signature_value(lod.z_displacement).hash(hasher);
            lod.mesh_resolution.hash(hasher);
        }

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        kind.hash(&mut hasher);
        proxy.source.hash(&mut hasher);
        proxy.layer_index.hash(&mut hasher);
        proxy.frontier_node_seed.hash(&mut hasher);
        proxy.frontier_lineage_depth.hash(&mut hasher);
        proxy.window_mode.hash(&mut hasher);
        quantized_signature_value(meshing_window.center_local.x).hash(&mut hasher);
        quantized_signature_value(meshing_window.center_local.y).hash(&mut hasher);
        quantized_signature_value(meshing_window.center_local.z).hash(&mut hasher);
        quantized_signature_value(meshing_window.size_local.x).hash(&mut hasher);
        quantized_signature_value(meshing_window.size_local.y).hash(&mut hasher);
        quantized_signature_value(meshing_window.size_local.z).hash(&mut hasher);
        effective_cells.hash(&mut hasher);
        lattice_window.min.hash(&mut hasher);
        lattice_window.max.hash(&mut hasher);
        lattice_window.cells.hash(&mut hasher);
        hash_lod_fields(&mut hasher, lod);
        hash_model(&mut hasher);
        hasher.finish()
    }

    #[inline]
    fn grid_index(ix: usize, iy: usize, iz: usize, axis_points: usize) -> usize {
        ix + iy * axis_points + iz * axis_points * axis_points
    }

    fn build_windowed_mandelbulb_mesh(proxy: &RenderProxy, tuning: MandelbulbTuning) -> Option<Mesh> {
        build_windowed_field_mesh(proxy, tuning.lod, |sample_uvw, layer_index| {
            mandelbulb_density_from_model_space(sample_uvw, layer_index, tuning)
        })
    }

    fn build_windowed_generator_mesh(
        proxy: &RenderProxy,
        lod: SurfaceLodTuning,
        snapshot: &PhenomenonStateSnapshot,
        generator: &LayerEchoGenerator,
    ) -> Option<Mesh> {
        build_windowed_field_mesh(proxy, lod, |sample_uvw, _layer_index| {
            let point_local = (sample_uvw - Vec3::splat(0.5)) * 2.0;
            generator.sample_density(snapshot, point_local)
        })
    }

    fn build_windowed_sierpinski_sponge_mesh(proxy: &RenderProxy, tuning: SierpinskiSpongeTuning) -> Option<Mesh> {
        let meshing_window = compute_meshing_window(proxy, tuning.lod.mesh_resolution);
        let bounds = compute_phenomenon_window_bounds(proxy.window_mode, meshing_window.center_local, meshing_window.size_local);
        let effective_iterations = compute_effective_sierpinski_iterations_for_bounds(proxy, tuning, bounds);
        build_windowed_sierpinski_topology_mesh(bounds, effective_iterations, tuning.domain_span)
    }

    #[cfg(test)]
    #[inline]
    fn compute_effective_sierpinski_iterations(proxy: &RenderProxy, tuning: SierpinskiSpongeTuning) -> u32 {
        let meshing_window = compute_meshing_window(proxy, tuning.lod.mesh_resolution);
        let bounds = compute_phenomenon_window_bounds(proxy.window_mode, meshing_window.center_local, meshing_window.size_local);
        compute_effective_sierpinski_iterations_for_bounds(proxy, tuning, bounds)
    }

    #[inline]
    fn compute_effective_sierpinski_iterations_for_bounds(proxy: &RenderProxy, tuning: SierpinskiSpongeTuning, bounds: PhenomenonModelWindowBounds) -> u32 {
        let cells = compute_effective_mesh_resolution(proxy, tuning.lod.mesh_resolution).max(1) as f32;
        // Visible span shrinks as subsection windowing zooms in. Exploit that to permit
        // deeper recursion while preserving a topology budget tied to mesh resolution.
        let window_span = bounds.span.max_element().max(MIN_WINDOW_SIZE_LOCAL);
        let sampled_span = (2.0 * tuning.domain_span * window_span).max(0.001);
        let smallest_feature_sample_requirement = 0.35f32;
        let feature_capacity = cells / (sampled_span * smallest_feature_sample_requirement);
        let max_resolvable_depth = if feature_capacity > 1.0 {
            feature_capacity.log(3.0).floor().max(1.0) as u32
        } else {
            1
        };
        let budget_cap = 6;
        tuning.iterations.clamp(1, max_resolvable_depth.min(budget_cap))
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    struct SierpinskiLeafCell {
        x: u32,
        y: u32,
        z: u32,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    struct SierpinskiWindowRemap {
        sample_min: Vec3,
        sample_span: Vec3,
        domain_span: f32,
    }

    impl SierpinskiWindowRemap {
        #[inline]
        fn from_bounds(bounds: PhenomenonModelWindowBounds, domain_span: f32) -> Self {
            let domain_span = domain_span.abs().max(1e-6);
            let seam_bounds = seam_safe_lattice_window(bounds.min, bounds.span, 1);
            let seam_denom = PHENOMENON_SEAM_LATTICE_DENOM as f32;
            let seam_min = Vec3::new(
                seam_bounds.min.x as f32 / seam_denom,
                seam_bounds.min.y as f32 / seam_denom,
                seam_bounds.min.z as f32 / seam_denom,
            );
            let seam_max = Vec3::new(
                seam_bounds.max.x as f32 / seam_denom,
                seam_bounds.max.y as f32 / seam_denom,
                seam_bounds.max.z as f32 / seam_denom,
            );
            let seam_span = (seam_max - seam_min).max(Vec3::splat(MIN_WINDOW_SIZE_LOCAL));

            let sample_min = (seam_min - Vec3::splat(0.5)) * (2.0 * domain_span);
            let sample_span = (seam_span * (2.0 * domain_span)).max(Vec3::splat(1e-6));
            Self {
                sample_min,
                sample_span,
                domain_span,
            }
        }

        #[inline]
        fn sample_max(self) -> Vec3 {
            self.sample_min + self.sample_span
        }

        #[inline]
        fn map_to_local(self, point: Vec3) -> Vec3 {
            let uvw = ((point / (2.0 * self.domain_span)) + Vec3::splat(0.5)).clamp(Vec3::ZERO, Vec3::ONE);
            model_local_position_from_sample_uvw(uvw, PHENOMENON_MODEL_LOCAL_SPAN_UNITS)
        }
    }

    #[inline]
    fn aabb_intersects(min_a: Vec3, max_a: Vec3, min_b: Vec3, max_b: Vec3) -> bool {
        min_a.x < max_b.x && max_a.x > min_b.x && min_a.y < max_b.y && max_a.y > min_b.y && min_a.z < max_b.z && max_a.z > min_b.z
    }

    #[inline]
    fn is_sierpinski_leaf_occupied(mut x: u32, mut y: u32, mut z: u32, iterations: u32) -> bool {
        for _ in 0..iterations {
            let cx = x % 3;
            let cy = y % 3;
            let cz = z % 3;
            let centered_axes = (cx == 1) as u32 + (cy == 1) as u32 + (cz == 1) as u32;
            if centered_axes >= 2 {
                return false;
            }
            x /= 3;
            y /= 3;
            z /= 3;
        }
        true
    }

    #[inline]
    fn sierpinski_leaf_cell_bounds(cell: SierpinskiLeafCell, leaf_size: f32) -> (Vec3, Vec3) {
        let min = Vec3::new(
            -1.0 + cell.x as f32 * leaf_size,
            -1.0 + cell.y as f32 * leaf_size,
            -1.0 + cell.z as f32 * leaf_size,
        );
        let max = min + Vec3::splat(leaf_size);
        (min, max)
    }

    fn collect_visible_sierpinski_leaf_cells(
        depth: u32,
        max_depth: u32,
        x: u32,
        y: u32,
        z: u32,
        cube_min: Vec3,
        cube_size: f32,
        clip_min: Vec3,
        clip_max: Vec3,
        out: &mut Vec<SierpinskiLeafCell>,
    ) {
        let cube_max = cube_min + Vec3::splat(cube_size);
        if !aabb_intersects(cube_min, cube_max, clip_min, clip_max) {
            return;
        }

        if depth == max_depth {
            out.push(SierpinskiLeafCell { x, y, z });
            return;
        }

        let child_size = cube_size / 3.0;
        for cz in 0..3u32 {
            for cy in 0..3u32 {
                for cx in 0..3u32 {
                    let centered_axes = (cx == 1) as u32 + (cy == 1) as u32 + (cz == 1) as u32;
                    if centered_axes >= 2 {
                        continue;
                    }

                    let child_min = cube_min + Vec3::new(cx as f32 * child_size, cy as f32 * child_size, cz as f32 * child_size);
                    collect_visible_sierpinski_leaf_cells(
                        depth + 1,
                        max_depth,
                        x * 3 + cx,
                        y * 3 + cy,
                        z * 3 + cz,
                        child_min,
                        child_size,
                        clip_min,
                        clip_max,
                        out,
                    );
                }
            }
        }
    }

    #[inline]
    fn neighbor_cell_visible_in_window(neighbor: IVec3, grid_dim: u32, iterations: u32, leaf_size: f32, clip_min: Vec3, clip_max: Vec3) -> bool {
        if neighbor.x < 0 || neighbor.y < 0 || neighbor.z < 0 {
            return false;
        }
        let nx = neighbor.x as u32;
        let ny = neighbor.y as u32;
        let nz = neighbor.z as u32;
        if nx >= grid_dim || ny >= grid_dim || nz >= grid_dim {
            return false;
        }
        if !is_sierpinski_leaf_occupied(nx, ny, nz, iterations) {
            return false;
        }
        let (neighbor_min, neighbor_max) = sierpinski_leaf_cell_bounds(SierpinskiLeafCell { x: nx, y: ny, z: nz }, leaf_size);
        aabb_intersects(neighbor_min, neighbor_max, clip_min, clip_max)
    }

    #[inline]
    fn push_triangle(a: Vec3, b: Vec3, c: Vec3, out_positions: &mut Vec<[f32; 3]>, out_normals: &mut Vec<[f32; 3]>, out_uvs: &mut Vec<[f32; 2]>) {
        let normal = (b - a).cross(c - a);
        let len_sq = normal.length_squared();
        if len_sq <= 1e-10 {
            return;
        }

        let n = normal / len_sq.sqrt();
        for p in [a, b, c] {
            out_positions.push([p.x, p.y, p.z]);
            out_normals.push([n.x, n.y, n.z]);
            out_uvs.push([
                (p.x / PHENOMENON_MODEL_LOCAL_SPAN_UNITS + 0.5).clamp(0.0, 1.0),
                (p.y / PHENOMENON_MODEL_LOCAL_SPAN_UNITS + 0.5).clamp(0.0, 1.0),
            ]);
        }
    }

    #[inline]
    fn push_quad(a: Vec3, b: Vec3, c: Vec3, d: Vec3, out_positions: &mut Vec<[f32; 3]>, out_normals: &mut Vec<[f32; 3]>, out_uvs: &mut Vec<[f32; 2]>) {
        push_triangle(a, b, c, out_positions, out_normals, out_uvs);
        push_triangle(a, c, d, out_positions, out_normals, out_uvs);
    }

    fn build_windowed_sierpinski_topology_mesh(bounds: PhenomenonModelWindowBounds, iterations: u32, domain_span: f32) -> Option<Mesh> {
        if iterations == 0 {
            return None;
        }

        let remap = SierpinskiWindowRemap::from_bounds(bounds, domain_span);
        let clip_min = remap.sample_min;
        let clip_max = remap.sample_max();
        let sponge_min = Vec3::splat(-1.0);
        let sponge_max = Vec3::splat(1.0);
        if !aabb_intersects(clip_min, clip_max, sponge_min, sponge_max) {
            return None;
        }

        let grid_dim = 3u32.saturating_pow(iterations);
        if grid_dim == 0 {
            return None;
        }
        let leaf_size = 2.0 / grid_dim as f32;

        let mut leaf_cells = Vec::new();
        collect_visible_sierpinski_leaf_cells(0, iterations, 0, 0, 0, Vec3::splat(-1.0), 2.0, clip_min, clip_max, &mut leaf_cells);

        if leaf_cells.is_empty() {
            return None;
        }

        let mut out_positions = Vec::<[f32; 3]>::new();
        let mut out_normals = Vec::<[f32; 3]>::new();
        let mut out_uvs = Vec::<[f32; 2]>::new();

        // Vertex order for a unit cube:
        // 0:(0,0,0) 1:(1,0,0) 2:(1,1,0) 3:(0,1,0) 4:(0,0,1) 5:(1,0,1) 6:(1,1,1) 7:(0,1,1)
        const FACE_DEFS: [(IVec3, [usize; 4]); 6] = [
            (IVec3::new(-1, 0, 0), [0, 4, 7, 3]), // -X
            (IVec3::new(1, 0, 0), [1, 2, 6, 5]),  // +X
            (IVec3::new(0, -1, 0), [0, 1, 5, 4]), // -Y
            (IVec3::new(0, 1, 0), [3, 7, 6, 2]),  // +Y
            (IVec3::new(0, 0, -1), [0, 3, 2, 1]), // -Z
            (IVec3::new(0, 0, 1), [4, 5, 6, 7]),  // +Z
        ];

        for cell in leaf_cells {
            let (cell_min, cell_max) = sierpinski_leaf_cell_bounds(cell, leaf_size);
            let clipped_min = cell_min.max(clip_min);
            let clipped_max = cell_max.min(clip_max);
            if clipped_max.x <= clipped_min.x || clipped_max.y <= clipped_min.y || clipped_max.z <= clipped_min.z {
                continue;
            }

            let local = |p: Vec3| remap.map_to_local(p);
            let corners = [
                local(Vec3::new(clipped_min.x, clipped_min.y, clipped_min.z)),
                local(Vec3::new(clipped_max.x, clipped_min.y, clipped_min.z)),
                local(Vec3::new(clipped_max.x, clipped_max.y, clipped_min.z)),
                local(Vec3::new(clipped_min.x, clipped_max.y, clipped_min.z)),
                local(Vec3::new(clipped_min.x, clipped_min.y, clipped_max.z)),
                local(Vec3::new(clipped_max.x, clipped_min.y, clipped_max.z)),
                local(Vec3::new(clipped_max.x, clipped_max.y, clipped_max.z)),
                local(Vec3::new(clipped_min.x, clipped_max.y, clipped_max.z)),
            ];

            for (offset, face) in FACE_DEFS {
                let neighbor = IVec3::new(cell.x as i32, cell.y as i32, cell.z as i32) + offset;
                if neighbor_cell_visible_in_window(neighbor, grid_dim, iterations, leaf_size, clip_min, clip_max) {
                    continue;
                }
                push_quad(
                    corners[face[0]],
                    corners[face[1]],
                    corners[face[2]],
                    corners[face[3]],
                    &mut out_positions,
                    &mut out_normals,
                    &mut out_uvs,
                );
            }
        }

        if out_positions.is_empty() {
            return None;
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, out_positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, out_normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, out_uvs);
        Some(mesh)
    }

    fn build_windowed_field_mesh<F>(proxy: &RenderProxy, lod: SurfaceLodTuning, mut density_from_model_space: F) -> Option<Mesh>
    where
        F: FnMut(Vec3, u8) -> f32,
    {
        let meshing_window = compute_meshing_window(proxy, lod.mesh_resolution);
        let bounds = compute_phenomenon_window_bounds(proxy.window_mode, meshing_window.center_local, meshing_window.size_local);
        let cells = effective_lattice_cells_for_bounds(bounds, meshing_window.resolution);
        let lattice_window = seam_safe_lattice_window(bounds.min, bounds.span, cells);
        let axis_points = lattice_window.axis_points();
        let seam_denom = PHENOMENON_SEAM_LATTICE_DENOM as f32;

        let mut points = vec![Vec3::ZERO; axis_points * axis_points * axis_points];
        let mut field = vec![0.0f32; axis_points * axis_points * axis_points];

        for iz in 0..axis_points {
            for iy in 0..axis_points {
                for ix in 0..axis_points {
                    let idx = grid_index(ix, iy, iz, axis_points);
                    let lattice_coord = lattice_window.lattice_coord(ix, iy, iz);
                    let sample_uvw = Vec3::new(
                        lattice_coord.x as f32 / seam_denom,
                        lattice_coord.y as f32 / seam_denom,
                        lattice_coord.z as f32 / seam_denom,
                    );
                    let signed_distance = density_from_model_space(sample_uvw, proxy.layer_index);
                    field[idx] = signed_distance - lod.iso_level;
                    points[idx] = model_local_position_from_sample_uvw(sample_uvw, PHENOMENON_MODEL_LOCAL_SPAN_UNITS);
                }
            }
        }

        let cube_corners: [[usize; 3]; 8] = [[0, 0, 0], [1, 0, 0], [1, 1, 0], [0, 1, 0], [0, 0, 1], [1, 0, 1], [1, 1, 1], [0, 1, 1]];
        let tets: [[usize; 4]; 6] = [[0, 5, 1, 6], [0, 1, 2, 6], [0, 2, 3, 6], [0, 3, 7, 6], [0, 7, 4, 6], [0, 4, 5, 6]];

        let mut out_positions = Vec::<[f32; 3]>::new();
        let mut out_normals = Vec::<[f32; 3]>::new();
        let mut out_uvs = Vec::<[f32; 2]>::new();

        for iz in 0..cells {
            for iy in 0..cells {
                for ix in 0..cells {
                    let mut cube_points = [Vec3::ZERO; 8];
                    let mut cube_values = [0.0f32; 8];
                    for (corner_i, [ox, oy, oz]) in cube_corners.iter().copied().enumerate() {
                        let gx = ix + ox;
                        let gy = iy + oy;
                        let gz = iz + oz;
                        let idx = grid_index(gx, gy, gz, axis_points);
                        cube_points[corner_i] = points[idx];
                        cube_values[corner_i] = field[idx];
                    }

                    for tet in tets {
                        let p = [cube_points[tet[0]], cube_points[tet[1]], cube_points[tet[2]], cube_points[tet[3]]];
                        let s = [cube_values[tet[0]], cube_values[tet[1]], cube_values[tet[2]], cube_values[tet[3]]];
                        emit_tetra_surface(p, s, &mut out_positions, &mut out_normals, &mut out_uvs);
                    }
                }
            }
        }

        if out_positions.is_empty() {
            return None;
        }

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default());
        mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, out_positions);
        mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, out_normals);
        mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, out_uvs);
        Some(mesh)
    }

    #[inline]
    fn model_local_position_from_sample_uvw(sample_uvw: Vec3, span_units: f32) -> Vec3 {
        let uvw = sample_uvw.clamp(Vec3::ZERO, Vec3::ONE);
        Vec3::new((uvw.x - 0.5) * span_units, (uvw.y - 0.5) * span_units, (uvw.z - 0.5) * span_units)
    }

    fn emit_tetra_surface(
        points: [Vec3; 4],
        values: [f32; 4],
        out_positions: &mut Vec<[f32; 3]>,
        out_normals: &mut Vec<[f32; 3]>,
        out_uvs: &mut Vec<[f32; 2]>,
    ) {
        let mut inside = [0usize; 4];
        let mut outside = [0usize; 4];
        let mut inside_count = 0usize;
        let mut outside_count = 0usize;

        for i in 0..4 {
            // Signed-distance convention: negative = inside, positive = outside.
            if values[i] <= 0.0 {
                inside[inside_count] = i;
                inside_count += 1;
            } else {
                outside[outside_count] = i;
                outside_count += 1;
            }
        }

        if inside_count == 0 || inside_count == 4 {
            return;
        }

        let edge_point = |a_i: usize, b_i: usize| interpolate_iso(points[a_i], values[a_i], points[b_i], values[b_i]);

        match inside_count {
            1 => {
                let i = inside[0];
                let a = outside[0];
                let b = outside[1];
                let c = outside[2];
                let inside_ref = points[i];
                let outside_ref = (points[a] + points[b] + points[c]) / 3.0;
                push_oriented_triangle(
                    edge_point(i, a),
                    edge_point(i, b),
                    edge_point(i, c),
                    inside_ref,
                    outside_ref,
                    out_positions,
                    out_normals,
                    out_uvs,
                );
            }
            3 => {
                let o = outside[0];
                let a = inside[0];
                let b = inside[1];
                let c = inside[2];
                let inside_ref = (points[a] + points[b] + points[c]) / 3.0;
                let outside_ref = points[o];
                push_oriented_triangle(
                    edge_point(o, a),
                    edge_point(o, c),
                    edge_point(o, b),
                    inside_ref,
                    outside_ref,
                    out_positions,
                    out_normals,
                    out_uvs,
                );
            }
            2 => {
                let a = inside[0];
                let b = inside[1];
                let c = outside[0];
                let d = outside[1];
                let inside_ref = (points[a] + points[b]) * 0.5;
                let outside_ref = (points[c] + points[d]) * 0.5;

                let p0 = edge_point(a, c);
                let p1 = edge_point(b, c);
                let p2 = edge_point(b, d);
                let p3 = edge_point(a, d);
                push_oriented_triangle(p0, p1, p2, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
                push_oriented_triangle(p0, p2, p3, inside_ref, outside_ref, out_positions, out_normals, out_uvs);
            }
            _ => {}
        }
    }

    #[inline]
    fn push_oriented_triangle(
        a: Vec3,
        mut b: Vec3,
        mut c: Vec3,
        inside_ref: Vec3,
        outside_ref: Vec3,
        out_positions: &mut Vec<[f32; 3]>,
        out_normals: &mut Vec<[f32; 3]>,
        out_uvs: &mut Vec<[f32; 2]>,
    ) {
        let mut normal = (b - a).cross(c - a);
        let mut len_sq = normal.length_squared();
        if len_sq <= 1e-10 {
            return;
        }

        // Force a stable winding: normals should point from inside towards outside.
        let expected_outward = outside_ref - inside_ref;
        if expected_outward.length_squared() > 1e-10 && normal.dot(expected_outward) < 0.0 {
            std::mem::swap(&mut b, &mut c);
            normal = (b - a).cross(c - a);
            len_sq = normal.length_squared();
            if len_sq <= 1e-10 {
                return;
            }
        }

        let n = normal / len_sq.sqrt();
        for p in [a, b, c] {
            out_positions.push([p.x, p.y, p.z]);
            out_normals.push([n.x, n.y, n.z]);
            out_uvs.push([
                (p.x / PHENOMENON_MODEL_LOCAL_SPAN_UNITS + 0.5).clamp(0.0, 1.0),
                (p.y / PHENOMENON_MODEL_LOCAL_SPAN_UNITS + 0.5).clamp(0.0, 1.0),
            ]);
        }
    }

    #[inline]
    fn interpolate_iso(a_pos: Vec3, a_val: f32, b_pos: Vec3, b_val: f32) -> Vec3 {
        let denom = a_val - b_val;
        let t = if denom.abs() <= 1e-6 { 0.5 } else { (a_val / denom).clamp(0.0, 1.0) };
        a_pos + (b_pos - a_pos) * t
    }

    #[inline]
    fn grid_coord_native_absolute(coord: &GridVec) -> Vec3 {
        let mut acc_x = 0.0_f64;
        let mut acc_y = 0.0_f64;
        let mut acc_z = 0.0_f64;
        let mut factor = 1.0_f64;
        let mut cursor = coord;

        loop {
            acc_x += cursor.xyz.x as f64 * factor;
            acc_y += cursor.xyz.y as f64 * factor;
            acc_z += cursor.xyz.z as f64 * factor;

            let Some(parent) = cursor.parent.as_ref() else {
                break;
            };
            factor *= 10.0;
            cursor = parent.as_ref();
        }

        let native_unit = PHENOMENON_MODEL_LOCAL_SPAN_UNITS as f64;
        Vec3::new((acc_x * native_unit) as f32, (acc_y * native_unit) as f32, (acc_z * native_unit) as f32)
    }

    #[inline]
    fn phenomenon_node_center_native_absolute(node: &PhenomenonNode) -> Vec3 {
        let mut acc_x = 0.0_f64;
        let mut acc_y = 0.0_f64;
        let mut acc_z = 0.0_f64;
        let mut factor = 1.0_f64;

        for local_cell in node.lineage.cells.iter().rev() {
            let cell = local_cell.as_ivec3();
            acc_x += cell.x as f64 * factor;
            acc_y += cell.y as f64 * factor;
            acc_z += cell.z as f64 * factor;
            factor *= 10.0;
        }

        let native_unit = PHENOMENON_MODEL_LOCAL_SPAN_UNITS as f64;
        Vec3::new((acc_x * native_unit) as f32, (acc_y * native_unit) as f32, (acc_z * native_unit) as f32)
    }

    #[inline]
    fn phenomenon_node_grid_coord(node: &PhenomenonNode) -> Option<GridVec> {
        let mut lineage = node.lineage.cells.iter();
        let root = lineage.next().copied()?;
        let mut coord = GridVec::new_root(GridXyz::from_local_cell3(root));
        for local_cell in lineage {
            coord = GridVec::new(coord, GridXyz::from_local_cell3(*local_cell));
        }
        if coord.scale != node.scale {
            return None;
        }
        Some(coord)
    }

    #[inline]
    fn phenomenon_node_center_native_local(node: &PhenomenonNode, origin_offset: &GridVec) -> Option<Vec3> {
        let coord = phenomenon_node_grid_coord(node)?;
        let (center_native_local, _visual_scale) = coord.to_native_visual(origin_offset.clone());
        Some(center_native_local)
    }

    fn select_frontier_node_for_view<'a, I>(
        nodes: I,
        view_scale: Scale,
        view_pos_native_local: Vec3,
        origin_offset: &GridVec,
    ) -> Option<(Scale, Vec3, u64, u32)>
    where
        I: IntoIterator<Item = &'a PhenomenonNode>,
    {
        let mut best: Option<(u8, f32, u64, Scale, Vec3, u32)> = None;

        for node in nodes {
            let Some(center_native_local) = phenomenon_node_center_native_local(node, origin_offset) else {
                continue;
            };
            let scale_distance = (node.scale.index_from_top() as i16 - view_scale.index_from_top() as i16).abs() as u8;
            let distance_sq = center_native_local.distance_squared(view_pos_native_local);

            let is_better = match best {
                None => true,
                Some((best_scale_distance, best_distance_sq, best_seed, _, _, _)) => {
                    scale_distance < best_scale_distance
                        || (scale_distance == best_scale_distance && distance_sq < best_distance_sq)
                        || (scale_distance == best_scale_distance && (distance_sq - best_distance_sq).abs() <= 0.01 && node.seed.0 < best_seed)
                }
            };

            if is_better {
                best = Some((scale_distance, distance_sq, node.seed.0, node.scale, center_native_local, node.lineage.depth()));
            }
        }

        best.map(|(_, _, seed, scale, center_native, depth)| (scale, center_native, seed, depth))
    }

    fn select_frontier_nodes_for_view<'a, I>(
        nodes: I,
        view_scale: Scale,
        view_pos_native_local: Vec3,
        origin_offset: &GridVec,
        coarser_levels: u8,
        finer_levels: u8,
    ) -> Vec<FrontierSelection>
    where
        I: IntoIterator<Item = &'a PhenomenonNode>,
    {
        let view_index = view_scale.index_from_top() as i16;
        let min_scale_index = (view_index - coarser_levels as i16).max(0);
        let max_scale_index = (view_index + finer_levels as i16).min((Scale::SCALE_LEVEL_COUNT.saturating_sub(1)) as i16);

        let mut best_by_scale: BTreeMap<u8, (f32, u64, FrontierSelection)> = BTreeMap::new();
        for node in nodes {
            let scale_index = node.scale.index_from_top() as i16;
            if scale_index < min_scale_index || scale_index > max_scale_index {
                continue;
            }

            let Some(center_native_local) = phenomenon_node_center_native_local(node, origin_offset) else {
                continue;
            };
            let distance_sq = center_native_local.distance_squared(view_pos_native_local);
            let selection = FrontierSelection {
                scale: node.scale,
                center_native_local,
                seed: node.seed.0,
                lineage_depth: node.lineage.depth(),
            };
            let key = node.scale.index_from_top();

            let replace = match best_by_scale.get(&key) {
                None => true,
                Some((best_distance_sq, best_seed, _)) => {
                    distance_sq < *best_distance_sq || ((distance_sq - *best_distance_sq).abs() <= 0.01 && selection.seed < *best_seed)
                }
            };
            if replace {
                best_by_scale.insert(key, (distance_sq, selection.seed, selection));
            }
        }

        let mut selections = best_by_scale.into_values().map(|(_, _, selection)| selection).collect::<Vec<_>>();
        selections.sort_by(|a, b| {
            let a_scale_distance = (a.scale.index_from_top() as i16 - view_index).abs();
            let b_scale_distance = (b.scale.index_from_top() as i16 - view_index).abs();
            a_scale_distance
                .cmp(&b_scale_distance)
                .then_with(|| a.scale.index_from_top().cmp(&b.scale.index_from_top()))
                .then_with(|| a.seed.cmp(&b.seed))
        });
        selections
    }
}

#[inline]
fn compute_render_proxy_windowing(
    scale_diff: i8,
    chunk_center_native: Vec3,
    view_pos_native: Vec3,
    player_local_zoom: f32,
) -> (RenderProxyWindowMode, Vec3, Vec3) {
    #[inline]
    fn local_window_bounds(center_local: Vec3, size_local: Vec3) -> (Vec3, Vec3) {
        let center01 = center_local.clamp(Vec3::splat(-0.5), Vec3::splat(0.5)) + Vec3::splat(0.5);
        let size01 = size_local.abs().clamp(Vec3::splat(MIN_WINDOW_SIZE_LOCAL), Vec3::ONE);
        let mut min = (center01 - size01 * 0.5).clamp(Vec3::ZERO, Vec3::ONE);
        let mut max = (center01 + size01 * 0.5).clamp(Vec3::ZERO, Vec3::ONE);
        if max.x < min.x {
            std::mem::swap(&mut max.x, &mut min.x);
        }
        if max.y < min.y {
            std::mem::swap(&mut max.y, &mut min.y);
        }
        if max.z < min.z {
            std::mem::swap(&mut max.z, &mut min.z);
        }
        (min, max)
    }

    #[inline]
    fn compose_local_windows(base_center_local: Vec3, base_size_local: Vec3, nested_center_local: Vec3, nested_size_local: Vec3) -> (Vec3, Vec3) {
        let (base_min, base_max) = local_window_bounds(base_center_local, base_size_local);
        let (nested_min, nested_max) = local_window_bounds(nested_center_local, nested_size_local);
        let base_span = (base_max - base_min).max(Vec3::splat(MIN_WINDOW_SIZE_LOCAL));
        let mut composed_min = base_min + nested_min * base_span;
        let mut composed_max = base_min + nested_max * base_span;
        composed_min = composed_min.clamp(Vec3::ZERO, Vec3::ONE);
        composed_max = composed_max.clamp(Vec3::ZERO, Vec3::ONE);
        if composed_max.x < composed_min.x {
            std::mem::swap(&mut composed_max.x, &mut composed_min.x);
        }
        if composed_max.y < composed_min.y {
            std::mem::swap(&mut composed_max.y, &mut composed_min.y);
        }
        if composed_max.z < composed_min.z {
            std::mem::swap(&mut composed_max.z, &mut composed_min.z);
        }
        let size_local = (composed_max - composed_min).max(Vec3::splat(MIN_WINDOW_SIZE_LOCAL));
        let center_local = ((composed_min + composed_max) * 0.5) - Vec3::splat(0.5);
        (center_local, size_local)
    }

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

    let (mut mode, mut center_local, mut size_local) = if scale_diff <= 0 {
        // Even full-entity mode tracks local center for future nested windowing composition.
        let center = Vec3::new(
            continuous_axis_center(chunk_center_native.x as f64, view_pos_native.x as f64, scale_diff) as f32,
            continuous_axis_center(chunk_center_native.y as f64, view_pos_native.y as f64, scale_diff) as f32,
            continuous_axis_center(chunk_center_native.z as f64, view_pos_native.z as f64, scale_diff) as f32,
        );
        (RenderProxyWindowMode::FullEntity, center, Vec3::ONE)
    } else {
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
    };

    if scale_diff > 0 {
        let presentation_window_scale = player_local_zoom.clamp(MIN_WINDOW_SIZE_LOCAL, 1.0);
        if presentation_window_scale < 0.999_999 {
            let (composed_center, composed_size) = compose_local_windows(center_local, size_local, center_local, Vec3::splat(presentation_window_scale));
            center_local = composed_center;
            size_local = composed_size;
            mode = RenderProxyWindowMode::WindowedSubsection;
        }
    }

    (mode, center_local, size_local)
}

#[cfg(any())]
mod legacy_tests {
    use super::*;
    use crate::usf::phenomenon::PhenomenonLineage;
    use crate::usf::pos::grid::types::GridVec;
    use crate::usf::pos::types::GridXyz;
    use crate::usf::pos::types::LocalCell3;

    fn default_lod(mesh_resolution: u32) -> SurfaceLodTuning {
        SurfaceLodTuning {
            visibility_threshold: 0.45,
            scale_boost: 0.8,
            z_displacement: 200.0,
            mesh_resolution,
            iso_level: 0.0,
        }
    }

    fn default_mandelbulb_tuning(mesh_resolution: u32) -> MandelbulbTuning {
        MandelbulbTuning {
            power: 8.0,
            iterations: 10,
            bailout: 4.0,
            z_span: 1.2,
            lod: default_lod(mesh_resolution),
        }
    }

    fn default_sierpinski_tuning(mesh_resolution: u32) -> SierpinskiSpongeTuning {
        SierpinskiSpongeTuning {
            iterations: 5,
            domain_span: 1.1,
            hole_bias: 0.0,
            lod: default_lod(mesh_resolution),
        }
    }

    fn sample_proxy(window_mode: RenderProxyWindowMode, window_center_local: Vec3, window_size_local: Vec3) -> RenderProxy {
        RenderProxy {
            source: Entity::PLACEHOLDER,
            layer_index: 35,
            depth_bias: 0.0,
            frontier_node_seed: 0,
            frontier_lineage_depth: 0,
            window_mode,
            window_center_local,
            window_size_local,
            coarse_context_persistent: true,
        }
    }

    fn sample_node(scale: Scale, cell3: IVec3, seed: u64) -> PhenomenonNode {
        let local_cell = LocalCell3::new_local(cell3.x, cell3.y, cell3.z);
        let depth = scale.index_from_top() as usize;
        let mut cells = Vec::with_capacity(depth + 1);
        cells.push(LocalCell3::ZERO);
        for _ in 0..depth {
            cells.push(local_cell);
        }
        let lineage = PhenomenonLineage::from_cells(cells);
        PhenomenonNode {
            phenomenon_id: PhenomenonId(0),
            scale,
            lineage,
            local_cell,
            parent: None,
            local_index: 0,
            seed: crate::usf::phenomenon::PhenomenonNodeSeed(seed),
        }
    }

    #[test]
    fn camera_distance_decreases_monotonically_with_zoom() {
        let fov_min = 8.0;
        let fov_max = 100.0;

        let zoom_a = 0.1;
        let zoom_b = 1.0;
        let zoom_c = 10.0;

        let fov_a = zoom_to_fov_radians(zoom_a, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);
        let fov_b = zoom_to_fov_radians(zoom_b, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);
        let fov_c = zoom_to_fov_radians(zoom_c, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);

        let d_a = camera_distance_from_zoom_and_fov(zoom_a, fov_a);
        let d_b = camera_distance_from_zoom_and_fov(zoom_b, fov_b);
        let d_c = camera_distance_from_zoom_and_fov(zoom_c, fov_c);

        assert!(
            d_a > d_b && d_b > d_c,
            "expected distance ordering d(0.1) > d(1.0) > d(10.0), got {d_a} >? {d_b} >? {d_c}"
        );
    }

    #[test]
    fn camera_distance_and_fov_pair_produces_inverse_zoom_view_span() {
        let fov_min = 8.0;
        let fov_max = 100.0;

        let z1 = 1.0;
        let z10 = 10.0;

        let fov1 = zoom_to_fov_radians(z1, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);
        let fov10 = zoom_to_fov_radians(z10, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);

        let d1 = camera_distance_from_zoom_and_fov(z1, fov1);
        let d10 = camera_distance_from_zoom_and_fov(z10, fov10);

        let span1 = d1 * (fov1 * 0.5).tan();
        let span10 = d10 * (fov10 * 0.5).tan();
        let ratio = span1 / span10;

        assert!((ratio - 10.0).abs() < 0.01, "expected ~10x span ratio, got {ratio}");
    }

    #[test]
    fn grid_coord_native_absolute_accumulates_parent_lineage() {
        let root = GridVec::new_root(GridXyz::new_local(1, 0, 0));
        let child = GridVec::new(root, GridXyz::new_local(2, 0, 0));

        let absolute = grid_coord_native_absolute(&child);
        assert_eq!(absolute, Vec3::new(12_000.0, 0.0, 0.0));
    }

    #[test]
    fn pinned_origin_node_moves_opposite_to_origin_offset_in_local_frame() {
        let origin_offset = GridVec::new_root(GridXyz::new_local(2, 0, 0));
        let origin_absolute = grid_coord_native_absolute(&origin_offset);
        let root_node = sample_node(Scale::MAX, IVec3::ZERO, 1);
        let root_absolute = phenomenon_node_center_native_absolute(&root_node);

        let root_local = root_absolute - origin_absolute;
        assert_eq!(root_local, Vec3::new(-2_000.0, 0.0, 0.0));
    }

    #[test]
    fn frontier_debug_stats_select_primary_seed_without_global_root() {
        let mut app = App::new();
        app.init_resource::<PhenomenonDebugStats>();
        app.add_systems(Update, update_frontier_debug_stats_system);

        let mut chunk_loader = ChunkLoader::default();
        chunk_loader.scale = Scale::MAX.zoomed_in();
        app.world_mut().spawn((Player, chunk_loader, Transform::default()));
        app.world_mut().spawn(sample_node(Scale::MAX, IVec3::new(0, 0, 0), 11));
        app.world_mut().spawn(sample_node(Scale::MAX.zoomed_in(), IVec3::new(2, 0, 0), 22));

        app.update();

        let stats = app.world().resource::<PhenomenonDebugStats>();
        assert_eq!(stats.frontier_primary_seed, 22);
        assert_eq!(stats.frontier_primary_scale_index, Scale::MAX.zoomed_in().index_from_top() as u32);
        assert_eq!(stats.frontier_proxy_spawns_frame, 0);
        assert_eq!(stats.frontier_proxy_despawns_frame, 0);
    }

    #[test]
    fn frontier_debug_stats_fallback_is_stable_when_no_nodes_exist() {
        let mut app = App::new();
        app.init_resource::<PhenomenonDebugStats>();
        app.add_systems(Update, update_frontier_debug_stats_system);
        app.world_mut().spawn((Player, ChunkLoader::default(), Transform::default()));

        app.update();

        let stats = app.world().resource::<PhenomenonDebugStats>();
        assert_eq!(stats.frontier_primary_seed, 0);
        assert_eq!(stats.frontier_primary_scale_index, Scale::MAX.index_from_top() as u32);
        assert_eq!(stats.frontier_proxy_spawns_frame, 0);
        assert_eq!(stats.frontier_proxy_despawns_frame, 0);
    }

    #[test]
    fn chunk_wire_transform_handles_finer_coord_than_origin_scale() {
        let origin_offset = GridVec::build().push((0, 0, 0)).finish();
        let finer_coord = GridVec::build().push((0, 0, 0)).push((0, 0, 0)).finish();

        let transform = chunk_wire_transform(&finer_coord, &origin_offset, Quat::IDENTITY, Vec3::ZERO, 1.0, 1000.0, 0.1);

        assert!(transform.translation.is_finite());
        assert!(transform.scale.is_finite());
    }

    #[test]
    fn full_entity_mode_for_same_or_finer_scale() {
        let (mode, center, size) = compute_render_proxy_windowing(0, Vec3::ZERO, Vec3::new(123.0, -45.0, 12.0), 1.0);
        assert_eq!(mode, RenderProxyWindowMode::FullEntity);
        assert_eq!(center, Vec3::ZERO);
        assert_eq!(size, Vec3::ONE);
    }

    #[test]
    fn frontier_selection_tracks_scale_transition() {
        let coarse = sample_node(Scale::MAX, IVec3::new(0, 0, 0), 11);
        let fine = sample_node(Scale::MAX.zoomed_in(), IVec3::new(1, 0, 0), 22);
        let origin_offset = GridVec::new_root(GridXyz::new_local(0, 0, 0));

        let selected_at_coarse = select_frontier_node_for_view([&coarse, &fine], Scale::MAX, Vec3::ZERO, &origin_offset).unwrap();
        assert_eq!(selected_at_coarse.0, coarse.scale);

        let selected_at_fine = select_frontier_node_for_view([&coarse, &fine], Scale::MAX.zoomed_in(), Vec3::ZERO, &origin_offset).unwrap();
        assert_eq!(selected_at_fine.0, fine.scale);
    }

    #[test]
    fn frontier_selection_uses_nearest_node_within_scale() {
        let a = sample_node(Scale::MAX.zoomed_in(), IVec3::new(-2, 0, 0), 101);
        let b = sample_node(Scale::MAX.zoomed_in(), IVec3::new(3, 0, 0), 102);
        let origin_offset = GridVec::new_root(GridXyz::new_local(0, 0, 0));

        let selected = select_frontier_node_for_view([&a, &b], Scale::MAX.zoomed_in(), Vec3::new(2_900.0, 0.0, 0.0), &origin_offset).unwrap();
        assert_eq!(selected.0, b.scale);
    }

    #[test]
    fn frontier_selection_returns_multi_scale_band() {
        let s0 = Scale::MAX;
        let s1 = s0.zoomed_in();
        let s2 = s1.zoomed_in();
        let nodes = [
            sample_node(s0, IVec3::new(0, 0, 0), 1),
            sample_node(s1, IVec3::new(1, 0, 0), 2),
            sample_node(s2, IVec3::new(2, 0, 0), 3),
        ];
        let origin_offset = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let selections = select_frontier_nodes_for_view(nodes.iter(), s1, Vec3::ZERO, &origin_offset, 1, 1);

        assert!(selections.len() >= 2);
        assert!(selections.iter().any(|selection| selection.scale == s0));
        assert!(selections.iter().any(|selection| selection.scale == s1));
        assert!(selections.iter().any(|selection| selection.scale == s2));
    }

    #[test]
    fn windowed_mode_scales_down_with_coarser_level() {
        let (mode, center, size) = compute_render_proxy_windowing(1, Vec3::ZERO, Vec3::ZERO, 1.0);
        assert_eq!(mode, RenderProxyWindowMode::WindowedSubsection);
        assert!((center.x - 0.05).abs() < 1e-6);
        assert!((center.y - 0.05).abs() < 1e-6);
        assert!((center.z - 0.05).abs() < 1e-6);
        assert!((size.x - 0.1).abs() < 1e-6);
        assert!((size.y - 0.1).abs() < 1e-6);
        assert!((size.z - 0.1).abs() < 1e-6);
    }

    #[test]
    fn window_center_tracks_viewpoint_inside_chunk() {
        // scale_diff=1 => chunk span is 10,000 native units.
        let (mode, center, size) = compute_render_proxy_windowing(1, Vec3::ZERO, Vec3::new(2_500.0, 2_500.0, 2_500.0), 1.0);
        assert_eq!(mode, RenderProxyWindowMode::WindowedSubsection);
        assert!(center.x > 0.0 && center.y > 0.0 && center.z > 0.0);
        assert!((size.x - 0.1).abs() < 1e-6);
        assert!((size.y - 0.1).abs() < 1e-6);
        assert!((size.z - 0.1).abs() < 1e-6);
    }

    #[test]
    fn effective_mesh_resolution_increases_for_smaller_window() {
        let broad = sample_proxy(RenderProxyWindowMode::WindowedSubsection, Vec3::ZERO, Vec3::splat(0.9));
        let narrow = RenderProxy {
            window_size_local: Vec3::splat(0.1),
            ..broad
        };
        assert!(compute_effective_mesh_resolution(&narrow, 12) > compute_effective_mesh_resolution(&broad, 12));
    }

    #[test]
    fn full_entity_mesh_resolution_is_decade_quantized_for_cross_scale_alignment() {
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        let resolution = compute_effective_mesh_resolution(&proxy, 23);
        assert_eq!(resolution % 10, 0);
        assert!(resolution >= 10);
    }

    #[test]
    fn phenomenon_window_full_entity_uses_unit_bounds() {
        let bounds = compute_phenomenon_window_bounds(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        assert_eq!(bounds.min, Vec3::ZERO);
        assert_eq!(bounds.max, Vec3::ONE);
        assert_eq!(bounds.span, Vec3::ONE);
    }

    #[test]
    fn phenomenon_windowed_subsection_bounds_clamp_and_span() {
        let bounds = compute_phenomenon_window_bounds(RenderProxyWindowMode::WindowedSubsection, Vec3::ZERO, Vec3::splat(0.5));
        assert!((bounds.min.x - 0.25).abs() < 1e-6);
        assert!((bounds.min.y - 0.25).abs() < 1e-6);
        assert!((bounds.min.z - 0.25).abs() < 1e-6);
        assert!((bounds.max.x - 0.75).abs() < 1e-6);
        assert!((bounds.max.y - 0.75).abs() < 1e-6);
        assert!((bounds.max.z - 0.75).abs() < 1e-6);
        assert!((bounds.span.x - 0.5).abs() < 1e-6);
        assert!((bounds.span.y - 0.5).abs() < 1e-6);
        assert!((bounds.span.z - 0.5).abs() < 1e-6);
    }

    #[test]
    fn windowed_subsection_positions_remain_local_subvolume() {
        let bounds = compute_phenomenon_window_bounds(RenderProxyWindowMode::WindowedSubsection, Vec3::ZERO, Vec3::splat(0.1));
        let lattice = seam_safe_lattice_window(bounds.min, bounds.span, 10);
        let seam_denom = PHENOMENON_SEAM_LATTICE_DENOM as f32;

        let min_uvw = Vec3::new(
            lattice.min.x as f32 / seam_denom,
            lattice.min.y as f32 / seam_denom,
            lattice.min.z as f32 / seam_denom,
        );
        let max_uvw = Vec3::new(
            lattice.max.x as f32 / seam_denom,
            lattice.max.y as f32 / seam_denom,
            lattice.max.z as f32 / seam_denom,
        );
        let min_local = model_local_position_from_sample_uvw(min_uvw, PHENOMENON_MODEL_LOCAL_SPAN_UNITS);
        let max_local = model_local_position_from_sample_uvw(max_uvw, PHENOMENON_MODEL_LOCAL_SPAN_UNITS);

        let span = max_local - min_local;
        assert!(span.x > 90.0 && span.x < 110.0);
        assert!(span.y > 90.0 && span.y < 110.0);
        assert!(span.z > 90.0 && span.z < 110.0);
    }

    #[test]
    fn meshing_seam_contract_uses_identical_boundary_lattice_samples() {
        let left = seam_safe_lattice_window(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.5, 1.0, 1.0), 16);
        let right = seam_safe_lattice_window(Vec3::new(0.5, 0.0, 0.0), Vec3::new(0.5, 1.0, 1.0), 16);

        for y in 0..=16 {
            for z in 0..=16 {
                let left_boundary = left.lattice_coord(16, y, z);
                let right_boundary = right.lattice_coord(0, y, z);
                assert_eq!(left_boundary, right_boundary);
            }
        }
    }

    #[test]
    fn phenomenon_mesh_builds_triangles_for_full_window() {
        let tuning = default_mandelbulb_tuning(8);
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);

        let mesh = build_windowed_mandelbulb_mesh(&proxy, tuning).expect("expected non-empty mesh");
        let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            panic!("mesh missing positions");
        };
        assert!(positions.len() > 0);
    }

    #[test]
    fn phenomenon_mesh_changes_with_window_signature() {
        let tuning = default_mandelbulb_tuning(8);
        let mut a = sample_proxy(RenderProxyWindowMode::WindowedSubsection, Vec3::ZERO, Vec3::splat(0.5));
        let sig_a = compute_mandelbulb_surface_signature(&a, tuning);
        a.window_center_local = Vec3::new(0.1, 0.0, 0.0);
        let sig_b = compute_mandelbulb_surface_signature(&a, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn surface_signature_tracks_window_mode() {
        let tuning = default_mandelbulb_tuning(8);
        let a = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        let b = RenderProxy {
            window_mode: RenderProxyWindowMode::WindowedSubsection,
            ..a
        };
        let sig_a = compute_mandelbulb_surface_signature(&a, tuning);
        let sig_b = compute_mandelbulb_surface_signature(&b, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn surface_signature_tracks_frontier_seed() {
        let tuning = default_mandelbulb_tuning(8);
        let mut a = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        let mut b = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        a.frontier_node_seed = 11;
        b.frontier_node_seed = 22;
        let sig_a = compute_mandelbulb_surface_signature(&a, tuning);
        let sig_b = compute_mandelbulb_surface_signature(&b, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn surface_signature_tracks_phenomenon_kind() {
        let lod = default_lod(8);
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        let mandelbulb_sig = compute_model_surface_signature(&proxy, PhenomenonKind::Mandelbulb, lod, |_| {});
        let sierpinski_sig = compute_model_surface_signature(&proxy, PhenomenonKind::SierpinskiSponge, lod, |_| {});
        assert_ne!(mandelbulb_sig, sierpinski_sig);
    }

    #[test]
    fn phenomenon_sierpinski_mesh_builds_triangles_for_full_window() {
        let tuning = default_sierpinski_tuning(8);
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);

        let mesh = build_windowed_sierpinski_sponge_mesh(&proxy, tuning).expect("expected non-empty mesh");
        let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            panic!("mesh missing positions");
        };
        assert!(positions.len() > 0);
    }

    #[test]
    fn phenomenon_sierpinski_signature_changes_with_window() {
        let tuning = default_sierpinski_tuning(8);
        let mut a = sample_proxy(RenderProxyWindowMode::WindowedSubsection, Vec3::ZERO, Vec3::splat(0.5));
        let sig_a = compute_sierpinski_sponge_surface_signature(&a, tuning);
        a.window_center_local = Vec3::new(-0.1, 0.1, 0.0);
        let sig_b = compute_sierpinski_sponge_surface_signature(&a, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn phenomenon_sierpinski_mesh_has_non_degenerate_triangles() {
        let tuning = default_sierpinski_tuning(30);
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        let mesh = build_windowed_sierpinski_sponge_mesh(&proxy, tuning).expect("expected non-empty mesh");
        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|values| values.as_float3())
            .expect("expected Float32x3 positions");
        assert!(positions.len() >= 3);
        assert_eq!(positions.len() % 3, 0);

        for tri in positions.chunks_exact(3) {
            let a = Vec3::from_array(tri[0]);
            let b = Vec3::from_array(tri[1]);
            let c = Vec3::from_array(tri[2]);
            let area2 = (b - a).cross(c - a).length();
            assert!(area2 > 1e-6, "degenerate triangle detected: {tri:?}");
        }
    }

    #[test]
    fn phenomenon_sierpinski_mesh_attributes_are_finite() {
        let tuning = default_sierpinski_tuning(30);
        let proxy = sample_proxy(RenderProxyWindowMode::WindowedSubsection, Vec3::new(0.2, -0.1, 0.05), Vec3::splat(0.2));
        let mesh = build_windowed_sierpinski_sponge_mesh(&proxy, tuning).expect("expected non-empty mesh");

        let positions = mesh
            .attribute(Mesh::ATTRIBUTE_POSITION)
            .and_then(|values| values.as_float3())
            .expect("expected Float32x3 positions");
        for p in positions {
            assert!(p.iter().all(|v| v.is_finite()));
        }

        let normals = mesh
            .attribute(Mesh::ATTRIBUTE_NORMAL)
            .and_then(|values| values.as_float3())
            .expect("expected Float32x3 normals");
        for n in normals {
            assert!(n.iter().all(|v| v.is_finite()));
        }
    }

    #[test]
    fn sierpinski_effective_iterations_track_mesh_resolution() {
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec3::ZERO, Vec3::ONE);
        let low_res = SierpinskiSpongeTuning {
            iterations: 6,
            domain_span: 1.0,
            hole_bias: 0.0,
            lod: default_lod(18),
        };
        let high_res = SierpinskiSpongeTuning {
            lod: default_lod(40),
            ..low_res
        };

        let low = compute_effective_sierpinski_iterations(&proxy, low_res);
        let high = compute_effective_sierpinski_iterations(&proxy, high_res);
        assert!(low >= 1);
        assert!(high >= low);
        assert!(high <= high_res.iterations);
    }

    #[test]
    fn interpolate_iso_midpoint_for_symmetric_values() {
        let p = interpolate_iso(Vec3::ZERO, 1.0, Vec3::splat(2.0), -1.0);
        assert!((p.x - 1.0).abs() < 1e-6);
        assert!((p.y - 1.0).abs() < 1e-6);
        assert!((p.z - 1.0).abs() < 1e-6);
    }

    #[test]
    fn mandelbulb_point_maps_mid_layer_center_sample_to_zero_z() {
        let p = map_model_space_to_mandelbulb_point(Vec3::new(0.5, 0.5, 0.5), 35, 1.25);
        assert!(p.z.abs() < 1e-6);
    }

    #[test]
    fn mandelbulb_field_is_finite() {
        let tuning = MandelbulbTuning {
            power: 8.0,
            iterations: 12,
            bailout: 4.0,
            z_span: 1.2,
            lod: SurfaceLodTuning {
                visibility_threshold: 0.2,
                scale_boost: 0.4,
                z_displacement: 120.0,
                mesh_resolution: 8,
                iso_level: 0.0,
            },
        };
        let field = mandelbulb_density_from_model_space(Vec3::new(0.5, 0.5, 0.5), 35, tuning);
        assert!(field.is_finite());
    }

    #[test]
    fn sierpinski_field_is_finite() {
        let field = sample_sierpinski_sponge_signed_distance(Vec3::new(0.1, -0.2, 0.15), 5, 0.0);
        assert!(field.is_finite());
    }

    #[test]
    fn sierpinski_center_is_removed() {
        let field = sample_sierpinski_sponge_signed_distance(Vec3::ZERO, 5, 0.0);
        assert!(field > 0.0);
    }

    #[test]
    fn sierpinski_corner_remains_solid() {
        let field = sample_sierpinski_sponge_signed_distance(Vec3::new(0.9, 0.9, 0.9), 1, 0.0);
        assert!(field < 0.0);
    }

    #[test]
    fn sierpinski_outside_cube_is_positive() {
        let field = sample_sierpinski_sponge_signed_distance(Vec3::new(1.4, 0.0, 0.0), 5, 0.0);
        assert!(field > 0.0);
    }

    #[test]
    fn mandelbulb_signed_distance_separates_center_and_far_point() {
        let power = 8.0;
        let iterations = 12;
        let bailout = 4.0;
        let center = sample_mandelbulb_signed_distance(Vec3::ZERO, power, iterations, bailout);
        let far = sample_mandelbulb_signed_distance(Vec3::new(2.5, 2.5, 2.5), power, iterations, bailout);
        assert!(center < 0.0);
        assert!(far > 0.0);
        assert!(center < far);
    }

    #[test]
    fn effective_lattice_cells_clamps_to_quantized_window_span() {
        let bounds = PhenomenonModelWindowBounds {
            min: Vec3::new(0.5, 0.5, 0.5),
            max: Vec3::new(0.500_000_05, 0.500_000_05, 0.500_000_05),
            span: Vec3::splat(0.000_000_05),
        };
        let effective = effective_lattice_cells_for_bounds(bounds, 32);
        assert_eq!(effective, 1);
    }

    #[test]
    fn zone_frontier_seed_selection_prefers_nearest_matching_scale_node() {
        let origin_offset = GridVec::new_root(GridXyz::new_local(0, 0, 0));
        let target_coord = GridVec::build().push((0, 0, 0)).push((2, 0, 0)).finish();
        let near_same_scale = sample_node(target_coord.scale, IVec3::new(2, 0, 0), 9001);
        let far_same_scale = sample_node(target_coord.scale, IVec3::new(-4, 0, 0), 9002);
        let closer_wrong_scale = sample_node(target_coord.scale.zoomed_out(), IVec3::new(0, 0, 0), 7777);
        let zone_center_native_local = phenomenon_node_center_native_local(&near_same_scale, &origin_offset).unwrap();

        let selected = select_frontier_seed_for_zone(
            [&near_same_scale, &far_same_scale, &closer_wrong_scale],
            target_coord.scale,
            zone_center_native_local,
            &origin_offset,
        )
        .unwrap();
        assert_eq!(selected.0, 9001);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::pos::grid::types::GridVec;

    #[test]
    fn camera_distance_decreases_monotonically_with_zoom() {
        let fov_min = 8.0;
        let fov_max = 100.0;

        let zoom_a = 0.1;
        let zoom_b = 1.0;
        let zoom_c = 10.0;

        let fov_a = zoom_to_fov_radians(zoom_a, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);
        let fov_b = zoom_to_fov_radians(zoom_b, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);
        let fov_c = zoom_to_fov_radians(zoom_c, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);

        let d_a = camera_distance_from_zoom_and_fov(zoom_a, fov_a);
        let d_b = camera_distance_from_zoom_and_fov(zoom_b, fov_b);
        let d_c = camera_distance_from_zoom_and_fov(zoom_c, fov_c);

        assert!(d_a > d_b && d_b > d_c, "expected d(0.1) > d(1.0) > d(10.0), got {d_a} >? {d_b} >? {d_c}");
    }

    #[test]
    fn camera_distance_and_fov_pair_produces_inverse_zoom_view_span() {
        let fov_min = 8.0;
        let fov_max = 100.0;

        let z1 = 1.0;
        let z10 = 10.0;

        let fov1 = zoom_to_fov_radians(z1, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);
        let fov10 = zoom_to_fov_radians(z10, CAMERA_EFFECTIVE_ZOOM_MIN, CAMERA_EFFECTIVE_ZOOM_MAX, fov_min, fov_max);

        let d1 = camera_distance_from_zoom_and_fov(z1, fov1);
        let d10 = camera_distance_from_zoom_and_fov(z10, fov10);

        let span1 = d1 * (fov1 * 0.5).tan();
        let span10 = d10 * (fov10 * 0.5).tan();
        let ratio = span1 / span10;

        assert!((ratio - 10.0).abs() < 0.01, "expected ~10x span ratio, got {ratio}");
    }

    #[test]
    fn chunk_wire_transform_handles_finer_coord_than_origin_scale() {
        let origin_offset = GridVec::build().push((0, 0, 0)).finish();
        let finer_coord = GridVec::build().push((0, 0, 0)).push((0, 0, 0)).finish();

        let transform = chunk_wire_transform(&finer_coord, &origin_offset, Quat::IDENTITY, Vec3::ZERO, 1.0, 1000.0, 0.1);

        assert!(transform.translation.is_finite());
        assert!(transform.scale.is_finite());
    }

    #[test]
    fn full_entity_mode_for_same_or_finer_scale() {
        let (mode, center, size) = compute_render_proxy_windowing(0, Vec3::ZERO, Vec3::new(123.0, -45.0, 12.0), 1.0);
        assert_eq!(mode, RenderProxyWindowMode::FullEntity);
        assert!(center.x >= -0.5 && center.x <= 0.5);
        assert!(center.y >= -0.5 && center.y <= 0.5);
        assert!(center.z >= -0.5 && center.z <= 0.5);
        assert_eq!(size, Vec3::ONE);
    }

    #[test]
    fn windowed_mode_scales_down_with_coarser_level() {
        let (mode, center, size) = compute_render_proxy_windowing(1, Vec3::ZERO, Vec3::ZERO, 1.0);
        assert_eq!(mode, RenderProxyWindowMode::WindowedSubsection);
        assert!((center.x - 0.05).abs() < 1e-6);
        assert!((center.y - 0.05).abs() < 1e-6);
        assert!((center.z - 0.05).abs() < 1e-6);
        assert!((size.x - 0.1).abs() < 1e-6);
        assert!((size.y - 0.1).abs() < 1e-6);
        assert!((size.z - 0.1).abs() < 1e-6);
    }

    #[test]
    fn window_center_tracks_viewpoint_inside_chunk() {
        let (mode, center, size) = compute_render_proxy_windowing(1, Vec3::ZERO, Vec3::new(2_500.0, 2_500.0, 2_500.0), 1.0);
        assert_eq!(mode, RenderProxyWindowMode::WindowedSubsection);
        assert!(center.x > 0.0 && center.y > 0.0 && center.z > 0.0);
        assert!((size.x - 0.1).abs() < 1e-6);
        assert!((size.y - 0.1).abs() < 1e-6);
        assert!((size.z - 0.1).abs() < 1e-6);
    }

    #[test]
    fn same_scale_windowing_preserves_full_entity_with_local_zoom() {
        let (mode, center, size) = compute_render_proxy_windowing(0, Vec3::ZERO, Vec3::ZERO, 0.5);
        assert_eq!(mode, RenderProxyWindowMode::FullEntity);
        assert!(center.length() < 1e-6);
        assert_eq!(size, Vec3::ONE);
    }

    #[test]
    fn finer_scale_windowing_composes_player_local_zoom() {
        let (mode, center, size) = compute_render_proxy_windowing(1, Vec3::ZERO, Vec3::ZERO, 0.5);
        assert_eq!(mode, RenderProxyWindowMode::WindowedSubsection);
        assert!(center.x.abs() <= 0.5 && center.y.abs() <= 0.5 && center.z.abs() <= 0.5);
        assert!((size.x - 0.05).abs() < 1e-6);
        assert!((size.y - 0.05).abs() < 1e-6);
        assert!((size.z - 0.05).abs() < 1e-6);
    }

    #[test]
    fn coarser_scale_windowing_ignores_player_local_zoom() {
        let (mode, _center, size) = compute_render_proxy_windowing(-1, Vec3::ZERO, Vec3::ZERO, 0.25);
        assert_eq!(mode, RenderProxyWindowMode::FullEntity);
        assert_eq!(size, Vec3::ONE);
    }

    #[test]
    fn planar_world_delta_preserves_magnitude_when_yawed() {
        let intent = Vec3::new(3.0, 4.0, 99.0);
        let delta = world_space_planar_delta_from_intent(intent, std::f32::consts::FRAC_PI_2);
        assert!((delta.length() - 5.0).abs() < 1e-5);
        assert!(delta.z.abs() < 1e-6);
    }

    #[test]
    fn planar_world_delta_ignores_vertical_only_intent() {
        let delta = world_space_planar_delta_from_intent(Vec3::new(0.0, 0.0, 42.0), 1.0);
        assert_eq!(delta, Vec3::ZERO);
    }
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
    mut zoom_factor: ResMut<ZoomFactor>,
    mut dev_zoom_factor: ResMut<DevZoomFactor>,
    mut zoom_initialized: Local<bool>,
) {
    const ZOOM_SCROLL_DEADZONE: f32 = 0.05;

    let local_min_zoom = CONFIG().get::<f32>("usf/scale/local_min").max(f32::EPSILON);
    let local_max_zoom = CONFIG().get::<f32>("usf/scale/local_max").max(local_min_zoom * 1.001);
    let local_zoom_center = (local_min_zoom * local_max_zoom).sqrt();
    let base_zoom_speed = CONFIG().get::<f32>("camera/base_zoom_speed").abs();
    let min_dev_zoom = CONFIG().get::<f32>("camera/min_dev_zoom").max(f32::EPSILON);
    let max_dev_zoom = CONFIG().get::<f32>("camera/max_dev_zoom").max(min_dev_zoom * 1.001);
    let dev_zoom_speed = CONFIG().get::<f32>("camera/dev_zoom_speed").abs();
    let chunk_load_gate_enabled = CONFIG().get::<bool>("workflow/chunk_load_gate_enabled");

    if !*zoom_initialized {
        zoom_factor.0 = local_zoom_center;
        *zoom_initialized = true;
    }

    if !input_mode.is_game() || virtual_paused.0 || (chunk_load_gate_enabled && chunk_load_gate.as_ref().is_some_and(|gate| gate.is_locked())) {
        scroll_message_reader.clear();
        return;
    }

    let shift_pressed = keys.pressed(KeyCode::ShiftLeft) || keys.pressed(KeyCode::ShiftRight);
    let base_zoom_step = (1.0 + base_zoom_speed * 0.01).max(1.001);
    let dev_zoom_step = (1.0 + dev_zoom_speed * 0.01).max(1.001);
    let input_local_zoom_min = local_min_zoom / base_zoom_step;
    let input_local_zoom_max = local_max_zoom * base_zoom_step;

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
            zoom_factor.0 = (zoom_factor.0 * zoom_multiplier).clamp(input_local_zoom_min, input_local_zoom_max);
        }
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
fn clamp_local_zoom_accumulator(local_zoom: f32, local_min: f32, local_max: f32) -> f32 {
    let min = local_min.max(f32::MIN_POSITIVE);
    let max = local_max.max(min * 1.001);
    let max_exclusive = (max - max.abs().max(1.0) * 1e-6).max(min * 1.001);
    local_zoom.clamp(min, max_exclusive)
}

#[inline]
fn apply_discrete_scale_rebase_events(
    chunk_loader: &mut ChunkLoader,
    local_zoom: &mut f32,
    logical_world_pos: &mut Vec3,
    local_min: f32,
    local_max: f32,
) -> (u32, u32) {
    let mut zoom_in_events = 0_u32;
    let mut zoom_out_events = 0_u32;
    let min = local_min.max(f32::MIN_POSITIVE);
    let max = local_max.max(min * 1.001);
    let pivot_factor = (max / min).max(1.001);

    // Perform at most one discrete rebase per frame to keep transitions continuous and debuggable.
    if *local_zoom < min {
        if chunk_loader.scale == Scale::MIN {
            *local_zoom = min;
        } else {
            *logical_world_pos = chunk_loader.zoom_in(*logical_world_pos);
            *local_zoom *= pivot_factor;
            zoom_in_events = 1;
        }
    } else if *local_zoom >= max {
        if chunk_loader.scale != Scale::MAX {
            *logical_world_pos = chunk_loader.zoom_out(*logical_world_pos);
            *local_zoom /= pivot_factor;
            zoom_out_events = 1;
        }
    }

    *local_zoom = clamp_local_zoom_accumulator(*local_zoom, min, max);
    (zoom_in_events, zoom_out_events)
}

#[tracing::instrument(skip_all)]
pub(super) fn apply_usf_player_pivots_system(
    mut zoom_factor: ResMut<ZoomFactor>,
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
    let translation_policy = chunk_loader.usf_transform.translation.policy;
    let translation_local_min = translation_policy.local_min as f32;
    let translation_local_max = translation_policy.local_max as f32;
    let rotation_policy = chunk_loader.usf_transform.rotation.policy;
    let rotation_local_min = rotation_policy.local_min;
    let rotation_local_max = rotation_policy.local_max;
    let workflow_in_flight = chunk_load_gate_enabled && workflow_state.as_ref().is_some_and(|state| !state.is_idle());

    if gate_locked {
        // Hard freeze mode: do not process additional pivot transitions while input is locked.
        zoom_factor.0 = clamp_local_zoom_accumulator(zoom_factor.0, local_min, local_max);
        chunk_loader.usf_transform.scale.set_local(zoom_factor.0 as f64);
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

        let would_cross_scale_boundary = zoom_factor.0 < local_min || zoom_factor.0 >= local_max;
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
            zoom_factor.0 = clamp_local_zoom_accumulator(zoom_factor.0, local_min, local_max);
            chunk_loader.usf_transform.scale.set_local(zoom_factor.0 as f64);
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

            let (zoom_in_events, zoom_out_events) =
                apply_discrete_scale_rebase_events(&mut chunk_loader, &mut zoom_factor.0, &mut player_transform.translation, local_min, local_max);
            chunk_loader.usf_transform.scale.set_local(zoom_factor.0 as f64);
            let translation_grid_delta = chunk_loader.apply_translation_pivot(&mut player_transform.translation);
            chunk_loader.apply_rotation_pivot();
            chunk_actor.coord = chunk_loader.coord.clone();

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
                zoom_factor.0 = clamp_local_zoom_accumulator(zoom_factor.0, local_min, local_max);
                chunk_loader.usf_transform.scale.set_local(zoom_factor.0 as f64);
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
                    chunk_loader.scale,
                    zoom_factor.0,
                    zoom_in_events,
                    zoom_out_events,
                    translation_grid_delta,
                    player_transform.translation
                );
            }
        }
    }

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
