use bevy::prelude::*;
use tokio::task::JoinHandle;

use crate::{player::types::PlayerWorkflow, workflow::composite_workflow_context::ScopedCompositeWorkflowContext};

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub struct PlayerWorkflowQueue(pub Vec<PlayerWorkflow>);

#[derive(Resource, Reflect, Default)]
#[reflect(Resource)]
pub enum PlayerLifecycle {
    #[default]
    None,
    Spawning(#[reflect(ignore)] Option<JoinHandle<ScopedCompositeWorkflowContext>>),
    Despawning(#[reflect(ignore)] Option<JoinHandle<ScopedCompositeWorkflowContext>>),
    PendingActivation(Entity),
    Active(Entity),
}
impl std::fmt::Debug for PlayerLifecycle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PlayerLifecycle::None => write!(f, "None"),
            PlayerLifecycle::Spawning(_) => write!(f, "Spawning"),
            PlayerLifecycle::Despawning(_) => write!(f, "Despawning"),
            PlayerLifecycle::PendingActivation(entity) => {
                write!(f, "PendingActivation({:?})", entity)
            }
            PlayerLifecycle::Active(entity) => write!(f, "Active({:?})", entity),
        }
    }
}
