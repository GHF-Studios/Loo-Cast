//! Physical-boundary contact preparation and scope geometry.

use super::*;

// Speed extends physical preparation only when it closes on terrain.
const EXTERIOR_CONTACT_PREPARATION_SECONDS: f64 = 1.5;
const EXTERIOR_CONTACT_GUARD_CHUNKS: f32 = 2.0;

/// Contact evidence for one semantic body and one spatial demand source.
/// Full-SDF clearance and outer-shell clearance answer different questions:
/// cave air remains inside the body's exterior envelope.
pub(super) struct CelestialContact {
    pub(super) boundary_center: Option<UsfPosition>,
    pub(super) signed_clearance_metres: f64,
    pub(super) outer_clearance_metres: f64,
    pub(super) closing_speed_metres_per_second: f64,
}

pub(super) fn observe_celestial_contact(
    body_origin: &UsfPosition,
    body_frame: &UsfSemanticFrame,
    field: &CelestialVoxelField,
    source: VoxelDemandSource,
    motions: &SpatialDemandMotionSnapshot,
) -> Option<CelestialContact> {
    let source_local_metres = body_frame
        .world_to_local_metres(
            body_origin,
            &source.scope.center(),
            SpatialScale::ZERO,
            f64::MAX,
        )
        .ok()?;
    let signed_clearance_metres = field.signed_distance_local_metres(source_local_metres)?;
    let outer_clearance_metres = field
        .outer_signed_distance_local_metres(source_local_metres)
        .unwrap_or(signed_clearance_metres);
    let boundary_center = field
        .boundary_near(body_origin, *body_frame, &source.scope.center(), f64::MAX)
        .map(|(boundary, _)| boundary);

    // Only motion toward the nearest full-SDF boundary extends preparation.
    let closing_speed_metres_per_second = boundary_center
        .and_then(|boundary| {
            boundary
                .relative_at_scale_bounded_f64(&source.scope.center(), SpatialScale::ZERO, f64::MAX)
                .ok()
        })
        .map_or(0.0, |toward_boundary_metres| {
            let distance = toward_boundary_metres.length();
            if !distance.is_finite() || distance <= f64::EPSILON {
                return 0.0;
            }
            let direction = toward_boundary_metres / distance;
            motions
                .velocity_metres_per_second(source.scope.source())
                .dot(direction)
                .max(0.0)
        });

    Some(CelestialContact {
        boundary_center,
        signed_clearance_metres,
        outer_clearance_metres,
        closing_speed_metres_per_second,
    })
}

pub(super) fn priority_focus_within_scope(
    boundary_center: Option<UsfPosition>,
    scope: SpatialDemandScope,
    scale: SpatialScale,
) -> Option<UsfPosition> {
    const BOUNDARY_MARGIN_CHUNKS: f32 = 2.0;
    const BOUNDARY_MARGIN_NATIVE: f32 = 1.0;
    const CONTAINMENT_TOLERANCE_NATIVE: f32 = 0.001;

    boundary_center.filter(|boundary| {
        let bound = scope.half_extent_native().length()
            + MATERIALIZATION_CHUNK_SIZE as f32 * BOUNDARY_MARGIN_CHUNKS
            + BOUNDARY_MARGIN_NATIVE;
        boundary
            .relative_at_scale_bounded(&scope.center(), scale, bound)
            .ok()
            .is_some_and(|delta| {
                let half = scope.half_extent_native();
                delta.x.abs() <= half.x + CONTAINMENT_TOLERANCE_NATIVE
                    && delta.y.abs() <= half.y + CONTAINMENT_TOLERANCE_NATIVE
                    && delta.z.abs() <= half.z + CONTAINMENT_TOLERANCE_NATIVE
            })
    })
}

pub(super) fn realization_plan(
    source: VoxelDemandSource,
    domain: VoxelScaleDomain,
) -> UsfRefinementPlan {
    let tip_extent = source
        .refinement_half_extent_native
        .unwrap_or(Vec3::splat(domain.local_patch_half_extent_native()));

    UsfRefinementPlan::new(
        source.scope.scale(),
        source.minimum_realization_scale,
        domain.realization_slices(),
        tip_extent,
        source.scope.priority().saturating_add(1_000),
    )
    .with_residency_halo_native(Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5))
}

pub(super) fn materialization_residency_extent(half_extent_native: Vec3) -> Vec3 {
    half_extent_native + Vec3::splat(MATERIALIZATION_CHUNK_SIZE as f32 * 0.5)
}

fn corridor_scope_between(
    source: SpatialDemandScope,
    boundary: UsfPosition,
    target_scale: SpatialScale,
    base_half_extent_native: Vec3,
    priority: i32,
    maximum_corridor_native: f32,
) -> Option<SpatialDemandScope> {
    let delta = boundary
        .relative_at_scale_bounded(
            &source.center(),
            target_scale,
            maximum_corridor_native.max(1.0),
        )
        .ok()?;
    let midpoint = source
        .center()
        .translated_at_scale(target_scale, delta * 0.5)
        .ok()?;
    let half_extent = base_half_extent_native + delta.abs() * 0.5;

    Some(SpatialDemandScope::at_scale(
        source.source(),
        target_scale,
        midpoint,
        half_extent,
        priority,
    ))
}

pub(super) fn celestial_contact_volume_demand(
    boundary_center: Option<UsfPosition>,
    signed_clearance_metres: f64,
    outer_clearance_metres: f64,
    closing_speed_metres_per_second: f64,
    source: SpatialDemandScope,
    target_scale: SpatialScale,
    half_extent_native: Vec3,
    priority: i32,
) -> Option<SpatialDemandScope> {
    if !signed_clearance_metres.is_finite() || !outer_clearance_metres.is_finite() {
        return None;
    }

    let metres_per_native = target_scale.metres_per_native();

    //
    // The old 8192-native activation radius belonged to a world where dense
    // terrain also carried far visual context. Binary presentation owns that
    // now. Dense exterior terrain owns only near and predicted physical contact.
    let footprint_native = half_extent_native.length()
        + MATERIALIZATION_CHUNK_SIZE as f32 * EXTERIOR_CONTACT_GUARD_CHUNKS;
    let local_contact_horizon_metres = f64::from(footprint_native) * metres_per_native;
    let predictive_contact_horizon_metres =
        closing_speed_metres_per_second.max(0.0) * EXTERIOR_CONTACT_PREPARATION_SECONDS;
    let exterior_limit_metres = local_contact_horizon_metres + predictive_contact_horizon_metres;

    // Full SDF can be positive inside a cave. Only outer-shell clearance proves
    // the observer is truly outside the planetary volume.
    if outer_clearance_metres > exterior_limit_metres {
        return None;
    }

    let local_scope = SpatialDemandScope::at_scale(
        source.source(),
        target_scale,
        source.center(),
        half_extent_native,
        priority,
    );

    let Some(boundary) = boundary_center else {
        return Some(local_scope);
    };

    // This corridor is contact-preparation, not a giant deep-volume prism.
    let corridor_native = (footprint_native * 2.0).max(MATERIALIZATION_CHUNK_SIZE as f32 * 4.0);
    let corridor_metres = f64::from(corridor_native) * metres_per_native;

    if signed_clearance_metres.abs() <= corridor_metres {
        if let Some(scope) = corridor_scope_between(
            source,
            boundary,
            target_scale,
            half_extent_native,
            priority,
            corridor_native + footprint_native,
        ) {
            return Some(scope);
        }
    }

    if outer_clearance_metres > 0.0 {
        // Outside: prepare actual terrain, not an all-air cube around the view.
        Some(SpatialDemandScope::at_scale(
            source.source(),
            target_scale,
            boundary,
            half_extent_native,
            priority,
        ))
    } else {
        // Inside the body envelope, including cave voids, stay subject-local.
        Some(local_scope)
    }
}
