use super::components::*;
use super::structs::*;
use super::wrappers::*;
use crate::operations::singletons::*;
use crate::operations::structs::*;
use crate::operations::traits::*;
use bevy::prelude::*;
use tokio::sync::oneshot;

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
    callback: Option<oneshot::Sender<CreateEntityResult>>,
}
impl Operation for CreateEntity {
    type Args = CreateEntityArgs;
    type Result = CreateEntityResult;

    fn new(args: CreateEntityArgs, callback: oneshot::Sender<CreateEntityResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let entity = world.spawn((
            Transform::from_translation(self.args.entity_position.extend(0.0)),
            SpacetimeEntity::new(),
        )).id();

        let spacetime_entity_component = match world.get::<SpacetimeEntity>(entity) {
            Some(spacetime_entity_component) => spacetime_entity_component,
            None => {
                self.callback.send(CreateEntityResult::Err(()));
                return;
            },
        };

        self.callback.send(CreateEntityResult::Ok {
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
    callback: Option<oneshot::Sender<DestroyEntityResult>>,
}
impl Operation for DestroyEntity {
    type Args = DestroyEntityArgs;
    type Result = DestroyEntityResult;

    fn new(args: DestroyEntityArgs, callback: oneshot::Sender<DestroyEntityResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let entity = {
            let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                Ok(main_type_registry) => main_type_registry,
                Err(_) => {
                    self.callback.send(DestroyEntityResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    self.callback.send(DestroyEntityResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.entity_id) {
                Some(entity) => *entity,
                None => {
                    self.callback.send(DestroyEntityResult::Err(()));
                    return;
                },
            }
        };

        if !world.despawn(entity) {
            self.callback.send(DestroyEntityResult::Err(()));
            return;
        }

        self.callback.send(DestroyEntityResult::Ok(()));
    }
}
