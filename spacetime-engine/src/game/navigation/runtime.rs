//! Controlled-subject semantic navigation adapters.

use bevy::prelude::*;

use crate::{
    ecs::UsfOwnershipQuery,
    game::control::LocalControlSubject,
    spatial::{
        SpatialRefinementDemand, SpatialScale, UsfApproachRefinement, UsfNavigationContext,
        UsfPosition, UsfScaleLayer, UsfSemanticFrame, UsfSpatialFrame,
        UsfSpatialTransitionApplied, UsfSpatialTransitionCause, UsfTravelBoundaryResolver,
        UsfTravelInfluence, UsfTravelInfluenceKind, UsfTravelNeighborhood, UsfViewContext,
        UsfViewRenderAnchor,
    },
};

use super::{
    ApproachRefinementState, NavigationAudit, NavigationPresentationProfile,
    NavigationPresentationState, PrimaryBodyContext, TravelEnvelope, TravelProfile, TravelState,
};

fn requested_transition_rebases_approach(cause: UsfSpatialTransitionCause) -> bool {
    matches!(cause, UsfSpatialTransitionCause::Requested)
}

/// Rebase future refinement state after an authoritative relocation.
///
/// An explicit relocation/control rechart can discontinuously replace the
/// controlled subject's interaction chart. Refinement intent at the previous
/// location is discarded; navigation does not choose the new interaction Scale.
pub(super) fn reconcile_approach_after_requested_transition(
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
                scale,
                transition.active_scale,
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

/// Refreshes the sparse travel neighborhood and derives the characteristic
/// spatial length currently being navigated.
pub(super) fn sync_navigation_context(
    time: Res<Time>,
    frame: Res<UsfSpatialFrame>,
    influences: Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &UsfTravelInfluence,
        Option<&UsfTravelBoundaryResolver>,
    )>,
    subject: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &mut UsfTravelNeighborhood,
            &mut UsfNavigationContext,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (body, layer, mut neighborhood, mut navigation) = subject.into_inner();
    let scale = layer.scale();
    let Ok(position) = frame
        .origin()
        .translated_at_scale(scale, body.translation)
    else {
        return;
    };

    neighborhood.advance(time.delta_secs().max(0.0));
    if neighborhood.needs_refresh(&position, scale) {
        neighborhood.refresh(
            position,
            scale,
            influences.iter().map(|(entity, anchor, semantic_frame, influence, boundary)| {
                (entity, *anchor, *semantic_frame, *influence, boundary.cloned())
            }),
        );
    }

    let resolved = UsfNavigationContext::resolve(&position, scale, &neighborhood);
    if *navigation != resolved {
        debug!(
            subject_scale = %scale,
            kind = ?resolved.kind(),
            source_scale = ?resolved.source_scale(),
            characteristic_metres = resolved.characteristic_length_scale0(),
            neighborhood_entries = neighborhood.len(),
            "controlled-subject navigation context changed"
        );
        *navigation = resolved;
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
        .clamp(
            f64::from(minimum.exponent()),
            f64::from(maximum.exponent()),
        ) as i8;
    SpatialScale::new(exponent).expect("clamped USF resolution scale")
}


/// Semantic planner for approaching refinable structure.
///
/// It owns neither rendering nor interaction. It determines how much spatial
/// resolution is needed and how much finer reality should be realized ahead of
/// the moving subject.
pub(super) fn plan_approach_refinement(
    frame: Res<UsfSpatialFrame>,
    subject: Single<
        (
            &Transform,
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
        Option<&UsfTravelBoundaryResolver>,
    )>,
) {
    let (
        body,
        layer,
        profile,
        envelope,
        mut state,
        mut realization_demand,
    ) = subject.into_inner();

    let observer_scale = layer.scale();
    let Ok(observer) = frame
        .origin()
        .translated_at_scale(observer_scale, body.translation)
    else {
        return;
    };

    let mut selected = None::<(
        UsfPosition,
        UsfSemanticFrame,
        UsfTravelInfluence,
        UsfApproachRefinement,
        Option<UsfTravelBoundaryResolver>,
        f64,
    )>;
    for (anchor, semantic_frame, influence, refinement, boundary) in &refinable {
        let Some(measurement) = influence.measure_from_at_scale(
            anchor,
            *semantic_frame,
            &observer,
            observer_scale,
            boundary,
        )
        else {
            continue;
        };
        let relative = measurement.relative_proximity();
        if relative > profile.approach.activation_radii {
            continue;
        }
        if selected
            .as_ref()
            .is_none_or(|(_, _, _, _, _, current)| relative < *current)
        {
            selected = Some((
                *anchor,
                *semantic_frame,
                *influence,
                *refinement,
                boundary.cloned(),
                relative,
            ));
        }
    }

    let Some((anchor, semantic_frame, influence, refinement, boundary, _)) = selected else {
        state.active = false;
        state.minimum_scale = layer.scale();
        state.realization_target_scale = layer.scale();
        realization_demand.clear();
        return;
    };

    let Some(measurement) = influence.measure_from_at_scale(
        &anchor,
        semantic_frame,
        &observer,
        observer_scale,
        boundary.as_ref(),
    ) else {
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
        (measurement.boundary_clearance_scale0() - envelope.lookahead_metres).max(1.0);
    let divisor = profile.approach.resolution_divisor.max(f64::EPSILON);
    let future_resolution = (future_clearance / divisor).max(1.0);
    state.realization_target_scale = scale_for_resolution(
        future_resolution,
        refinement.minimum_scale(),
        influence.scale(),
    );

    realization_demand.request_through(state.realization_target_scale);
}

fn presentation_exponent_for_characteristic(
    characteristic_metres: f64,
    profile: &NavigationPresentationProfile,
) -> f32 {
    let raw = characteristic_metres.max(1.0e-35).log10() as f32;
    raw.clamp(
        profile.minimum_scale.exponent() as f32,
        profile.maximum_scale.exponent() as f32,
    )
}

/// Default primary-view presentation planner.
///
/// Presentation consumes semantic navigation context but remains independent
/// from interaction ownership/refinement. The first structured context snaps
/// out of the meaningless S35 bootstrap immediately; later changes are smooth.
pub(super) fn sync_navigation_presentation(
    time: Res<Time>,
    navigation: Single<&UsfNavigationContext, With<LocalControlSubject>>,
    view: Single<
        (
            &NavigationPresentationProfile,
            &mut NavigationPresentationState,
            &mut UsfViewContext,
        ),
        With<UsfViewRenderAnchor>,
    >,
) {
    let navigation = navigation.into_inner();
    let (profile, mut state, mut view) = view.into_inner();

    // Fallback has no semantic structure from which to choose a meaningful
    // presentation scale. Keep the bootstrap unresolved until structure exists.
    let Some(source_scale) = navigation.source_scale() else {
        return;
    };

    let automatic = presentation_exponent_for_characteristic(
        navigation.characteristic_length_scale0(),
        profile,
    );
    state.automatic_target_exponent = automatic;

    let bias_limit = profile.maximum_manual_bias_decades.max(0.0);
    state.manual_bias_decades = state.manual_bias_decades.clamp(-bias_limit, bias_limit);

    let effective = (automatic + state.manual_bias_decades).clamp(
        profile.minimum_scale.exponent() as f32,
        profile.maximum_scale.exponent() as f32,
    );
    state.effective_target_exponent = effective;

    if !state.initialized {
        view.set_continuous_exponent(effective);
        state.initialized = true;
        info!(
            source_scale = %source_scale,
            characteristic_metres = navigation.characteristic_length_scale0(),
            target_exponent = effective,
            "resolved initial USF presentation scale from semantic navigation context"
        );
        return;
    }

    let current = view.continuous_exponent();
    let max_step =
        profile.response_decades_per_second.max(0.0) * time.delta_secs().max(0.0);
    let next = if effective < current {
        (current - max_step).max(effective)
    } else {
        (current + max_step).min(effective)
    };

    if (next - current).abs() > 1.0e-4 {
        view.set_continuous_exponent(next);
    }
}

/// Publishes one compact end-to-end contract snapshot for diagnostics.
///
/// This observes already-resolved state; it owns no navigation or view policy.
pub(super) fn audit_navigation_contract(
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &UsfNavigationContext,
            &PrimaryBodyContext,
            &ApproachRefinementState,
        ),
        With<LocalControlSubject>,
    >,
    view: Single<
        (&UsfViewContext, &NavigationPresentationState),
        With<UsfViewRenderAnchor>,
    >,
    mut audit: ResMut<NavigationAudit>,
    mut was_unhealthy: Local<bool>,
) {
    let (entity, layer, navigation, primary, approach) = subject.into_inner();
    let (view, presentation) = view.into_inner();

    let characteristic = navigation.characteristic_length_scale0();
    let view_exponent = view.continuous_exponent();
    let target_exponent = presentation.effective_target_exponent();

    let structured = navigation.source_scale().is_some();
    let healthy = characteristic.is_finite()
        && characteristic > 0.0
        && view_exponent.is_finite()
        && target_exponent.is_finite()
        && (!structured || presentation.initialized());

    let next = NavigationAudit {
        healthy,
        subject: Some(entity),
        subject_scale: Some(layer.scale()),
        primary_body: primary.entity(),
        primary_clearance_metres: primary
            .is_resolved()
            .then_some(primary.clearance_metres()),
        navigation_source_scale: navigation.source_scale(),
        characteristic_length_metres: characteristic,
        approach_active: approach.active,
        realization_target_scale: approach.active.then_some(approach.realization_target_scale),
        view_exponent,
        presentation_target_exponent: target_exponent,
    };

    if !healthy && !*was_unhealthy {
        error!(?next, "navigation/presentation contract became unhealthy");
    }
    *was_unhealthy = !healthy;
    *audit = next;
}

/// Resolves one primary hard body and derives travel telemetry from that same
/// context. Physical gravity is queried independently from the gravity-field
/// subsystem; navigation owns only body-relative travel geometry.
pub(super) fn sync_travel_state(
    frame: Res<UsfSpatialFrame>,
    subject: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &UsfTravelNeighborhood,
            &TravelProfile,
            &mut TravelState,
            &mut PrimaryBodyContext,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (body, layer, neighborhood, profile, mut state, mut primary) =
        subject.into_inner();

    let Ok(position) = frame
        .origin()
        .translated_at_scale(layer.scale(), body.translation)
    else {
        return;
    };

    let nearest = neighborhood
        .measurements_from(&position, layer.scale())
        .filter(|(_, _, influence, _)| {
            matches!(influence.kind(), UsfTravelInfluenceKind::HardBody)
        })
        .min_by(|(_, _, _, a), (_, _, _, b)| {
            a.boundary_clearance_scale0()
                .total_cmp(&b.boundary_clearance_scale0())
        });

    let Some((entity, anchor, influence, measurement)) = nearest else {
        *state = TravelState::default();
        *primary = PrimaryBodyContext::default();
        return;
    };

    let radius = measurement.extent_radius_scale0();
    let clearance = measurement.boundary_clearance_scale0();
    let handoff = profile.planetary_handoff_clearance(radius);

    state.nearest_body_clearance_scale0 = Some(clearance);
    state.nearest_body_radius_scale0 = Some(radius);
    state.planetary_handoff_clearance_scale0 = Some(handoff);
    state.planetary_handoff_available = clearance <= handoff;
    state.planetary_context =
        measurement.relative_proximity() <= profile.approach.activation_radii;
    state.critical_dropout = clearance <= handoff;

    *primary = PrimaryBodyContext::resolved(
        entity,
        anchor,
        radius,
        influence.scale(),
        measurement.center_distance_scale0(),
        clearance,
    );
}

#[cfg(test)]
mod presentation_tests {
    use super::*;

    #[test]
    fn presentation_scale_tracks_canonical_navigation_length() {
        let profile = NavigationPresentationProfile::default();

        assert_eq!(presentation_exponent_for_characteristic(1.0, &profile), 0.0);
        assert_eq!(
            presentation_exponent_for_characteristic(1_000_000.0, &profile),
            6.0,
        );
        assert_eq!(
            presentation_exponent_for_characteristic(1.0e30, &profile),
            30.0,
        );
    }

    #[test]
    fn presentation_scale_respects_current_content_floor() {
        let profile = NavigationPresentationProfile::default();
        assert_eq!(
            presentation_exponent_for_characteristic(1.0e-12, &profile),
            SpatialScale::ZERO.exponent() as f32,
        );
    }
}
