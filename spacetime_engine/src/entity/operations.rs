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
impl OpArgs for CreateEntityArgs {}
pub enum CreateEntityResult {
    Ok{
        entity_id: InstanceID<Entity>
    },
    Err(()),
}
impl OpResult for CreateEntityResult {}
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
    type Args = CreateEntityArgs;
    type Result = CreateEntityResult;

    fn execute(&self, world: &mut World) {
        let entity = world.spawn((
            Transform::from_translation(self.args.entity_position.extend(0.0)),
            SpacetimeEntity::new(),
        )).id();

        let spacetime_entity_component = match world.get::<SpacetimeEntity>(entity) {
            Some(spacetime_entity_component) => spacetime_entity_component,
            None => {
                (self.callback)(CreateEntityResult::Err(()));
                return;
            },
        };

        (self.callback)(CreateEntityResult::Ok {
            entity_id: spacetime_entity_component.id(),
        });
    }
}

pub struct DestroyEntityArgs {
    pub entity_id: InstanceID<Entity>,
}
impl OpArgs for DestroyEntityArgs {}
pub enum DestroyEntityResult {
    Ok(()),
    Err(()),
}
impl OpResult for DestroyEntityResult {}
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
    type Args = DestroyEntityArgs;
    type Result = DestroyEntityResult;

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
