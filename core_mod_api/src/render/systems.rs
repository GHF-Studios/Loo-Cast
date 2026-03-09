use crate::bevy::asset::RenderAssetUsages;
use crate::bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{Extent3d, PrimitiveTopology, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use crate::chunk::components::{ChunkActor, ChunkLoader};
use crate::chunk::resources::{ChunkActionWorkflowState, ChunkLoadGate};
use crate::config::statics::CONFIG;
use crate::core::protocol::PlayerMotionIntent;
use crate::input::states::InputMode;
use crate::player::components::Player;
use crate::render::{
    components::{
        EntityProxyLink, GlobalPhenomenonRoot, LogicProxy, MainCamera, PhenomenonModelCamera, PhenomenonModelSurface, ProxySyncRevision, RenderProxy,
        RenderProxyWindowMode, UiCamera,
    },
    functions::{PHENOMENON_MODEL_LOCAL_SPAN_UNITS, draw_primary_window_ui, new_phenomenon_model_proxy_bundle},
    materials::PhenomenonSurfaceMaterial,
    resources::{DevZoomFactor, GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState, ViewScale, ZoomFactor},
};
use crate::time::resources::VirtualPaused;
use crate::usf::phenomenon::{Phenomenon, PhenomenonId, PhenomenonKind, PhenomenonModel};
use crate::usf::pos::grid::types::GridVec;
use crate::usf::scale::Scale;
use std::hash::{Hash, Hasher};

const MIN_WINDOW_SIZE_LOCAL: f32 = 0.0001;

#[inline]
fn configured_default_phenomenon_kind() -> PhenomenonKind {
    let configured = CONFIG().get::<String>("render/phenomenon/default_kind");
    PhenomenonKind::from_config_value(&configured)
}

pub(super) fn pre_setup_phase_0(mut commands: Commands, mut images: ResMut<Assets<Image>>, windows: Query<&Window>) {
    // Reserve camera entities
    let egui_camera = commands.spawn(()).id();
    let ui_camera = commands.spawn(UiCamera).id();
    let main_camera = commands.spawn(MainCamera).id();
    let phenomenon_model_camera = commands.spawn(PhenomenonModelCamera).id();
    super::functions::reserve_camera_entities(egui_camera, ui_camera, main_camera, phenomenon_model_camera);

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

#[tracing::instrument(skip_all)]
pub(super) fn ensure_global_phenomenon_root_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut phenomenon_surface_materials: ResMut<Assets<PhenomenonSurfaceMaterial>>,
    global_phenomenon_root_query: Query<Entity, With<GlobalPhenomenonRoot>>,
    player_loader_query: Query<&ChunkLoader, With<Player>>,
) {
    if !global_phenomenon_root_query.is_empty() {
        return;
    }

    let source_scale = player_loader_query.single().map(|loader| loader.coord.scale).unwrap_or(Scale::MAX);
    let depth_bias = CONFIG().get::<i8>("chunk/z_offset") as f32;
    let phenomenon_kind = configured_default_phenomenon_kind();

    let phenomenon_entity = commands
        .spawn((
            Name::new("global_phenomenon"),
            Phenomenon {
                id: PhenomenonId(0),
                kind: phenomenon_kind,
            },
        ))
        .id();

    let phenomenon_render_proxy_entity = commands
        .spawn((
            Name::new("global_phenomenon_render_proxy"),
            GlobalPhenomenonRoot,
            new_phenomenon_model_proxy_bundle(Vec2::ZERO, 1.0, phenomenon_entity, source_scale, depth_bias),
        ))
        .id();

    let surface_mesh = meshes.add(Mesh::from(Cuboid::from_size(Vec3::splat(PHENOMENON_MODEL_LOCAL_SPAN_UNITS))));
    let surface_material = phenomenon_surface_materials.add(PhenomenonSurfaceMaterial::for_phenomenon_kind(phenomenon_kind));

    commands.entity(phenomenon_render_proxy_entity).with_children(|parent| {
        parent.spawn((
            Name::new("global_phenomenon_surface"),
            Mesh3d(surface_mesh),
            MeshMaterial3d(surface_material),
            Transform::default(),
            Visibility::Visible,
            PhenomenonModelSurface::default(),
        ));
    });

    commands.entity(phenomenon_render_proxy_entity).insert(PhenomenonModel {
        phenomenon_entity,
        scale: source_scale,
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
        let mut proxy_transforms = params.p2();
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
}

#[tracing::instrument(skip_all)]
pub(super) fn update_global_phenomenon_proxy_system(
    zoom_factor: Res<ZoomFactor>,
    dev_zoom_factor: Res<DevZoomFactor>,
    mut params: ParamSet<(
        Single<(&ChunkLoader, &Transform), With<Player>>,
        Query<(&mut Transform, &mut RenderProxy), With<GlobalPhenomenonRoot>>,
    )>,
) {
    let (world_rotation, world_rotation_origin, coord_scale, scale_diff, view_pos_native, camera_zoom) = {
        let (chunk_loader, chunk_loader_transform) = *params.p0();
        let world_rotation = chunk_loader.world_rotation_quat();
        let world_rotation_origin = chunk_loader_transform.translation;
        let local_view_pos = Vec2::new(
            chunk_loader.usf_transform.translation.x.local as f32,
            chunk_loader.usf_transform.translation.y.local as f32,
        );
        let grid_origin_at_current_scale = GridVec::new_at_scale(chunk_loader.origin_offset.scale, IVec2::ZERO);
        let coarse_view_pos = chunk_loader.origin_offset.clone().to_native_logical(grid_origin_at_current_scale);
        let view_pos_native = coarse_view_pos + local_view_pos;
        let camera_zoom = (zoom_factor.0 * dev_zoom_factor.0).max(f32::EPSILON);

        // One global phenomenon in 3D: scale progression selects a deeper subsection window,
        // but does not spawn/scale independent world objects.
        let coord_scale = chunk_loader.coord.scale;
        let scale_diff = coord_scale.index_from_top() as i8;

        (world_rotation, world_rotation_origin, coord_scale, scale_diff, view_pos_native, camera_zoom)
    };

    let mut global_proxy_query = params.p1();
    for (mut proxy_transform, mut proxy_state) in global_proxy_query.iter_mut() {
        // Keep the global phenomenon anchored to the player frame in XYZ;
        // only the subsection window changes across USF boundaries.
        proxy_transform.translation = world_rotation_origin + world_rotation * Vec3::new(0.0, 0.0, proxy_state.depth_bias);
        proxy_transform.scale = Vec3::ONE;
        proxy_transform.rotation = world_rotation;
        proxy_state.layer_index = coord_scale.render_layer_index();
        let (window_mode, window_center_local, window_size_local) = compute_render_proxy_windowing(scale_diff, camera_zoom, Vec2::ZERO, view_pos_native);
        proxy_state.window_mode = window_mode;
        proxy_state.window_center_local = window_center_local;
        proxy_state.window_size_local = window_size_local;
        proxy_state.coarse_context_persistent = true;
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct PhenomenonModelWindowBounds {
    min: Vec2,
    max: Vec2,
    span: Vec2,
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

    fn build_mesh(self, proxy: &RenderProxy) -> Option<Mesh> {
        match self {
            Self::Mandelbulb(tuning) => build_windowed_mandelbulb_mesh(proxy, tuning),
            Self::SierpinskiSponge(tuning) => build_windowed_sierpinski_sponge_mesh(proxy, tuning),
        }
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
    proxy_query: Query<(&Children, &RenderProxy, &PhenomenonModel)>,
    phenomenon_query: Query<&Phenomenon>,
    mut surface_query: Query<(
        &mut Mesh3d,
        &MeshMaterial3d<PhenomenonSurfaceMaterial>,
        &mut Transform,
        &mut Visibility,
        &mut PhenomenonModelSurface,
    )>,
) {
    for (children, proxy, phenomenon_model) in proxy_query.iter() {
        let model = phenomenon_query
            .get(phenomenon_model.phenomenon_entity)
            .ok()
            .and_then(|phenomenon| PhenomenonGeometryModel::from_kind(phenomenon.kind));

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
                surface_state.last_signature = signature;
                if let Some(mesh) = model.build_mesh(proxy) {
                    if let Some(existing) = meshes.get_mut(&mesh3d.0) {
                        *existing = mesh;
                    } else {
                        mesh3d.0 = meshes.add(mesh);
                    }
                    *visibility = Visibility::Visible;
                } else {
                    *visibility = Visibility::Hidden;
                }
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
fn proxy_window_scale(proxy: &RenderProxy) -> f32 {
    proxy.window_size_local.abs().max_element().clamp(MIN_WINDOW_SIZE_LOCAL, 1.0)
}

#[inline]
fn compute_effective_mesh_resolution(proxy: &RenderProxy, base_mesh_resolution: u32) -> usize {
    let base_resolution = base_mesh_resolution as f32;
    let window_scale = proxy_window_scale(proxy);
    // Keep detail stable across scale layers; only window size should drive dynamic tessellation.
    let window_boost = 1.0 + (1.0 - window_scale) * 1.5;
    (base_resolution * window_boost).round().clamp(8.0, 40.0) as usize
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
fn compute_phenomenon_window_bounds(window_mode: RenderProxyWindowMode, window_center_local: Vec2, window_size_local: Vec2) -> PhenomenonModelWindowBounds {
    if matches!(window_mode, RenderProxyWindowMode::FullEntity) {
        return PhenomenonModelWindowBounds {
            min: Vec2::ZERO,
            max: Vec2::ONE,
            span: Vec2::ONE,
        };
    }

    let center01 = window_center_local.clamp(Vec2::splat(-0.5), Vec2::splat(0.5)) + Vec2::splat(0.5);
    let size01 = window_size_local.abs().clamp(Vec2::splat(MIN_WINDOW_SIZE_LOCAL), Vec2::ONE);
    let mut window_min = (center01 - size01 * 0.5).clamp(Vec2::ZERO, Vec2::ONE);
    let mut window_max = (center01 + size01 * 0.5).clamp(Vec2::ZERO, Vec2::ONE);
    if window_max.x < window_min.x {
        std::mem::swap(&mut window_max.x, &mut window_min.x);
    }
    if window_max.y < window_min.y {
        std::mem::swap(&mut window_max.y, &mut window_min.y);
    }
    let span = (window_max - window_min).max(Vec2::splat(MIN_WINDOW_SIZE_LOCAL));

    PhenomenonModelWindowBounds {
        min: window_min,
        max: window_max,
        span,
    }
}

#[inline]
fn mandelbulb_density_from_model_space(local_uv: Vec2, local_w: f32, layer_index: u8, tuning: MandelbulbTuning) -> f32 {
    let point = map_model_space_to_mandelbulb_point(local_uv, local_w, layer_index, tuning.z_span);
    sample_mandelbulb_signed_distance(point, tuning.power, tuning.iterations, tuning.bailout)
}

#[inline]
fn map_model_space_to_mandelbulb_point(local_uv: Vec2, local_w: f32, layer_index: u8, z_span: f32) -> Vec3 {
    let uv = local_uv.clamp(Vec2::ZERO, Vec2::ONE);
    let w = local_w.clamp(0.0, 1.0);
    let x = (uv.x - 0.5) * 3.0;
    let y = (uv.y - 0.5) * 3.0;
    let local_z = (w - 0.5) * 2.0 * z_span;

    // Keep one coherent global fractal across all scales; do not offset the sampled slice per layer.
    let _ = layer_index;
    let layer_bias = 0.0;

    Vec3::new(x, y, local_z + layer_bias)
}

#[inline]
fn sierpinski_sponge_density_from_model_space(local_uv: Vec2, local_w: f32, layer_index: u8, tuning: SierpinskiSpongeTuning) -> f32 {
    let point = map_model_space_to_sierpinski_point(local_uv, local_w, layer_index, tuning.domain_span);
    sample_sierpinski_sponge_signed_distance(point, tuning.iterations, tuning.hole_bias)
}

#[inline]
fn map_model_space_to_sierpinski_point(local_uv: Vec2, local_w: f32, layer_index: u8, domain_span: f32) -> Vec3 {
    let uv = local_uv.clamp(Vec2::ZERO, Vec2::ONE);
    let w = local_w.clamp(0.0, 1.0);
    let x = (uv.x - 0.5) * 2.0 * domain_span;
    let y = (uv.y - 0.5) * 2.0 * domain_span;
    let z = (w - 0.5) * 2.0 * domain_span;

    // Keep one coherent global fractal across all scales; do not offset the sampled slice per layer.
    let _ = layer_index;
    let layer_bias = 0.0;

    Vec3::new(x, y, z + layer_bias)
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MeshingWindow {
    center_local: Vec2,
    size_local: Vec2,
    resolution: usize,
}

#[inline]
fn compute_meshing_window(proxy: &RenderProxy, base_mesh_resolution: u32) -> MeshingWindow {
    let resolution = compute_effective_mesh_resolution(proxy, base_mesh_resolution);

    if matches!(proxy.window_mode, RenderProxyWindowMode::FullEntity) {
        return MeshingWindow {
            center_local: Vec2::ZERO,
            size_local: Vec2::ONE,
            resolution,
        };
    }

    let size_local = proxy.window_size_local.abs().clamp(Vec2::splat(MIN_WINDOW_SIZE_LOCAL), Vec2::ONE);
    let step_local = (size_local / resolution as f32).max(Vec2::splat(MIN_WINDOW_SIZE_LOCAL));

    // Snap subsection center to marching-grid voxels, so tiny camera deltas don't trigger full remesh every frame.
    let mut center_local = (proxy.window_center_local / step_local).round() * step_local;
    let min_center = Vec2::splat(-0.5) + size_local * 0.5;
    let max_center = Vec2::splat(0.5) - size_local * 0.5;
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
        let cell = Vec3::new(
            p.x.rem_euclid(2.0) - 1.0,
            p.y.rem_euclid(2.0) - 1.0,
            p.z.rem_euclid(2.0) - 1.0,
        );
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
    proxy.layer_index.hash(&mut hasher);
    proxy.window_mode.hash(&mut hasher);
    quantized_signature_value(meshing_window.center_local.x).hash(&mut hasher);
    quantized_signature_value(meshing_window.center_local.y).hash(&mut hasher);
    quantized_signature_value(meshing_window.size_local.x).hash(&mut hasher);
    quantized_signature_value(meshing_window.size_local.y).hash(&mut hasher);
    meshing_window.resolution.hash(&mut hasher);
    hash_lod_fields(&mut hasher, lod);
    hash_model(&mut hasher);
    hasher.finish()
}

#[inline]
fn grid_index(ix: usize, iy: usize, iz: usize, axis_points: usize) -> usize {
    ix + iy * axis_points + iz * axis_points * axis_points
}

fn build_windowed_mandelbulb_mesh(proxy: &RenderProxy, tuning: MandelbulbTuning) -> Option<Mesh> {
    build_windowed_field_mesh(proxy, tuning.lod, |sample_uv, sample_w, layer_index| {
        mandelbulb_density_from_model_space(sample_uv, sample_w, layer_index, tuning)
    })
}

fn build_windowed_sierpinski_sponge_mesh(proxy: &RenderProxy, tuning: SierpinskiSpongeTuning) -> Option<Mesh> {
    let effective_iterations = compute_effective_sierpinski_iterations(proxy, tuning);
    let tuned = SierpinskiSpongeTuning {
        iterations: effective_iterations,
        ..tuning
    };
    build_windowed_field_mesh(proxy, tuning.lod, |sample_uv, sample_w, layer_index| {
        sierpinski_sponge_density_from_model_space(sample_uv, sample_w, layer_index, tuned)
    })
}

#[inline]
fn compute_effective_sierpinski_iterations(proxy: &RenderProxy, tuning: SierpinskiSpongeTuning) -> u32 {
    let cells = compute_effective_mesh_resolution(proxy, tuning.lod.mesh_resolution).max(1) as f32;
    // A sponge subdivision factor is 3, so detail depth should track log3(grid cells)
    // to avoid unresolved micro-structure aliasing into triangle jitter.
    let max_resolvable_depth = cells.log(3.0).floor().max(1.0) as u32;
    tuning.iterations.clamp(1, max_resolvable_depth)
}

fn build_windowed_field_mesh<F>(proxy: &RenderProxy, lod: SurfaceLodTuning, mut density_from_model_space: F) -> Option<Mesh>
where
    F: FnMut(Vec2, f32, u8) -> f32,
{
    let meshing_window = compute_meshing_window(proxy, lod.mesh_resolution);
    let cells = meshing_window.resolution;
    let axis_points = cells + 1;
    let bounds = compute_phenomenon_window_bounds(proxy.window_mode, meshing_window.center_local, meshing_window.size_local);

    let mut points = vec![Vec3::ZERO; axis_points * axis_points * axis_points];
    let mut field = vec![0.0f32; axis_points * axis_points * axis_points];

    for iz in 0..axis_points {
        let w = iz as f32 / cells as f32;
        for iy in 0..axis_points {
            let v = iy as f32 / cells as f32;
            for ix in 0..axis_points {
                let u = ix as f32 / cells as f32;
                let idx = grid_index(ix, iy, iz, axis_points);
                let sample_uv = bounds.min + Vec2::new(u, v) * bounds.span;
                let signed_distance = density_from_model_space(sample_uv, w, proxy.layer_index);
                field[idx] = signed_distance - lod.iso_level;
                points[idx] = Vec3::new(
                    (u - 0.5) * PHENOMENON_MODEL_LOCAL_SPAN_UNITS,
                    (v - 0.5) * PHENOMENON_MODEL_LOCAL_SPAN_UNITS,
                    (w - 0.5) * PHENOMENON_MODEL_LOCAL_SPAN_UNITS,
                );
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

fn emit_tetra_surface(points: [Vec3; 4], values: [f32; 4], out_positions: &mut Vec<[f32; 3]>, out_normals: &mut Vec<[f32; 3]>, out_uvs: &mut Vec<[f32; 2]>) {
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
fn compute_render_proxy_windowing(scale_diff: i8, camera_zoom: f32, chunk_center_native: Vec2, view_pos_native: Vec2) -> (RenderProxyWindowMode, Vec2, Vec2) {
    if scale_diff <= 0 {
        return (RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);
    }

    let coarse_factor = 10.0_f64.powi(scale_diff as i32);
    if !coarse_factor.is_finite() || coarse_factor <= 0.0 {
        return (RenderProxyWindowMode::WindowedSubsection, Vec2::ZERO, Vec2::splat(0.001));
    }

    let chunk_span = 1000.0_f64 * coarse_factor;
    let center01_x = ((view_pos_native.x as f64 - (chunk_center_native.x as f64 - chunk_span * 0.5)) / chunk_span).clamp(0.0, 1.0) as f32;
    let center01_y = ((view_pos_native.y as f64 - (chunk_center_native.y as f64 - chunk_span * 0.5)) / chunk_span).clamp(0.0, 1.0) as f32;
    let center_local = Vec2::new(center01_x, center01_y) - Vec2::splat(0.5);

    let zoom_term = camera_zoom.max(0.0001) as f64;
    let window_size = (zoom_term / coarse_factor).clamp(MIN_WINDOW_SIZE_LOCAL as f64, 1.0) as f32;

    (RenderProxyWindowMode::WindowedSubsection, center_local, Vec2::splat(window_size))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    fn sample_proxy(window_mode: RenderProxyWindowMode, window_center_local: Vec2, window_size_local: Vec2) -> RenderProxy {
        RenderProxy {
            source: Entity::PLACEHOLDER,
            layer_index: 35,
            depth_bias: 0.0,
            window_mode,
            window_center_local,
            window_size_local,
            coarse_context_persistent: true,
        }
    }

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

    #[test]
    fn effective_mesh_resolution_increases_for_smaller_window() {
        let broad = sample_proxy(RenderProxyWindowMode::WindowedSubsection, Vec2::ZERO, Vec2::splat(0.9));
        let narrow = RenderProxy {
            window_size_local: Vec2::splat(0.1),
            ..broad
        };
        assert!(compute_effective_mesh_resolution(&narrow, 12) > compute_effective_mesh_resolution(&broad, 12));
    }

    #[test]
    fn phenomenon_window_full_entity_uses_unit_bounds() {
        let bounds = compute_phenomenon_window_bounds(RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);
        assert_eq!(bounds.min, Vec2::ZERO);
        assert_eq!(bounds.max, Vec2::ONE);
        assert_eq!(bounds.span, Vec2::ONE);
    }

    #[test]
    fn phenomenon_windowed_subsection_bounds_clamp_and_span() {
        let bounds = compute_phenomenon_window_bounds(RenderProxyWindowMode::WindowedSubsection, Vec2::ZERO, Vec2::splat(0.5));
        assert!((bounds.min.x - 0.25).abs() < 1e-6);
        assert!((bounds.min.y - 0.25).abs() < 1e-6);
        assert!((bounds.max.x - 0.75).abs() < 1e-6);
        assert!((bounds.max.y - 0.75).abs() < 1e-6);
        assert!((bounds.span.x - 0.5).abs() < 1e-6);
        assert!((bounds.span.y - 0.5).abs() < 1e-6);
    }

    #[test]
    fn phenomenon_mesh_builds_triangles_for_full_window() {
        let tuning = default_mandelbulb_tuning(8);
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);

        let mesh = build_windowed_mandelbulb_mesh(&proxy, tuning).expect("expected non-empty mesh");
        let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            panic!("mesh missing positions");
        };
        assert!(positions.len() > 0);
    }

    #[test]
    fn phenomenon_mesh_changes_with_window_signature() {
        let tuning = default_mandelbulb_tuning(8);
        let mut a = sample_proxy(RenderProxyWindowMode::WindowedSubsection, Vec2::ZERO, Vec2::splat(0.5));
        let sig_a = compute_mandelbulb_surface_signature(&a, tuning);
        a.window_center_local = Vec2::new(0.1, 0.0);
        let sig_b = compute_mandelbulb_surface_signature(&a, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn surface_signature_tracks_window_mode() {
        let tuning = default_mandelbulb_tuning(8);
        let a = sample_proxy(RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);
        let b = RenderProxy {
            window_mode: RenderProxyWindowMode::WindowedSubsection,
            ..a
        };
        let sig_a = compute_mandelbulb_surface_signature(&a, tuning);
        let sig_b = compute_mandelbulb_surface_signature(&b, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn surface_signature_tracks_phenomenon_kind() {
        let lod = default_lod(8);
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);
        let mandelbulb_sig = compute_model_surface_signature(&proxy, PhenomenonKind::Mandelbulb, lod, |_| {});
        let sierpinski_sig = compute_model_surface_signature(&proxy, PhenomenonKind::SierpinskiSponge, lod, |_| {});
        assert_ne!(mandelbulb_sig, sierpinski_sig);
    }

    #[test]
    fn phenomenon_sierpinski_mesh_builds_triangles_for_full_window() {
        let tuning = default_sierpinski_tuning(8);
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);

        let mesh = build_windowed_sierpinski_sponge_mesh(&proxy, tuning).expect("expected non-empty mesh");
        let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            panic!("mesh missing positions");
        };
        assert!(positions.len() > 0);
    }

    #[test]
    fn phenomenon_sierpinski_signature_changes_with_window() {
        let tuning = default_sierpinski_tuning(8);
        let mut a = sample_proxy(RenderProxyWindowMode::WindowedSubsection, Vec2::ZERO, Vec2::splat(0.5));
        let sig_a = compute_sierpinski_sponge_surface_signature(&a, tuning);
        a.window_center_local = Vec2::new(-0.1, 0.1);
        let sig_b = compute_sierpinski_sponge_surface_signature(&a, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn sierpinski_effective_iterations_track_mesh_resolution() {
        let proxy = sample_proxy(RenderProxyWindowMode::FullEntity, Vec2::ZERO, Vec2::ONE);
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
        let p = map_model_space_to_mandelbulb_point(Vec2::new(0.5, 0.5), 0.5, 35, 1.25);
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
        let field = mandelbulb_density_from_model_space(Vec2::new(0.5, 0.5), 0.5, 35, tuning);
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
}

#[tracing::instrument(skip_all)]
pub(super) fn enforce_main_camera_depth_contract_system(
    mut main_camera_query: Query<(&mut Transform, &mut Projection), (With<MainCamera>, Without<Player>)>,
    player_transform_query: Query<&Transform, (With<Player>, Without<MainCamera>)>,
) {
    let Ok((mut camera_transform, mut projection)) = main_camera_query.single_mut() else {
        return;
    };

    camera_transform.translation.z = player_transform_query
        .single()
        .map(|transform| transform.translation.z + Scale::CANONICAL_Z_SPACING)
        .unwrap_or(Scale::CANONICAL_CAMERA_Z);

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
pub(super) fn enforce_phenomenon_model_camera_depth_contract_system(
    mut phenomenon_model_camera_query: Query<(&mut Transform, &mut Projection), With<PhenomenonModelCamera>>,
) {
    let Ok((mut camera_transform, mut projection)) = phenomenon_model_camera_query.single_mut() else {
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

            // Zoom should not drag the player in local XY.
            // Preserve XY across scale folds, then run translation/rotation pivots normally.
            let local_xy_before_scale = player_transform.translation.truncate();
            let scale_pivot = chunk_loader.apply_scale_pivot(&mut zoom_factor.0, &mut player_transform.translation);
            if scale_pivot.lower_crossings > 0 || scale_pivot.upper_crossings > 0 {
                player_transform.translation.x = local_xy_before_scale.x;
                player_transform.translation.y = local_xy_before_scale.y;
            }
            let translation_grid_delta = chunk_loader.apply_translation_pivot(&mut player_transform.translation);
            chunk_loader.apply_rotation_pivot();
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
            }
        }
    }

    // Keep commit-buffer accumulation internal. Rendering should never show values outside strict local bounds.
    let display_zoom = zoom_factor.0.clamp(local_min, local_max);
    let camera_zoom = (display_zoom * dev_zoom_factor.0).max(f32::EPSILON);

    // Keep player visual scale stable; zoom should control camera framing, not player mesh size.
    player_transform.scale = Vec3::ONE;

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
