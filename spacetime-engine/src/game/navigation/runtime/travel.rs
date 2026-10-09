//! Subject-owned travel state and primary body selection.

use super::*;
use crate::game::flight::{FlightSafetyLevel, FlightSafetyState};
use crate::game::navigation::{
    AdaptiveCruise, NavigationCapabilities, TravelAssistance, TravelAssistanceCommand,
    TravelAssistanceRequest, TravelAssistanceState, TravelAssistanceTransitionReason,
};

/// Resolve explicit assistance requests and automatic safety dropouts.
///
/// This is the ordinary runtime ownership point for `TravelAssistanceState`.
pub(in crate::game::navigation) fn resolve_travel_assistance(
    time: Res<Time>,
    mut requests: MessageReader<TravelAssistanceRequest>,
    mut subjects: Query<(
        Entity,
        &NavigationCapabilities,
        &TravelState,
        &TravelProfile,
        Option<&FlightSafetyState>,
        &mut TravelAssistanceState,
        &mut AdaptiveCruise,
    )>,
) {
    for (entity, capabilities, travel, profile, safety, mut assistance, mut cruise) in &mut subjects
    {
        assistance.tick_cooldown(time.delta_secs());
        if assistance.mode() == TravelAssistance::Cruise
            && (travel.critical_dropout
                || safety.is_some_and(|state| state.level() == FlightSafetyLevel::Emergency)
                || !capabilities.cruise())
        {
            warn!(
                subject = ?entity,
                time_to_contact_seconds = ?safety.and_then(|state| state.time_to_contact_seconds()),
                collision_ready = safety.is_some_and(|state| state.interaction_ready()),
                "Lattice Cruise emergency dropout"
            );
            assistance.emergency_dropout(profile.cruise.emergency_cooldown_seconds);
            cruise.was_active = false;
            cruise.throttle = 0.0;
            cruise.speed_metres_per_second = 0.0;
        }
        if assistance.is_spooling() {
            if !capabilities.cruise()
                || !travel.cruise_entry_available
                || safety.is_some_and(|state| state.level() == FlightSafetyLevel::Emergency)
            {
                assistance.cancel_spool();
            } else if assistance.tick_spool(time.delta_secs()) {
                assistance.engage_cruise();
            }
        }
    }

    for request in requests.read() {
        let Ok((_, capabilities, travel, profile, safety, mut assistance, mut cruise)) =
            subjects.get_mut(request.entity())
        else {
            continue;
        };

        let requested = match request.command() {
            TravelAssistanceCommand::ToggleCruise => {
                if assistance.mode() == TravelAssistance::Cruise || assistance.is_spooling() {
                    TravelAssistance::Manual
                } else {
                    TravelAssistance::Cruise
                }
            }
            TravelAssistanceCommand::Set(value) => value,
        };

        match requested {
            TravelAssistance::Manual => {
                assistance.cancel_spool();
                if assistance.mode() != TravelAssistance::Manual {
                    assistance.disengage(TravelAssistanceTransitionReason::PilotDisengaged);
                }
                cruise.was_active = false;
                cruise.throttle = 0.0;
                cruise.speed_metres_per_second = 0.0;
            }
            TravelAssistance::Cruise => {
                if capabilities.cruise()
                    && travel.cruise_entry_available
                    && !safety.is_some_and(|state| state.level() == FlightSafetyLevel::Emergency)
                    && assistance.drive_ready()
                {
                    assistance.begin_spool(profile.cruise.charge_seconds);
                }
            }
        }
    }
}

/// Resolves one primary hard body and derives travel telemetry from that same
/// context. Physical gravity is queried independently from the gravity-field
/// subsystem; navigation owns only body-relative travel geometry.
pub(in crate::game::navigation) fn sync_travel_state(
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &UsfTravelNeighborhood,
            &TravelProfile,
            &mut TravelState,
            &mut PrimaryBodyContext,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (entity, layer, neighborhood, profile, mut state, mut primary) = subject.into_inner();
    let Some(semantic) = ownership.semantic_of(entity) else {
        return;
    };
    let Ok(position) = semantic_positions.get(semantic) else {
        return;
    };

    let nearest = neighborhood
        .measurements_from(position, layer.scale())
        .filter(|(_, _, influence, _)| matches!(influence.kind(), UsfTravelInfluenceKind::HardBody))
        .min_by(|(_, _, _, a), (_, _, _, b)| {
            a.boundary_clearance_metres()
                .total_cmp(&b.boundary_clearance_metres())
        });

    let Some((entity, anchor, influence, measurement)) = nearest else {
        *state = TravelState::default();
        *primary = PrimaryBodyContext::default();
        return;
    };

    let radius = measurement.extent_radius_metres();
    let clearance = measurement.boundary_clearance_metres();
    let handoff = profile.planetary_handoff_clearance(radius);

    state.nearest_body_clearance_metres = Some(clearance);
    state.nearest_body_radius_metres = Some(radius);
    state.planetary_handoff_clearance_metres = Some(handoff);
    state.planetary_handoff_available = clearance <= handoff;
    state.critical_dropout = clearance <= handoff;
    state.cruise_entry_available =
        clearance > handoff * profile.cruise.reentry_clearance_multiplier;

    *primary = PrimaryBodyContext::resolved(
        entity,
        anchor,
        radius,
        influence.scale(),
        measurement.center_distance_metres(),
        clearance,
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ecs::{UsfAuthorityPartitionOf, UsfEntity, UsfLogicalRealizationOf},
        spatial::UsfSemanticFrame,
    };
    use bevy::math::DVec3;

    #[test]
    fn travel_observes_semantic_position_even_when_runtime_chart_is_stale() {
        let mut app = App::new();
        app.add_systems(Update, sync_travel_state);
        let zero = UsfPosition::zero(SpatialScale::new(0).unwrap());
        let near_surface = zero.translated_metres_f64(DVec3::X * 6_401_000.0).unwrap();
        let body = app.world_mut().spawn_empty().id();
        let semantic = app.world_mut().spawn((UsfEntity, near_surface)).id();
        let partition = app
            .world_mut()
            .spawn(UsfAuthorityPartitionOf(semantic))
            .id();
        let mut neighborhood = UsfTravelNeighborhood::default();
        neighborhood.refresh_if_needed(
            0.0,
            near_surface,
            SpatialScale::new(6).unwrap(),
            DVec3::ZERO,
            || {
                [(
                    body,
                    zero,
                    UsfSemanticFrame::identity(),
                    UsfTravelInfluence::hard_body(SpatialScale::new(6).unwrap(), 6.4),
                    None,
                )]
            },
        );
        let realization = app
            .world_mut()
            .spawn((
                LocalControlSubject,
                UsfLogicalRealizationOf(partition),
                Transform::from_xyz(1_000.0, 0.0, 0.0),
                UsfScaleLayer::new(SpatialScale::new(6).unwrap()),
                neighborhood,
                TravelProfile::spacecraft(),
                TravelState::default(),
                PrimaryBodyContext::default(),
            ))
            .id();
        app.update();
        let state = app.world().get::<TravelState>(realization).unwrap();
        let clearance = state.nearest_body_clearance_metres.unwrap();
        assert!((clearance - 1_000.0).abs() < 1.0);
        assert!(state.critical_dropout);
    }
}
