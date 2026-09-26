//! One semantic Earth authority with an all-scale voxel realization ladder.
//!
//! There is no separate whole-body Earth mesh. Every terrain surface comes from
//! the same `CelestialVoxelField` through `VoxelWorld` materialization.

use bevy::prelude::*;

use crate::{
    config::EngineConfig,
    ecs::{UsfEntity, UsfManifestationOf, UsfManifestations},
    physics::gravity::RadialGravitySource,
    procedural_assets::ProceduralAssetLibrary,
    spatial::{
        SPATIAL_SCALE_MIN, SpatialScale, UsfApproachRefinement, UsfChartMask,
        UsfPosition, UsfScaleLayer, UsfTravelInfluence,
    },
    voxel::{
        CelestialVoxelField, VoxelAuthority, VoxelBase, VoxelCollisionDisabled,
        VoxelEditingDisabled, VoxelPinnedDemand, VoxelPresentationMaterial, VoxelScaleDomain,
        VoxelStreaming, VoxelWorld,
    },
};
use crate::game::world::WorldMemberOf;

use super::super::{
    BodySurfaceSite, FixtureArrivalSite,
    definition::BodyDefinition, landmarks::UniverseLandmarkIndex,
};

const SYSTEM_SCALE: i8 = 8;
const CELESTIAL_VOXEL_MIN_SCALE: i8 = SPATIAL_SCALE_MIN;
const HUMAN_SURFACE_INTERACTION_SCALE: i8 = 0;
const CELESTIAL_COARSE_TARGET_RADIUS_NATIVE: f64 = 32.0;
/// Keep bootstrap generation intentionally tiny: the coarse slice gets enough
/// residency to contain the whole Earth; every finer slice gets exactly one
/// materialization at the same canonical surface ray.
const CELESTIAL_BOOTSTRAP_SHELL_MARGIN_NATIVE: f32 = 2.0;

#[derive(Component, Debug, Clone, Copy)]
pub(in crate::game::world::fixture) struct CelestialBodyAuthority;

pub(super) fn spawn_body(
    commands: &mut Commands,
    parent: Entity,
    definition: &BodyDefinition,
    assets: &ProceduralAssetLibrary,
    config: &EngineConfig,
    landmarks: &mut UniverseLandmarkIndex,
    arrival_site: &mut FixtureArrivalSite,
) {
    let system_scale = scale(SYSTEM_SCALE);
    let center = UsfPosition::from_scale_native_f64(
        definition.center_metres,
        SpatialScale::ZERO,
        SpatialScale::MIN,
    )
    .expect("authored Earth center must be canonically addressable");
    let radius_metres = definition.radius_metres;
    let detail_root = celestial_coarsest_scale(radius_metres);
    let field = CelestialVoxelField::new(
        center,
        radius_metres,
        detail_root,
        definition.seed,
        definition.profile,
    );
    let name = definition.name;

    let all_voxel_scales =
        VoxelScaleDomain::contiguous(SpatialScale::MIN, detail_root).realization_slices();
    let scale_domain = VoxelScaleDomain::contiguous(SpatialScale::MIN, detail_root)
        .with_collision_slices(all_voxel_scales)
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
            UsfApproachRefinement::new(SpatialScale::MIN),
        ))
        .id();

    let local_surface_material = assets.debug_grid.clone();
    let bootstrap_direction = definition
        .arrival_direction
        .unwrap_or(Vec3::Y)
        .normalize_or_zero();
    assert!(
        bootstrap_direction != Vec3::ZERO,
        "Earth bootstrap surface direction must be non-zero"
    );

    for raw in CELESTIAL_VOXEL_MIN_SCALE..=detail_root.exponent() {
        let terrain_scale = scale(raw);
        let grid_origin = center
            .reexpressed_at(terrain_scale)
            .expect("Earth realization origin must re-express at its scale");
        let base = field.realization(terrain_scale);

        // One semantic Earth already owns this whole scale ladder. Pinned demand
        // merely guarantees a tiny deterministic bootstrap realization so the
        // ladder can become visible/collidable before player-driven demand has
        // had a chance to expand it.
        let bootstrap_demand = if terrain_scale == detail_root {
            let radius_native = terrain_scale.metres_to_native_f64(radius_metres);
            assert!(
                radius_native.is_finite()
                    && radius_native >= 0.0
                    && radius_native <= f64::from(f32::MAX),
                "coarsest Earth radius must fit its own bounded realization chart"
            );
            VoxelPinnedDemand::shell(
                grid_origin,
                radius_native as f32,
                CELESTIAL_BOOTSTRAP_SHELL_MARGIN_NATIVE,
            )
        } else {
            let surface = field
                .surface_position(bootstrap_direction, terrain_scale)
                .expect("Earth bootstrap surface must exist at every supported scale");
            // Zero extent is intentional: streaming resolves this to exactly the
            // one materialization containing the canonical bootstrap point.
            VoxelPinnedDemand::cuboid(surface, Vec3::ZERO)
        };

        let mut terrain = commands.spawn((
            Name::new(format!("{name} S{terrain_scale} Terrain")),
            WorldMemberOf(parent),
            UsfScaleLayer::new(terrain_scale),
            UsfManifestationOf(semantic),
            bootstrap_demand,
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
    }

    // Intentionally no UsfSceneryPresentation / celestial_surface_mesh.
    landmarks.register_body(definition, center, system_scale);
    if let Some(direction) = definition.arrival_direction {
        let site = body_surface_site(
            semantic,
            field,
            bootstrap_direction,
            scale(HUMAN_SURFACE_INTERACTION_SCALE),
        )
        .expect("authored Earth arrival direction must resolve a voxel surface");
        arrival_site.set(site);
    }
}

fn body_surface_site(
    body: Entity,
    field: CelestialVoxelField,
    direction: Vec3,
    scale: SpatialScale,
) -> Option<BodySurfaceSite> {
    let up = direction.normalize_or_zero();
    if up == Vec3::ZERO {
        return None;
    }

    let surface = field.surface_position(up, scale).ok()?;
    BodySurfaceSite::new(body, surface, up, scale)
}

fn celestial_coarsest_scale(radius_metres: f64) -> SpatialScale {
    let raw = (radius_metres / CELESTIAL_COARSE_TARGET_RADIUS_NATIVE)
        .max(1.0)
        .log10()
        .ceil()
        .clamp(
            f64::from(CELESTIAL_VOXEL_MIN_SCALE),
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
                "Earth authority invariant violation"
            );
        }
    }

    if invalid == 0 {
        info!(
            bodies = entries.len(),
            minimum_voxel_scale = %SpatialScale::MIN,
            "Earth-only voxel authority invariants healthy; deterministic bootstrap spine active"
        );
        *completed = true;
    }
}
