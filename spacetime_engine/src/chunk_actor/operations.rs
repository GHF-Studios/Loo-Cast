use bevy::prelude::*;
use crate::{chunk::{components::Chunk, singletons::SERIALIZED_CHUNK_STORAGE, wrappers::ChunkInstanceRegistry}, entity::wrappers::EntityInstanceRegistry, core::{components::Serialized, singletons::MAIN_TYPE_REGISTRY, traits::*}, operations::traits::*};
use crate::core::structs::*;
use super::{components::ChunkActor, wrappers::ChunkActorInstanceRegistry};
use tokio::sync::oneshot;

pub struct UpgradeToChunkActorArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_actor_start_chunk_id: InstanceID<Chunk>,
}
impl OpArgs for UpgradeToChunkActorArgs {}
pub enum UpgradeToChunkActorResult {
    Ok{
        chunk_actor_id: InstanceID<ChunkActor>,
    },
    Err(()),
}
impl OpResult for UpgradeToChunkActorResult {}
pub struct UpgradeToChunkActor {
    args: UpgradeToChunkActorArgs,
    callback: Option<oneshot::Sender<UpgradeToChunkActorResult>>,
}
impl Operation for UpgradeToChunkActor {
    type Args = UpgradeToChunkActorArgs;
    type Result = UpgradeToChunkActorResult;

    fn new(args: UpgradeToChunkActorArgs, callback: oneshot::Sender<UpgradeToChunkActorResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                self.callback.send(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
            Some(entity_instance_registry) => entity_instance_registry,
            None => {
                self.callback.send(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        let target_entity = match entity_instance_registry.get(self.args.target_entity_id) {
            Some(target_entity) => *target_entity,
            None => {
                self.callback.send(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        drop(main_type_registry);

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                self.callback.send(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        if target_entity_raw.contains::<ChunkActor>() {
            self.callback.send(UpgradeToChunkActorResult::Err(()));
            return;
        }

        target_entity_raw.insert(ChunkActor::new(self.args.chunk_actor_start_chunk_id));

        let chunk_actor_id = match target_entity_raw.get::<ChunkActor>() {
            Some(chunk_actor) => chunk_actor.id(),
            None => {
                self.callback.send(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        self.callback.send(UpgradeToChunkActorResult::Ok {
            chunk_actor_id,
        });
    }
}

pub struct DowngradeFromChunkActorArgs {
    pub chunk_actor_entity_id: InstanceID<Entity>,
    pub chunk_actor_id: InstanceID<ChunkActor>,
}
impl OpArgs for DowngradeFromChunkActorArgs {}
pub enum DowngradeFromChunkActorResult {
    Ok(()),
    Err(()),
}
impl OpResult for DowngradeFromChunkActorResult {}
pub struct DowngradeFromChunkActor {
    args: DowngradeFromChunkActorArgs,
    callback: Option<oneshot::Sender<DowngradeFromChunkActorResult>>,
}
impl Operation for DowngradeFromChunkActor {
    type Args = DowngradeFromChunkActorArgs;
    type Result = DowngradeFromChunkActorResult;

    fn new(args: DowngradeFromChunkActorArgs, callback: oneshot::Sender<DowngradeFromChunkActorResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
            Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
            None => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_entity = match chunk_actor_instance_registry.get(self.args.chunk_actor_id) {
            Some(chunk_actor_entity) => *chunk_actor_entity,
            None => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_entity_raw = match world.get_entity(chunk_actor_entity) {
            Some(chunk_actor_raw) => chunk_actor_raw,
            None => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        if !chunk_actor_entity_raw.contains::<ChunkActor>() {
            self.callback.send(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        if chunk_actor_entity_raw.contains::<Serialized>() {
            self.callback.send(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        let chunk_actor = match chunk_actor_entity_raw.get::<ChunkActor>() {
            Some(chunk_actor) => chunk_actor,
            None => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_id = chunk_actor.current_chunk();

        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_entity = match chunk_instance_registry.get(chunk_id) {
            Some(chunk_entity) => *chunk_entity,
            None => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_position = match world.get::<Chunk>(chunk_entity) {
            Some(chunk) => chunk.position(),
            None => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => {
                if serialized_chunk_storage.contains_key(&chunk_position) {
                    self.callback.send(DowngradeFromChunkActorResult::Err(()));
                    return;
                }
            },
            Err(_) => {
                self.callback.send(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        if !chunk_actor_instance_registry.is_managed(self.args.chunk_actor_id) {
            self.callback.send(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        if !chunk_actor_instance_registry.is_registered(self.args.chunk_actor_id) {
            self.callback.send(DowngradeFromChunkActorResult::Err(()));
        }
    }
}
