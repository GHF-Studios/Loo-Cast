//! Semantic and local runtime control authority.
//!
//! Control authority is orthogonal to constituency, manifestation, navigation
//! and locomotion. This module owns the invariant that local control has exactly
//! one semantic controller and one runtime target; focus is reconciled by an
//! explicit local-control adapter rather than by vehicle-specific code.

use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    spatial::{
        UsfInteractionProjection, UsfSpatialTransitionQueue, UsfViewAnchor,
    },
};

use super::GameSet;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalController;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalControlSubject;

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
/// The target semantic subject is resolved from [`UsfManifestationOf`]. Callers
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
    manifestations: Query<&UsfManifestationOf>,
    current: Query<(Entity, &UsfManifestationOf), With<LocalControlSubject>>,
    relationships: Query<&ControlledBy>,
    mut applied: MessageWriter<LocalControlTransferApplied>,
    mut rejected: MessageWriter<LocalControlTransferRejected>,
) {
    // Local control is singular. If multiple requests arrive in one frame, the
    // most recent intent supersedes earlier intents transactionally.
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

    let Ok(target_manifestation) = manifestations.get(request.manifestation) else {
        rejected.write(LocalControlTransferRejected {
            request,
            reason: LocalControlTransferRejection::TargetIsNotManifestation,
        });
        return;
    };
    let target_subject = target_manifestation.0;

    let previous = current
        .iter()
        .next()
        .map(|(manifestation, subject)| (manifestation, subject.0));

    // Repair duplicate local-subject markers if corruption ever occurs.
    for (manifestation, subject) in &current {
        if manifestation != request.manifestation {
            commands
                .entity(manifestation)
                .remove::<LocalControlSubject>();
        }

        if subject.0 != request.controller
            && relationships
                .get(subject.0)
                .is_ok_and(|relationship| relationship.0 == request.controller)
        {
            commands.entity(subject.0).remove::<ControlledBy>();
        }
    }

    if target_subject == request.controller {
        commands.entity(target_subject).remove::<ControlledBy>();
    } else {
        commands
            .entity(target_subject)
            .insert(ControlledBy(request.controller));
    }

    commands
        .entity(request.manifestation)
        .insert(LocalControlSubject);

    applied.write(LocalControlTransferApplied {
        controller: request.controller,
        previous_subject: previous.map(|(_, subject)| subject),
        previous_manifestation: previous.map(|(manifestation, _)| manifestation),
        subject: target_subject,
        manifestation: request.manifestation,
    });
}

/// Current game policy: the primary view/interaction focus follows local
/// control. This is an adapter, not part of semantic control authority.
fn reconcile_local_control_focus(
    mut commands: Commands,
    mut applied: MessageReader<LocalControlTransferApplied>,
    view_anchors: Query<Entity, With<UsfViewAnchor>>,
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
        UsfViewAnchor,
        UsfInteractionProjection,
    ));
}

fn audit_local_control_invariants(
    controllers: Query<Entity, With<LocalController>>,
    subjects: Query<(Entity, &UsfManifestationOf), With<LocalControlSubject>>,
    relationships: Query<&ControlledBy>,
    view_anchors: Query<Entity, With<UsfViewAnchor>>,
    mut audit: ResMut<LocalControlAudit>,
    mut previous: Local<Option<LocalControlAudit>>,
) {
    let controller_count = controllers.iter().count();
    let subject_count = subjects.iter().count();
    let view_anchor_count = view_anchors.iter().count();

    let controller = (controller_count == 1)
        .then(|| controllers.iter().next())
        .flatten();
    let subject = (subject_count == 1)
        .then(|| subjects.iter().next())
        .flatten();

    let semantic_authority_valid = match (controller, subject) {
        (Some(controller), Some((_, manifestation))) if manifestation.0 == controller => {
            relationships.get(manifestation.0).is_err()
        }
        (Some(controller), Some((_, manifestation))) => relationships
            .get(manifestation.0)
            .is_ok_and(|relationship| relationship.0 == controller),
        _ => false,
    };

    let focus_valid = match subject {
        Some((manifestation, _)) if view_anchor_count == 1 => {
            view_anchors.iter().next() == Some(manifestation)
        }
        _ => false,
    };

    let next = LocalControlAudit {
        healthy: controller_count == 1
            && subject_count == 1
            && view_anchor_count == 1
            && semantic_authority_valid
            && focus_valid,
        controller_count,
        subject_count,
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
                reconcile_local_control_focus.in_set(ControlActionSet::Reconcile),
            )
            .add_systems(
                Update,
                audit_local_control_invariants.in_set(ControlActionSet::Validate),
            );
    }
}
