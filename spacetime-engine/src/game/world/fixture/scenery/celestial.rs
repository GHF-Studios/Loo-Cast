//! One semantic authority per celestial body.
//!
//! The authority owns canonical identity, procedural field, navigation shape,
//! gravity, refinement capability and edit history. Scale-local voxel worlds are
//! manifestations of that authority; decorations are presentation projections.

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    ecs::{UsfEntity, UsfManifestationOf, UsfManifestations, UsfPresentationProjectionOf},
    physics::gravity::RadialGravitySource,
    procedural_assets::ProceduralAssetLibrary,
    spatial::{
        SpatialScale, UsfApproachRefinement, UsfChartMask, UsfPosition,
        UsfScaleLayer, UsfSceneryPresentation, UsfTravelInfluence,
    },
    voxel::{
        celestial_surface_mesh, CelestialBodyProfile, CelestialVoxelField, VoxelAuthority, VoxelBase,
        VoxelCollisionDisabled, VoxelEditingDisabled, VoxelPresentationMaterial,
        VoxelScaleDomain, VoxelStreaming, VoxelWorld,
    },
};
use crate::game::world::WorldMemberOf;

use super::super::{
    BodySurfaceSite, FixtureArrivalSite,
    definition::BodyDefinition, landmarks::UniverseLandmarkIndex,
};

const SYSTEM_SCALE: i8 = 8;
const CELESTIAL_MACRO_VOXEL_MIN_SCALE: i8 = 0;
const HUMAN_SURFACE_INTERACTION_SCALE: i8 = 0;
const CELESTIAL_COARSE_TARGET_RADIUS_NATIVE: f64 = 32.0;

#[derive(Component, Debug, Clone, Copy)]
pub(in crate::game::world::fixture) struct CelestialBodyAuthority;

/// Construct one body once, then derive every consumer from its field.
pub(super) fn spawn_body(
    commands: &mut Commands,
    parent: Entity,
    definition: &BodyDefinition,
    assets: &ProceduralAssetLibrary,
    meshes: &mut Assets<Mesh>,
    config: &EngineConfig,
    landmarks: &mut UniverseLandmarkIndex,
    arrival_site: &mut FixtureArrivalSite,
) {
    let system_scale = scale(SYSTEM_SCALE);
    let center = UsfPosition::from_scale_native_f64(
        definition.center_metres, SpatialScale::ZERO, SpatialScale::MIN,
    ).expect("authored body center must be canonically addressable");
    let radius_metres = definition.radius_metres;
    let detail_root = celestial_coarsest_scale(radius_metres);
    let realization_coarsest = detail_root;
    let field = CelestialVoxelField::new(
        center, radius_metres, detail_root, definition.seed, definition.profile,
    );
    let name = definition.name;

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
            WorldMemberOf(parent),
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
                definition.gravity_metres_per_second2,
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
            WorldMemberOf(parent),
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
    let far_material = match field.profile() {
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
        meshes.add(celestial_surface_mesh(field, scale(CELESTIAL_MACRO_VOXEL_MIN_SCALE), system_scale)),
        far_material,
    );

    landmarks.register_body(definition, center, system_scale);
    if let Some(direction) = definition.arrival_direction {
        let site = body_surface_site(
            semantic, center, field, direction, scale(HUMAN_SURFACE_INTERACTION_SCALE),
        ).expect("authored arrival direction must resolve a celestial surface");
        arrival_site.set(site);
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
) {
    commands.spawn((
        Name::new(name.into()),
        WorldMemberOf(parent),
        UsfPresentationProjectionOf(manifestation),
        UsfSceneryPresentation::from_anchor(anchor, scale),
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::IDENTITY,
        Visibility::Inherited,
    ));

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

pub(in crate::game::world::fixture) fn audit_world_authority(
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
