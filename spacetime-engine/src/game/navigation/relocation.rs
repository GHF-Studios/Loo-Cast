//! Controlled canonical relocation request and navigation-owned coverage planning.

use bevy::prelude::*;

use crate::{
    ecs::UsfOwnershipQuery,
    game::{control::LocalControlSubject, locomotion::ScaleInteractionProxy},
    physics::PhysicalBoxHull,
    spatial::{
        SpatialDemandSource, SpatialScale, UsfApproachRefinement, UsfPosition, UsfScaleRoleMask,
        UsfSemanticFrame, UsfSpatialTransition, UsfSpatialTransitions, UsfTransitionVelocity,
        UsfTravelBoundaryProvider, UsfTravelInfluence, UsfTravelInfluenceKind,
    },
};

/// Request to relocate the currently controlled semantic subject.
///
/// Adapters specify destination intent. Navigation owns destination refinement
/// and collision-coverage planning; spatial transitions own transaction commit.
#[derive(Message, Debug, Clone, Copy)]
pub struct ControlledRelocationRequest {
    arrival: UsfPosition,
    interaction_scale: Option<SpatialScale>,
    view_exponent: Option<f32>,
    velocity: UsfTransitionVelocity,
}

impl ControlledRelocationRequest {
    pub const fn new(arrival: UsfPosition, velocity: UsfTransitionVelocity) -> Self {
        Self {
            arrival,
            interaction_scale: None,
            view_exponent: None,
            velocity,
        }
    }

    pub const fn with_interaction_scale(mut self, scale: SpatialScale) -> Self {
        self.interaction_scale = Some(scale);
        self
    }

    pub const fn with_view_exponent(mut self, exponent: f32) -> Self {
        self.view_exponent = Some(exponent);
        self
    }
}

pub(super) fn resolve_controlled_relocations(
    mut requests: MessageReader<ControlledRelocationRequest>,
    ownership: UsfOwnershipQuery,
    controlled: Query<
        (
            Entity,
            &SpatialDemandSource,
            &PhysicalBoxHull,
            Option<&ScaleInteractionProxy>,
        ),
        With<LocalControlSubject>,
    >,
    influences: Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &UsfTravelInfluence,
        Option<&UsfTravelBoundaryProvider>,
        Option<&UsfApproachRefinement>,
    )>,
    mut transitions: ResMut<UsfSpatialTransitions>,
) {
    let Ok((manifestation, demand, hull, proxy)) = controlled.single() else {
        for _ in requests.read() {}
        return;
    };
    let Some(subject) = ownership.semantic_of(manifestation) else {
        for _ in requests.read() {}
        return;
    };

    for request in requests.read() {
        let mut transition = UsfSpatialTransition::new(subject, request.arrival, request.velocity);

        if let Some(view_exponent) = request.view_exponent {
            transition = transition.with_view_exponent(view_exponent);
        }

        if let Some(scale) = request.interaction_scale {
            transition = transition.with_scale(scale);

            if let Some((authority, boundary_distance_native)) =
                refinable_hard_body_gate(request.arrival, scale, demand, &influences)
            {
                let clearance_metres = proxy.map_or(0.0, |proxy| proxy.clearance_metres());
                let subject_radius_native = scale
                    .metres_to_native_f32(hull.bounding_radius_metres() + clearance_metres)
                    .max(0.0);

                transition = transition.requiring_coverage_from(
                    authority,
                    UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION),
                    boundary_distance_native + subject_radius_native,
                );
            }
        }

        transitions.relocate(transition);
    }
}

fn refinable_hard_body_gate(
    arrival: UsfPosition,
    target_scale: SpatialScale,
    demand: &SpatialDemandSource,
    influences: &Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &UsfTravelInfluence,
        Option<&UsfTravelBoundaryProvider>,
        Option<&UsfApproachRefinement>,
    )>,
) -> Option<(Entity, f32)> {
    let demand_extent = demand.half_extent_native().max_element();
    if !demand_extent.is_finite() || demand_extent <= 0.0 {
        return None;
    }

    let metres_per_native = target_scale.metres_per_native();
    if !metres_per_native.is_finite() || metres_per_native <= 0.0 {
        return None;
    }

    let mut best = None::<(Entity, f32)>;
    for (entity, anchor, semantic_frame, influence, boundary, refinement) in influences {
        if refinement.is_none() || !matches!(influence.kind(), UsfTravelInfluenceKind::HardBody) {
            continue;
        }

        let Some(measurement) = influence.measure_from_at_scale(
            anchor,
            *semantic_frame,
            &arrival,
            target_scale,
            boundary,
        ) else {
            continue;
        };

        let boundary_distance_metres =
            measurement.boundary_clearance_metres() + measurement.penetration_depth_metres();
        let distance_native = (boundary_distance_metres / metres_per_native) as f32;
        if !distance_native.is_finite() || distance_native > demand_extent {
            continue;
        }

        if best.is_none_or(|(_, current)| distance_native < current) {
            best = Some((entity, distance_native.max(0.0)));
        }
    }

    best
}
