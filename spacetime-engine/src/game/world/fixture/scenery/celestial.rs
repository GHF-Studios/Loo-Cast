//! One semantic celestial authority with demand-created voxel realizations.
//!
//! Semantic body authority exists independently of scale-local voxel worlds.
//! Those worlds are disposable representations created only for demanded Scales.

use bevy::prelude::*;

use crate::{
    ecs::{
        UsfAuthorityPartitionOf, UsfAuthorityPartitions, UsfEntity,
    },
    physics::gravity::RadialGravitySource,
    procedural_assets::ProceduralAssetLibrary,
    spatial::{
        SPATIAL_SCALE_MIN, SpatialScale, UsfApproachRefinement, UsfChartMask,
        UsfPosition, UsfSemanticFrame, UsfTravelBoundaryResolver, UsfTravelInfluence,
    },
    voxel::{
        CelestialVoxelField, CelestialVoxelRealizationPolicy, VoxelAuthority,
        VoxelScaleDomain,
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

#[derive(Component, Debug, Clone, Copy)]
pub(in crate::game::world::fixture) struct CelestialBodyAuthority;

pub(super) fn spawn_body(
    commands: &mut Commands,
    parent: Entity,
    definition: &BodyDefinition,
    assets: &ProceduralAssetLibrary,
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
    let frame = UsfSemanticFrame::identity();
    let radius_metres = definition.radius_metres;
    let detail_root = celestial_coarsest_scale(radius_metres);
    let field = CelestialVoxelField::new(
        radius_metres,
        detail_root,
        scale(definition.surface_detail_scale),
        definition.seed,
        definition.profile,
    );
    let name = definition.name;

    audit_canonical_surface_relief(name, field);

    //
    // The domain describes where this semantic body *can* realize a capability;
    // it does not select which Scale is currently physical. That selection is
    // demand/control policy in voxel::realization::roles_for_scale().
    //
    // Hard-coding collision to human S0 made an explicitly S+1-controlled ship
    // incapable of ever receiving terrain collision. Every supported terrain
    // chart may provide physical capability, but only the current controlled
    // interaction Scale is granted COLLISION/EDITING roles.
    let physical_slices =
        UsfChartMask::inclusive_range(SpatialScale::MIN, detail_root);
    let scale_domain = VoxelScaleDomain::contiguous(SpatialScale::MIN, detail_root)
        .with_collision_slices(physical_slices)
        .with_editing_slices(physical_slices);

    let nav_scale = celestial_coarsest_scale(radius_metres);
    let semantic = commands
        .spawn((
            Name::new(name),
            WorldMemberOf(parent),
            CelestialBodyAuthority,
            UsfEntity,
            center,
            frame,
            field,
            scale_domain,
            CelestialVoxelRealizationPolicy::new(
                assets.debug_grid.clone(),
            ),
            VoxelAuthority::default(),
            UsfTravelInfluence::hard_body(
                nav_scale,
                nav_scale.metres_to_native_f64(radius_metres),
            ),
            UsfTravelBoundaryResolver::new(field),
            RadialGravitySource::new(
                radius_metres,
                nav_scale,
                definition.gravity_metres_per_second2,
            ),
            UsfApproachRefinement::new(SpatialScale::MIN),
        ))
        .id();

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

    // Whole-body presentation is derived generically by voxel::planetary_surface.
    // The fixture owns semantic body input only; no presentation entity is authored here.
    landmarks.register_body(definition, center, system_scale);
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
}

fn audit_canonical_surface_relief(
    name: &str,
    field: CelestialVoxelField,
) {
    let mut minimum = f64::INFINITY;
    let mut maximum = f64::NEG_INFINITY;
    let mut valid = 0usize;
    let samples = 512usize;
    let golden_ratio = (1.0 + 5.0_f32.sqrt()) * 0.5;

    for index in 0..samples {
        let i = index as f32 + 0.5;
        let n = samples as f32;
        let y = 1.0 - 2.0 * i / n;
        let horizontal = (1.0 - y * y).max(0.0).sqrt();
        let theta = std::f32::consts::TAU * index as f32 / golden_ratio;
        let direction = Vec3::new(
            theta.cos() * horizontal,
            y,
            theta.sin() * horizontal,
        )
        .normalize();

        let Ok(surface) = field.surface_local_metres(direction) else {
            continue;
        };
        let relief = surface.length() - field.radius_metres();
        if relief.is_finite() {
            minimum = minimum.min(relief);
            maximum = maximum.max(relief);
            valid += 1;
        }
    }

    if valid == 0 {
        error!(
            body = name,
            "canonical celestial terrain audit produced no valid surface samples"
        );
        return;
    }

    info!(
        body = name,
        samples = valid,
        minimum_relief_metres = minimum,
        maximum_relief_metres = maximum,
        relief_span_metres = maximum - minimum,
        "canonical celestial terrain relief audit"
    );
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
    for (
        entity,
        name,
        position,
        field,
        voxel_authority,
        travel,
        gravity,
        refinement,
        partitions,
    ) in entries.iter().copied()
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
                "Earth authority invariant violation"
            );
        }
    }

    if invalid == 0 {
        info!(
            bodies = entries.len(),
            minimum_voxel_scale = %SpatialScale::MIN,
            "Earth semantic/partition invariants healthy; dense voxel and regional presentation realizations are demand-owned"
        );
        *completed = true;
    }
}
