use bevy::ecs::component::ComponentId;
use bevy::ecs::world::DeferredWorld;
use bevy::prelude::*;
use crate::entity::components::*;
use crate::entity::events::*;
use crate::entity::resources::*;
use crate::entity::structs::*;

pub fn on_add_entity(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;
    
    let spacetime_entity_component = match world.get::<SpacetimeEntity>(entity_reference) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let entity_id = spacetime_entity_component.id;

    let entity_request_id = {
        let entity_request_registry = match world.get_resource::<EntityRequestRegistry>() {
            Some(entity_request_registry) => entity_request_registry,
            None => {
                panic!("Failed to get entity request registry!");
            }
        };

        match entity_request_registry
            .loaded_entity_requests()
            .values()
            .clone()
            .find(|request| request.entity_id == entity_id)
            .map(|request| {
                request.entity_request_id
            }) {
            Some(entity_request_id) => entity_request_id,
            None => {
                panic!("Failed to get entity request id currently associated with entity '{:?}'!", entity_reference);
            }
        }
    };

    {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        let is_entity_creating = entity_registry.is_entity_creating(entity_id);
        let is_entity_loading = entity_registry.is_entity_loading(entity_id);

        if is_entity_creating && is_entity_loading {
            panic!("Entity '{:?}' is both creating and loading!", entity_id);
        } else if is_entity_creating {
            entity_registry.load_entity(entity_id, entity_reference);
            entity_registry.stop_creating_entity(entity_id);
    
            let mut entity_request_registry = match world.get_resource_mut::<EntityRequestRegistry>() {
                Some(entity_request_registry) => entity_request_registry,
                None => {
                    panic!("Failed to get entity request registry!");
                }
            };

            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(CreatedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
            }));

        } else if is_entity_loading {
            entity_registry.load_entity(entity_id, entity_reference);
            entity_registry.stop_loading_entity(entity_id);
    
            let mut entity_request_registry = match world.get_resource_mut::<EntityRequestRegistry>() {
                Some(entity_request_registry) => entity_request_registry,
                None => {
                    panic!("Failed to get entity request registry!");
                }
            };

            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(LoadedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
            }));
        } else {
            panic!("Entity '{:?}' is neither creating nor loading!", entity_id);
        }
    }
}

pub fn on_remove_entity(
    mut world: DeferredWorld,
    entity: Entity,
    _component: ComponentId,
) {
    let entity_reference = entity;

    let spacetime_entity_component = match world.get::<SpacetimeEntity>(entity) {
        Some(spacetime_entity_component) => spacetime_entity_component,
        None => {
            panic!("Failed to get spacetime entity component!");
        }
    };

    let entity_id = spacetime_entity_component.id;

    let entity_request_id =  {
        let entity_request_registry = match world.get_resource::<EntityRequestRegistry>() {
            Some(entity_request_registry) => entity_request_registry,
            None => {
                panic!("Failed to get entity request registry!");
            }
        };

        match entity_request_registry
            .loaded_entity_requests()
            .values()
            .clone()
            .find(|request| request.entity_id == entity_id)
            .map(|request| {
                request.entity_request_id
            }) {
                Some(entity_request_id) => entity_request_id,
                None => {
                    panic!("Failed to get entity request id currently associated with entity '{:?}'!", entity_reference);
                }
            }
    };

    {
        let mut entity_registry = match world.get_resource_mut::<EntityRegistry>() {
            Some(entity_registry) => entity_registry,
            None => {
                panic!("Failed to get entity registry!");
            }
        };

        let is_entity_destroying = entity_registry.is_entity_destroying(entity_id);
        let is_entity_saving = entity_registry.is_entity_saving(entity_id);

        if is_entity_destroying && is_entity_saving {
            panic!("Entity '{:?}' is both destroying and saving!", entity_id);
        } else if is_entity_destroying {
            entity_registry.save_entity(entity_id);
            entity_registry.unregister_entity(entity_id);
            entity_registry.stop_destroying_entity(entity_id);

            let mut entity_request_registry = match world.get_resource_mut::<EntityRequestRegistry>() {
                Some(entity_request_registry) => entity_request_registry,
                None => {
                    panic!("Failed to get entity request registry!");
                }
            };
            
            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(DestroyedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
            }));
        } else if is_entity_saving {
            entity_registry.save_entity(entity_id);
            entity_registry.stop_saving_entity(entity_id);
    
            let mut entity_request_registry = match world.get_resource_mut::<EntityRequestRegistry>() {
                Some(entity_request_registry) => entity_request_registry,
                None => {
                    panic!("Failed to get entity request registry!");
                }
            };

            entity_request_registry.unload_entity_request(entity_request_id);
    
            world.send_event(SavedEntity(EntityResponse::Success {
                entity_request_id,
                entity_id,
            }));
        } else {
            panic!("Entity '{:?}' is neither destroying nor saving!", entity_id);
        }
    }
}