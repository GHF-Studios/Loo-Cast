use super::components::*;
use super::structs::*;
use super::wrappers::*;
use crate::operations::components::*;
use crate::operations::singletons::*;
use crate::operations::structs::*;
use crate::operations::traits::*;
use bevy::prelude::*;

pub struct CreateEntityArgs {
    pub entity_position: EntityPosition,
}
pub enum CreateEntityResult {
    Ok{
        entity_id: InstanceID<Entity>
    },
    Err(()),
}
pub struct CreateEntity {
    args: CreateEntityArgs,
    callback: fn(CreateEntityResult),
}
impl CreateEntity {
    pub fn new(args: CreateEntityArgs, callback: Option<fn(CreateEntityResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for CreateEntity {
    fn execute(&self, world: &mut World) {
        let entity = world.spawn((
            Transform::from_translation(self.args.entity_position.extend(0.0)),
            SpacetimeEntity::new(),
            ReactOnAdd::<SpacetimeEntity>::new({
                let callback = self.callback;
                move |spacetime_entity| {
                    callback(CreateEntityResult::Ok {
                        entity_id: spacetime_entity.id(),
                    });
                }
            })
        )).id();

        // TODO: Read this comment.
        // Maybe instead of immediately trying to access the hook-modified spacetime entity component,
        // we should just have some temporary marker/data-transmission component 'ReactOnAdd<T>' which contains the callback, and the callback parameter.
        // We would just insert the 'ReactOnAdd<T>' component here, and the hook would remove it and call the callback with the given parameter.
        // TODO: Do the same for 'ReactOnRemove<T>'.
        let spacetime_entity_component = match world.get::<SpacetimeEntity>(entity) {
            Some(spacetime_entity_component) => spacetime_entity_component,
            None => {
                (self.callback)(CreateEntityResult::Err(()));
                return;
            },
        };

        // TODO: Remove
        warn!("Insert: Created entity: {:?}!", spacetime_entity_component.id());
        (self.callback)(CreateEntityResult::Ok {
            entity_id: spacetime_entity_component.id(),
        });
    }
}

pub struct DestroyEntityArgs {
    pub entity_id: InstanceID<Entity>,
}
pub enum DestroyEntityResult {
    Ok(()),
    Err(()),
}
pub struct DestroyEntity {
    args: DestroyEntityArgs,
    callback: fn(DestroyEntityResult),
}
impl DestroyEntity {
    pub fn new(args: DestroyEntityArgs, callback: Option<fn(DestroyEntityResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DestroyEntity {
    fn execute(&self, world: &mut World) {
        let entity = {
            let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                Ok(main_type_registry) => main_type_registry,
                Err(_) => {
                    (self.callback)(DestroyEntityResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    (self.callback)(DestroyEntityResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.entity_id) {
                Some(entity) => *entity,
                None => {
                    (self.callback)(DestroyEntityResult::Err(()));
                    return;
                },
            }
        };

        if !world.despawn(entity) {
            (self.callback)(DestroyEntityResult::Err(()));
            return;
        }

        (self.callback)(DestroyEntityResult::Ok(()));
    }
}
