use bevy::prelude::*;
use std::collections::{HashMap, HashSet};
use super::id::structs::*;
use super::structs::*;
use super::types::*;

#[derive(Resource, Debug, Default)]
pub(in crate) struct EntityRegistry {
    registered_entities: HashSet<EntityID>,
    loaded_entities: HashMap<EntityID, EntityReference>,
    currently_creating_entities: HashSet<EntityID>,
    currently_destroying_entities: HashSet<EntityID>,
    currently_loading_entities: HashSet<EntityID>,
    currently_saving_entities: HashSet<EntityID>,
    create_entity_requests: HashMap<EntityRequestID, EntityRequest>,
    destroy_entity_requests: HashMap<EntityRequestID, EntityRequest>,
    load_entity_requests: HashMap<EntityRequestID, EntityRequest>,
    save_entity_requests: HashMap<EntityRequestID, EntityRequest>,
    next_entity_id: EntityID,
    recycled_entity_ids: Vec<EntityID>,
}

// TODO: Implement rudimentary checks within this and all other registries to enforce basic invariants
// Examples include: Redundant/Overwrite operations, Registration/Loading State Variants, etc.
impl EntityRegistry {
    pub(in crate) fn register_entity(&mut self) -> EntityID {
        let entity_id = self.get_unused_entity_id();
        
        self.registered_entities.insert(entity_id);

        trace!("Registered entity with id: '{:?}'", entity_id);

        entity_id
    }

    pub(in crate) fn unregister_entity(&mut self, entity_id: EntityID) {
        self.registered_entities.retain(|&other_entity_id| entity_id != other_entity_id);

        trace!("Unregistered entity with id: '{:?}'", entity_id);

        self.recycle_entity_id(entity_id);
    }

    pub(in crate) fn load_entity(&mut self, entity_id: EntityID, entity_reference: EntityReference) {
        self.loaded_entities.insert(entity_id, entity_reference);

        trace!("Loaded entity reference '{:?}' with id: '{:?}'", entity_reference, entity_id);
    }

    pub(in crate) fn save_entity(&mut self, entity_id: EntityID) -> Option<EntityReference> {
        let removed_entity = self.loaded_entities.remove(&entity_id);

        trace!("Unloaded entity with id: '{:?}'", entity_id);

        removed_entity
    }

    pub(in crate) fn start_creating_entity(&mut self, request: EntityRequest) {
        self.create_entity_requests.insert(request.entity_request_id, request.clone());
        self.currently_creating_entities.insert(request.entity_id);

        trace!("Started creating entity with id: '{:?}'", request.entity_id);
    }

    pub(in crate) fn stop_creating_entity(&mut self, entity_id: EntityID) {
        self.create_entity_requests.retain(|_, request| request.entity_id != entity_id);
        self.currently_creating_entities.remove(&entity_id);

        trace!("Stopped creating entity with id: '{:?}'", entity_id);
    }

    pub(in crate) fn start_destroying_entity(&mut self, request: EntityRequest) {
        self.destroy_entity_requests.insert(request.entity_request_id, request.clone());
        self.currently_destroying_entities.insert(request.entity_id);

        trace!("Started destroying entity with id: '{:?}'", request.entity_id);
    }

    pub(in crate) fn stop_destroying_entity(&mut self, entity_id: EntityID) {
        self.destroy_entity_requests.retain(|_, request| request.entity_id != entity_id);
        self.currently_destroying_entities.remove(&entity_id);

        trace!("Stopped destroying entity with id: '{:?}'", entity_id);
    }

    pub(in crate) fn start_loading_entity(&mut self, request: EntityRequest) {
        self.load_entity_requests.insert(request.entity_request_id, request.clone());
        self.currently_loading_entities.insert(request.entity_id);

        trace!("Started loading entity with id: '{:?}'", request.entity_id);
    }

    pub(in crate) fn stop_loading_entity(&mut self, entity_id: EntityID) {
        self.load_entity_requests.retain(|_, request| request.entity_id != entity_id);
        self.currently_loading_entities.remove(&entity_id);

        trace!("Stopped loading entity with id: '{:?}'", entity_id);
    }

    pub(in crate) fn start_saving_entity(&mut self, request: EntityRequest) {
        self.save_entity_requests.insert(request.entity_request_id, request.clone());
        self.currently_saving_entities.insert(request.entity_id);

        trace!("Started saving entity with id: '{:?}'", request.entity_id);
    }

    pub(in crate) fn stop_saving_entity(&mut self, entity_id: EntityID) {
        self.save_entity_requests.retain(|_, request| request.entity_id != entity_id);
        self.currently_saving_entities.remove(&entity_id);

        trace!("Stopped saving entity with id: '{:?}'", entity_id);
    }

    pub(in crate) fn is_entity_registered(&self, entity_id: EntityID) -> bool {
        self.registered_entities.contains(&entity_id)
    }

    pub(in crate) fn is_entity_loaded(&self, entity_id: EntityID) -> bool {
        self.loaded_entities.contains_key(&entity_id)
    }

    pub(in crate) fn is_entity_creating(&self, entity_id: EntityID) -> bool {
        self.currently_creating_entities.contains(&entity_id)
    }

    pub(in crate) fn is_entity_destroying(&self, entity_id: EntityID) -> bool {
        self.currently_destroying_entities.contains(&entity_id)
    }

    pub(in crate) fn is_entity_loading(&self, entity_id: EntityID) -> bool {
        self.currently_loading_entities.contains(&entity_id)
    }

    pub(in crate) fn is_entity_saving(&self, entity_id: EntityID) -> bool {
        self.currently_saving_entities.contains(&entity_id)
    }

    pub(in crate) fn registered_entities(&self) -> &HashSet<EntityID> {
        &self.registered_entities
    }

    pub(in crate) fn registered_entities_mut(&mut self) -> &mut HashSet<EntityID> {
        &mut self.registered_entities
    }

    pub(in crate) fn get_loaded_entity_reference(&self, entity_id: &EntityID) -> Option<EntityReference> {
        self.loaded_entities.get(entity_id).copied()
    }

    pub(in crate) fn get_loaded_entity_id(&self, entity_reference: &EntityReference) -> Option<EntityID> {
        self.loaded_entities.iter().find(|(_, e)| **e == *entity_reference).map(|(id, _)| *id)
    }

    pub(in crate) fn loaded_entity_reference(&self, entity_id: EntityID) -> EntityReference {
        self.loaded_entities[&entity_id]
    }

    pub(in crate) fn loaded_entity_id(&self, entity_reference: EntityReference) -> EntityID {
        self.loaded_entities.iter().find(|(_, e)| **e == entity_reference).map(|(id, _)| *id).unwrap()
    }

    pub(in crate) fn loaded_entities(&self) -> &HashMap<EntityID, EntityReference> {
        &self.loaded_entities
    }

    pub(in crate) fn loaded_entities_mut(&mut self) -> &mut HashMap<EntityID, EntityReference> {
        &mut self.loaded_entities
    }

    pub(in crate) fn loaded_entity_ids(&self) -> HashSet<EntityID> {
        self.loaded_entities.keys().copied().collect()
    }

    pub(in crate) fn loaded_entity_references(&self) -> HashSet<EntityReference> {
        self.loaded_entities.values().copied().collect()
    }

    pub(in crate) fn create_entity_requests(&self) -> &HashMap<EntityRequestID, EntityRequest> {
        &self.create_entity_requests
    }

    pub(in crate) fn create_entity_requests_mut(&mut self) -> &mut HashMap<EntityRequestID, EntityRequest> {
        &mut self.create_entity_requests
    }

    pub(in crate) fn destroy_entity_requests(&self) -> &HashMap<EntityRequestID, EntityRequest> {
        &self.destroy_entity_requests
    }

    pub(in crate) fn destroy_entity_requests_mut(&mut self) -> &mut HashMap<EntityRequestID, EntityRequest> {
        &mut self.destroy_entity_requests
    }

    pub(in crate) fn load_entity_requests(&self) -> &HashMap<EntityRequestID, EntityRequest> {
        &self.load_entity_requests
    }

    pub(in crate) fn load_entity_requests_mut(&mut self) -> &mut HashMap<EntityRequestID, EntityRequest> {
        &mut self.load_entity_requests
    }

    pub(in crate) fn save_entity_requests(&self) -> &HashMap<EntityRequestID, EntityRequest> {
        &self.save_entity_requests
    }

    pub(in crate) fn save_entity_requests_mut(&mut self) -> &mut HashMap<EntityRequestID, EntityRequest> {
        &mut self.save_entity_requests
    }

    fn get_unused_entity_id(&mut self) -> EntityID {
        if let Some(recycled_entity_id) = self.recycled_entity_ids.pop() {
            trace!("Used recycled entity id: '{:?}'", recycled_entity_id);

            recycled_entity_id
        } else {
            trace!("Generated new entity id: '{:?}'", self.next_entity_id);

            let entity_id = self.next_entity_id;
            self.next_entity_id += 1;
            entity_id
        }
    }

    fn recycle_entity_id(&mut self, entity_id: EntityID) {
        self.recycled_entity_ids.push(entity_id);

        trace!("Recycled entity id: '{:?}'", entity_id);
    }
}

#[derive(Resource, Debug, Default)]
pub(in crate) struct EntityRequestRegistry {
    registered_entity_requests: HashSet<EntityRequestID>,
    loaded_entity_requests: HashMap<EntityRequestID, EntityRequest>,
    next_entity_request_id: EntityRequestID,
}

impl EntityRequestRegistry {
    pub(in crate) fn register_entity_request(&mut self, entity_request_id: EntityRequestID) {
        self.registered_entity_requests.insert(entity_request_id);

        trace!("Registered entity request with id: '{:?}'", entity_request_id);
    }

    pub(in crate) fn unregister_entity_request(&mut self, entity_request_id: EntityRequestID) {
        self.registered_entity_requests.remove(&entity_request_id);

        trace!("Unregistered entity request with id: '{:?}'", entity_request_id);
    }

    pub(in crate) fn load_entity_request(&mut self, request: EntityRequest) {
        self.loaded_entity_requests.insert(request.entity_request_id, request.clone());

        trace!("Loaded entity request with id: '{:?}'", request.entity_request_id);
    }

    pub(in crate) fn unload_entity_request(&mut self, entity_request_id: EntityRequestID) -> Option<EntityRequest> {
        let removed_entity_request = self.loaded_entity_requests.remove(&entity_request_id);

        trace!("Unloaded entity request with id: '{:?}'", entity_request_id);

        removed_entity_request
    }

    pub(in crate) fn is_entity_request_registered(&self, entity_request_id: EntityRequestID) -> bool {
        self.registered_entity_requests.contains(&entity_request_id)
    }

    pub(in crate) fn is_entity_request_loaded(&self, entity_request_id: EntityRequestID) -> bool {
        self.loaded_entity_requests.contains_key(&entity_request_id)
    }

    pub(in crate) fn registered_entity_requests(&self) -> &HashSet<EntityRequestID> {
        &self.registered_entity_requests
    }

    pub(in crate) fn registered_entity_requests_mut(&mut self) -> &mut HashSet<EntityRequestID> {
        &mut self.registered_entity_requests
    }

    pub(in crate) fn get_loaded_entity_request(&self, entity_request_id: EntityRequestID) -> Option<EntityRequest> {
        self.loaded_entity_requests.get(&entity_request_id).copied()
    }

    pub(in crate) fn get_loaded_entity_request_id(&self, entity_request: &EntityRequest) -> Option<EntityRequestID> {
        self.loaded_entity_requests.iter().find(|(_, r)| **r == *entity_request).map(|(id, _)| *id)
    }

    pub(in crate) fn loaded_entity_request(&self, entity_request_id: EntityRequestID) -> EntityRequest {
        self.loaded_entity_requests[&entity_request_id]
    }

    pub(in crate) fn loaded_entity_request_id(&self, entity_request: EntityRequest) -> EntityRequestID {
        self.loaded_entity_requests.iter().find(|(_, r)| **r == entity_request).map(|(id, _)| *id).unwrap()
    }

    pub(in crate) fn loaded_entity_requests(&self) -> &HashMap<EntityRequestID, EntityRequest> {
        &self.loaded_entity_requests
    }

    pub(in crate) fn loaded_entity_requests_mut(&mut self) -> &mut HashMap<EntityRequestID, EntityRequest> {
        &mut self.loaded_entity_requests
    }

    pub(in crate) fn get_unused_entity_request_id(&mut self) -> EntityRequestID {
        let entity_request_id = self.next_entity_request_id;

        self.next_entity_request_id = EntityRequestID(entity_request_id.0 + 1);

        entity_request_id
    }
}