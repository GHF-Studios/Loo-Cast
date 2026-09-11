//! Ordinary geometric portals.
//!
//! This module intentionally knows nothing about USF manifestation.
//!
//! A portal is currently a conventional game mechanic:
//!
//! - an entity exists at exactly one world-space `Transform`;
//! - crossing a portal teleports that transform;
//! - portal cameras render the destination before the crossing happens.
//!
//! That discontinuity is precisely what later USF work will attack.

use std::f32::consts::PI;

use bevy::{
    asset::{load_internal_asset, uuid_handle},
    camera::{RenderTarget, visibility::RenderLayers},
    prelude::*,
    reflect::TypePath,
    render::render_resource::{AsBindGroup, Extent3d, TextureFormat},
    shader::{Shader, ShaderRef},
    window::{PrimaryWindow, WindowResized},
};

use super::{GameSet, SimulationSet};

const WORLD_LAYER: usize = 0;
pub const MAIN_PORTAL_LAYER: usize = 1;

const MAX_RECURSION_DEPTH: u8 = 4;

const PORTAL_SHADER: Handle<Shader> =
    uuid_handle!("14bc976d-50ec-4c03-a025-2d39405bb2fa");

/// Configures the built-in portal pair.
///
/// `visual_recursion_depth` counts portal traversals visible inside another
/// portal view. Cost grows exponentially, so deliberately keep it small.
#[derive(Resource, Debug, Clone)]
pub struct PortalConfig {
    pub size: Vec2,
    pub visual_recursion_depth: u8,
    pub render_scale: f32,
    pub first: Transform,
    pub second: Transform,
}

impl Default for PortalConfig {
    fn default() -> Self {
        Self {
            size: Vec2::new(2.5, 3.5),
            visual_recursion_depth: 2,
            render_scale: 0.65,

            // Faces roughly toward the initial player position.
            first: Transform::from_xyz(-3.5, 1.75, -3.0),

            // Faces toward -X.
            second: Transform::from_xyz(4.5, 1.75, -1.0)
                .with_rotation(Quat::from_rotation_y(
                    -std::f32::consts::FRAC_PI_2,
                )),
        }
    }
}

/// One portal aperture.
///
/// The local +Z direction is considered the portal's front.
#[derive(Component, Debug, Clone, Copy)]
pub struct Portal {
    pub destination: Entity,
    pub half_size: Vec2,
}

/// The portal pair created by the built-in test scene.
#[derive(Resource, Debug, Clone, Copy)]
pub struct PortalPair {
    pub first: Entity,
    pub second: Entity,
}

/// Marks the ordinary camera from whose perspective portal views are derived.
#[derive(Component)]
pub struct PortalView;

/// Marks something whose movement may cross portals.
///
/// The previous position exists solely for continuous plane-crossing
/// detection; portal traversal is based on a segment, not mere overlap.
#[derive(Component, Debug, Default)]
pub struct PortalTraveler {
    previous_position: Option<Vec3>,
}

impl PortalTraveler {
    pub fn new(position: Vec3) -> Self {
        Self {
            previous_position: Some(position),
        }
    }
}

/// Optional linear velocity transformed alongside a portal traveler.
///
/// The portal mechanic does not care whether this belongs to a projectile,
/// rigid body, spell, particle, or anything else.
#[derive(Component, Debug, Clone, Copy)]
pub struct PortalVelocity(pub Vec3);

#[derive(Component)]
struct PortalRenderCamera {
    /// Binary-tree node identifying the sequence of traversed portals.
    node: usize,
}

#[derive(Resource)]
struct PortalRenderTargets(Vec<Handle<Image>>);

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct PortalMaterial {
    #[texture(0)]
    #[sampler(1)]
    texture: Handle<Image>,
}

impl Material for PortalMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(PORTAL_SHADER.clone())
    }
}

pub struct PortalPlugin;

impl Plugin for PortalPlugin {
    fn build(&self, app: &mut App) {
        load_internal_asset!(
            app,
            PORTAL_SHADER,
            "portal.wgsl",
            Shader::from_wgsl
        );

        app.init_resource::<PortalConfig>()
            .add_plugins(MaterialPlugin::<PortalMaterial>::default())
            .add_systems(Startup, setup_portals)
            .add_systems(
                Update,
                teleport_travelers.in_set(SimulationSet::Topology),
            )
            .add_systems(
                Update,
                (
                    update_portal_cameras,
                    resize_render_targets,
                )
                    .in_set(GameSet::Presentation),
            );
    }
}

fn setup_portals(
    mut commands: Commands,
    config: Res<PortalConfig>,
    window: Single<&Window, With<PrimaryWindow>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut portal_materials: ResMut<Assets<PortalMaterial>>,
    mut images: ResMut<Assets<Image>>,
) {
    assert!(
        config.visual_recursion_depth <= MAX_RECURSION_DEPTH,
        "portal recursion depth exceeds {MAX_RECURSION_DEPTH}"
    );

    assert!(
        config.size.x > 0.0 && config.size.y > 0.0,
        "portal size must be positive"
    );

    let first = commands.spawn_empty().id();
    let second = commands.spawn_empty().id();

    commands.entity(first).insert((
        Name::new("Portal A"),
        Portal {
            destination: second,
            half_size: config.size / 2.0,
        },
        config.first,
    ));

    commands.entity(second).insert((
        Name::new("Portal B"),
        Portal {
            destination: first,
            half_size: config.size / 2.0,
        },
        config.second,
    ));

    let pair = PortalPair { first, second };
    commands.insert_resource(pair);

    let vertical =
        meshes.add(Cuboid::new(0.12, config.size.y + 0.12, 0.12));

    let horizontal =
        meshes.add(Cuboid::new(config.size.x + 0.12, 0.12, 0.12));

    spawn_frame(
        &mut commands,
        first,
        vertical.clone(),
        horizontal.clone(),
        materials.add(Color::srgb(0.1, 0.35, 1.0)),
        config.size,
    );

    spawn_frame(
        &mut commands,
        second,
        vertical,
        horizontal,
        materials.add(Color::srgb(1.0, 0.35, 0.05)),
        config.size,
    );

    let surface_mesh =
        meshes.add(Rectangle::new(config.size.x, config.size.y));

    let terminal_material = materials.add(StandardMaterial {
        base_color: Color::BLACK,
        unlit: true,
        ..default()
    });

    let render_size =
        render_size(&window, config.render_scale);

    let mut targets = Vec::new();

    build_render_node(
        &mut commands,
        pair,
        1,
        0,
        config.visual_recursion_depth,
        &surface_mesh,
        &terminal_material,
        render_size,
        &mut portal_materials,
        &mut images,
        &mut targets,
    );

    commands.insert_resource(PortalRenderTargets(targets));
}

/// Builds a binary render tree.
///
/// Node 1 is the main camera. Its children are the views through A and B.
/// Every portal camera gets another two children until recursion terminates.
///
/// This is intentionally a true tree rather than a single alternating camera
/// chain: either portal can appear inside any preceding portal view.
#[allow(clippy::too_many_arguments)]
fn build_render_node(
    commands: &mut Commands,
    pair: PortalPair,
    node: usize,
    depth: u8,
    max_depth: u8,
    mesh: &Handle<Mesh>,
    terminal_material: &Handle<StandardMaterial>,
    render_size: UVec2,
    materials: &mut Assets<PortalMaterial>,
    images: &mut Assets<Image>,
    targets: &mut Vec<Handle<Image>>,
) {
    for (side, portal) in [
        (0usize, pair.first),
        (1usize, pair.second),
    ] {
        if depth == max_depth {
            spawn_terminal_surface(
                commands,
                portal,
                node,
                mesh,
                terminal_material,
            );

            continue;
        }

        let child = node * 2 + side;

        let image = images.add(Image::new_target_texture(
            render_size.x,
            render_size.y,
            TextureFormat::Rgba8Unorm,
            Some(TextureFormat::Rgba8UnormSrgb),
        ));

        targets.push(image.clone());

        let material = materials.add(PortalMaterial {
            texture: image.clone(),
        });

        spawn_portal_surface(
            commands,
            portal,
            node,
            mesh,
            material,
        );

        commands.spawn((
            Name::new(format!("Portal Camera {child}")),
            Camera3d::default(),
            Camera {
                // Deeper dependencies must render first.
                order: -(node_depth(child) as isize),
                clear_color: Color::srgb(0.015, 0.015, 0.02).into(),
                ..default()
            },
            RenderTarget::Image(image.into()),
            Projection::Perspective(PerspectiveProjection::default()),
            Transform::default(),
            RenderLayers::layer(WORLD_LAYER).with(child),
            PortalRenderCamera { node: child },
        ));

        build_render_node(
            commands,
            pair,
            child,
            depth + 1,
            max_depth,
            mesh,
            terminal_material,
            render_size,
            materials,
            images,
            targets,
        );
    }
}

fn spawn_frame(
    commands: &mut Commands,
    portal: Entity,
    vertical: Handle<Mesh>,
    horizontal: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    size: Vec2,
) {
    commands.entity(portal).with_children(|parent| {
        for x in [-size.x / 2.0, size.x / 2.0] {
            parent.spawn((
                Mesh3d(vertical.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(x, 0.0, 0.0),
            ));
        }

        for y in [-size.y / 2.0, size.y / 2.0] {
            parent.spawn((
                Mesh3d(horizontal.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_xyz(0.0, y, 0.0),
            ));
        }
    });
}

fn spawn_portal_surface(
    commands: &mut Commands,
    portal: Entity,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: Handle<PortalMaterial>,
) {
    commands.entity(portal).with_children(|parent| {
        parent.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material),
            Transform::from_xyz(0.0, 0.0, 0.002),
            RenderLayers::layer(layer),
        ));
    });
}

fn spawn_terminal_surface(
    commands: &mut Commands,
    portal: Entity,
    layer: usize,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
) {
    commands.entity(portal).with_children(|parent| {
        parent.spawn((
            Mesh3d(mesh.clone()),
            MeshMaterial3d(material.clone()),
            Transform::from_xyz(0.0, 0.0, 0.002),
            RenderLayers::layer(layer),
        ));
    });
}

/// Teleports ordinary concrete Bevy entities.
///
/// No logical duplication occurs. Before crossing there is one entity on one
/// side; after crossing the same entity has a different transform.
fn teleport_travelers(
    portals: Query<(Entity, &Portal, &Transform)>,
    mut travelers: Query<(
        &mut Transform,
        &mut PortalTraveler,
        Option<&mut PortalVelocity>,
    ), Without<Portal>>,
) {
    for (mut transform, mut traveler, mut velocity) in &mut travelers {
        let current = transform.translation;

        let Some(previous) = traveler.previous_position else {
            traveler.previous_position = Some(current);
            continue;
        };

        let mut crossed = None;

        for (portal_entity, portal, source) in &portals {
            let Ok((_, _, destination)) =
                portals.get(portal.destination)
            else {
                continue;
            };

            let inverse = source.to_matrix().inverse();

            let previous_local =
                inverse.transform_point3(previous);

            let current_local =
                inverse.transform_point3(current);

            // Enter only from the portal's front.
            if previous_local.z <= 0.0 || current_local.z > 0.0 {
                continue;
            }

            let denominator =
                previous_local.z - current_local.z;

            if denominator.abs() <= f32::EPSILON {
                continue;
            }

            let fraction =
                previous_local.z / denominator;

            let crossing =
                previous_local.lerp(current_local, fraction);

            if crossing.x.abs() > portal.half_size.x
                || crossing.y.abs() > portal.half_size.y
            {
                continue;
            }

            crossed = Some((
                portal_entity,
                *source,
                *destination,
            ));

            break;
        }

        if let Some((_portal, source, destination)) = crossed {
            let mapping =
                portal_mapping(&source, &destination);

            *transform = Transform::from_matrix(
                mapping * transform.to_matrix(),
            );

            if let Some(velocity) = velocity.as_deref_mut() {
                velocity.0 =
                    mapping.transform_vector3(velocity.0);
            }
        }

        // Record the post-topology position, preventing an immediate
        // re-trigger at the destination portal.
        traveler.previous_position =
            Some(transform.translation);
    }
}

/// Keeps every recursively rendered camera equivalent to the primary view
/// after applying its sequence of portal traversals.
fn update_portal_cameras(
    mut cameras: ParamSet<(
        Single<
            (&Transform, &Projection),
            (
                With<PortalView>,
                Without<PortalRenderCamera>,
                Without<Portal>,
            ),
        >,
        Query<
            (
                &PortalRenderCamera,
                &mut Transform,
                &mut Projection,
            ),
            (
                Without<PortalView>,
                Without<Portal>,
            ),
        >,
    )>,
    pair: Res<PortalPair>,
    portals: Query<&Transform, With<Portal>>,
) {
    let (primary_transform, primary_projection) = {
        let primary = cameras.p0();
        let (transform, projection) = primary.into_inner();
        (*transform, projection.clone())
    };

    let Projection::Perspective(primary_projection) =
        primary_projection
    else {
        return;
    };

    let Ok(first) = portals.get(pair.first) else {
        return;
    };

    let Ok(second) = portals.get(pair.second) else {
        return;
    };

    for (camera, mut transform, mut projection) in &mut cameras.p1() {
        let (mapped, destination) =
            map_camera_path(
                &primary_transform,
                camera.node,
                first,
                second,
            );

        let mut perspective = primary_projection.clone();

        perspective.near_clip_plane =
            portal_clip_plane(&mapped, destination);

        *transform = mapped;
        *projection = Projection::Perspective(perspective);
    }
}

/// Resizes all recursive render targets while preserving their handles.
///
/// Because portal sampling uses normalized screen coordinates, the render
/// texture may intentionally run below native window resolution.
fn resize_render_targets(
    mut events: MessageReader<WindowResized>,
    window: Single<&Window, With<PrimaryWindow>>,
    config: Res<PortalConfig>,
    targets: Option<Res<PortalRenderTargets>>,
    mut images: ResMut<Assets<Image>>,
) {
    if events.read().next().is_none() {
        return;
    }

    let Some(targets) = targets else {
        return;
    };

    let size = render_size(&window, config.render_scale);

    let extent = Extent3d {
        width: size.x,
        height: size.y,
        depth_or_array_layers: 1,
    };

    for handle in &targets.0 {
        if let Some(mut image) = images.get_mut(handle) {
            image.resize(extent);
        }
    }
}

/// Maps a transform from the front of `source` to the front of `destination`.
///
/// The half-turn converts motion into the source portal into motion out of the
/// destination portal.
fn portal_mapping(
    source: &Transform,
    destination: &Transform,
) -> Mat4 {
    destination.to_matrix()
        * Mat4::from_rotation_y(PI)
        * source.to_matrix().inverse()
}

fn map_camera_path<'a>(
    primary: &Transform,
    node: usize,
    first: &'a Transform,
    second: &'a Transform,
) -> (Transform, &'a Transform) {
    let mut transform = *primary;
    let mut destination = first;

    let depth = node_depth(node);

    for shift in (0..depth).rev() {
        let second_portal = (node >> shift) & 1 == 1;

        let (source, target) = if second_portal {
            (second, first)
        } else {
            (first, second)
        };

        transform = Transform::from_matrix(
            portal_mapping(source, target)
                * transform.to_matrix(),
        );

        destination = target;
    }

    (transform, destination)
}

/// Replaces the ordinary near plane with the destination portal plane.
///
/// Bevy 0.19's perspective projection explicitly exposes this for portal and
/// mirror rendering.
fn portal_clip_plane(
    camera: &Transform,
    destination: &Transform,
) -> Vec4 {
    let mut normal_world =
        destination.rotation * Vec3::Z;

    let plane_point =
        destination.translation + normal_world * 0.01;

    let camera_to_plane =
        plane_point - camera.translation;

    // The projection API requires the normal to point away from the camera.
    if normal_world.dot(camera_to_plane) < 0.0 {
        normal_world = -normal_world;
    }

    let view_rotation = camera.rotation.inverse();

    let normal_view =
        (view_rotation * normal_world).normalize();

    let point_view =
        view_rotation * camera_to_plane;

    normal_view.extend(
        -normal_view.dot(point_view),
    )
}

fn render_size(
    window: &Window,
    scale: f32,
) -> UVec2 {
    let scale = scale.clamp(0.1, 1.0);

    UVec2::new(
        ((window.physical_width() as f32 * scale).round() as u32)
            .max(1),
        ((window.physical_height() as f32 * scale).round() as u32)
            .max(1),
    )
}

fn node_depth(node: usize) -> usize {
    usize::BITS as usize
        - node.leading_zeros() as usize
        - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn opposite_traversals_are_inverse() {
        let first = Transform::from_xyz(-3.0, 1.0, 4.0)
            .with_rotation(Quat::from_rotation_y(0.3));

        let second = Transform::from_xyz(7.0, 2.0, -1.0)
            .with_rotation(Quat::from_rotation_y(-1.2));

        let round_trip =
            portal_mapping(&second, &first)
                * portal_mapping(&first, &second);

        assert!(round_trip.abs_diff_eq(Mat4::IDENTITY, 0.0001));
    }
}