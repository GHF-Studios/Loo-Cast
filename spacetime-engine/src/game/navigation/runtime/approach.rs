//! Controlled-subject refinement intent and transition reconciliation.

use super::*;

fn requested_transition_rebases_approach(cause: UsfSpatialTransitionCause) -> bool {
    matches!(cause, UsfSpatialTransitionCause::Requested)
}

/// Rebase future refinement state after an authoritative relocation.
///
/// An explicit relocation/control rechart can discontinuously replace the
/// controlled subject's interaction chart. Refinement intent at the previous
/// location is discarded; navigation does not choose the new interaction Scale.
pub(in crate::game::navigation) fn reconcile_approach_after_requested_transition(
    mut transitions: MessageReader<UsfSpatialTransitionApplied>,
    ownership: UsfOwnershipQuery,
    mut subjects: Query<
        (Entity, &UsfScaleLayer, &mut ApproachRefinementState),
        With<LocalControlSubject>,
    >,
) {
    for transition in transitions.read() {
        if !requested_transition_rebases_approach(transition.cause) {
            continue;
        }

        for (entity, layer, mut approach) in &mut subjects {
            if ownership.semantic_of(entity) != Some(transition.subject) {
                continue;
            }

            let scale = layer.scale();
            debug_assert_eq!(
                scale, transition.active_scale,
                "navigation reconciliation must observe the applied runtime scale",
            );

            approach.active = false;
            approach.minimum_scale = scale;
            approach.realization_target_scale = scale;

            debug!(
                subject = ?transition.subject,
                previous_scale = %transition.previous_scale,
                active_scale = %scale,
                "rebased approach refinement after requested spatial transition"
            );
        }
    }
}

fn scale_for_resolution(
    resolution_metres: f64,
    minimum: SpatialScale,
    maximum: SpatialScale,
) -> SpatialScale {
    let exponent = resolution_metres
        .max(1.0e-35)
        .log10()
        .ceil()
        .clamp(f64::from(minimum.exponent()), f64::from(maximum.exponent()))
        as i8;
    SpatialScale::new(exponent).expect("clamped USF resolution scale")
}

/// Semantic planner for approaching refinable structure.
///
/// It owns neither rendering nor interaction. It determines how much spatial
/// resolution is needed and how much finer reality should be realized ahead of
/// the moving subject.
pub(in crate::game::navigation) fn plan_approach_refinement(
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &TravelProfile,
            &TravelEnvelope,
            &mut ApproachRefinementState,
            &mut SpatialRefinementDemand,
        ),
        With<LocalControlSubject>,
    >,
    refinable: Query<(
        &UsfPosition,
        &UsfSemanticFrame,
        &UsfTravelInfluence,
        &UsfApproachRefinement,
        Option<&UsfTravelBoundaryProvider>,
    )>,
) {
    let (entity, layer, profile, envelope, mut state, mut realization_demand) =
        subject.into_inner();

    let observer_scale = layer.scale();
    let Some(semantic) = ownership.semantic_of(entity) else {
        return;
    };
    let Ok(observer) = semantic_positions.get(semantic) else {
        return;
    };

    let selected =
        refinable
            .iter()
            .filter_map(
                |(anchor, semantic_frame, influence, refinement, boundary)| {
                    let measurement = influence.measure_from_at_scale(
                        anchor,
                        *semantic_frame,
                        observer,
                        observer_scale,
                        boundary,
                    )?;
                    (measurement.relative_proximity() <= profile.approach.activation_radii)
                        .then_some((*influence, *refinement, measurement))
                },
            )
            .reduce(|current, candidate| {
                if current.2.relative_proximity() <= candidate.2.relative_proximity() {
                    current
                } else {
                    candidate
                }
            });

    let Some((influence, refinement, measurement)) = selected else {
        state.active = false;
        state.minimum_scale = layer.scale();
        state.realization_target_scale = layer.scale();
        realization_demand.clear();
        return;
    };

    state.active = true;
    state.minimum_scale = refinement.minimum_scale();

    // Clearance, speed and lookahead may change how much reality is prepared
    // ahead. They are deliberately unable to rechart the controlled subject.
    let future_clearance =
        (measurement.boundary_clearance_metres() - envelope.lookahead_metres).max(1.0);
    let divisor = profile.approach.resolution_divisor.max(f64::EPSILON);
    let future_resolution = (future_clearance / divisor).max(1.0);
    state.realization_target_scale = scale_for_resolution(
        future_resolution,
        refinement.minimum_scale(),
        influence.scale(),
    );

    realization_demand.request_through(state.realization_target_scale);
}
