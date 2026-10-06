//! Ordered local-control transfer transaction and adapter reconciliation.
//!
//! Transfer changes semantic control first. The view/interaction adapter then
//! follows the accepted manifestation and requests any needed chart handoff.

use super::model::*;
use crate::{
    ecs::UsfOwnershipQuery,
    spatial::{
        UsfInteractionProjection, UsfInteractionRequirement, UsfInteractionScaleAffinity,
        UsfPosition, UsfSpatialAnchor, UsfSpatialTransition, UsfSpatialTransitions,
        UsfTransitionVelocity, UsfViewAnchor,
    },
};
use bevy::prelude::*;

pub(super) fn resolve_local_control_transfers(
    mut commands: Commands,
    mut requests: MessageReader<LocalControlTransferRequest>,
    controllers: Query<(), With<LocalController>>,
    ownership: UsfOwnershipQuery,
    current: Query<Entity, With<LocalControlSubject>>,
    affinities: Query<&UsfInteractionScaleAffinity>,
    relationships: Query<&ControlledBy>,
    mut applied: MessageWriter<LocalControlTransferApplied>,
    mut rejected: MessageWriter<LocalControlTransferRejected>,
) {
    let Some(request) = requests.read().last().copied() else {
        return;
    };

    if controllers.get(request.controller()).is_err() {
        rejected.write(LocalControlTransferRejected {
            request,
            reason: LocalControlTransferRejection::ControllerIsNotLocal,
        });
        return;
    }

    let Some(target_subject) = ownership.semantic_of(request.manifestation()) else {
        rejected.write(LocalControlTransferRejected {
            request,
            reason: LocalControlTransferRejection::TargetIsNotManifestation,
        });
        return;
    };

    if affinities.get(request.manifestation()).is_err() {
        rejected.write(LocalControlTransferRejected {
            request,
            reason: LocalControlTransferRejection::TargetHasNoInteractionScaleAffinity,
        });
        return;
    }

    let previous = current.iter().next().and_then(|realization| {
        ownership
            .semantic_of(realization)
            .map(|subject| (realization, subject))
    });

    for realization in &current {
        if realization != request.manifestation() {
            commands.entity(realization).remove::<LocalControlSubject>();
        }

        if let Some(subject) = ownership.semantic_of(realization)
            && subject != request.controller()
            && relationships
                .get(subject)
                .is_ok_and(|relationship| relationship.0 == request.controller())
        {
            commands.entity(subject).remove::<ControlledBy>();
        }
    }

    if target_subject == request.controller() {
        commands.entity(target_subject).remove::<ControlledBy>();
    } else {
        commands
            .entity(target_subject)
            .insert(ControlledBy(request.controller()));
    }

    commands
        .entity(request.manifestation())
        .insert(LocalControlSubject);

    applied.write(LocalControlTransferApplied {
        controller: request.controller(),
        previous_subject: previous.map(|(_, subject)| subject),
        previous_manifestation: previous.map(|(realization, _)| realization),
        subject: target_subject,
        manifestation: request.manifestation(),
    });
}

/// Current game policy: the primary view/interaction focus follows local
/// control. This is an adapter, not part of semantic control authority.
pub(super) fn reconcile_local_control_adapters(
    mut commands: Commands,
    mut applied: MessageReader<LocalControlTransferApplied>,
    view_anchors: Query<Entity, With<UsfViewAnchor>>,
    spatial_anchors: Query<Entity, With<UsfSpatialAnchor>>,
    view_targets: Query<Entity, With<LocalViewTarget>>,
    affinities: Query<&UsfInteractionScaleAffinity>,
    semantic_positions: Query<&UsfPosition>,
    mut transitions: ResMut<UsfSpatialTransitions>,
) {
    let Some(transfer) = applied.read().last().copied() else {
        return;
    };

    for anchor in &view_anchors {
        if anchor != transfer.manifestation {
            commands.entity(anchor).remove::<UsfViewAnchor>();
        }
    }
    for target in &view_targets {
        if target != transfer.manifestation {
            commands.entity(target).remove::<LocalViewTarget>();
        }
    }
    for anchor in &spatial_anchors {
        if anchor != transfer.manifestation {
            commands.entity(anchor).remove::<UsfSpatialAnchor>();
        }
    }

    if let Some(previous) = transfer.previous_manifestation {
        if previous != transfer.manifestation {
            commands
                .entity(previous)
                .remove::<UsfInteractionProjection>();
        }
    }

    if let Some(previous_subject) = transfer.previous_subject {
        if previous_subject != transfer.subject {
            transitions.clear_interaction_requirement(previous_subject);
        }
    }

    commands.entity(transfer.manifestation).insert((
        LocalViewTarget,
        UsfViewAnchor,
        UsfSpatialAnchor,
        UsfInteractionProjection,
    ));

    let Ok(affinity) = affinities.get(transfer.manifestation) else {
        error!(
            manifestation = ?transfer.manifestation,
            "controlled manifestation lost its interaction Scale affinity during focus reconciliation"
        );
        return;
    };
    let Ok(&position) = semantic_positions.get(transfer.subject) else {
        error!(
            subject = ?transfer.subject,
            "controlled semantic subject has no canonical position during Scale handoff"
        );
        return;
    };

    // Control transfer is the semantic event that may select a different USF
    // interaction Scale. Queue a one-shot rechart even when the manifestation
    // already happens to carry that UsfScaleLayer: the global primary
    // interaction resource still belongs to the previous controlled subject.
    let mut transition = UsfSpatialTransition::new(
        transfer.subject,
        position,
        UsfTransitionVelocity::PreserveCanonical,
    )
    .with_scale(affinity.scale());

    if !affinity.required_roles().is_empty() {
        transition = transition
            .requiring_coverage(affinity.required_roles(), affinity.coverage_radius_native());
    }

    transitions.relocate(transition);
}

/// Continuously reassert the controlled manifestation's authored Scale affinity.
///
/// This is intentionally boring stable state. Movement, altitude, terrain
/// clearance, locomotion regime and view zoom are absent from this function.
pub(super) fn refresh_controlled_interaction_requirement(
    ownership: UsfOwnershipQuery,
    subject: Single<(Entity, &UsfInteractionScaleAffinity), With<LocalControlSubject>>,
    mut transitions: ResMut<UsfSpatialTransitions>,
) {
    let (manifestation, affinity) = subject.into_inner();
    let Some(semantic) = ownership.semantic_of(manifestation) else {
        error!(
            manifestation = ?manifestation,
            "controlled manifestation has no semantic owner for interaction Scale affinity"
        );
        return;
    };

    let mut requirement = UsfInteractionRequirement::new(
        semantic,
        affinity.scale(),
        UsfTransitionVelocity::PreserveCanonical,
    );
    if !affinity.required_roles().is_empty() {
        requirement = requirement
            .requiring_coverage(affinity.required_roles(), affinity.coverage_radius_native());
    }
    transitions.require_interaction(requirement);
}

pub(super) fn audit_local_control_invariants(
    controllers: Query<Entity, With<LocalController>>,
    subjects: Query<Entity, With<LocalControlSubject>>,
    ownership: UsfOwnershipQuery,
    relationships: Query<&ControlledBy>,
    view_targets: Query<Entity, With<LocalViewTarget>>,
    view_anchors: Query<Entity, With<UsfViewAnchor>>,
    spatial_anchors: Query<Entity, With<UsfSpatialAnchor>>,
    mut audit: ResMut<LocalControlAudit>,
    mut previous: Local<Option<LocalControlAudit>>,
) {
    let controller_count = controllers.iter().count();
    let subject_count = subjects.iter().count();
    let view_target_count = view_targets.iter().count();
    let view_anchor_count = view_anchors.iter().count();
    let spatial_anchor_count = spatial_anchors.iter().count();

    let controller = (controller_count == 1)
        .then(|| controllers.iter().next())
        .flatten();
    let subject = (subject_count == 1)
        .then(|| subjects.iter().next())
        .flatten()
        .and_then(|realization| {
            ownership
                .semantic_of(realization)
                .map(|semantic| (realization, semantic))
        });

    let semantic_authority_valid = match (controller, subject) {
        (Some(controller), Some((_, semantic))) if semantic == controller => {
            relationships.get(semantic).is_err()
        }
        (Some(controller), Some((_, semantic))) => relationships
            .get(semantic)
            .is_ok_and(|relationship| relationship.0 == controller),
        _ => false,
    };

    let focus_valid = match subject {
        Some((realization, _))
            if view_target_count == 1 && view_anchor_count == 1 && spatial_anchor_count == 1 =>
        {
            view_targets.iter().next() == Some(realization)
                && view_anchors.iter().next() == Some(realization)
                && spatial_anchors.iter().next() == Some(realization)
        }
        _ => false,
    };

    let next = LocalControlAudit {
        healthy: controller_count == 1
            && subject_count == 1
            && view_target_count == 1
            && view_anchor_count == 1
            && spatial_anchor_count == 1
            && semantic_authority_valid
            && focus_valid,
        controller_count,
        subject_count,
        view_target_count,
        view_anchor_count,
        semantic_authority_valid,
        focus_valid,
    };

    if previous.is_none_or(|previous| previous != next) {
        if next.healthy {
            debug!(?next, "local control invariants healthy");
        } else {
            error!(?next, "local control invariant violation");
        }
        *previous = Some(next);
    }

    *audit = next;
}
