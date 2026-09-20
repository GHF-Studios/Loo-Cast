//! Rigged first-pass visual realization of one complete semantic universe branch.
//!
//! This is intentionally simple and deterministic. The purpose is not realism;
//! it is to exercise the real worldgen -> procedural asset -> multiscale
//! presentation path from cosmic structure down to local ecology.

use std::any::Any;

use bevy::{color::LinearRgba, math::DVec3, mesh::VertexAttributeValues, prelude::*};

use crate::{
    procedural_assets::ProceduralAssetLibrary,
    spatial::{SpatialScale, UsfDistanceMeshLod, UsfPosition, UsfSceneryPresentation},
    voxel::VoxelQueryPosition,
    worldgen::{
        COSMIC_MATTER_DISTRIBUTION, ECOLOGY, GALAXY_INTERSTELLAR_MEDIUM, PLANETARY_BODY,
        STELLAR_SYSTEM_ENVIRONMENT, CosmicMatterDistributionState, EcologyState,
        GalaxyInterstellarMediumState, PhenomenonId, PhenomenonRegistry, PlanetaryBodyState,
        StellarSystemEnvironmentState, WorldgenNode, WorldgenStore,
    },
};

use super::scale_stack::{ProceduralScaleStack, volume_for_scale_context};

const COSMIC_SCALE: i8 = 24;
const GALAXY_SCALE: i8 = 18;
const SYSTEM_SCALE: i8 = 8;

#[derive(Component)]
struct RiggedUniverseScenery;

pub(super) fn spawn_universe_scenery(
    mut commands: Commands,
    stacks: Query<(Entity, &ProceduralScaleStack)>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    assets: Res<ProceduralAssetLibrary>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (stack_entity, stack) in &stacks {
        let target = UsfPosition::default();
        let leaf = worldgen
            .contextualize_to(stack.root(), target, SpatialScale::ZERO, &registry)
            .expect("rigged universe branch must remain canonically addressable")
            .expect("rigged universe branch must refine to scale zero");

        let lineage = worldgen.lineage(leaf);
        let cosmic = state_at::<CosmicMatterDistributionState>(
            &lineage,
            COSMIC_SCALE,
            COSMIC_MATTER_DISTRIBUTION,
        );
        let galaxy = state_at::<GalaxyInterstellarMediumState>(
            &lineage,
            GALAXY_SCALE,
            GALAXY_INTERSTELLAR_MEDIUM,
        );
        let system = state_at::<StellarSystemEnvironmentState>(
            &lineage,
            SYSTEM_SCALE,
            STELLAR_SYSTEM_ENVIRONMENT,
        );
        let planet = state_at::<PlanetaryBodyState>(&lineage, SYSTEM_SCALE, PLANETARY_BODY);
        let ecology = state_at::<EcologyState>(&lineage, 0, ECOLOGY);
        let leaf_seed = lineage
            .first()
            .expect("scale-zero branch has a leaf")
            .context()
            .seed() as u32;

        let root = commands
            .spawn((
                Name::new("Rigged Full-Stack Universe Scenery"),
                RiggedUniverseScenery,
                ChildOf(stack_entity),
                Transform::IDENTITY,
                Visibility::Inherited,
            ))
            .id();

        spawn_cosmic_web(&mut commands, root, cosmic, &mut meshes, &mut materials);
        spawn_galaxy(&mut commands, root, galaxy, &mut meshes, &mut materials);
        spawn_stellar_system(
            &mut commands,
            root,
            system,
            planet,
            &assets,
            &mut meshes,
            &mut materials,
        );

        let volume = volume_for_scale_context(&worldgen, leaf);
        spawn_local_ecology(
            &mut commands,
            root,
            volume,
            ecology,
            leaf_seed,
            &mut meshes,
            &mut materials,
        );
    }
}

fn scale(raw: i8) -> SpatialScale {
    SpatialScale::new(raw).expect("rigged scenery scale is valid")
}

fn state_at<T: Any + Copy>(
    lineage: &[&WorldgenNode],
    raw_scale: i8,
    id: PhenomenonId,
) -> T {
    lineage
        .iter()
        .copied()
        .find(|node| node.context().spatial_scale() == scale(raw_scale))
        .and_then(|node| node.state::<T>(id))
        .copied()
        .expect("rigged universe semantic state must exist at its handoff scale")
}

fn scenery_entity(
    commands: &mut Commands,
    parent: Entity,
    name: impl Into<String>,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    scale: SpatialScale,
    absolute: DVec3,
    rotation: Quat,
) {
    commands.spawn((
        Name::new(name.into()),
        ChildOf(parent),
        UsfSceneryPresentation::new(absolute, scale),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_rotation(rotation),
        Visibility::Inherited,
    ));
}

fn spawn_cosmic_web(
    commands: &mut Commands,
    parent: Entity,
    state: CosmicMatterDistributionState,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let strength = state.filament_strength.clamp(0.0, 1.0);
    let filament_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.16, 0.28 + strength * 0.28, 0.70, 0.16),
        emissive: LinearRgba::rgb(0.12, 0.35 + strength * 0.55, 1.5),
        emissive_exposure_weight: 0.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let node_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.55, 0.72, 1.0, 0.34),
        emissive: LinearRgba::rgb(0.8, 1.2, 2.8),
        emissive_exposure_weight: 0.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let points = [
        DVec3::new(-36.0, -7.0, -24.0),
        DVec3::new(-18.0, 5.0, -5.0),
        DVec3::new(2.0, -2.0, 11.0),
        DVec3::new(24.0, 8.0, 2.0),
        DVec3::new(38.0, -4.0, -18.0),
        DVec3::new(8.0, 14.0, -31.0),
        DVec3::new(-24.0, 17.0, 19.0),
    ];
    let links = [(0, 1), (1, 2), (2, 3), (3, 4), (2, 5), (1, 6), (5, 6), (3, 5)];

    let node_mesh = meshes.add(Sphere::new(1.25 + strength * 0.9));
    for (index, point) in points.iter().copied().enumerate() {
        scenery_entity(
            commands,
            parent,
            format!("Cosmic Web Node {index}"),
            node_mesh.clone(),
            node_material.clone(),
            scale(COSMIC_SCALE),
            point,
            Quat::IDENTITY,
        );
    }

    for (index, (a, b)) in links.into_iter().enumerate() {
        let start = points[a];
        let end = points[b];
        let delta = end - start;
        let length = delta.length();
        let midpoint = (start + end) * 0.5;
        let direction =
            Vec3::new(delta.x as f32, delta.y as f32, delta.z as f32).normalize_or_zero();
        let rotation = if direction == Vec3::ZERO {
            Quat::IDENTITY
        } else {
            Quat::from_rotation_arc(Vec3::Y, direction)
        };
        let mesh = meshes.add(Capsule3d::new(0.28 + strength * 0.28, length as f32));
        scenery_entity(
            commands,
            parent,
            format!("Cosmic Filament {index}"),
            mesh,
            filament_material.clone(),
            scale(COSMIC_SCALE),
            midpoint,
            rotation,
        );
    }
}

fn spawn_galaxy(
    commands: &mut Commands,
    parent: Entity,
    state: GalaxyInterstellarMediumState,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let density = state.stellar_density.clamp(0.0, 1.0);
    let galaxy_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.30, 0.42, 0.82, 0.11 + density * 0.09),
        emissive: LinearRgba::rgb(0.35, 0.55, 1.3 + density * 1.2),
        emissive_exposure_weight: 0.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        ..default()
    });
    let core_material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.78, 0.42, 0.42),
        emissive: LinearRgba::rgb(3.0, 1.7, 0.55),
        emissive_exposure_weight: 0.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });
    let cloud_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.55, 0.68, 1.0, 0.34),
        emissive: LinearRgba::rgb(0.65, 0.9, 2.2),
        emissive_exposure_weight: 0.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        ..default()
    });

    let center = DVec3::new(220.0, -8.0, -70.0);
    scenery_entity(
        commands,
        parent,
        "Host Galaxy Disk",
        meshes.add(Cylinder::new(310.0, 7.0)),
        galaxy_material,
        scale(GALAXY_SCALE),
        center,
        Quat::from_euler(EulerRot::XYZ, 0.10, 0.0, -0.28),
    );
    scenery_entity(
        commands,
        parent,
        "Host Galaxy Core",
        meshes.add(Sphere::new(31.0)),
        core_material,
        scale(GALAXY_SCALE),
        center,
        Quat::IDENTITY,
    );

    let cloud_mesh = meshes.add(Sphere::new(5.0 + density * 4.0));
    for arm in 0..3 {
        for step in 0..12 {
            let t = step as f64 / 11.0;
            let radius = 42.0 + t * 245.0;
            let angle = arm as f64 * std::f64::consts::TAU / 3.0 + t * 5.2;
            let absolute = center
                + DVec3::new(
                    angle.cos() * radius,
                    ((step * 17 + arm * 11) as f64).sin() * 3.5,
                    angle.sin() * radius,
                );
            scenery_entity(
                commands,
                parent,
                format!("Galaxy Arm {arm} Cloud {step}"),
                cloud_mesh.clone(),
                cloud_material.clone(),
                scale(GALAXY_SCALE),
                absolute,
                Quat::IDENTITY,
            );
        }
    }
}

fn spawn_stellar_system(
    commands: &mut Commands,
    parent: Entity,
    system: StellarSystemEnvironmentState,
    planet: PlanetaryBodyState,
    assets: &ProceduralAssetLibrary,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let system_scale = scale(SYSTEM_SCALE);

    const SUN_RADIUS: f32 = 6.957;
    const EARTH_RADIUS: f32 = 0.06371;
    const EARTH_ORBIT: f64 = 1496.0;
    const MOON_RADIUS: f32 = 0.01737;
    const MOON_ORBIT: f64 = 3.844;

    let earth_radius = EARTH_RADIUS * planet.radius_earth.clamp(0.7, 1.35);
    let sun_radius = SUN_RADIUS * system.host_mass_solar.clamp(0.7, 1.3).powf(0.7);

    let earth_center = DVec3::new(0.0, -(earth_radius as f64), 0.0);
    let sun_center = DVec3::new(-EARTH_ORBIT, 0.0, 0.0);
    let moon_center = earth_center + DVec3::new(MOON_ORBIT, 0.18, 0.22);

    scenery_entity(
        commands,
        parent,
        "Rigged Sun",
        meshes.add(Sphere::new(sun_radius)),
        assets.star_surface.clone(),
        system_scale,
        sun_center,
        Quat::IDENTITY,
    );
    scenery_entity(
        commands,
        parent,
        "Rigged Earth",
        meshes.add(Sphere::new(earth_radius)),
        assets.planet_surface.clone(),
        system_scale,
        earth_center,
        Quat::from_rotation_y(0.45),
    );

    let atmosphere_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.22, 0.48, 1.0, 0.10 + planet.water_inventory * 0.06),
        emissive: LinearRgba::rgb(0.02, 0.06, 0.18),
        emissive_exposure_weight: 0.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        ..default()
    });
    scenery_entity(
        commands,
        parent,
        "Rigged Earth Atmosphere",
        meshes.add(Sphere::new(earth_radius * 1.025)),
        atmosphere_material,
        system_scale,
        earth_center,
        Quat::IDENTITY,
    );

    let moon_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.42, 0.43, 0.45),
        perceptual_roughness: 1.0,
        metallic: 0.0,
        ..default()
    });
    // One semantic Moon, several disposable geometric representations.
    //
    // Every level samples the same deterministic macro surface. Finer levels
    // increase tessellation and add higher-frequency relief, so approach reveals
    // actual structure rather than merely swapping one smooth sphere for another.
    let moon_far = meshes.add(lunar_surface_mesh(MOON_RADIUS, 2, 0));
    let moon_low = meshes.add(lunar_surface_mesh(MOON_RADIUS, 3, 1));
    let moon_medium = meshes.add(lunar_surface_mesh(MOON_RADIUS, 4, 2));
    let moon_high = meshes.add(lunar_surface_mesh(MOON_RADIUS, 5, 3));

    commands.spawn((
        Name::new("Rigged Moon"),
        ChildOf(parent),
        UsfSceneryPresentation::new(moon_center, system_scale),
        UsfDistanceMeshLod::new(
            MOON_RADIUS as f64,
            [
                (14.0, moon_high.clone()),
                (45.0, moon_medium),
                (140.0, moon_low),
                (f64::INFINITY, moon_far.clone()),
            ],
        ),
        Mesh3d(moon_far),
        MeshMaterial3d(moon_material),
        Transform::from_rotation(Quat::from_rotation_y(-0.8)),
        Visibility::Inherited,
    ));
}

fn lunar_surface_mesh(radius: f32, subdivisions: u32, detail: u8) -> Mesh {
    let mut mesh = Sphere::new(radius)
        .mesh()
        .ico(subdivisions)
        .expect("lunar icosphere subdivision is valid");

    let Some(VertexAttributeValues::Float32x3(positions)) =
        mesh.attribute_mut(Mesh::ATTRIBUTE_POSITION)
    else {
        panic!("lunar icosphere positions must be Float32x3");
    };

    for position in positions {
        let direction = Vec3::from_array(*position).normalize_or_zero();
        if direction == Vec3::ZERO {
            continue;
        }

        let relief = lunar_relative_relief(direction, detail);
        *position = (direction * radius * (1.0 + relief)).to_array();
    }

    mesh.compute_smooth_normals();
    mesh
}

/// Deterministic lunar-looking radial displacement.
///
/// This is a presentation recipe, not authoritative geology. Large features are
/// shared by every LOD; progressively finer bands become available as mesh
/// tessellation can actually represent them.
fn lunar_relative_relief(direction: Vec3, detail: u8) -> f32 {
    let mut height =
        (direction.dot(Vec3::new(1.7, -2.3, 0.9)) * 5.0).sin() * 0.0014
        + (direction.dot(Vec3::new(-3.1, 0.7, 2.4)) * 8.0).sin() * 0.0008;

    let craters = [
        (Vec3::new(0.82, 0.21, 0.53), 0.36_f32, 0.0100_f32),
        (Vec3::new(-0.51, 0.70, 0.49), 0.27, 0.0070),
        (Vec3::new(0.18, -0.88, 0.44), 0.23, 0.0060),
        (Vec3::new(-0.77, -0.23, -0.59), 0.19, 0.0048),
        (Vec3::new(0.39, 0.48, -0.79), 0.16, 0.0040),
        (Vec3::new(-0.08, -0.35, 0.93), 0.13, 0.0034),
        (Vec3::new(0.63, -0.66, -0.40), 0.11, 0.0028),
        (Vec3::new(-0.33, 0.14, -0.93), 0.095, 0.0024),
    ];

    for (raw_center, crater_radius, depth) in craters {
        let center = raw_center.normalize();
        let distance = direction.distance(center);
        let q = distance / crater_radius;

        if q < 1.0 {
            let bowl = 1.0 - q * q;
            height -= depth * bowl * bowl;
        }

        let rim_distance = ((q - 1.0) / 0.22).abs();
        if rim_distance < 1.0 {
            let rim = 1.0 - rim_distance;
            height += depth * 0.28 * rim * rim;
        }
    }

    if detail >= 1 {
        height +=
            (direction.dot(Vec3::new(4.3, 7.1, -5.2)) * 18.0).sin() * 0.00055
            + (direction.dot(Vec3::new(-6.7, 2.9, 5.6)) * 23.0).cos() * 0.00035;
    }
    if detail >= 2 {
        height +=
            (direction.dot(Vec3::new(11.1, -8.7, 6.3)) * 41.0).sin() * 0.00020
            + (direction.dot(Vec3::new(-9.4, 12.6, 7.8)) * 53.0).cos() * 0.00014;
    }
    if detail >= 3 {
        height +=
            (direction.dot(Vec3::new(17.0, 13.0, -19.0)) * 83.0).sin() * 0.00008;
    }

    height
}

fn spawn_local_ecology(
    commands: &mut Commands,
    parent: Entity,
    volume: crate::voxel::ProceduralVolume,
    ecology: EcologyState,
    seed: u32,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let world_origin = VoxelQueryPosition::new(UsfPosition::zero(SpatialScale::ZERO));
    let trunk_mesh = meshes.add(Cylinder::new(0.14, 3.0));
    let canopy_mesh = meshes.add(Sphere::new(1.15));
    let plant_mesh = meshes.add(Cylinder::new(0.035, 0.55));

    let trunk_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.22, 0.095, 0.035),
        perceptual_roughness: 0.96,
        ..default()
    });
    let canopy_material = materials.add(StandardMaterial {
        base_color: Color::srgb(
            0.055,
            0.24 + ecology.forest_affinity * 0.20,
            0.055 + ecology.productivity * 0.06,
        ),
        perceptual_roughness: 0.95,
        ..default()
    });
    let plant_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.12, 0.30 + ecology.grass_affinity * 0.28, 0.075),
        perceptual_roughness: 0.98,
        ..default()
    });

    let tree_count = (8.0 + ecology.forest_affinity * 18.0).round() as u32;
    for index in 0..tree_count {
        let x = signed_unit(hash(seed ^ 0x71EE_0001 ^ index.wrapping_mul(17))) * 54.0;
        let z = signed_unit(hash(seed ^ 0x71EE_0002 ^ index.wrapping_mul(31))) * 54.0;
        let mut position = Vec3::new(x, 0.0, z);
        if (position.x * position.x + position.z * position.z).sqrt() < 9.0 {
            position.x += 14.0;
        }
        let Some(height) = surface_height(volume, world_origin, position.x, position.z) else {
            continue;
        };

        scenery_entity(
            commands,
            parent,
            format!("Biome Tree {index} Trunk"),
            trunk_mesh.clone(),
            trunk_material.clone(),
            SpatialScale::ZERO,
            DVec3::new(position.x as f64, (height + 1.5) as f64, position.z as f64),
            Quat::IDENTITY,
        );
        scenery_entity(
            commands,
            parent,
            format!("Biome Tree {index} Canopy"),
            canopy_mesh.clone(),
            canopy_material.clone(),
            SpatialScale::ZERO,
            DVec3::new(position.x as f64, (height + 3.25) as f64, position.z as f64),
            Quat::IDENTITY,
        );
    }

    let plant_count = (12.0 + ecology.grass_affinity * 30.0).round() as u32;
    for index in 0..plant_count {
        let x = signed_unit(hash(seed ^ 0x61A5_0001 ^ index.wrapping_mul(43))) * 44.0;
        let z = signed_unit(hash(seed ^ 0x61A5_0002 ^ index.wrapping_mul(59))) * 44.0;
        let Some(height) = surface_height(volume, world_origin, x, z) else {
            continue;
        };

        scenery_entity(
            commands,
            parent,
            format!("Biome Plant {index}"),
            plant_mesh.clone(),
            plant_material.clone(),
            SpatialScale::ZERO,
            DVec3::new(x as f64, (height + 0.275) as f64, z as f64),
            Quat::from_rotation_z(signed_unit(hash(seed ^ index)) * 0.22),
        );
    }
}

fn surface_height(
    volume: crate::voxel::ProceduralVolume,
    world_origin: VoxelQueryPosition,
    x: f32,
    z: f32,
) -> Option<f32> {
    let point = world_origin.translated(Vec3::new(x, 0.0, z)).ok()?;
    Some(volume.reference_surface_height_at(world_origin, point))
}

fn hash(mut value: u32) -> u32 {
    value ^= value >> 16;
    value = value.wrapping_mul(0x7FEB_352D);
    value ^= value >> 15;
    value = value.wrapping_mul(0x846C_A68B);
    value ^ (value >> 16)
}

fn signed_unit(value: u32) -> f32 {
    (value as f64 / u32::MAX as f64) as f32 * 2.0 - 1.0
}
