//! Observer-local coarse navigation context.

use super::*;

/// Refreshes the sparse travel neighborhood and derives the characteristic
/// spatial length currently being navigated.
///
/// Keeping this outside Cruise means ordinary coarse movement and Cruise can
/// share the same semantic neighborhood without each owning a private scan.
pub(in crate::game::player) fn sync_navigation_context(
    time: Res<Time>,
    frame: Res<UsfSpatialFrame>,
    influences: Query<(Entity, &UsfTravelInfluence)>,
    player: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &mut UsfTravelNeighborhood,
            &mut UsfNavigationContext,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (body, layer, mut neighborhood, mut navigation) = player.into_inner();
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
        *navigation = resolved;
    }
}


const APPROACH_REFINEMENT_ACTIVATION_RADII: f64 = 256.0;
const APPROACH_REFINEMENT_RATE_DECADES_PER_SECOND: f32 = 6.0;
const APPROACH_RESOLUTION_DIVISOR: f64 = 4.0;
const FINAL_HANDOFF_COVERAGE_RADIUS_NATIVE: f32 = 32.0;

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
/// It owns neither rendering nor interaction. It only determines how much
/// spatial resolution is needed now and how much finer reality should be
/// realized ahead of the moving subject.
pub(in crate::game::player) fn plan_approach_refinement(
    time: Res<Time>,
    frame: Res<UsfSpatialFrame>,
    player: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &PlayerTravelEnvelope,
            &mut PlayerApproachRefinementState,
            &mut SpatialRefinementDemand,
        ),
        With<LocalControlSubject>,
    >,
    refinable: Query<(&UsfTravelInfluence, &UsfApproachRefinement)>,
) {
    let (body, layer, envelope, mut state, mut realization_demand) = player.into_inner();
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
        if relative > APPROACH_REFINEMENT_ACTIVATION_RADII {
            continue;
        }
        if selected.is_none_or(|(_, _, current)| relative < current) {
            selected = Some((*influence, *refinement, relative));
        }
    }

    let Some((influence, refinement, _)) = selected else {
        state.active = false;
        realization_demand.clear();
        return;
    };
    let Some(measurement) = influence.measure_from(&observer) else {
        state.active = false;
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
    let max_step =
        APPROACH_REFINEMENT_RATE_DECADES_PER_SECOND * time.delta_secs().max(0.0);
    state.continuous_exponent = if target_exponent < current {
        (current - max_step).max(target_exponent)
    } else {
        (current + max_step).min(target_exponent)
    };

    state.interaction_target_scale = scale_for_resolution(
        10.0_f64.powf(f64::from(state.continuous_exponent)),
        refinement.minimum_scale(),
        influence.scale(),
    );

    // Realization leads interaction by the current braking/lookahead horizon.
    let future_clearance = (measurement.boundary_clearance_scale0()
        - envelope.lookahead_metres)
        .max(1.0);
    let future_resolution = (future_clearance / APPROACH_RESOLUTION_DIVISOR).max(1.0);
    state.realization_target_scale = scale_for_resolution(
        future_resolution,
        refinement.minimum_scale(),
        influence.scale(),
    );

    realization_demand.request_through(state.realization_target_scale);
}

/// Presentation adapter for approach refinement.
///
/// It consumes planner state. Camera/view state has no authority over physical
/// interaction or materialization.
pub(in crate::game::player) fn sync_approach_presentation(
    state: Single<&PlayerApproachRefinementState, With<LocalControlSubject>>,
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
) {
    if !state.active {
        return;
    }

    if (view.continuous_exponent() - state.continuous_exponent).abs() > 1.0e-4 {
        view.set_continuous_exponent(state.continuous_exponent);
    }
}

/// Interaction adapter for approach refinement.
///
/// A requested Scale Slice remains pending intent until required realized
/// coverage exists. Velocity semantics come from the locomotion state machine.
pub(in crate::game::player) fn request_approach_interaction_handoff(
    frame: Res<UsfSpatialFrame>,
    player: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &UsfManifestationOf,
            &ControlledSubjectLocomotion,
            &PlayerApproachRefinementState,
        ),
        With<LocalControlSubject>,
    >,
    mut transitions: ResMut<UsfSpatialTransitionQueue>,
) {
    let (body, layer, manifestation, locomotion, state) = player.into_inner();
    if !state.active || state.interaction_target_scale == layer.scale() {
        return;
    }

    let Ok(observer) = frame
        .origin()
        .translated_at_scale(layer.scale(), body.translation)
    else {
        return;
    };

    let velocity = match locomotion.velocity_semantics() {
        PlayerVelocitySemantics::PreserveNative => UsfTransitionVelocity::PreserveNative,
        PlayerVelocitySemantics::PreserveCanonical => UsfTransitionVelocity::PreserveCanonical,
        PlayerVelocitySemantics::Zero => UsfTransitionVelocity::Zero,
    };

    let transition = UsfSpatialTransition::new(manifestation.0, observer, velocity)
        .with_scale(state.interaction_target_scale);

    let transition = if state.interaction_target_scale == state.minimum_scale {
        transition.requiring_coverage(
            UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION),
            FINAL_HANDOFF_COVERAGE_RADIUS_NATIVE,
        )
    } else {
        transition
    };

    transitions.request(transition);
}


const GRAVITY_FIELD_RADIUS_MULTIPLIER: f64 = 8.0;

pub(in crate::game::player) fn sync_planetary_gravity(
    frame: Res<UsfSpatialFrame>,
    sources: Query<&UsfRadialGravitySource>,
    player: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &mut CharacterLocomotionFrame,
            &mut CharacterMovementConfig,
            &mut PlayerTravelState,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (body, layer, mut locomotion, mut movement, mut travel) = player.into_inner();
    let Ok(position) = frame
        .origin()
        .translated_at_scale(layer.scale(), body.translation)
    else {
        return;
    };

    let mut selected = None::<(f64, Vec3, UsfRadialGravitySource)>;
    for source in &sources {
        let radius_native =
            source.field_scale().scale0_to_native_f64(source.radius_scale0());
        let bound_native = (radius_native * GRAVITY_FIELD_RADIUS_MULTIPLIER)
            .max(radius_native + 1.0)
            .min(f64::from(f32::MAX)) as f32;

        let Ok(relative) = position.relative_at_scale_bounded(
            &source.center(),
            source.field_scale(),
            bound_native,
        ) else {
            continue;
        };

        let distance_scale0 =
            f64::from(relative.length()) * source.field_scale().scale0_units_per_native();
        if distance_scale0 > source.radius_scale0() * GRAVITY_FIELD_RADIUS_MULTIPLIER {
            continue;
        }

        let clearance = (distance_scale0 - source.radius_scale0()).abs();
        if selected.is_none_or(|(current, _, _)| clearance < current) {
            selected = Some((clearance, relative, *source));
        }
    }

    let Some((_, relative, source)) = selected else {
        travel.local_gravity = 0.0;
        movement.gravity = 0.0;
        return;
    };

    let up = relative.normalize_or_zero();
    if up != Vec3::ZERO {
        locomotion.up = up;
    }

    let distance_scale0 =
        f64::from(relative.length()) * source.field_scale().scale0_units_per_native();
    let radius = source.radius_scale0();
    let gravity_factor = if distance_scale0 >= radius {
        // Outside a spherical source: ordinary inverse-square falloff.
        (radius / distance_scale0.max(f64::EPSILON)).powi(2)
    } else {
        // Inside a uniform spherical source, enclosed mass falls with r^3,
        // therefore gravitational acceleration falls linearly toward zero at
        // the center. This is also a sane penetration fallback: missing/tunneling
        // collision must never turn a planet or moon into an artificial black hole.
        (distance_scale0 / radius).clamp(0.0, 1.0)
    };
    let gravity = (f64::from(source.surface_gravity()) * gravity_factor)
        .clamp(0.0, f64::from(f32::MAX)) as f32;

    travel.local_gravity = gravity;
    // Preserve canonical acceleration while expressing the kernel in the
    // current Scale Slice's bounded native units.
    movement.gravity = layer.scale().metres_to_native_f32(gravity);
}

pub(in crate::game::player) fn sync_travel_state(
    frame: Res<UsfSpatialFrame>,
    player: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &UsfTravelNeighborhood,
            &mut PlayerTravelState,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (body, layer, neighborhood, mut state) = player.into_inner();
    let Ok(position) = frame
        .origin()
        .translated_at_scale(layer.scale(), body.translation)
    else {
        return;
    };

    let nearest = neighborhood
        .influences()
        .filter(|influence| matches!(influence.kind(), UsfTravelInfluenceKind::HardBody))
        .filter_map(|influence| influence.measure_from(&position))
        .min_by(|a, b| {
            a.boundary_clearance_scale0()
                .total_cmp(&b.boundary_clearance_scale0())
        });

    state.nearest_body_clearance_scale0 =
        nearest.map(|measurement| measurement.boundary_clearance_scale0());
    state.nearest_body_radius_scale0 =
        nearest.map(|measurement| measurement.extent_radius_scale0());
    state.planetary_handoff_clearance_scale0 =
        nearest.map(|measurement| planetary_handoff_clearance(measurement.extent_radius_scale0()));
    state.planetary_handoff_available = nearest.is_some_and(|measurement| {
        measurement.boundary_clearance_scale0()
            <= planetary_handoff_clearance(measurement.extent_radius_scale0())
    });
    state.planetary_context = nearest.is_some_and(|measurement| {
        measurement.relative_proximity() <= APPROACH_REFINEMENT_ACTIVATION_RADII
    });
    state.critical_dropout = nearest.is_some_and(|measurement| {
        measurement.boundary_clearance_scale0()
            <= critical_dropout_clearance(measurement.extent_radius_scale0())
    });

}
