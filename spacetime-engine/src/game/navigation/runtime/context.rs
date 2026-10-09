//! Sparse navigation neighborhood and characteristic scale observation.

use super::*;

/// Refreshes the sparse travel neighborhood and derives the characteristic
/// spatial length currently being navigated.
pub(in crate::game::navigation) fn sync_navigation_context(
    time: Res<Time>,
    ownership: UsfOwnershipQuery,
    semantic_positions: Query<&UsfPosition>,
    influences: Query<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &UsfTravelInfluence,
        Option<&UsfTravelBoundaryProvider>,
    )>,
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &UsfCanonicalMotion,
            &mut UsfTravelNeighborhood,
            &mut UsfNavigationContext,
        ),
        With<LocalControlSubject>,
    >,
) {
    let (entity, layer, motion, mut neighborhood, mut navigation) = subject.into_inner();
    let scale = layer.scale();
    let Some(semantic) = ownership.semantic_of(entity) else {
        return;
    };
    let Ok(position) = semantic_positions.get(semantic) else {
        return;
    };

    neighborhood.refresh_if_needed(
        time.delta_secs().max(0.0),
        *position,
        scale,
        motion.velocity_metres_per_second(),
        || {
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
        },
    );

    let resolved = UsfNavigationContext::resolve(position, scale, &neighborhood);
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
