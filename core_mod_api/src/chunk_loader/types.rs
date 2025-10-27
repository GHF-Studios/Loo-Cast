use bevy::prelude::*;

use crate::usf::scale::Scale;

#[derive(Debug, Clone, Reflect)]
pub struct ChunkLoaderId {
    id: String,
    entity: Entity,
    scale: Scale,
}
impl ChunkLoaderId {
    pub fn new(id: String, entity: Entity, scale: Scale) -> Self {
        Self {
            id,
            entity,
            scale,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn entity(&self) -> &Entity {
        &self.entity
    }

    pub fn scale(&self) -> &Scale {
        &self.scale
    }

    pub fn scale_mut(&mut self) -> &mut Scale {
        &mut self.scale
    }
}
impl Default for ChunkLoaderId {
    fn default() -> Self {
        Self {
            id: "PLACEHOLDER".to_string(),
            entity: Entity::from_raw(0),
            scale: Scale::default(),
        }
    }
}
impl std::hash::Hash for ChunkLoaderId {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}
impl PartialEq for ChunkLoaderId {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for ChunkLoaderId {}
impl PartialOrd for ChunkLoaderId {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ChunkLoaderId {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id.cmp(&other.id)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoader {
    pub id: ChunkLoaderId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Reflect)]
pub struct RemovedChunkLoaderObservation {
    pub entity: Entity,
}