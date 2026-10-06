//! Control relationships, transfer messages, and invariant reports.

use bevy::prelude::*;

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
