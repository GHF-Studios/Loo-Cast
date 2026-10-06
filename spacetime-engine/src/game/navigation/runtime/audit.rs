//! Read-only navigation and presentation contract diagnostics.

use super::*;

/// Publishes one compact end-to-end contract snapshot for diagnostics.
///
/// This observes already-resolved state; it owns no navigation or view policy.
pub(in crate::game::navigation) fn audit_navigation_contract(
    subject: Single<
        (
            Entity,
            &UsfScaleLayer,
            &UsfNavigationContext,
            &PrimaryBodyContext,
            &ApproachRefinementState,
        ),
        With<LocalControlSubject>,
    >,
    view: Single<(&UsfViewContext, &NavigationPresentationState), With<UsfViewRenderAnchor>>,
    mut audit: ResMut<NavigationAudit>,
    mut was_unhealthy: Local<bool>,
) {
    let (entity, layer, navigation, primary, approach) = subject.into_inner();
    let (view, presentation) = view.into_inner();

    let characteristic = navigation.characteristic_length_metres();
    let view_exponent = view.continuous_exponent();
    let target_exponent = presentation.effective_target_exponent();

    let structured = navigation.source_scale().is_some();
    let healthy = characteristic.is_finite()
        && characteristic > 0.0
        && view_exponent.is_finite()
        && target_exponent.is_finite()
        && (!structured || presentation.initialized());

    let next = NavigationAudit {
        healthy,
        subject: Some(entity),
        subject_scale: Some(layer.scale()),
        primary_body: primary.entity(),
        primary_clearance_metres: primary.is_resolved().then_some(primary.clearance_metres()),
        navigation_source_scale: navigation.source_scale(),
        characteristic_length_metres: characteristic,
        approach_active: approach.active,
        realization_target_scale: approach.active.then_some(approach.realization_target_scale),
        view_exponent,
        presentation_target_exponent: target_exponent,
    };

    if !healthy && !*was_unhealthy {
        error!(?next, "navigation/presentation contract became unhealthy");
    }
    *was_unhealthy = !healthy;
    *audit = next;
}
