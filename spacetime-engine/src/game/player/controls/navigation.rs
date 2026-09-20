//! Observer-local coarse navigation context.

use super::*;

/// Refreshes the sparse travel neighborhood and derives the characteristic
/// spatial length currently being navigated.
///
/// Keeping this outside Cruise means ordinary coarse movement and Cruise can
/// share the same semantic neighborhood without each owning a private scan.
pub(in crate::game::player) fn sync_navigation_context(
    time: Res<Time>,
    frames: Res<UsfScaleLayerFrames>,
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
    let absolute = frames.absolute(scale, body.translation);

    neighborhood.advance(time.delta_secs().max(0.0));
    if neighborhood.needs_refresh(absolute, scale) {
        neighborhood.refresh(
            absolute,
            scale,
            &frames,
            influences
                .iter()
                .map(|(entity, influence)| (entity, *influence)),
        );
    }

    let resolved = UsfNavigationContext::resolve(absolute, scale, &frames, &neighborhood);
    if *navigation != resolved {
        *navigation = resolved;
    }
}
