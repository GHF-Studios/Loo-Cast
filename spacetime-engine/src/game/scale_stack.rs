//! Demand-driven visible realization of positive USF spatial scales.
//!
//! The observer owns transition demand; this realizer owns only the disposable
//! presentation entities needed to satisfy that demand. Semantic state remains
//! in worldgen. Distance LOD is intentionally a separate future demand source.

use std::collections::{HashMap, HashSet};

use bevy::{
    asset::RenderAssetUsages,
    camera::visibility::NoFrustumCulling,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{
    spatial::{SpatialScale, UsfPosition, UsfScalePresentation, UsfViewFrame},
    voxel::{ProceduralTerrain, VoxelQueryPosition},
};

const EARTH_RADIUS_M: f64 = 6_371_000.0;
const AU_M: f64 = 149_597_870_700.0;
const SUN_RADIUS_M: f64 = 696_340_000.0;
const GALACTIC_CENTER_DISTANCE_M: f64 = 2.46e20;
const MILKY_WAY_RADIUS_M: f64 = 4.75e20;

#[derive(Debug, Clone)]
struct ScaleStackAssets {
    unit_sphere: Handle<Mesh>,
    landscape_material: Handle<StandardMaterial>,
    planet_material: Handle<StandardMaterial>,
    star_material: Handle<StandardMaterial>,
    galaxy_material: Handle<StandardMaterial>,
    cosmic_material: Handle<StandardMaterial>,
}

/// One semantic source capable of realizing observer-scale presentations.
///
/// `active` is cache/lifecycle state only. It is never semantic identity.
#[derive(Component)]
pub(super) struct ProceduralScaleStack {
    anchor: UsfPosition,
    terrain: ProceduralTerrain,
    active: HashMap<SpatialScale, Entity>,
    assets: Option<ScaleStackAssets>,
}

impl ProceduralScaleStack {
    pub(super) fn new(anchor: UsfPosition, terrain: ProceduralTerrain) -> Self {
        Self {
            anchor,
            terrain,
            active: HashMap::new(),
            assets: None,
        }
    }
}

pub(super) fn sync_scale_stack(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    view: Res<UsfViewFrame>,
    mut stacks: Query<(Entity, &mut ProceduralScaleStack)>,
) {
    for (world_entity, mut stack) in &mut stacks {
        if stack.assets.is_none() {
            stack.assets = Some(create_assets(&mut meshes, &mut materials));
        }

        // S0 is realized by the voxel system. This realizer owns only positive
        // authored scales currently participating in the observer transition.
        let desired = view
            .active_scale_demands()
            .into_iter()
            .flatten()
            .filter(|demand| demand.scale() > SpatialScale::ZERO && demand.contribution() > 0.001)
            .map(|demand| demand.scale())
            .collect::<HashSet<_>>();

        let stale = stack
            .active
            .iter()
            .filter_map(|(scale, entity)| (!desired.contains(scale)).then_some((*scale, *entity)))
            .collect::<Vec<_>>();
        for (scale, entity) in stale {
            stack.active.remove(&scale);
            commands.entity(entity).despawn();
        }

        let missing = desired
            .iter()
            .copied()
            .filter(|scale| !stack.active.contains_key(scale))
            .collect::<Vec<_>>();
        for scale in missing {
            let root = spawn_scale_representation(
                &mut commands,
                &mut meshes,
                world_entity,
                stack.anchor,
                stack.terrain,
                scale,
                stack.assets.as_ref().expect("scale stack assets initialized"),
            );
            stack.active.insert(scale, root);
        }
    }
}

fn create_assets(
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) -> ScaleStackAssets {
    ScaleStackAssets {
        unit_sphere: meshes.add(Sphere::new(1.0)),
        landscape_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.18, 0.31, 0.16),
            perceptual_roughness: 1.0,
            ..default()
        }),
        planet_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.16, 0.27, 0.48),
            perceptual_roughness: 0.8,
            ..default()
        }),
        star_material: materials.add(StandardMaterial {
            base_color: Color::srgb(1.0, 0.82, 0.48),
            perceptual_roughness: 0.6,
            ..default()
        }),
        galaxy_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.55, 0.62, 0.86),
            perceptual_roughness: 0.9,
            ..default()
        }),
        cosmic_material: materials.add(StandardMaterial {
            base_color: Color::srgb(0.38, 0.30, 0.58),
            perceptual_roughness: 1.0,
            ..default()
        }),
    }
}

fn spawn_scale_representation(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    parent: Entity,
    anchor: UsfPosition,
    terrain: ProceduralTerrain,
    scale: SpatialScale,
    assets: &ScaleStackAssets,
) -> Entity {
    let root = commands
        .spawn((
            Name::new(format!("USF S{scale} Active Presentation")),
            ChildOf(parent),
            UsfScalePresentation::new(anchor, scale),
            Transform::IDENTITY,
            Visibility::Inherited,
        ))
        .id();

    match scale.exponent() {
        1..=3 => {
            let mesh = coarse_landscape_mesh(terrain, anchor, scale);
            commands.spawn((
                Name::new(format!("S{scale} Landscape")),
                ChildOf(root),
                Mesh3d(meshes.add(mesh)),
                MeshMaterial3d(assets.landscape_material.clone()),
                NoFrustumCulling,
                Transform::IDENTITY,
                Visibility::Inherited,
            ));
        }
        4..=9 => spawn_planetary_body(
            commands,
            root,
            &assets.unit_sphere,
            &assets.planet_material,
            scale,
        ),
        10..=13 => spawn_planetary_system(
            commands,
            root,
            &assets.unit_sphere,
            &assets.star_material,
            &assets.planet_material,
            scale,
        ),
        14..=17 => spawn_stellar_neighborhood(
            commands,
            root,
            &assets.unit_sphere,
            &assets.star_material,
            scale,
        ),
        18..=19 => spawn_galaxy(
            commands,
            root,
            &assets.unit_sphere,
            &assets.galaxy_material,
            scale,
        ),
        20..=24 => spawn_cosmic_web(
            commands,
            root,
            &assets.unit_sphere,
            &assets.cosmic_material,
            scale,
        ),
        25..=35 => spawn_cosmological_field(
            commands,
            root,
            &assets.unit_sphere,
            &assets.cosmic_material,
            scale,
        ),
        _ => {}
    }

    root
}

fn native_units(meters: f64, scale: SpatialScale) -> f32 {
    (meters / 10.0_f64.powi(scale.exponent() as i32)) as f32
}

fn spawn_sphere(
    commands: &mut Commands,
    parent: Entity,
    name: impl Into<String>,
    mesh: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    translation: Vec3,
    scale: Vec3,
) {
    commands.spawn((
        Name::new(name.into()),
        ChildOf(parent),
        Mesh3d(mesh.clone()),
        MeshMaterial3d(material.clone()),
        NoFrustumCulling,
        Transform {
            translation,
            scale,
            ..default()
        },
        Visibility::Inherited,
    ));
}

fn spawn_planetary_body(
    commands: &mut Commands,
    root: Entity,
    sphere: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    scale: SpatialScale,
) {
    let radius = native_units(EARTH_RADIUS_M, scale).max(0.000_001);
    spawn_sphere(
        commands,
        root,
        format!("S{scale} Planet"),
        sphere,
        material,
        Vec3::NEG_Y * radius,
        Vec3::splat(radius),
    );
}

fn spawn_planetary_system(
    commands: &mut Commands,
    root: Entity,
    sphere: &Handle<Mesh>,
    star_material: &Handle<StandardMaterial>,
    planet_material: &Handle<StandardMaterial>,
    scale: SpatialScale,
) {
    let au = native_units(AU_M, scale);
    let sun_radius = native_units(SUN_RADIUS_M, scale).max(0.01);
    let star = Vec3::new(-au, 0.0, 0.0);

    spawn_sphere(
        commands,
        root,
        format!("S{scale} Host Star"),
        sphere,
        star_material,
        star,
        Vec3::splat(sun_radius.max(0.03)),
    );

    let orbits = [0.39_f32, 0.72, 1.0, 1.52, 5.2, 9.58, 19.2, 30.05];
    for (index, orbit) in orbits.into_iter().enumerate() {
        let angle = index as f32 * 1.73;
        let position = star + Vec3::new(angle.cos(), 0.0, angle.sin()) * (au * orbit);
        let visual_radius =
            (0.012 + index as f32 * 0.0015).max(native_units(EARTH_RADIUS_M, scale));
        spawn_sphere(
            commands,
            root,
            format!("S{scale} Planet {}", index + 1),
            sphere,
            planet_material,
            position,
            Vec3::splat(visual_radius),
        );
    }
}

fn spawn_stellar_neighborhood(
    commands: &mut Commands,
    root: Entity,
    sphere: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    scale: SpatialScale,
) {
    for i in 0..32 {
        let f = i as f32;
        let radius = 30.0 + (f * 37.0).rem_euclid(270.0);
        let position = Vec3::new(
            (f * 2.17).sin() * radius,
            (f * 1.31).sin() * radius * 0.35,
            (f * 0.93).cos() * radius,
        );
        let star_size = 0.35 + (f * 0.17).sin().abs() * 0.8;
        spawn_sphere(
            commands,
            root,
            format!("S{scale} Stellar Node {i}"),
            sphere,
            material,
            position,
            Vec3::splat(star_size),
        );
    }
}

fn spawn_galaxy(
    commands: &mut Commands,
    root: Entity,
    sphere: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    scale: SpatialScale,
) {
    let center = Vec3::NEG_X * native_units(GALACTIC_CENTER_DISTANCE_M, scale);
    let radius = native_units(MILKY_WAY_RADIUS_M, scale).max(1.0);

    for i in 0..48 {
        let t = i as f32 / 48.0;
        let arm = (i % 4) as f32 * std::f32::consts::FRAC_PI_2;
        let angle = arm + t * std::f32::consts::TAU * 2.4;
        let r = radius * (0.08 + 0.9 * t);
        let position = center
            + Vec3::new(
                angle.cos() * r,
                (angle * 3.0).sin() * radius * 0.025,
                angle.sin() * r,
            );
        let blob = (radius * 0.055).max(0.15);
        spawn_sphere(
            commands,
            root,
            format!("S{scale} Galactic Arm Blob {i}"),
            sphere,
            material,
            position,
            Vec3::new(blob * 1.8, blob * 0.35, blob),
        );
    }

    spawn_sphere(
        commands,
        root,
        format!("S{scale} Galactic Core"),
        sphere,
        material,
        center,
        Vec3::new(radius * 0.15, radius * 0.06, radius * 0.15),
    );
}

fn spawn_cosmic_web(
    commands: &mut Commands,
    root: Entity,
    sphere: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    scale: SpatialScale,
) {
    for i in 0..28 {
        let f = i as f32;
        let x = -420.0 + f * 31.0;
        let position = Vec3::new(
            x,
            (f * 0.71).sin() * 90.0,
            (f * 0.43).sin() * 220.0 + (f * 1.31).cos() * 60.0,
        );
        let node = 8.0 + (f * 0.83).sin().abs() * 22.0;
        spawn_sphere(
            commands,
            root,
            format!("S{scale} Cosmic Web Node {i}"),
            sphere,
            material,
            position,
            Vec3::new(node * 1.8, node * 0.8, node),
        );
    }
}

fn spawn_cosmological_field(
    commands: &mut Commands,
    root: Entity,
    sphere: &Handle<Mesh>,
    material: &Handle<StandardMaterial>,
    scale: SpatialScale,
) {
    for i in 0..20 {
        let f = i as f32 + scale.exponent() as f32 * 0.37;
        let position = Vec3::new(
            (f * 1.17).sin() * 420.0,
            (f * 0.77).cos() * 360.0,
            (f * 1.91).sin() * 440.0,
        );
        let extent = 20.0 + (f * 0.61).sin().abs() * 75.0;
        spawn_sphere(
            commands,
            root,
            format!("S{scale} Statistical Density Region {i}"),
            sphere,
            material,
            position,
            Vec3::new(extent * 1.6, extent, extent * 1.25),
        );
    }
}

fn coarse_landscape_mesh(
    terrain: ProceduralTerrain,
    anchor: UsfPosition,
    scale: SpatialScale,
) -> Mesh {
    const CELLS: u32 = 64;
    const HALF_EXTENT_NATIVE: f32 = 2_000.0;

    let units_in_meters = 10.0_f32.powi(scale.exponent() as i32);
    let side = CELLS + 1;
    let step = HALF_EXTENT_NATIVE * 2.0 / CELLS as f32;
    let world_origin = VoxelQueryPosition::new(anchor);
    let mut positions = Vec::with_capacity((side * side) as usize);
    let mut heights = Vec::with_capacity((side * side) as usize);

    for z in 0..side {
        for x in 0..side {
            let sx = -HALF_EXTENT_NATIVE + x as f32 * step;
            let sz = -HALF_EXTENT_NATIVE + z as f32 * step;
            let query = world_origin
                .translated(Vec3::new(
                    sx * units_in_meters,
                    0.0,
                    sz * units_in_meters,
                ))
                .expect("bounded regional sample must translate canonically");
            let sy = terrain.height_at(world_origin, query) / units_in_meters;
            heights.push(sy);
            positions.push([sx, sy, sz]);
        }
    }

    let index = |x: u32, z: u32| -> usize { (z * side + x) as usize };
    let mut normals = Vec::with_capacity(positions.len());
    for z in 0..side {
        for x in 0..side {
            let left = heights[index(x.saturating_sub(1), z)];
            let right = heights[index((x + 1).min(CELLS), z)];
            let down = heights[index(x, z.saturating_sub(1))];
            let up = heights[index(x, (z + 1).min(CELLS))];
            let dx = (right - left) / if x == 0 || x == CELLS { step } else { step * 2.0 };
            let dz = (up - down) / if z == 0 || z == CELLS { step } else { step * 2.0 };
            normals.push(Vec3::new(-dx, 1.0, -dz).normalize().to_array());
        }
    }

    let mut indices = Vec::with_capacity((CELLS * CELLS * 6) as usize);
    for z in 0..CELLS {
        for x in 0..CELLS {
            let a = z * side + x;
            let b = a + 1;
            let c = a + side;
            let d = c + 1;
            indices.extend_from_slice(&[a, c, b, b, c, d]);
        }
    }

    Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, normals)
        .with_inserted_indices(Indices::U32(indices))
}
