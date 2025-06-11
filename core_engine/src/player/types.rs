use bevy::prelude::*;
use tokio::task::JoinHandle;

use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;

#[derive(Resource, Default)]
pub enum PlayerLifecycle {
    #[default]
    None,
    Spawning(Option<JoinHandle<ScopedCompositeWorkflowContext>>),
    Despawning(Option<JoinHandle<ScopedCompositeWorkflowContext>>),
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
