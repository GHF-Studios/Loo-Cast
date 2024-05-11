use bevy::prelude::*;
use std::collections::HashMap;
use super::id::structs::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct EntityManager {
    registered_entities: Vec<EntityID>,
    loaded_entities: HashMap<EntityID, Entity>,
    next_entity_id: EntityID,
    recycled_entity_ids: Vec<EntityID>,
}

impl EntityManager {
    pub fn register_entity(&mut self) -> EntityID {
        let entity_id = self.get_unused_entity_id();
        self.registered_entities.push(entity_id);

        entity_id
    }

    pub fn unregister_entity(&mut self, entity: EntityID) {
        self.registered_entities.retain(|&entity_id| entity_id != entity);
        self.recycle_entity_id(entity);
    }

    pub fn load_entity(&mut self, entity_id: EntityID, entity: Entity) {
        self.loaded_entities.insert(entity_id, entity);
    }

    pub fn unload_entity(&mut self, entity_id: EntityID) -> Option<Entity> {
        self.loaded_entities.remove(&entity_id)
    }

    pub fn is_entity_registered(&self, entity_id: EntityID) -> bool {
        self.registered_entities.contains(&entity_id)
    }

    pub fn is_entity_loaded(&self, entity_id: EntityID) -> bool {
        self.loaded_entities.contains_key(&entity_id)
    }

    pub fn get_entity(&self, entity_id: &EntityID) -> Option<Entity> {
        self.loaded_entities.get(entity_id).copied()
    }

    pub fn entity(&self, entity_id: EntityID) -> Entity {
        self.loaded_entities[&entity_id]
    }

    fn get_unused_entity_id(&mut self) -> EntityID {
        if let Some(recycled_entity_id) = self.recycled_entity_ids.pop() {
            recycled_entity_id
        } else {
            let entity_id = self.next_entity_id;
            self.next_entity_id += 1;
            entity_id
        }
    }

    fn recycle_entity_id(&mut self, entity_id: EntityID) {
        self.recycled_entity_ids.push(entity_id);
    }
}