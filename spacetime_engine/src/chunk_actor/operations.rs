use bevy::prelude::*;
use crate::{chunk::{components::Chunk, singletons::SERIALIZED_CHUNK_STORAGE, wrappers::ChunkInstanceRegistry}, entity::wrappers::EntityInstanceRegistry, operations::{components::Serialized, singletons::MAIN_TYPE_REGISTRY, structs::InstanceID, traits::*}};
use super::{components::ChunkActor, wrappers::ChunkActorInstanceRegistry};

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
    callback: fn(UpgradeToChunkActorResult),
}
impl UpgradeToChunkActor {
    pub fn new(args: UpgradeToChunkActorArgs, callback: Option<fn(UpgradeToChunkActorResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunkActor {
    type Args = UpgradeToChunkActorArgs;
    type Result = UpgradeToChunkActorResult;

    fn execute(&self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
            Some(entity_instance_registry) => entity_instance_registry,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        let target_entity = match entity_instance_registry.get(self.args.target_entity_id) {
            Some(target_entity) => *target_entity,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        drop(main_type_registry);

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        if target_entity_raw.contains::<ChunkActor>() {
            (self.callback)(UpgradeToChunkActorResult::Err(()));
            return;
        }

        target_entity_raw.insert(ChunkActor::new(self.args.chunk_actor_start_chunk_id));

        let chunk_actor_id = match target_entity_raw.get::<ChunkActor>() {
            Some(chunk_actor) => chunk_actor.id(),
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        (self.callback)(UpgradeToChunkActorResult::Ok {
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
    callback: fn(DowngradeFromChunkActorResult),
}
impl DowngradeFromChunkActor {
    pub fn new(args: DowngradeFromChunkActorArgs, callback: Option<fn(DowngradeFromChunkActorResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunkActor {
    type Args = DowngradeFromChunkActorArgs;
    type Result = DowngradeFromChunkActorResult;

    fn execute(&self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
            Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_entity = match chunk_actor_instance_registry.get(self.args.chunk_actor_id) {
            Some(chunk_actor_entity) => *chunk_actor_entity,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_actor_entity_raw = match world.get_entity(chunk_actor_entity) {
            Some(chunk_actor_raw) => chunk_actor_raw,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        if !chunk_actor_entity_raw.contains::<ChunkActor>() {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        if chunk_actor_entity_raw.contains::<Serialized>() {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        let chunk_actor = match chunk_actor_entity_raw.get::<ChunkActor>() {
            Some(chunk_actor) => chunk_actor,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_id = chunk_actor.current_chunk();

        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_entity = match chunk_instance_registry.get(chunk_id) {
            Some(chunk_entity) => *chunk_entity,
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        let chunk_position = match world.get::<Chunk>(chunk_entity) {
            Some(chunk) => chunk.position(),
            None => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => {
                if serialized_chunk_storage.contains_key(&chunk_position) {
                    (self.callback)(DowngradeFromChunkActorResult::Err(()));
                    return;
                }
            },
            Err(_) => {
                (self.callback)(DowngradeFromChunkActorResult::Err(()));
                return;
            },
        };

        if !chunk_actor_instance_registry.is_managed(self.args.chunk_actor_id) {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }

        if !chunk_actor_instance_registry.is_registered(self.args.chunk_actor_id) {
            (self.callback)(DowngradeFromChunkActorResult::Err(()));
            return;
        }
    }
}
