//! Semantic and local runtime control authority.
//!
//! Control authority is orthogonal to constituency, manifestation, navigation
//! and locomotion. This module owns the invariant that local control has exactly
//! one semantic controller and one runtime target; focus is reconciled by an
//! explicit local-control adapter rather than by vehicle-specific code.

use bevy::prelude::*;

use crate::{
    ecs::UsfOwnershipQuery,
    spatial::{
        UsfInteractionProjection, UsfInteractionRequirement, UsfInteractionScaleAffinity,
        UsfPosition, UsfSpatialAnchor, UsfSpatialTransition, UsfSpatialTransitionQueue,
        UsfTransitionVelocity, UsfViewAnchor,
    },
};

use super::GameSet;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalController;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalControlSubject;

/// Runtime manifestation followed by the primary local view.
///
/// Current game policy follows local control, but this marker is deliberately
/// independent so spectator cameras, remote observation and cinematic views do
/// not need to counterfeit control authority later.
#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalViewTarget;

/// Semantic control relationship. The relationship lives on the controlled
/// semantic subject and points at the semantic controller.
#[derive(Component, Debug)]
#[relationship(relationship_target = ControlledSubjects)]
pub struct ControlledBy(pub Entity);

#[derive(Component, Debug)]
#[relationship_target(relationship = ControlledBy)]
pub struct ControlledSubjects(Vec<Entity>);

impl ControlledSubjects {
    pub fn iter(&self) -> impl ExactSizeIterator<Item = Entity> + '_ {
        self.0.iter().copied()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// Stable controller-adapter extension points inside `RunFixedMainLoop`.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlSet {
    Sample,
    Request,
    CharacterIntent,
}

/// Ordered Update-time control-authority transaction.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControlActionSet {
    Request,
    Transfer,
    Reconcile,
    Validate,
}

/// Request transfer of one semantic controller to one runtime manifestation.
///
/// The target semantic subject is resolved through the generic USF ownership graph. Callers
/// never mutate [`LocalControlSubject`] or [`ControlledBy`] themselves.
#[derive(Message, Debug, Clone, Copy)]
pub struct LocalControlTransferRequest {
    controller: Entity,
    manifestation: Entity,
}

impl LocalControlTransferRequest {
    pub const fn new(controller: Entity, manifestation: Entity) -> Self {
        Self {
            controller,
            manifestation,
        }
    }

    pub const fn controller(self) -> Entity {
        self.controller
    }

    pub const fn manifestation(self) -> Entity {
        self.manifestation
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct LocalControlTransferApplied {
    pub controller: Entity,
    pub previous_subject: Option<Entity>,
    pub previous_manifestation: Option<Entity>,
    pub subject: Entity,
    pub manifestation: Entity,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LocalControlTransferRejection {
    ControllerIsNotLocal,
    TargetIsNotManifestation,
    TargetHasNoInteractionScaleAffinity,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct LocalControlTransferRejected {
    pub request: LocalControlTransferRequest,
    pub reason: LocalControlTransferRejection,
}

/// Live invariant report for diagnostics/HUD/telemetry.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq)]
pub struct LocalControlAudit {
    pub healthy: bool,
    pub controller_count: usize,
    pub subject_count: usize,
    pub view_target_count: usize,
    pub view_anchor_count: usize,
    pub semantic_authority_valid: bool,
    pub focus_valid: bool,
}

impl Default for LocalControlAudit {
    fn default() -> Self {
        Self {
            healthy: false,
            controller_count: 0,
            subject_count: 0,
            view_target_count: 0,
            view_anchor_count: 0,
            semantic_authority_valid: false,
            focus_valid: false,
        }
    }
}

fn apply_local_control_transfers(
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

    if controllers.get(request.controller).is_err() {
        rejected.write(LocalControlTransferRejected {
            request,
            reason: LocalControlTransferRejection::ControllerIsNotLocal,
        });
        return;
    }

    let Some(target_subject) = ownership.semantic_of(request.manifestation) else {
        rejected.write(LocalControlTransferRejected {
            request,
            reason: LocalControlTransferRejection::TargetIsNotManifestation,
        });
        return;
    };

    if affinities.get(request.manifestation).is_err() {
        rejected.write(LocalControlTransferRejected {
            request,
            reason: LocalControlTransferRejection::TargetHasNoInteractionScaleAffinity,
        });
        return;
    }

    let previous = current
        .iter()
        .next()
        .and_then(|realization| {
            ownership
                .semantic_of(realization)
                .map(|subject| (realization, subject))
        });

    for realization in &current {
        if realization != request.manifestation {
            commands.entity(realization).remove::<LocalControlSubject>();
        }

        if let Some(subject) = ownership.semantic_of(realization)
            && subject != request.controller
            && relationships
                .get(subject)
                .is_ok_and(|relationship| relationship.0 == request.controller)
        {
            commands.entity(subject).remove::<ControlledBy>();
        }
    }

    if target_subject == request.controller {
        commands.entity(target_subject).remove::<ControlledBy>();
    } else {
        commands
            .entity(target_subject)
            .insert(ControlledBy(request.controller));
    }

    commands.entity(request.manifestation).insert(LocalControlSubject);

    applied.write(LocalControlTransferApplied {
        controller: request.controller,
        previous_subject: previous.map(|(_, subject)| subject),
        previous_manifestation: previous.map(|(realization, _)| realization),
        subject: target_subject,
        manifestation: request.manifestation,
    });
}

/// Current game policy: the primary view/interaction focus follows local
/// control. This is an adapter, not part of semantic control authority.
// control-transfer-owns-interaction-scale-v1
fn reconcile_local_control_focus(
    mut commands: Commands,
    mut applied: MessageReader<LocalControlTransferApplied>,
    view_anchors: Query<Entity, With<UsfViewAnchor>>,
    spatial_anchors: Query<Entity, With<UsfSpatialAnchor>>,
    view_targets: Query<Entity, With<LocalViewTarget>>,
    affinities: Query<&UsfInteractionScaleAffinity>,
    semantic_positions: Query<&UsfPosition>,
    mut transitions: ResMut<UsfSpatialTransitionQueue>,
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
        transition = transition.requiring_coverage(
            affinity.required_roles(),
            affinity.coverage_radius_native(),
        );
    }

    transitions.request(transition);
}

/// Continuously reassert the controlled manifestation's authored Scale affinity.
///
/// This is intentionally boring stable state. Movement, altitude, terrain
/// clearance, locomotion regime and view zoom are absent from this function.
fn sync_controlled_interaction_scale_affinity(
    ownership: UsfOwnershipQuery,
    subject: Single<
        (Entity, &UsfInteractionScaleAffinity),
        With<LocalControlSubject>,
    >,
    mut transitions: ResMut<UsfSpatialTransitionQueue>,
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
        requirement = requirement.requiring_coverage(
            affinity.required_roles(),
            affinity.coverage_radius_native(),
        );
    }
    transitions.set_interaction_requirement(requirement);
}

fn audit_local_control_invariants(
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
            if view_target_count == 1
                && view_anchor_count == 1
                && spatial_anchor_count == 1 =>
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

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalControlAudit>()
            .register_type::<LocalController>()
            .register_type::<LocalControlSubject>()
            .register_type::<LocalViewTarget>()
            .add_message::<LocalControlTransferRequest>()
            .add_message::<LocalControlTransferApplied>()
            .add_message::<LocalControlTransferRejected>()
            .configure_sets(
                Update,
                (
                    ControlActionSet::Request,
                    ControlActionSet::Transfer,
                    ControlActionSet::Reconcile,
                    ControlActionSet::Validate,
                )
                    .chain()
                    .in_set(GameSet::Action),
            )
            .add_systems(
                Update,
                apply_local_control_transfers.in_set(ControlActionSet::Transfer),
            )
            .add_systems(
                Update,
                (
                    reconcile_local_control_focus,
                    sync_controlled_interaction_scale_affinity
                        .after(reconcile_local_control_focus),
                )
                    .in_set(ControlActionSet::Reconcile),
            )
            .add_systems(
                Update,
                audit_local_control_invariants.in_set(ControlActionSet::Validate),
            );
    }
}
