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
        With<Player>,
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


const APPROACH_TARGET_CLEARANCE_NATIVE: f64 = 4.0;
const SURFACE_S0_CAPTURE_CLEARANCE_SCALE0: f64 = 32_000.0;
const APPROACH_REFINEMENT_ACTIVATION_RADII: f64 = 256.0;
const APPROACH_REFINEMENT_RATE_DECADES_PER_SECOND: f32 = 6.0;

/// Automatically refines/coarsens the primary observer while approaching an
/// explicitly refinable semantic body. Ordinary USF transition machinery still
/// performs the actual rechart when the dominant scale crosses a boundary.
pub(in crate::game::player) fn sync_approach_refinement_view(
    time: Res<Time>,
    frame: Res<UsfSpatialFrame>,
    player: Single<
        (
            &Transform,
            &UsfScaleLayer,
            &UsfManifestationOf,
            &ControlledSubjectLocomotion,
        ),
        With<Player>,
    >,
    refinable: Query<(&UsfTravelInfluence, &UsfApproachRefinement)>,
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
    mut transitions: ResMut<UsfSpatialTransitionQueue>,
) {
    let (body, layer, manifestation, locomotion) = player.into_inner();
    let observer_scale = layer.scale();
    let Ok(observer) = frame
        .origin()
        .translated_at_scale(observer_scale, body.translation)
    else {
        return;
    };

    let mut selected = None::<(UsfTravelInfluence, UsfApproachRefinement, f64)>;
    for (influence, refinement) in &refinable {
        let Some(measurement) = influence.measure_from(&observer) else { continue; };
        let relative = measurement.relative_proximity();
        if relative > APPROACH_REFINEMENT_ACTIVATION_RADII { continue; }
        if selected.is_none_or(|(_,_,current)| relative < current) {
            selected = Some((*influence, *refinement, relative));
        }
    }

    let Some((influence, refinement, _)) = selected else { return; };
    let Some(measurement) = influence.measure_from(&observer) else { return; };

    let clearance_scale0 = measurement.boundary_clearance_scale0().max(1.0);
    let target = if locomotion.regime() != PlayerLocomotionRegime::Cruise
        && clearance_scale0 <= SURFACE_S0_CAPTURE_CLEARANCE_SCALE0
    {
        refinement.minimum_scale().exponent() as f32
    } else {
        (clearance_scale0 / APPROACH_TARGET_CLEARANCE_NATIVE)
            .log10()
            .clamp(
                refinement.minimum_scale().exponent() as f64,
                influence.scale().exponent() as f64,
            ) as f32
    };

    let current = view.continuous_exponent();
    let max_step = APPROACH_REFINEMENT_RATE_DECADES_PER_SECOND * time.delta_secs().max(0.0);
    let next = if target < current { (current-max_step).max(target) } else { (current+max_step).min(target) };
    if (next - current).abs() > 1.0e-4 {
        view.set_continuous_exponent(next);
    }

    // Interaction-slice selection is explicit semantic state. Presentation may
    // currently follow the same approach policy, but view state no longer owns
    // or implicitly recharts physics.
    let interaction_raw = next.ceil() as i8;
    if let Some(interaction_scale) = SpatialScale::new(interaction_raw)
        && interaction_scale != layer.scale()
    {
        let transition = UsfSpatialTransition::new(manifestation.0, observer)
            .with_scale(interaction_scale)
            .with_velocity(match locomotion.velocity_semantics() {
                PlayerVelocitySemantics::PreserveNative => UsfTransitionVelocity::PreserveNative,
                PlayerVelocitySemantics::PreserveCanonical => {
                    UsfTransitionVelocity::PreserveCanonical
                }
                PlayerVelocitySemantics::Zero => UsfTransitionVelocity::Zero,
            });

        // Only the final fine interaction handoff requires demonstrated fine
        // realization. Coarser scale navigation is allowed without inventing
        // terrain/collision requirements that do not belong there.
        let transition = if interaction_scale == refinement.minimum_scale() {
            transition.requiring_coverage(
                UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION),
                8_192.0,
            )
        } else {
            transition
        };

        transitions.request(transition);
    }
}


const GRAVITY_FIELD_RADIUS_MULTIPLIER: f64 = 8.0;
const PLANETARY_HANDOFF_RADIUS_FRACTION: f64 = 0.12;
const PLANETARY_HANDOFF_MIN_SCALE0: f64 = 20_000.0;
const PLANETARY_HANDOFF_MAX_SCALE0: f64 = 750_000.0;
const CRITICAL_DROPOUT_FRACTION: f64 = 0.25;

fn handoff_clearance(radius_scale0: f64) -> f64 {
    (radius_scale0 * PLANETARY_HANDOFF_RADIUS_FRACTION)
        .clamp(PLANETARY_HANDOFF_MIN_SCALE0, PLANETARY_HANDOFF_MAX_SCALE0)
}

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
        With<Player>,
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
        With<Player>,
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
        nearest.map(|measurement| handoff_clearance(measurement.extent_radius_scale0()));
    state.planetary_handoff_available = nearest.is_some_and(|measurement| {
        measurement.boundary_clearance_scale0()
            <= handoff_clearance(measurement.extent_radius_scale0())
    });
    state.planetary_context = nearest.is_some_and(|measurement| {
        measurement.relative_proximity() <= APPROACH_REFINEMENT_ACTIVATION_RADII
    });
    state.critical_dropout = nearest.is_some_and(|measurement| {
        measurement.boundary_clearance_scale0()
            <= handoff_clearance(measurement.extent_radius_scale0())
                * CRITICAL_DROPOUT_FRACTION
    });

}
