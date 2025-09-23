use bevy::ecs::entity::Entity;
use bevy::prelude::Reflect;
use tokio::task::JoinHandle;
use std::marker::PhantomData;

use crate::usf::scale::Scale;
use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;

#[derive(Reflect)]
pub struct ChunkActionWorkflowHandles<S: Scale> {
    #[reflect(ignore)]
    pub spawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    #[reflect(ignore)]
    pub despawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    #[reflect(ignore)]
    pub transfer: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    #[reflect(ignore)]
    pub phantom_scale: std::marker::PhantomData<S>,
}

#[derive(Debug, Clone, Reflect)]
pub struct ChunkOwnerId<S: Scale> {
    id: String,
    entity: Entity,
    #[reflect(ignore)]
    phantom_scale: std::marker::PhantomData<S>,
}
impl<S: Scale> ChunkOwnerId<S> {
    pub fn new(id: String, entity: Entity) -> Self {
        Self { id, entity, phantom_scale: PhantomData }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
impl<S: Scale> Default for ChunkOwnerId<S> {
    fn default() -> Self {
        Self {
            id: "PLACEHOLDER".to_string(),
            entity: Entity::from_raw(0),
            phantom_scale: PhantomData,
        }
    }
}
impl<S: Scale> std::hash::Hash for ChunkOwnerId<S> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl<S: Scale> PartialEq for ChunkOwnerId<S> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl<S: Scale> Eq for ChunkOwnerId<S> {}
impl<S: Scale> PartialOrd for ChunkOwnerId<S> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl<S: Scale> Ord for ChunkOwnerId<S> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}
