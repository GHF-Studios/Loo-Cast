//! Rigged first-pass visual realization of one complete semantic universe branch.
//!
//! This is intentionally simple and deterministic. The purpose is not realism;
//! it is to exercise the real worldgen -> procedural asset -> multiscale
//! presentation path from cosmic structure down to local ecology.

use std::any::Any;

use bevy::{color::LinearRgba, math::DVec3, prelude::*};

use crate::{
    config::EngineConfig,
    procedural_assets::ProceduralAssetLibrary,
    spatial::{
        SpatialScale, UsfApproachRefinement, UsfPosition,
        UsfScaleFallbackPresentation, UsfScaleLayer, UsfSceneryPresentation, UsfTravelInfluence,
    },
    voxel::{
        CelestialBodyProfile, MATERIALIZATION_CHUNK_SIZE, ProceduralCelestialBody, VoxelBase,
        VoxelCollisionDisabled, VoxelEditingDisabled, VoxelPinnedDemand, VoxelPresentationMaterial,
        VoxelQueryPosition, VoxelStreaming, VoxelWorld,
    },
    worldgen::{
        COSMIC_MATTER_DISTRIBUTION, ECOLOGY, GALAXY_INTERSTELLAR_MEDIUM, PLANETARY_BODY,
        STELLAR_SYSTEM_ENVIRONMENT, CosmicMatterDistributionState, EcologyState,
        GalaxyInterstellarMediumState, PhenomenonId, PhenomenonRegistry, PlanetaryBodyState,
        StellarSystemEnvironmentState, WorldgenNode, WorldgenStore,
    },
};

use super::{
    landmarks::UniverseLandmarkIndex,
    scale_stack::{ProceduralScaleStack, volume_for_scale_context},
};

const COSMIC_SCALE: i8 = 24;
const GALAXY_SCALE: i8 = 18;
const SYSTEM_SCALE: i8 = 8;

#[derive(Component)]
struct RiggedUniverseScenery;

pub(super) fn spawn_universe_scenery(
    config: Res<EngineConfig>,
    mut commands: Commands,
    stacks: Query<(Entity, &ProceduralScaleStack)>,
    registry: Res<PhenomenonRegistry>,
    mut worldgen: ResMut<WorldgenStore>,
    mut landmarks: ResMut<UniverseLandmarkIndex>,
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
            &config,
            &mut landmarks,
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
) -> Entity {
    commands.spawn((
        Name::new(name.into()),
        ChildOf(parent),
        UsfSceneryPresentation::new(absolute, scale),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_rotation(rotation),
        Visibility::Inherited,
    )).id()
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
    let galaxy_disk = scenery_entity(
        commands,
        parent,
        "Host Galaxy Disk",
        meshes.add(Cylinder::new(310.0, 7.0)),
        galaxy_material,
        scale(GALAXY_SCALE),
        center,
        Quat::from_euler(EulerRot::XYZ, 0.10, 0.0, -0.28),
    );
    // A galaxy is broad semantic/discovery context, not a solid body.
    // Finer contents (clouds, systems, stars...) provide the actual travel
    // constraints as they become locally relevant.
    commands.entity(galaxy_disk).insert(UsfTravelInfluence::region(
        center,
        scale(GALAXY_SCALE),
        310.0,
    ));
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

    let cloud_radius = 5.0 + density * 4.0;
    let cloud_mesh = meshes.add(Sphere::new(cloud_radius));
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
            let cloud = scenery_entity(
                commands,
                parent,
                format!("Galaxy Arm {arm} Cloud {step}"),
                cloud_mesh.clone(),
                cloud_material.clone(),
                scale(GALAXY_SCALE),
                absolute,
                Quat::IDENTITY,
            );
            // Nebula-like material is traversable. Its enclosing sphere only
            // tells navigation where the medium exists; speed inside is derived
            // from local feature size plus gas/turbulence/hazard proxies.
            commands.entity(cloud).insert(UsfTravelInfluence::medium(
                absolute,
                scale(GALAXY_SCALE),
                cloud_radius as f64,
                cloud_radius as f64 * 0.25,
                state.gas_fraction,
                state.turbulence,
                state.star_formation_potential * 0.35,
            ));
        }
    }
}

fn spawn_stellar_system(
    commands: &mut Commands,
    parent: Entity,
    system: StellarSystemEnvironmentState,
    planet: PlanetaryBodyState,
    assets: &ProceduralAssetLibrary,
    config: &EngineConfig,
    landmarks: &mut UniverseLandmarkIndex,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let system_scale = scale(SYSTEM_SCALE);

    const SUN_RADIUS: f64 = 6.957;
    const EARTH_RADIUS: f64 = 0.06371;
    const EARTH_ORBIT: f64 = 1496.0;
    const MOON_RADIUS: f64 = 0.01737;
    const MOON_ORBIT: f64 = 3.844;

    let earth_radius = EARTH_RADIUS * f64::from(planet.radius_earth.clamp(0.7, 1.35));
    let sun_radius = SUN_RADIUS * f64::from(system.host_mass_solar.clamp(0.7, 1.3).powf(0.7));

    let earth_center = DVec3::new(0.0, -earth_radius, 0.0);
    let sun_center = DVec3::new(-EARTH_ORBIT, 0.0, 0.0);
    let moon_center = earth_center + DVec3::new(MOON_ORBIT, 0.18, 0.22);

    landmarks.update_stellar_system(
        sun_center,
        sun_radius,
        earth_center,
        earth_radius,
        moon_center,
        MOON_RADIUS,
    );

    spawn_celestial_body_realizations(
        commands,
        parent,
        "Sun",
        sun_center,
        sun_radius,
        CelestialBodyProfile::Stellar,
        0x5355_4E21,
        assets,
        config,
    );
    spawn_celestial_body_realizations(
        commands,
        parent,
        "Earth",
        earth_center,
        earth_radius,
        CelestialBodyProfile::Rocky,
        0x4541_5254,
        assets,
        config,
    );
    spawn_celestial_body_realizations(
        commands,
        parent,
        "Moon",
        moon_center,
        MOON_RADIUS,
        CelestialBodyProfile::Lunar,
        0x4D4F_4F4E,
        assets,
        config,
    );

    // Travel/refinement semantics remain separate from render realization for
    // now; Pass 3 canonicalizes these influence centers themselves.
    commands.spawn((
        Name::new("Sun Travel Influence"),
        ChildOf(parent),
        UsfTravelInfluence::hard_body(sun_center, system_scale, sun_radius),
    ));
    commands.spawn((
        Name::new("Earth Travel Influence"),
        ChildOf(parent),
        UsfTravelInfluence::hard_body(earth_center, system_scale, earth_radius),
    ));
    commands.spawn((
        Name::new("Moon Travel Influence"),
        ChildOf(parent),
        UsfTravelInfluence::hard_body(moon_center, system_scale, MOON_RADIUS),
        UsfApproachRefinement::new(SpatialScale::ZERO),
    ));

    // Atmosphere is intentionally a different phenomenon from the solid body,
    // so it remains a separate transparent presentation.
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
        meshes.add(Sphere::new(earth_radius as f32 * 1.025)),
        atmosphere_material,
        system_scale,
        earth_center,
        Quat::IDENTITY,
    );
}

const CELESTIAL_FINEST_SCALE: i8 = 0;
const CELESTIAL_COARSE_TARGET_RADIUS_NATIVE: f64 = 32.0;

fn canonical_center_from_native(center: DVec3, source_scale: SpatialScale) -> UsfPosition {
    UsfPosition::from_scale_native_f64(center, source_scale, SpatialScale::ZERO)
        .expect("rigged celestial center must be canonically addressable")
}

fn celestial_coarsest_scale(radius_scale0: f64) -> SpatialScale {
    let raw = (radius_scale0 / CELESTIAL_COARSE_TARGET_RADIUS_NATIVE)
        .max(1.0)
        .log10()
        .ceil()
        .clamp(
            f64::from(CELESTIAL_FINEST_SCALE),
            f64::from(SpatialScale::MAX.exponent()),
        ) as i8;
    scale(raw)
}

fn spawn_celestial_body_realizations(
    commands: &mut Commands,
    parent: Entity,
    name: &str,
    center_system_native: DVec3,
    radius_system_native: f64,
    profile: CelestialBodyProfile,
    seed: u32,
    assets: &ProceduralAssetLibrary,
    config: &EngineConfig,
) {
    let system_scale = scale(SYSTEM_SCALE);
    let center = canonical_center_from_native(center_system_native, system_scale);
    let radius_scale0 = radius_system_native * system_scale.scale0_units_per_native();
    let coarsest = celestial_coarsest_scale(radius_scale0);

    for raw in CELESTIAL_FINEST_SCALE..=coarsest.exponent() {
        let terrain_scale = scale(raw);

        // Grid origin is representation-local and may be quantized to the scale;
        // the procedural body field itself retains the exact canonical center.
        let grid_origin = center
            .reexpressed_at(terrain_scale)
            .expect("celestial representation origin must re-express at its scale");
        let base = ProceduralCelestialBody::new(
            center,
            radius_scale0,
            terrain_scale,
            coarsest,
            seed,
            profile,
        );

        let mut entity = commands.spawn((
            Name::new(format!("{name} S{terrain_scale} Celestial Terrain")),
            ChildOf(parent),
            UsfScaleLayer::new(terrain_scale),
            VoxelWorld::new_at(VoxelBase::celestial_body(base), grid_origin),
            VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
            VoxelPresentationMaterial::new(assets.debug_grid.clone()),
            VoxelCollisionDisabled,
            VoxelEditingDisabled,
            Transform::IDENTITY,
            Visibility::Inherited,
        ));

        if terrain_scale == coarsest {
            // Keep only the coarsest whole-body shell resident at arbitrary
            // observer distance. Finer levels remain observer-demanded patches.
            let radius_native = terrain_scale.scale0_to_native_f64(radius_scale0) as f32;
            let pinned_center = center
                .reexpressed_at(terrain_scale)
                .expect("pinned body center must match its representation scale");
            let margin = MATERIALIZATION_CHUNK_SIZE as f32 * 1.5;
            entity.insert((
                VoxelPinnedDemand::shell(
                    pinned_center,
                    radius_native,
                    margin,
                ),
                UsfScaleFallbackPresentation::new(coarsest),
            ));
        }
    }
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
