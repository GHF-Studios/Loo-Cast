//! Subject-owned travel state and primary body selection.

use super::*;

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
    state.planetary_context = measurement.relative_proximity() <= profile.approach.activation_radii;
    state.critical_dropout = clearance <= handoff;
    state.cruise_entry_available =
        clearance > handoff * profile.cruise.reentry_clearance_multiplier;

    *primary = PrimaryBodyContext::resolved(
        entity,
        anchor,
        radius,
        influence.scale(),
        measurement.center_distance_scale0(),
        clearance,
    );
}
