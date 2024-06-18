use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::id::structs::*;
use super::types::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct EntityRegistry {
    registered_entities: HashSet<EntityID>,
    loaded_entities: HashMap<EntityID, EntityReference>,
    next_entity_id: EntityID,
    recycled_entity_ids: Vec<EntityID>,
}

impl EntityRegistry {
    pub fn register_entity(&mut self) -> EntityID {
        let entity_id = self.get_unused_entity_id();
        
        self.registered_entities.insert(entity_id);

        entity_id
    }

    pub fn register_entities(&mut self, count: usize) -> Vec<EntityID> {
        let mut entity_ids = Vec::with_capacity(count);

        for _ in 0..count {
            entity_ids.push(self.get_unused_entity_id());
        }

        self.registered_entities.extend(entity_ids.iter());

        entity_ids
    }

    pub fn unregister_entity(&mut self, entity_id: EntityID) {
        self.registered_entities.retain(|&other_entity_id| entity_id != other_entity_id);

        self.recycle_entity_id(entity_id);
    }

    pub fn unregister_entities(&mut self, entity_ids: HashSet<EntityID>) {
        self.registered_entities.retain(|&entity_id| !entity_ids.contains(&entity_id));

        for entity_id in entity_ids {
            self.recycle_entity_id(entity_id);
        }
    }

    pub fn load_entity(&mut self, entity_id: EntityID, entity_reference: EntityReference) {
        self.loaded_entities.insert(entity_id, entity_reference);
    }

    pub fn load_entities(&mut self, entities: HashMap<EntityID, EntityReference>) {
        self.loaded_entities.extend(entities);
    }

    pub fn unload_entity(&mut self, entity_id: EntityID) -> Option<EntityReference> {
        self.loaded_entities.remove(&entity_id)
    }

    pub fn unload_entities(&mut self, entity_ids: HashSet<EntityID>) {
        self.loaded_entities.retain(|&entity_id, _| !entity_ids.contains(&entity_id));
    }

    pub fn is_entity_registered(&self, entity_id: EntityID) -> bool {
        self.registered_entities.contains(&entity_id)
    }

    pub fn are_entities_registered(&self, entity_ids: HashSet<EntityID>) -> bool {
        for entity_id in entity_ids {
            if !self.registered_entities.contains(&entity_id) {
                return false;
            }
        }

        true
    }

    pub fn is_entity_loaded(&self, entity_id: EntityID) -> bool {
        self.loaded_entities.contains_key(&entity_id)
    }

    pub fn are_entities_loaded(&self, entity_ids: HashSet<EntityID>) -> bool {
        for entity_id in entity_ids {
            if !self.loaded_entities.contains_key(&entity_id) {
                return false;
            }
        }

        true
    }

    pub fn registered_entities(&self) -> &HashSet<EntityID> {
        &self.registered_entities
    }

    pub fn registered_entities_mut(&mut self) -> &mut HashSet<EntityID> {
        &mut self.registered_entities
    }

    pub fn get_loaded_entity_reference(&self, entity_id: &EntityID) -> Option<EntityReference> {
        self.loaded_entities.get(entity_id).copied()
    }

    pub fn get_loaded_entity_id(&self, entity_reference: &EntityReference) -> Option<EntityID> {
        self.loaded_entities.iter().find(|(_, e)| **e == *entity_reference).map(|(id, _)| *id)
    }

    pub fn loaded_entity_reference(&self, entity_id: EntityID) -> EntityReference {
        self.loaded_entities[&entity_id]
    }

    pub fn loaded_entity_id(&self, entity_reference: EntityReference) -> EntityID {
        self.loaded_entities.iter().find(|(_, e)| **e == entity_reference).map(|(id, _)| *id).unwrap()
    }

    pub fn loaded_entities(&self) -> &HashMap<EntityID, EntityReference> {
        &self.loaded_entities
    }

    pub fn loaded_entities_mut(&mut self) -> &mut HashMap<EntityID, EntityReference> {
        &mut self.loaded_entities
    }

    pub fn loaded_entity_ids(&self) -> HashSet<EntityID> {
        self.loaded_entities.keys().copied().collect()
    }

    pub fn loaded_entity_references(&self) -> HashSet<EntityReference> {
        self.loaded_entities.values().copied().collect()
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