//! View-owned presentation policy downstream of navigation context.

use super::*;

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
pub(in crate::game::navigation) fn sync_navigation_presentation(
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
        navigation.characteristic_length_metres(),
        profile,
    );
    let effective = state.resolve_target(automatic, profile);

    if !state.initialized() {
        view.set_continuous_exponent(effective);
        state.mark_initialized();
        info!(
            source_scale = %source_scale,
            characteristic_metres = navigation.characteristic_length_metres(),
            target_exponent = effective,
            "resolved initial USF presentation scale from semantic navigation context"
        );
        return;
    }

    let current = view.continuous_exponent();
    let max_step = profile.response_decades_per_second.max(0.0) * time.delta_secs().max(0.0);
    let next = if effective < current {
        (current - max_step).max(effective)
    } else {
        (current + max_step).min(effective)
    };

    if (next - current).abs() > 1.0e-4 {
        view.set_continuous_exponent(next);
    }
}
