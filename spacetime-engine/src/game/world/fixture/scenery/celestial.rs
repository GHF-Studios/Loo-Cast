//! Semantic celestial construction from authored fixture input.
//!
//! Construction creates semantic body authority only. Scale-local voxel realizations
//! remain disposable downstream representations created from demand.

use bevy::prelude::*;

use crate::game::world::ScenarioMemberOf;
use crate::worldgen::PhenomenonProvenance;
use crate::{
    ecs::{UsfAuthorityPartitionOf, UsfAuthorityPartitions, UsfEntity},
    physics::gravity::RadialGravitySource,
    procedural_assets::ProceduralPresentationAssets,
    spatial::{
        SPATIAL_SCALE_MIN, SpatialScale, UsfApproachRefinement, UsfSemanticBounds, UsfCanonicalMotion, UsfPosition,
        UsfScaleSliceMask, UsfSemanticFrame, UsfTravelBoundaryProvider, UsfTravelInfluence,
    },
    voxel::{
        CelestialVoxelField, CelestialVoxelRealizationPolicy, VoxelScaleDomain,
        VoxelSemanticAuthority,
    },
};

use super::super::{
    BodySurfaceSite, FixtureArrivalSite, definition::AuthoredCelestialBody,
    landmarks::UniverseLandmarkIndex,
};

const SYSTEM_SCALE: i8 = 8;
const CELESTIAL_VOXEL_MIN_SCALE: i8 = SPATIAL_SCALE_MIN;
const HUMAN_SURFACE_INTERACTION_SCALE: i8 = 0;
const CELESTIAL_COARSE_TARGET_RADIUS_NATIVE: f64 = 32.0;

#[derive(Component, Debug, Clone, Copy)]
pub(in crate::game::world::fixture) struct CelestialBodyAuthority;

pub(super) fn construct_authored_celestial_body(
    commands: &mut Commands,
    parent: Entity,
    definition: &AuthoredCelestialBody,
    provenance: PhenomenonProvenance,
    assets: &ProceduralPresentationAssets,
    landmarks: &mut UniverseLandmarkIndex,
    arrival_site: &mut FixtureArrivalSite,
) -> Entity {
    let system_scale = scale(SYSTEM_SCALE);
    let center = UsfPosition::from_scale_native_f64(
        definition.center_metres,
        SpatialScale::ZERO,
        SpatialScale::MIN,
    )
    .expect("authored Earth center must be canonically addressable");
    let frame = UsfSemanticFrame::identity();
    let radius_metres = definition.radius_metres;
    let detail_root = celestial_coarsest_scale(radius_metres);
    let field = CelestialVoxelField::sphere(radius_metres, detail_root);
    let name = definition.name;

    //
    // The domain describes where this semantic body *can* realize a capability;
    // it does not select which Scale is currently physical. That selection is
    // demand/control policy in voxel::realization::roles_for_scale().
    //
    // Hard-coding collision to human S0 made an explicitly S+1-controlled ship
    // incapable of ever receiving terrain collision. Every supported terrain
    // chart may provide physical capability, but only the current controlled
    // interaction Scale is granted COLLISION/EDITING roles.
    let physical_slices = UsfScaleSliceMask::inclusive_range(SpatialScale::MIN, detail_root);
    let scale_domain = VoxelScaleDomain::contiguous(SpatialScale::MIN, detail_root)
        .with_collision_slices(physical_slices)
        .with_editing_slices(physical_slices);

    let nav_scale = celestial_coarsest_scale(radius_metres);
    let semantic = commands
        .spawn((
            (
                Name::new(name),
                ScenarioMemberOf(parent),
                CelestialBodyAuthority,
                UsfEntity,
                provenance,
                center,
                frame,
                UsfCanonicalMotion::canonical_kinematic_at_rest(),
                field,
                scale_domain,
                CelestialVoxelRealizationPolicy::new(assets.debug_grid.clone()),
                VoxelSemanticAuthority::default(),
                UsfTravelInfluence::hard_body(nav_scale, nav_scale.metres_to_native_f64(radius_metres)),
                UsfTravelBoundaryProvider::new(field),
                RadialGravitySource::new(
                    radius_metres,
                    nav_scale,
                    definition.gravity_metres_per_second2,
                ),
            ),
            (
                UsfApproachRefinement::new(SpatialScale::MIN)
            )
        ))
        .id();

    // Disposable broadphase evidence for any observer/capability; the
    // authoritative body geometry remains in CelestialVoxelField.
    commands.entity(semantic).insert(UsfSemanticBounds::sphere(radius_metres));

    // One ordinary unsplit authority partition owns every scale-local voxel
    // realization. Scale is realization identity, never semantic identity.
    commands.spawn((
        Name::new(format!("{name} Authority Partition")),
        UsfAuthorityPartitionOf(semantic),
    ));

    let bootstrap_direction = definition
        .arrival_direction
        .unwrap_or(Vec3::Y)
        .normalize_or_zero();
    assert!(
        bootstrap_direction != Vec3::ZERO,
        "Earth bootstrap surface direction must be non-zero"
    );

    // The fixture owns semantic body input only; voxel presentation follows
    // spatial demand and is not authored here.
    landmarks.register_body(definition, semantic, system_scale);
    if definition.arrival_direction.is_some() {
        let site = body_surface_site(
            semantic,
            field,
            center,
            frame,
            bootstrap_direction,
            scale(HUMAN_SURFACE_INTERACTION_SCALE),
        )
        .expect("authored Earth arrival direction must resolve a voxel surface");
        arrival_site.set(site);
    }

    semantic
}

fn body_surface_site(
    body: Entity,
    field: CelestialVoxelField,
    body_origin: UsfPosition,
    body_frame: UsfSemanticFrame,
    direction: Vec3,
    scale: SpatialScale,
) -> Option<BodySurfaceSite> {
    let up = direction.normalize_or_zero();
    if up == Vec3::ZERO {
        return None;
    }

    let surface = field.surface_position(&body_origin, body_frame, up).ok()?;
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

pub(in crate::game::world::fixture) fn audit_fixture_semantic_authority(
    bodies: Query<
        (
            Entity,
            &Name,
            Option<&UsfPosition>,
            Option<&CelestialVoxelField>,
            Option<&VoxelSemanticAuthority>,
            Option<&UsfTravelInfluence>,
            Option<&RadialGravitySource>,
            Option<&UsfApproachRefinement>,
            Option<&UsfAuthorityPartitions>,
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
    for (entity, name, position, field, voxel_authority, travel, gravity, refinement, partitions) in
        entries.iter().copied()
    {
        let partition_count = partitions.map_or(0, UsfAuthorityPartitions::len);
        let valid = position.is_some()
            && field.is_some()
            && voxel_authority.is_some()
            && travel.is_some()
            && gravity.is_some()
            && refinement.is_some()
            && partition_count > 0;

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
                partitions = partition_count,
                "celestial fixture authority invariant violation"
            );
        }
    }

    if invalid == 0 {
        info!(
            bodies = entries.len(),
            minimum_voxel_scale = %SpatialScale::MIN,
            "celestial fixture semantic/partition invariants healthy; dense voxel and presentation realizations are demand-owned"
        );
        *completed = true;
    }
}
