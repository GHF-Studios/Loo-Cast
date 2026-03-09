use crate::bevy::asset::RenderAssetUsages;
use crate::bevy::input::mouse::{MouseScrollUnit, MouseWheel};
use crate::bevy::prelude::*;
use crate::bevy::render::render_resource::{Extent3d, PrimitiveTopology, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages};

use crate::chunk::components::{Chunk, ChunkActor, ChunkLoader};
use crate::chunk::resources::{ChunkActionWorkflowState, ChunkLoadGate};
use crate::config::statics::CONFIG;
use crate::core::protocol::PlayerMotionIntent;
use crate::input::states::InputMode;
use crate::player::components::Player;
use crate::render::{
    components::{
        EntityProxyLink, LogicProxy, MainCamera, PhenomenonModelCamera, PhenomenonModelSurface, ProxySyncRevision, RenderProxy, RenderProxyWindowMode, UiCamera,
    },
    functions::{PHENOMENON_MODEL_LOCAL_SPAN_UNITS, draw_primary_window_ui},
    materials::PhenomenonSurfaceMaterial,
    resources::{DevZoomFactor, GameViewRenderTarget, PrimaryWindowUiDockState, PrimaryWindowUiState, ViewScale, ZoomFactor},
};
use crate::time::resources::VirtualPaused;
use crate::usf::scale::Scale;
use std::hash::{Hash, Hasher};

const MIN_WINDOW_SIZE_LOCAL: f32 = 0.0001;

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

#[derive(Debug, Clone, Copy, PartialEq)]
struct PhenomenonModelWindowBounds {
    min: Vec2,
    max: Vec2,
    span: Vec2,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct MandelbulbTuning {
    power: f32,
    iterations: u32,
    bailout: f32,
    z_span: f32,
    visibility_threshold: f32,
    scale_boost: f32,
    z_displacement: f32,
    mesh_resolution: u32,
    iso_level: f32,
}

#[tracing::instrument(skip_all)]
pub(super) fn update_phenomenon_model_surfaces_system(
    time: Res<Time>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut surface_materials: ResMut<Assets<PhenomenonSurfaceMaterial>>,
    proxy_query: Query<(&Children, &RenderProxy)>,
    mut surface_query: Query<(
        &mut Mesh3d,
        &MeshMaterial3d<PhenomenonSurfaceMaterial>,
        &mut Transform,
        &mut Visibility,
        &mut PhenomenonModelSurface,
    )>,
) {
    let mandelbulb_enabled = CONFIG().get::<bool>("render/phenomenon_mandelbulb/enabled");
    let mandelbulb_tuning = if mandelbulb_enabled {
        Some(MandelbulbTuning {
            power: CONFIG().get::<f32>("render/phenomenon_mandelbulb/power").max(1.1),
            iterations: CONFIG().get::<u32>("render/phenomenon_mandelbulb/iterations").max(1),
            bailout: CONFIG().get::<f32>("render/phenomenon_mandelbulb/bailout").max(1.1),
            z_span: CONFIG().get::<f32>("render/phenomenon_mandelbulb/z_span").abs().max(0.01),
            visibility_threshold: CONFIG().get::<f32>("render/phenomenon_mandelbulb/visibility_threshold").clamp(0.0, 1.0),
            scale_boost: CONFIG().get::<f32>("render/phenomenon_mandelbulb/scale_boost").max(0.0),
            z_displacement: CONFIG().get::<f32>("render/phenomenon_mandelbulb/z_displacement"),
            mesh_resolution: CONFIG().get::<u32>("render/phenomenon_mandelbulb/mesh_resolution").clamp(6, 64),
            iso_level: CONFIG().get::<f32>("render/phenomenon_mandelbulb/iso_level").clamp(0.01, 0.99),
        })
    } else {
        None
    };

    for (children, proxy) in proxy_query.iter() {
        for child in children.iter() {
            let Ok((mut mesh3d, material3d, mut transform, mut visibility, mut surface_state)) = surface_query.get_mut(child) else {
                continue;
            };

            let Some(tuning) = mandelbulb_tuning else {
                *visibility = Visibility::Hidden;
                continue;
            };

            *transform = phenomenon_surface_transform(proxy, tuning);
            if let Some(surface_material) = surface_materials.get_mut(&material3d.0) {
                update_surface_material_params(surface_material, proxy, tuning, time.elapsed_secs());
            }

            let signature = compute_surface_signature(proxy, tuning);
            if signature != surface_state.last_signature {
                surface_state.last_signature = signature;
                if let Some(mesh) = build_windowed_mandelbulb_mesh(proxy, tuning) {
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
fn compute_effective_mesh_resolution(proxy: &RenderProxy, tuning: MandelbulbTuning) -> usize {
    let base_resolution = tuning.mesh_resolution as f32;
    let window_scale = proxy_window_scale(proxy);
    let layer_t = layer_norm(proxy.layer_index);
    let window_boost = 1.0 + (1.0 - window_scale) * 2.0;
    let layer_boost = 0.8 + layer_t * 0.8;
    (base_resolution * window_boost * layer_boost).round().clamp(6.0, 64.0) as usize
}

#[inline]
fn phenomenon_surface_transform(proxy: &RenderProxy, tuning: MandelbulbTuning) -> Transform {
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
fn update_surface_material_params(surface_material: &mut PhenomenonSurfaceMaterial, proxy: &RenderProxy, tuning: MandelbulbTuning, time_seconds: f32) {
    let layer_t = layer_norm(proxy.layer_index);
    let window_scale = proxy_window_scale(proxy);
    let shimmer = (time_seconds * 0.22 + layer_t * std::f32::consts::TAU).sin() * 0.5 + 0.5;

    let primary = Vec3::new(0.14, 0.48, 0.95).lerp(Vec3::new(0.22, 0.74, 0.98), layer_t);
    let secondary = Vec3::new(0.98, 0.58, 0.28).lerp(Vec3::new(0.96, 0.84, 0.52), 1.0 - layer_t);
    let glow = Vec3::new(0.26, 0.86, 1.0).lerp(Vec3::new(0.58, 1.0, 0.78), shimmer);
    let emissive_strength = (0.25 + tuning.visibility_threshold * 1.2).clamp(0.0, 2.0);

    surface_material.params.primary = primary.extend(1.0);
    surface_material.params.secondary = secondary.extend(1.0);
    surface_material.params.glow = glow.extend(1.0);
    surface_material.params.meta = Vec4::new(layer_t, window_scale, time_seconds, emissive_strength);
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
    sample_mandelbulb_density(point, tuning.power, tuning.iterations, tuning.bailout)
}

#[inline]
fn map_model_space_to_mandelbulb_point(local_uv: Vec2, local_w: f32, layer_index: u8, z_span: f32) -> Vec3 {
    let uv = local_uv.clamp(Vec2::ZERO, Vec2::ONE);
    let w = local_w.clamp(0.0, 1.0);
    let x = (uv.x - 0.5) * 3.0;
    let y = (uv.y - 0.5) * 3.0;
    let local_z = (w - 0.5) * 2.0 * z_span;

    let layer_t = layer_norm(layer_index);
    let layer_bias = (layer_t * 2.0 - 1.0) * z_span * 0.15;

    Vec3::new(x, y, local_z + layer_bias)
}

#[inline]
fn sample_mandelbulb_density(c: Vec3, power: f32, iterations: u32, bailout: f32) -> f32 {
    let mut z = c;
    let mut escaped_at = iterations;

    for i in 0..iterations {
        let r = z.length();
        if r > bailout {
            escaped_at = i;
            break;
        }

        let theta = if r > 1e-6 { (z.z / r).clamp(-1.0, 1.0).acos() } else { 0.0 };
        let phi = z.y.atan2(z.x);
        let zr = r.powf(power);
        let theta_p = theta * power;
        let phi_p = phi * power;

        z = Vec3::new(theta_p.sin() * phi_p.cos(), theta_p.sin() * phi_p.sin(), theta_p.cos()) * zr + c;
    }

    if escaped_at >= iterations {
        1.0
    } else {
        (1.0 - (escaped_at as f32 / iterations as f32)).clamp(0.0, 1.0)
    }
}

#[inline]
fn compute_surface_signature(proxy: &RenderProxy, tuning: MandelbulbTuning) -> u64 {
    #[inline]
    fn q(v: f32) -> i32 {
        (v * 10_000.0).round() as i32
    }

    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    proxy.layer_index.hash(&mut hasher);
    proxy.window_mode.hash(&mut hasher);
    q(proxy.window_center_local.x).hash(&mut hasher);
    q(proxy.window_center_local.y).hash(&mut hasher);
    q(proxy.window_size_local.x).hash(&mut hasher);
    q(proxy.window_size_local.y).hash(&mut hasher);
    compute_effective_mesh_resolution(proxy, tuning).hash(&mut hasher);
    q(tuning.iso_level).hash(&mut hasher);
    q(tuning.power).hash(&mut hasher);
    tuning.iterations.hash(&mut hasher);
    q(tuning.bailout).hash(&mut hasher);
    q(tuning.z_span).hash(&mut hasher);
    q(tuning.visibility_threshold).hash(&mut hasher);
    q(tuning.scale_boost).hash(&mut hasher);
    q(tuning.z_displacement).hash(&mut hasher);
    hasher.finish()
}

#[inline]
fn grid_index(ix: usize, iy: usize, iz: usize, axis_points: usize) -> usize {
    ix + iy * axis_points + iz * axis_points * axis_points
}

fn build_windowed_mandelbulb_mesh(proxy: &RenderProxy, tuning: MandelbulbTuning) -> Option<Mesh> {
    let cells = compute_effective_mesh_resolution(proxy, tuning);
    let axis_points = cells + 1;
    let bounds = compute_phenomenon_window_bounds(proxy.window_mode, proxy.window_center_local, proxy.window_size_local);

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
                let density = mandelbulb_density_from_model_space(sample_uv, w, proxy.layer_index, tuning);
                field[idx] = density - tuning.iso_level;
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
        if values[i] >= 0.0 {
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

    let mut push_triangle = |a: Vec3, b: Vec3, c: Vec3| {
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
    };

    let edge_point = |a_i: usize, b_i: usize| interpolate_iso(points[a_i], values[a_i], points[b_i], values[b_i]);

    match inside_count {
        1 => {
            let i = inside[0];
            let a = outside[0];
            let b = outside[1];
            let c = outside[2];
            push_triangle(edge_point(i, a), edge_point(i, b), edge_point(i, c));
        }
        3 => {
            let o = outside[0];
            let a = inside[0];
            let b = inside[1];
            let c = inside[2];
            push_triangle(edge_point(o, a), edge_point(o, c), edge_point(o, b));
        }
        2 => {
            let a = inside[0];
            let b = inside[1];
            let c = outside[0];
            let d = outside[1];

            let p0 = edge_point(a, c);
            let p1 = edge_point(b, c);
            let p2 = edge_point(b, d);
            let p3 = edge_point(a, d);
            push_triangle(p0, p1, p2);
            push_triangle(p0, p2, p3);
        }
        _ => {}
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

    #[test]
    fn effective_mesh_resolution_increases_for_smaller_window() {
        let tuning = MandelbulbTuning {
            power: 8.0,
            iterations: 10,
            bailout: 4.0,
            z_span: 1.2,
            visibility_threshold: 0.45,
            scale_boost: 0.8,
            z_displacement: 200.0,
            mesh_resolution: 12,
            iso_level: 0.45,
        };
        let broad = RenderProxy {
            source: Entity::PLACEHOLDER,
            layer_index: 20,
            depth_bias: 0.0,
            window_mode: RenderProxyWindowMode::WindowedSubsection,
            window_center_local: Vec2::ZERO,
            window_size_local: Vec2::splat(0.9),
            coarse_context_persistent: true,
        };
        let narrow = RenderProxy {
            window_size_local: Vec2::splat(0.1),
            ..broad
        };
        assert!(compute_effective_mesh_resolution(&narrow, tuning) > compute_effective_mesh_resolution(&broad, tuning));
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
        let tuning = MandelbulbTuning {
            power: 8.0,
            iterations: 10,
            bailout: 4.0,
            z_span: 1.2,
            visibility_threshold: 0.45,
            scale_boost: 0.8,
            z_displacement: 200.0,
            mesh_resolution: 8,
            iso_level: 0.45,
        };
        let proxy = RenderProxy {
            source: Entity::PLACEHOLDER,
            layer_index: 35,
            depth_bias: 0.0,
            window_mode: RenderProxyWindowMode::FullEntity,
            window_center_local: Vec2::ZERO,
            window_size_local: Vec2::ONE,
            coarse_context_persistent: true,
        };

        let mesh = build_windowed_mandelbulb_mesh(&proxy, tuning).expect("expected non-empty mesh");
        let Some(positions) = mesh.attribute(Mesh::ATTRIBUTE_POSITION) else {
            panic!("mesh missing positions");
        };
        assert!(positions.len() > 0);
    }

    #[test]
    fn phenomenon_mesh_changes_with_window_signature() {
        let tuning = MandelbulbTuning {
            power: 8.0,
            iterations: 10,
            bailout: 4.0,
            z_span: 1.2,
            visibility_threshold: 0.45,
            scale_boost: 0.8,
            z_displacement: 200.0,
            mesh_resolution: 8,
            iso_level: 0.45,
        };
        let mut a = RenderProxy {
            source: Entity::PLACEHOLDER,
            layer_index: 35,
            depth_bias: 0.0,
            window_mode: RenderProxyWindowMode::WindowedSubsection,
            window_center_local: Vec2::ZERO,
            window_size_local: Vec2::splat(0.5),
            coarse_context_persistent: true,
        };
        let sig_a = compute_surface_signature(&a, tuning);
        a.window_center_local = Vec2::new(0.1, 0.0);
        let sig_b = compute_surface_signature(&a, tuning);
        assert_ne!(sig_a, sig_b);
    }

    #[test]
    fn surface_signature_tracks_window_mode() {
        let tuning = MandelbulbTuning {
            power: 8.0,
            iterations: 10,
            bailout: 4.0,
            z_span: 1.2,
            visibility_threshold: 0.45,
            scale_boost: 0.8,
            z_displacement: 200.0,
            mesh_resolution: 8,
            iso_level: 0.45,
        };
        let a = RenderProxy {
            source: Entity::PLACEHOLDER,
            layer_index: 35,
            depth_bias: 0.0,
            window_mode: RenderProxyWindowMode::FullEntity,
            window_center_local: Vec2::ZERO,
            window_size_local: Vec2::ONE,
            coarse_context_persistent: true,
        };
        let b = RenderProxy {
            window_mode: RenderProxyWindowMode::WindowedSubsection,
            ..a
        };
        let sig_a = compute_surface_signature(&a, tuning);
        let sig_b = compute_surface_signature(&b, tuning);
        assert_ne!(sig_a, sig_b);
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
    fn mandelbulb_density_is_normalized() {
        let tuning = MandelbulbTuning {
            power: 8.0,
            iterations: 12,
            bailout: 4.0,
            z_span: 1.2,
            visibility_threshold: 0.2,
            scale_boost: 0.4,
            z_displacement: 120.0,
            mesh_resolution: 8,
            iso_level: 0.45,
        };
        let d = mandelbulb_density_from_model_space(Vec2::new(0.5, 0.5), 0.5, 35, tuning);
        assert!((0.0..=1.0).contains(&d));
    }

    #[test]
    fn mandelbulb_density_prefers_origin_over_far_point() {
        let power = 8.0;
        let iterations = 12;
        let bailout = 4.0;
        let center = sample_mandelbulb_density(Vec3::ZERO, power, iterations, bailout);
        let far = sample_mandelbulb_density(Vec3::new(2.5, 2.5, 2.5), power, iterations, bailout);
        assert!(center >= far);
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
