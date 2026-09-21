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


const APPROACH_TARGET_CLEARANCE_NATIVE: f64 = 4.0;
const APPROACH_REFINEMENT_ACTIVATION_RADII: f64 = 256.0;
const APPROACH_REFINEMENT_RATE_DECADES_PER_SECOND: f32 = 6.0;

/// Automatically refines/coarsens the primary observer while approaching an
/// explicitly refinable semantic body. Ordinary USF transition machinery still
/// performs the actual rechart when the dominant scale crosses a boundary.
pub(in crate::game::player) fn sync_approach_refinement_view(
    time: Res<Time>,
    frames: Res<UsfScaleLayerFrames>,
    player: Single<(&Transform, &UsfScaleLayer), With<Player>>,
    refinable: Query<(&UsfTravelInfluence, &UsfApproachRefinement)>,
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
) {
    let (body, layer) = player.into_inner();
    let observer_scale = layer.scale();
    let observer_absolute = frames.absolute(observer_scale, body.translation);

    let mut selected = None::<(UsfTravelInfluence, UsfApproachRefinement, f64)>;
    for (influence, refinement) in &refinable {
        let Some(measurement) = influence.measure_from(observer_absolute, observer_scale, &frames) else { continue; };
        let relative = measurement.relative_proximity();
        if relative > APPROACH_REFINEMENT_ACTIVATION_RADII { continue; }
        if selected.is_none_or(|(_,_,current)| relative < current) {
            selected = Some((*influence, *refinement, relative));
        }
    }

    let Some((influence, refinement, _)) = selected else { return; };
    let Some(measurement) = influence.measure_from(observer_absolute, observer_scale, &frames) else { return; };

    let clearance_scale0 = measurement.boundary_clearance_scale0().max(1.0);
    let target = (clearance_scale0 / APPROACH_TARGET_CLEARANCE_NATIVE)
        .log10()
        .clamp(
            refinement.minimum_scale().exponent() as f64,
            influence.scale().exponent() as f64,
        ) as f32;

    let current = view.continuous_exponent();
    let max_step = APPROACH_REFINEMENT_RATE_DECADES_PER_SECOND * time.delta_secs().max(0.0);
    let next = if target < current { (current-max_step).max(target) } else { (current+max_step).min(target) };
    if (next-current).abs() > 1.0e-4 { view.set_continuous_exponent(next); }
}
