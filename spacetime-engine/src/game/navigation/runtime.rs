//! Controlled-subject semantic navigation adapters.

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    game::{
        control::LocalControlSubject,
        locomotion::{ControlledSubjectLocomotion, VelocitySemantics},
    },
    physics::character::{CharacterLocomotionFrame, CharacterMovementConfig},
    spatial::{
        SpatialRefinementDemand, SpatialScale, UsfApproachRefinement,
        UsfInteractionRequirement, UsfNavigationContext, UsfRadialGravitySource,
        UsfScaleLayer, UsfScaleRoleMask, UsfSpatialFrame, UsfSpatialTransitionQueue,
        UsfTransitionVelocity, UsfTravelInfluence, UsfTravelInfluenceKind,
        UsfTravelNeighborhood, UsfViewContext, UsfViewRenderAnchor,
    },
};

use super::{
    ApproachRefinementState, NavigationAudit, NavigationPresentationProfile,
    NavigationPresentationState, PrimaryBodyContext, TravelEnvelope, TravelProfile, TravelState,
};

/// Refreshes the sparse travel neighborhood and derives the characteristic
/// spatial length currently being navigated.
pub(super) fn sync_navigation_context(
    time: Res<Time>,
    frame: Res<UsfSpatialFrame>,
    influences: Query<(Entity, &UsfTravelInfluence)>,
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
            influences
                .iter()
                .map(|(entity, influence)| (entity, *influence)),
        );
    }

    let resolved = UsfNavigationContext::resolve(&position, scale, &neighborhood);
    if *navigation != resolved {
        info!(
            subject_scale = %scale,
            kind = ?resolved.kind(),
            source_scale = ?resolved.source_scale(),
            characteristic_metres = resolved.characteristic_length_scale0(),
            neighborhood_entries = neighborhood.len(),
            "resolved controlled-subject navigation context"
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

/// Resolves the next physical interaction chart.
///
/// Scales coarser than `maximum` contain no meaningful body-local interaction
/// representation, so entry may jump directly to that envelope boundary.
/// Inside the envelope, every scale is one decimal digit of responsibility and
/// handoff proceeds exactly one adjacent digit at a time.
fn next_interaction_digit(
    current: SpatialScale,
    desired: SpatialScale,
    maximum: SpatialScale,
) -> SpatialScale {
    if current > maximum {
        return maximum;
    }
    if desired == current {
        return current;
    }

    let step = if desired < current { -1 } else { 1 };
    SpatialScale::new(current.exponent() + step)
        .expect("adjacent interaction digit remains inside USF scale bounds")
}

/// Semantic planner for approaching refinable structure.
///
/// It owns neither rendering nor interaction. It determines how much spatial
/// resolution is needed and how much finer reality should be realized ahead of
/// the moving subject.
pub(super) fn plan_approach_refinement(
    time: Res<Time>,
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
    refinable: Query<(&UsfTravelInfluence, &UsfApproachRefinement)>,
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

    let mut selected = None::<(UsfTravelInfluence, UsfApproachRefinement, f64)>;
    for (influence, refinement) in &refinable {
        let Some(measurement) = influence.measure_from(&observer) else {
            continue;
        };
        let relative = measurement.relative_proximity();
        if relative > profile.approach.activation_radii {
            continue;
        }
        if selected.is_none_or(|(_, _, current)| relative < current) {
            selected = Some((*influence, *refinement, relative));
        }
    }

    let Some((influence, refinement, _)) = selected else {
        state.active = false;
        state.interaction_target_scale = layer.scale();
        state.realization_target_scale = layer.scale();
        realization_demand.clear();
        return;
    };
    let Some(measurement) = influence.measure_from(&observer) else {
        state.active = false;
        state.interaction_target_scale = layer.scale();
        state.realization_target_scale = layer.scale();
        realization_demand.clear();
        return;
    };

    if !state.active {
        state.active = true;
        state.continuous_exponent = f32::from(layer.scale().exponent());
    }

    state.minimum_scale = refinement.minimum_scale();

    let target_exponent = envelope
        .required_resolution_metres
        .max(1.0e-35)
        .log10()
        .clamp(
            f64::from(refinement.minimum_scale().exponent()),
            f64::from(influence.scale().exponent()),
        ) as f32;

    let current = state.continuous_exponent;
    let max_step = profile.approach.refinement_rate_decades_per_second
        * time.delta_secs().max(0.0);
    state.continuous_exponent = if target_exponent < current {
        (current - max_step).max(target_exponent)
    } else {
        (current + max_step).min(target_exponent)
    };

    let desired_interaction_scale = scale_for_resolution(
        10.0_f64.powf(f64::from(state.continuous_exponent)),
        refinement.minimum_scale(),
        influence.scale(),
    );
    state.interaction_target_scale = next_interaction_digit(
        layer.scale(),
        desired_interaction_scale,
        influence.scale(),
    );

    // Realization leads interaction by the current travel lookahead horizon.
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

/// Publishes the current continuous interaction requirement.
///
/// This is intentionally not a one-shot transition command. Publishing the
/// current Scale Slice is meaningful: it explicitly supersedes/cancels an older
/// finer requirement that may still be waiting for coverage.
pub(super) fn sync_approach_interaction_requirement(
    subject: Single<
        (
            &UsfScaleLayer,
            &UsfManifestationOf,
            &ControlledSubjectLocomotion,
            &TravelProfile,
            &PrimaryBodyContext,
            &ApproachRefinementState,
        ),
        With<LocalControlSubject>,
    >,
    mut transitions: ResMut<UsfSpatialTransitionQueue>,
) {
    let (layer, manifestation, locomotion, profile, primary, state) =
        subject.into_inner();

    let velocity = match locomotion.velocity_semantics() {
        VelocitySemantics::PreserveNative => UsfTransitionVelocity::PreserveNative,
        VelocitySemantics::PreserveCanonical => UsfTransitionVelocity::PreserveCanonical,
        VelocitySemantics::Zero => UsfTransitionVelocity::Zero,
    };

    let target_scale = if state.active {
        state.interaction_target_scale
    } else {
        layer.scale()
    };

    let mut requirement =
        UsfInteractionRequirement::new(manifestation.0, target_scale, velocity);

    if state.active
        && target_scale != layer.scale()
        && let Some(authority) = primary.entity()
    {
        requirement = requirement.requiring_coverage_from(
            authority,
            UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION),
            profile.approach.interaction_handoff_coverage_radius_native,
        );
    }

    transitions.set_interaction_requirement(requirement);
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
        interaction_target_scale: approach.active.then_some(approach.interaction_target_scale),
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

const GRAVITY_FIELD_RADIUS_MULTIPLIER: f64 = 8.0;

/// Projects the already-resolved primary body into the subject's local physical
/// frame. Body selection itself is centralized in [`sync_travel_state`].
pub(super) fn sync_planetary_gravity(
    frame: Res<UsfSpatialFrame>,
    subject: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &PrimaryBodyContext,
            &mut CharacterLocomotionFrame,
            &mut CharacterMovementConfig,
            &mut TravelState,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (body, layer, primary, mut locomotion, mut movement, mut travel) =
        subject.into_inner();

    if !primary.is_resolved() || primary.surface_gravity_metres_per_second2() <= 0.0 {
        travel.local_gravity = 0.0;
        movement.gravity = 0.0;
        return;
    }

    let Ok(position) = frame
        .origin()
        .translated_at_scale(layer.scale(), body.translation)
    else {
        return;
    };

    let radius = primary.radius_metres();
    let field_scale = primary.field_scale();
    let radius_native = field_scale.scale0_to_native_f64(radius);
    let bound_native = (radius_native * GRAVITY_FIELD_RADIUS_MULTIPLIER)
        .max(radius_native + 1.0)
        .min(f64::from(f32::MAX)) as f32;

    let Ok(relative) = position.relative_at_scale_bounded(
        &primary.center(),
        field_scale,
        bound_native,
    ) else {
        travel.local_gravity = 0.0;
        movement.gravity = 0.0;
        return;
    };

    let distance_scale0 =
        f64::from(relative.length()) * field_scale.scale0_units_per_native();
    if distance_scale0 > radius * GRAVITY_FIELD_RADIUS_MULTIPLIER {
        travel.local_gravity = 0.0;
        movement.gravity = 0.0;
        return;
    }

    let up = relative.normalize_or_zero();
    if up != Vec3::ZERO {
        locomotion.up = up;
    }

    let gravity_factor = if distance_scale0 >= radius {
        (radius / distance_scale0.max(f64::EPSILON)).powi(2)
    } else {
        // Uniform-sphere fallback prevents missing collision from becoming an
        // artificial black-hole acceleration toward the center.
        (distance_scale0 / radius).clamp(0.0, 1.0)
    };
    let gravity = (f64::from(primary.surface_gravity_metres_per_second2()) * gravity_factor)
        .clamp(0.0, f64::from(f32::MAX)) as f32;

    travel.local_gravity = gravity;
    movement.gravity = layer.scale().metres_to_native_f32(gravity);
}

/// Resolves one primary hard body and derives travel telemetry from that same
/// context. Gravity/orbit consumers no longer perform independent body scans.
pub(super) fn sync_travel_state(
    frame: Res<UsfSpatialFrame>,
    gravity_sources: Query<&UsfRadialGravitySource>,
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
        .influences_with_entities()
        .filter(|(_, influence)| matches!(influence.kind(), UsfTravelInfluenceKind::HardBody))
        .filter_map(|(entity, influence)| {
            influence
                .measure_from(&position)
                .map(|measurement| (entity, influence, measurement))
        })
        .min_by(|(_, _, a), (_, _, b)| {
            a.boundary_clearance_scale0()
                .total_cmp(&b.boundary_clearance_scale0())
        });

    let Some((entity, influence, measurement)) = nearest else {
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

    let gravity = gravity_sources.get(entity).ok().copied();
    let center = gravity.map_or(influence.anchor(), UsfRadialGravitySource::center);
    let field_scale = gravity.map_or(influence.scale(), UsfRadialGravitySource::field_scale);
    let surface_gravity =
        gravity.map_or(0.0, UsfRadialGravitySource::surface_gravity);
    let gravity_radius =
        gravity.map_or(radius, UsfRadialGravitySource::radius_scale0);

    *primary = PrimaryBodyContext::resolved(
        entity,
        center,
        gravity_radius,
        field_scale,
        surface_gravity,
        measurement.center_distance_scale0(),
        clearance,
    );
}

#[cfg(test)]
mod interaction_digit_tests {
    use super::*;

    #[test]
    fn entry_jumps_to_coarsest_physical_body_digit() {
        let s35 = SpatialScale::new(35).unwrap();
        let s6 = SpatialScale::new(6).unwrap();
        let s0 = SpatialScale::ZERO;
        assert_eq!(next_interaction_digit(s35, s0, s6), s6);
    }

    #[test]
    fn physical_body_dropout_advances_one_digit_at_a_time() {
        let s6 = SpatialScale::new(6).unwrap();
        let s5 = SpatialScale::new(5).unwrap();
        let s0 = SpatialScale::ZERO;
        assert_eq!(next_interaction_digit(s6, s0, s6), s5);
    }

    #[test]
    fn physical_body_dropout_coarsens_one_digit_at_a_time() {
        let s3 = SpatialScale::new(3).unwrap();
        let s4 = SpatialScale::new(4).unwrap();
        let s6 = SpatialScale::new(6).unwrap();
        assert_eq!(next_interaction_digit(s3, s6, s6), s4);
    }
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
