use bevy::ecs::entity::Entity;
use tokio::task::JoinHandle;

use crate::workflow::composite_workflow_context::ScopedCompositeWorkflowContext;

pub struct ChunkActionWorkflowHandles {
    pub spawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    pub despawn: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
    pub transfer: Option<JoinHandle<ScopedCompositeWorkflowContext>>,
}

#[derive(Debug, Clone)]
pub struct ChunkOwnerId {
    id: String,
    entity: Entity,
}
impl ChunkOwnerId {
    pub fn new(id: String, entity: Entity) -> Self {
        Self {
            id,
            entity
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn entity(&self) -> Entity {
        self.entity
    }
}
impl Default for ChunkOwnerId {
    fn default() -> Self {
        Self {
            id: "PLACEHOLDER".to_string(),
            entity: Entity::from_raw(0)
        }
    }
}
impl std::hash::Hash for ChunkOwnerId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for ChunkOwnerId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for ChunkOwnerId {}
impl PartialOrd for ChunkOwnerId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}
impl Ord for ChunkOwnerId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}