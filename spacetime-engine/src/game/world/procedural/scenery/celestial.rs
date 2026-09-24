//! One semantic authority per celestial body.
//!
//! The authority owns canonical identity, procedural field, navigation shape,
//! gravity, refinement capability and edit history. Scale-local voxel worlds are
//! manifestations of that authority; decorations are presentation projections.

use bevy::{math::DVec3, prelude::*};

use crate::{
    config::EngineConfig,
    ecs::{UsfEntity, UsfManifestationOf, UsfManifestations, UsfPresentationProjectionOf},
    physics::gravity::RadialGravitySource,
    procedural_assets::ProceduralAssetLibrary,
    spatial::{
        SpatialScale, UsfApproachRefinement, UsfChartMask, UsfPosition,
        UsfScaleFallbackPresentation, UsfScaleLayer, UsfSceneryPresentation, UsfTravelInfluence,
    },
    voxel::{
        CelestialBodyProfile, CelestialVoxelField, VoxelAuthority, VoxelBase,
        VoxelCollisionDisabled, VoxelEditingDisabled, VoxelPresentationMaterial,
        VoxelScaleDomain, VoxelStreaming, VoxelWorld,
    },
    worldgen::{PlanetaryBodyState, StellarSystemEnvironmentState},
};

use super::super::{
    BodySurfaceSite, ProceduralArrivalSite,
    landmarks::UniverseLandmarkIndex,
};

const SYSTEM_SCALE: i8 = 8;
const CELESTIAL_MACRO_VOXEL_MIN_SCALE: i8 = 0;
const HUMAN_SURFACE_INTERACTION_SCALE: i8 = 0;
const CELESTIAL_COARSE_TARGET_RADIUS_NATIVE: f64 = 32.0;

#[derive(Component, Debug, Clone, Copy)]
pub(in crate::game::world::procedural) struct CelestialBodyAuthority;

#[derive(Debug, Clone, Copy)]
struct SpawnedCelestialBody {
    semantic: Entity,
    center: UsfPosition,
    field: CelestialVoxelField,
    coarse_manifestation: Entity,
    coarsest_voxel_scale: SpatialScale,
}

pub(super) fn spawn_stellar_system(
    commands: &mut Commands,
    parent: Entity,
    system: StellarSystemEnvironmentState,
    planet: PlanetaryBodyState,
    assets: &ProceduralAssetLibrary,
    config: &EngineConfig,
    landmarks: &mut UniverseLandmarkIndex,
    arrival_site: &mut ProceduralArrivalSite,
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

    // The authored branch deliberately puts Earth's north-pole surface at the
    // local metre origin. That is content placement, not USF scale privilege.
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

    let _sun = spawn_celestial_body(
        commands,
        parent,
        "Sun",
        sun_center,
        sun_radius,
        CelestialBodyProfile::Stellar,
        274.0,
        0x5355_4E21,
        assets,
        meshes,
        config,
    );
    let earth = spawn_celestial_body(
        commands,
        parent,
        "Earth",
        earth_center,
        earth_radius,
        CelestialBodyProfile::Rocky,
        9.80665,
        0x4541_5254,
        assets,
        meshes,
        config,
    );
    let _moon = spawn_celestial_body(
        commands,
        parent,
        "Moon",
        moon_center,
        MOON_RADIUS,
        CelestialBodyProfile::Lunar,
        1.62,
        0x4D4F_4F4E,
        assets,
        meshes,
        config,
    );

    // Atmosphere is a presentation of Earth, not another body identity.
    let atmosphere_material = materials.add(StandardMaterial {
        base_color: Color::srgba(0.22, 0.48, 1.0, 0.10 + planet.water_inventory * 0.06),
        emissive: bevy::color::LinearRgba::rgb(0.02, 0.06, 0.18),
        emissive_exposure_weight: 0.0,
        alpha_mode: AlphaMode::Blend,
        unlit: true,
        double_sided: true,
        ..default()
    });
    // The simple global atmosphere shell is also a FAR presentation.
    // Near a surface it is not a local atmosphere model and must disappear
    // before it can masquerade as ground/geometry around the observer.
    spawn_body_projection(
        commands,
        parent,
        earth.coarse_manifestation,
        "Earth Far Atmosphere",
        earth.center,
        system_scale,
        meshes.add(Sphere::new(earth_radius as f32 * 1.025)),
        atmosphere_material,
        Quat::IDENTITY,
        Some(earth.coarsest_voxel_scale),
    );

    // Fixture bootstrap policy currently chooses one Earth surface site.
    // The site itself is generic body-relative navigation data resolved from the
    // authoritative celestial field. Nothing about controlled-subject bootstrap
    // knows the words "Earth", "north pole", or "tree".
    let site = body_surface_site(
        earth.semantic,
        earth.center,
        earth.field,
        Vec3::Y,
        SpatialScale::ZERO,
    )
    .expect("fixture arrival surface must be canonically representable");
    arrival_site.set(site);

    debug!(
        earth = ?earth.semantic,
        "spawned stellar system with one semantic authority per celestial body"
    );
}

fn spawn_celestial_body(
    commands: &mut Commands,
    parent: Entity,
    name: &'static str,
    center_system_native: DVec3,
    radius_system_native: f64,
    profile: CelestialBodyProfile,
    surface_gravity_metres_per_second2: f32,
    seed: u32,
    assets: &ProceduralAssetLibrary,
    meshes: &mut Assets<Mesh>,
    config: &EngineConfig,
) -> SpawnedCelestialBody {
    let system_scale = scale(SYSTEM_SCALE);
    let center = canonical_center_from_native(center_system_native, system_scale);
    let radius_metres = radius_system_native * system_scale.metres_per_native();
    let detail_root = celestial_coarsest_scale(radius_metres);

    // Representation envelope: voxel terrain exists only through the body's
    // own coarsest meaningful voxel scale. The stellar-system authoring scale
    // describes placement; it must never force a sub-voxel planet realization.
    let realization_coarsest = detail_root;
    let field = CelestialVoxelField::new(
        center,
        radius_metres,
        detail_root,
        seed,
        profile,
    );

    debug_assert!(
        detail_root.metres_to_native_f64(radius_metres) >= 1.0,
        "celestial voxel detail root must resolve the body by at least one native unit"
    );

    let scale_domain = VoxelScaleDomain::contiguous(
        scale(CELESTIAL_MACRO_VOXEL_MIN_SCALE),
        realization_coarsest,
    )
    .with_collision_slices(
        VoxelScaleDomain::contiguous(
            scale(CELESTIAL_MACRO_VOXEL_MIN_SCALE),
            realization_coarsest,
        )
        .realization_slices(),
    )
    .with_editing_slices(UsfChartMask::from_scale(scale(
        HUMAN_SURFACE_INTERACTION_SCALE,
    )));

    let nav_scale = celestial_coarsest_scale(radius_metres);
    let semantic = commands
        .spawn((
            Name::new(name),
            ChildOf(parent),
            CelestialBodyAuthority,
            UsfEntity,
            center,
            field,
            scale_domain,
            VoxelAuthority::default(),
            UsfTravelInfluence::hard_body_at(
                center,
                nav_scale,
                nav_scale.metres_to_native_f64(radius_metres),
            ),
            RadialGravitySource::new(
                center,
                radius_metres,
                nav_scale,
                surface_gravity_metres_per_second2,
            ),
            UsfApproachRefinement::new(scale(CELESTIAL_MACRO_VOXEL_MIN_SCALE)),
        ))
        .id();

    let mut coarse_manifestation = None;

    // Local voxel terrain is still under active representation debugging.
    // Keep the high-contrast grid + per-chunk vertex colors here so chunk
    // boundaries, seams and runtime motion remain immediately visible.
    // Profile materials belong to the persistent macro-body presentation.
    let local_surface_material = assets.debug_grid.clone();

    for raw in CELESTIAL_MACRO_VOXEL_MIN_SCALE..=realization_coarsest.exponent() {
        let terrain_scale = scale(raw);
        let grid_origin = center
            .reexpressed_at(terrain_scale)
            .expect("celestial representation origin must re-express at its scale");
        let base = field.realization(terrain_scale);

        let mut terrain = commands.spawn((
            Name::new(format!("{name} S{terrain_scale} Terrain")),
            ChildOf(parent),
            UsfScaleLayer::new(terrain_scale),
            UsfManifestationOf(semantic),
            VoxelWorld::new_at(VoxelBase::celestial_body(base), grid_origin),
            VoxelStreaming::new(config.voxel.streaming.default_load_budget_per_frame),
            VoxelPresentationMaterial::new(local_surface_material.clone()),
            Transform::IDENTITY,
            Visibility::Inherited,
        ));

        if !scale_domain.collides(terrain_scale) {
            terrain.insert(VoxelCollisionDisabled);
        }
        if !scale_domain.editable(terrain_scale) {
            terrain.insert(VoxelEditingDisabled);
        }

        let terrain_entity = terrain.id();

        if terrain_scale == realization_coarsest {
            // Coarsest voxel terrain remains an ordinary demand-driven
            // realization. It is NOT a permanent whole-body cache.
            coarse_manifestation = Some(terrain_entity);
        }
    }

    let coarse_manifestation = coarse_manifestation
        .expect("celestial body must have a coarsest realization");

    // Whole-body appearance is inherited macro context for the same semantic
    // authority. Fine voxel terrain refines only bounded local apertures; it
    // must never globally replace the rest of the planet/moon/star.
    let far_material = match profile {
        CelestialBodyProfile::Stellar => assets.star_surface.clone(),
        CelestialBodyProfile::Rocky => assets.planet_surface.clone(),
        CelestialBodyProfile::Lunar => assets.lunar_surface.clone(),
    };
    spawn_body_projection(
        commands,
        parent,
        coarse_manifestation,
        format!("{name} Far Body"),
        center,
        system_scale,
        meshes.add(Sphere::new(radius_system_native as f32)),
        far_material,
        Quat::IDENTITY,
        None,
    );

    SpawnedCelestialBody {
        semantic,
        center,
        field,
        coarse_manifestation,
        coarsest_voxel_scale: realization_coarsest,
    }
}

fn body_surface_site(
    body: Entity,
    center: UsfPosition,
    field: CelestialVoxelField,
    direction: Vec3,
    scale: SpatialScale,
) -> Option<BodySurfaceSite> {
    let up = direction.normalize_or_zero();
    if up == Vec3::ZERO {
        return None;
    }

    let realization = field.realization(scale);
    let surface_radius = realization.surface_radius_native(up);
    let surface = center
        .translated_at_scale(scale, up * surface_radius)
        .ok()?;

    BodySurfaceSite::new(body, surface, up, scale)
}

fn spawn_body_projection(
    commands: &mut Commands,
    parent: Entity,
    manifestation: Entity,
    name: impl Into<String>,
    anchor: UsfPosition,
    scale: SpatialScale,
    mesh: Handle<Mesh>,
    material: Handle<StandardMaterial>,
    rotation: Quat,
    fallback_beyond: Option<SpatialScale>,
) {
    let mut projection = commands.spawn((
        Name::new(name.into()),
        ChildOf(parent),
        UsfPresentationProjectionOf(manifestation),
        UsfSceneryPresentation::from_anchor(anchor, scale),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_rotation(rotation),
        Visibility::Inherited,
    ));

    if let Some(fallback_beyond) = fallback_beyond {
        projection.insert(UsfScaleFallbackPresentation::new(fallback_beyond));
    }
}

fn canonical_center_from_native(center: DVec3, source_scale: SpatialScale) -> UsfPosition {
    UsfPosition::from_scale_native_f64(center, source_scale, SpatialScale::MIN)
        .expect("celestial center must be canonically addressable")
}

fn celestial_coarsest_scale(radius_metres: f64) -> SpatialScale {
    let raw = (radius_metres / CELESTIAL_COARSE_TARGET_RADIUS_NATIVE)
        .max(1.0)
        .log10()
        .ceil()
        .clamp(
            f64::from(CELESTIAL_MACRO_VOXEL_MIN_SCALE),
            f64::from(SpatialScale::MAX.exponent()),
        ) as i8;
    scale(raw)
}

fn scale(raw: i8) -> SpatialScale {
    SpatialScale::new(raw).expect("celestial scale is valid")
}

pub(in crate::game::world::procedural) fn audit_world_authority(
    bodies: Query<
        (
            Entity,
            &Name,
            Option<&UsfPosition>,
            Option<&CelestialVoxelField>,
            Option<&VoxelAuthority>,
            Option<&UsfTravelInfluence>,
            Option<&RadialGravitySource>,
            Option<&UsfApproachRefinement>,
            Option<&UsfManifestations>,
        ),
        With<CelestialBodyAuthority>,
    >,
    mut completed: Local<bool>,
) {
    if *completed {
        return;
    }

    let entries = bodies.iter().collect::<Vec<_>>();
    if entries.is_empty() {
        return;
    }

    let mut invalid = 0usize;
    for (
        entity,
        name,
        position,
        field,
        voxel_authority,
        travel,
        gravity,
        refinement,
        manifestations,
    ) in entries.iter().copied()
    {
        let valid = position.is_some()
            && field.is_some()
            && voxel_authority.is_some()
            && travel.is_some()
            && gravity.is_some()
            && refinement.is_some()
            && manifestations.is_some_and(|manifestations| !manifestations.is_empty());

        if !valid {
            invalid += 1;
            error!(
                ?entity,
                body = %name,
                has_position = position.is_some(),
                has_field = field.is_some(),
                has_voxel_authority = voxel_authority.is_some(),
                has_travel = travel.is_some(),
                has_gravity = gravity.is_some(),
                has_refinement = refinement.is_some(),
                manifestations = manifestations.map_or(0, UsfManifestations::len),
                "celestial-body authority invariant violation"
            );
        }
    }

    if invalid == 0 {
        info!(
            bodies = entries.len(),
            "celestial-body authority invariants healthy"
        );
        *completed = true;
    }
}
