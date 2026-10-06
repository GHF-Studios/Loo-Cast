//! Subject-owned travel state and primary body selection.

use super::*;
use crate::game::navigation::{
    AdaptiveCruise, NavigationCapabilities, TravelAssistance, TravelAssistanceCommand,
    TravelAssistanceRequest, TravelAssistanceState, TravelAssistanceTransitionReason,
};

/// Resolve explicit assistance requests and automatic safety dropouts.
///
/// This is the ordinary runtime ownership point for `TravelAssistanceState`.
pub(in crate::game::navigation) fn resolve_travel_assistance(
    mut requests: MessageReader<TravelAssistanceRequest>,
    mut subjects: Query<(
        Entity,
        &NavigationCapabilities,
        &TravelState,
        &mut TravelAssistanceState,
        &mut AdaptiveCruise,
    )>,
) {
    for (_, capabilities, travel, mut assistance, mut cruise) in &mut subjects {
        if assistance.mode() == TravelAssistance::Cruise
            && (travel.critical_dropout || !capabilities.cruise())
        {
            assistance.disengage(TravelAssistanceTransitionReason::CriticalApproach);
            cruise.was_active = false;
            cruise.throttle = 0.0;
            cruise.speed_metres_per_second = 0.0;
        }
    }

    for request in requests.read() {
        let Ok((_, capabilities, travel, mut assistance, mut cruise)) =
            subjects.get_mut(request.entity())
        else {
            continue;
        };

        let requested = match request.command() {
            TravelAssistanceCommand::ToggleCruise => {
                if assistance.mode() == TravelAssistance::Cruise {
                    TravelAssistance::Manual
                } else {
                    TravelAssistance::Cruise
                }
            }
            TravelAssistanceCommand::Set(value) => value,
        };

        match requested {
            TravelAssistance::Manual => {
                if assistance.mode() != TravelAssistance::Manual {
                    assistance.disengage(TravelAssistanceTransitionReason::PilotDisengaged);
                }
                cruise.was_active = false;
                cruise.throttle = 0.0;
                cruise.speed_metres_per_second = 0.0;
            }
            TravelAssistance::Cruise => {
                if capabilities.cruise() && travel.cruise_entry_available {
                    assistance.engage_cruise();
                }
            }
        }
    }
}

/// Resolves one primary hard body and derives travel telemetry from that same
/// context. Physical gravity is queried independently from the gravity-field
/// subsystem; navigation owns only body-relative travel geometry.
pub(in crate::game::navigation) fn sync_travel_state(
    frame: Res<UsfRuntimeChartState>,
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
    let (body, layer, neighborhood, profile, mut state, mut primary) = subject.into_inner();

    let Ok(position) = frame
        .origin()
        .translated_at_scale(layer.scale(), body.translation)
    else {
        return;
    };

    let nearest = neighborhood
        .measurements_from(&position, layer.scale())
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
