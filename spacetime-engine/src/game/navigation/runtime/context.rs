//! Sparse navigation neighborhood and characteristic scale observation.

use super::*;

/// Refreshes the sparse travel neighborhood and derives the characteristic
/// spatial length currently being navigated.
pub(in crate::game::navigation) fn sync_navigation_context(
    time: Res<Time>,
    frame: Res<UsfRuntimeChartState>,
    influences: Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &UsfTravelInfluence,
        Option<&UsfTravelBoundaryProvider>,
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
    let Ok(position) = frame.origin().translated_at_scale(scale, body.translation) else {
        return;
    };

    neighborhood.refresh_if_needed(time.delta_secs().max(0.0), position, scale, || {
        influences
            .iter()
            .map(|(entity, anchor, semantic_frame, influence, boundary)| {
                (
                    entity,
                    *anchor,
                    *semantic_frame,
                    *influence,
                    boundary.cloned(),
                )
            })
    });

    let resolved = UsfNavigationContext::resolve(&position, scale, &neighborhood);
    if *navigation != resolved {
        debug!(
            subject_scale = %scale,
            kind = ?resolved.kind(),
            source_scale = ?resolved.source_scale(),
            characteristic_metres = resolved.characteristic_length_metres(),
            neighborhood_entries = neighborhood.len(),
            "controlled-subject navigation context changed"
        );
        *navigation = resolved;
    }
}
