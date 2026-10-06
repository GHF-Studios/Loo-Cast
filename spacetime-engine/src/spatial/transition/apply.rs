//! Apply admitted canonical transitions to runtime chart participants.

use super::*;

/// Reinterpret an observer-local follower in the new interaction chart.
/// Position offsets stay numerically fixed; velocity follows the selected
/// physical policy. Neither runtime vector becomes canonical position truth.
fn rechart_participant(
    transform: &mut Transform,
    layer: &mut UsfScaleLayer,
    position: Option<Mut<'_, Position>>,
    motion: Option<Mut<'_, UsfCanonicalMotion>>,
    velocity: Option<Mut<'_, LinearVelocity>>,
    old_anchor_runtime: Vec3,
    target_scale: SpatialScale,
    velocity_policy: UsfTransitionVelocity,
    transition_factor: f32,
    belongs_to_subject: bool,
) {
    let local_offset = transform.translation - old_anchor_runtime;
    transform.translation = local_offset;
    layer.set_scale(target_scale);
    if let Some(mut position) = position {
        position.0 = local_offset;
    }
    if !belongs_to_subject {
        return;
    }

    match (motion, velocity) {
        (Some(mut motion), Some(mut velocity)) => match velocity_policy {
            UsfTransitionVelocity::Zero => {
                motion.stop();
                velocity.0 = Vec3::ZERO;
            }
            UsfTransitionVelocity::PreserveCanonical => {
                velocity.0 = motion.native_velocity(target_scale);
            }
            UsfTransitionVelocity::PreserveNative => {
                motion.set_from_native_velocity(target_scale, velocity.0);
            }
        },
        (Some(mut motion), None) if velocity_policy == UsfTransitionVelocity::Zero => motion.stop(),
        (None, Some(mut velocity)) => match velocity_policy {
            UsfTransitionVelocity::Zero => velocity.0 = Vec3::ZERO,
            UsfTransitionVelocity::PreserveCanonical => velocity.0 *= transition_factor,
            UsfTransitionVelocity::PreserveNative => {}
        },
        _ => {}
    }
}

pub(in crate::spatial) fn apply_spatial_transitions(
    mut view: Single<&mut UsfViewContext, With<UsfViewRenderAnchor>>,
    mut active: ResMut<UsfPrimaryInteractionSlice>,
    mut frame: ResMut<UsfRuntimeChartState>,
    mut transitions: ResMut<UsfSpatialTransitions>,
    handoff_guards: Res<UsfInteractionHandoffGuards>,
    coverage: Res<UsfScaleCoverageSnapshot>,
    ownership: UsfOwnershipQuery,
    mut participants: ParamSet<(
        Query<
            (Entity, &Transform, &UsfScaleLayer, &UsfLogicalRealizationOf),
            (
                With<UsfSpatialAnchor>,
                With<UsfLogicalRealizationOf>,
                With<UsfInteractionProjection>,
                Without<ChildOf>,
            ),
        >,
        Query<
            (
                Entity,
                &mut Transform,
                &mut UsfScaleLayer,
                Option<&mut Position>,
                Option<&mut LinearVelocity>,
                Option<&mut UsfCanonicalMotion>,
                Option<&UsfLogicalRealizationOf>,
            ),
            (With<UsfInteractionProjection>, Without<ChildOf>),
        >,
    )>,
    mut semantic_positions: Query<&mut UsfPosition>,
    mut applied: MessageWriter<UsfSpatialTransitionApplied>,
    mut wait_fingerprint: Local<Option<InteractionHandoffWaitFingerprint>>,
) {
    let (anchor_entity, old_anchor_runtime, previous_scale, subject) = {
        let anchors = participants.p0();
        let Some((entity, transform, layer, realization)) = anchors.iter().next() else {
            return;
        };
        let Some(subject) = ownership.semantic_for(realization) else {
            error!(
                realization = ?entity,
                partition = ?realization.0,
                "USF spatial anchor has no semantic owner"
            );
            return;
        };
        (entity, transform.translation, layer.scale(), subject)
    };

    let Ok(current_position) = frame
        .origin()
        .translated_at_scale(previous_scale, old_anchor_runtime)
    else {
        return;
    };

    let Some(transition) =
        ResolvedTransition::take(&mut transitions, subject, current_position, previous_scale)
    else {
        return;
    };
    let Some(ResolvedTransition {
        position,
        target_scale,
        view_exponent,
        velocity_policy,
        cause,
        ..
    }) = transition.admit(
        subject,
        previous_scale,
        &mut active,
        &mut transitions,
        &coverage,
        &handoff_guards,
        &mut wait_fingerprint,
    )
    else {
        return;
    };

    let transition_factor =
        10.0_f32.powi(previous_scale.exponent() as i32 - target_scale.exponent() as i32);
    if !transition_factor.is_finite() {
        error!(
            previous_scale = %previous_scale,
            target_scale = %target_scale,
            "USF runtime chart transition factor is non-finite"
        );
        return;
    }

    let Ok(mut semantic) = semantic_positions.get_mut(subject) else {
        return;
    };

    *semantic = position;

    // Changing the runtime chart must never change semantic precision.
    // The exact canonical subject position becomes the frame origin.
    let chart_origin = *semantic;

    for (_entity, mut transform, mut layer, position, velocity, motion, realization) in
        &mut participants.p1()
    {
        if realization
            .is_none_or(|realization| ownership.semantic_for(realization) != Some(subject))
        {
            continue;
        }

        let belongs_to_subject = realization
            .is_some_and(|realization| ownership.semantic_for(realization) == Some(subject));
        rechart_participant(
            &mut transform,
            &mut layer,
            position,
            motion,
            velocity,
            old_anchor_runtime,
            target_scale,
            velocity_policy,
            transition_factor,
            belongs_to_subject,
        );
    }

    frame.reanchor(chart_origin);

    // Presentation attached to a one-shot transition is part of the accepted
    // transaction. A coverage-gated relocation must not visually jump into a
    // representation that does not exist yet.
    if let Some(exponent) = view_exponent {
        view.set_continuous_exponent(exponent);
    }

    active.complete_handoff(target_scale);

    applied.write(UsfSpatialTransitionApplied {
        subject,
        anchor: anchor_entity,
        previous_scale,
        active_scale: target_scale,
        cause,
    });

    debug!(
        subject = ?subject,
        previous_scale = %previous_scale,
        active_scale = %target_scale,
        cause = ?cause,
        "applied canonical USF spatial transition"
    );
}
