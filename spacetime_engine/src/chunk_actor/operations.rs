use bevy::prelude::*;
use crate::{chunk::{components::Chunk, singletons::SERIALIZED_CHUNK_STORAGE, wrappers::ChunkInstanceRegistry}, entity::wrappers::EntityInstanceRegistry, operations::{components::Serialized, singletons::MAIN_TYPE_REGISTRY, structs::InstanceID, traits::Operation}};
use super::{components::ChunkActor, wrappers::ChunkActorInstanceRegistry};

pub struct UpgradeToChunkActorArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_actor_start_chunk_id: InstanceID<Chunk>,
}
pub enum UpgradeToChunkActorResult {
    Ok{
        chunk_actor_id: InstanceID<ChunkActor>,
    },
    Err(()),
}
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
    fn execute(&self, world: &mut World) {
        println!("UpgradeToChunkActor::execute");
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        println!("UpgradeToChunkActor::execute 1");

        let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
            Some(entity_instance_registry) => entity_instance_registry,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        println!("UpgradeToChunkActor::execute 2");

        let target_entity = match entity_instance_registry.get(self.args.target_entity_id) {
            Some(target_entity) => *target_entity,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        drop(main_type_registry);

        print!("UpgradeToChunkActor::execute 3");

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        println!("UpgradeToChunkActor::execute 4");

        if target_entity_raw.contains::<ChunkActor>() {
            (self.callback)(UpgradeToChunkActorResult::Err(()));
            return;
        }

        println!("UpgradeToChunkActor::execute 5");

        target_entity_raw.insert(ChunkActor::new(self.args.chunk_actor_start_chunk_id));

        println!("UpgradeToChunkActor::execute 6");

        let chunk_actor_id = match target_entity_raw.get::<ChunkActor>() {
            Some(chunk_actor) => chunk_actor.id(),
            None => {
                (self.callback)(UpgradeToChunkActorResult::Err(()));
                return;
            },
        };

        println!("UpgradeToChunkActor::execute 7");

        (self.callback)(UpgradeToChunkActorResult::Ok {
            chunk_actor_id,
        });

        println!("UpgradeToChunkActor::execute 8");
    }
}

pub struct DowngradeFromChunkActorArgs {
    pub chunk_actor_entity_id: InstanceID<Entity>,
    pub chunk_actor_id: InstanceID<ChunkActor>,
}
pub enum DowngradeFromChunkActorResult {
    Ok(()),
    Err(()),
}
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
