use bevy::prelude::*;
use crate::{chunk_actor::{components::ChunkActor, wrappers::ChunkActorInstanceRegistry}, chunk_loader::components::ChunkLoader, entity::wrappers::EntityInstanceRegistry, operations::{components::Serialized, singletons::MAIN_TYPE_REGISTRY, structs::InstanceID, traits::Operation}};
use super::{components::Chunk, singletons::SERIALIZED_CHUNK_STORAGE, structs::ChunkPosition, wrappers::ChunkInstanceRegistry};

pub struct UpgradeToChunkArgs {
    pub target_entity_id: InstanceID<Entity>,
    pub chunk_position: ChunkPosition,
    pub chunk_owner: Option<InstanceID<ChunkLoader>>,
}
pub enum UpgradeToChunkResult {
    Ok{
        chunk_id: InstanceID<Chunk>,
    },
    Err(()),
}
pub struct UpgradeToChunk {
    args: UpgradeToChunkArgs,
    callback: fn(UpgradeToChunkResult),
}
impl UpgradeToChunk {
    pub fn new(args: UpgradeToChunkArgs, callback: Option<fn(UpgradeToChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UpgradeToChunk {
    fn execute(&self, world: &mut World) {
        if world.query::<&Chunk>().iter(world).any(|chunk| chunk.position() == self.args.chunk_position) {
            (self.callback)(UpgradeToChunkResult::Err(()));
            return;
        }

        match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => {
                if serialized_chunk_storage.contains_key(&self.args.chunk_position) {
                    (self.callback)(UpgradeToChunkResult::Err(()));
                    return;
                }
            },
            Err(_) => {
                (self.callback)(UpgradeToChunkResult::Err(()));
                return;
            },
        };

        let target_entity = {
            let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                Ok(main_type_registry) => main_type_registry,
                Err(_) => {
                    (self.callback)(UpgradeToChunkResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    (self.callback)(UpgradeToChunkResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.target_entity_id) {
                Some(entity) => *entity,
                None => {
                    (self.callback)(UpgradeToChunkResult::Err(()));
                    return;
                },
            }
        };

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                (self.callback)(UpgradeToChunkResult::Err(()));
                return;
            },
        };

        target_entity_raw.insert(Chunk::new(self.args.chunk_position, self.args.chunk_owner));

        let chunk_id = match target_entity_raw.get::<Chunk>() {
            Some(chunk) => chunk.id(),
            None => {
                (self.callback)(UpgradeToChunkResult::Err(()));
                return;
            },
        };

        (self.callback)(UpgradeToChunkResult::Ok {
            chunk_id,
        });
    }
}

pub struct DowngradeFromChunkArgs {
    pub chunk_entity_id: InstanceID<Entity>,
    pub chunk_id: InstanceID<Chunk>,
}
pub enum DowngradeFromChunkResult {
    Ok(()),
    Err(()),
}
pub struct DowngradeFromChunk {
    args: DowngradeFromChunkArgs,
    callback: fn(DowngradeFromChunkResult),
}
impl DowngradeFromChunk {
    pub fn new(args: DowngradeFromChunkArgs, callback: Option<fn(DowngradeFromChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for DowngradeFromChunk {
    fn execute(&self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        let chunk_entity = match chunk_instance_registry.get(self.args.chunk_id) {
            Some(chunk) => *chunk,
            None => {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        let chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
            Some(chunk_raw) => chunk_raw,
            None => {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        if !chunk_entity_raw.contains::<Chunk>() {
            (self.callback)(DowngradeFromChunkResult::Err(()));
            return;
        }

        if chunk_entity_raw.contains::<Serialized>() {
            (self.callback)(DowngradeFromChunkResult::Err(()));
            return;
        }

        match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => {
                if serialized_chunk_storage.contains_key(&chunk_entity_raw.get::<Chunk>().unwrap().position()) {
                    (self.callback)(DowngradeFromChunkResult::Err(()));
                    return;
                }
            },
            Err(_) => {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        if !chunk_instance_registry.is_managed(self.args.chunk_id) {
            (self.callback)(DowngradeFromChunkResult::Err(()));
            return;
        }

        if !chunk_instance_registry.is_registered(self.args.chunk_id) {
            (self.callback)(DowngradeFromChunkResult::Err(()));
            return;
        }

        let chunk = chunk_entity_raw.get::<Chunk>().unwrap();

        let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
            Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
            None => {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        for registered_chunk_actor in chunk.registered_chunk_actors().clone() {
            let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                Some(chunk_actor_entity) => *chunk_actor_entity,
                None => {
                    (self.callback)(DowngradeFromChunkResult::Err(()));
                    return;
                },
            };

            let mut chunk_actor_entity_raw = match world.get_entity_mut(chunk_actor_entity) {
                Some(chunk_actor_raw) => chunk_actor_raw,
                None => {
                    (self.callback)(DowngradeFromChunkResult::Err(()));
                    return;
                },
            };

            if !chunk_actor_entity_raw.contains::<ChunkActor>() {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            }

            if chunk_actor_entity_raw.contains::<Serialized>() {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            }

            if !chunk_actor_instance_registry.is_managed(registered_chunk_actor) {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            }

            if !chunk_actor_instance_registry.is_registered(registered_chunk_actor) {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            }

            chunk_actor_entity_raw.remove::<ChunkActor>();
        }

        let mut chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
            Some(chunk_raw) => chunk_raw,
            None => {
                (self.callback)(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        chunk_entity_raw.remove::<Chunk>();
        (self.callback)(DowngradeFromChunkResult::Ok(()));
    }
}

pub struct LoadChunkArgs {
    pub chunk_position: ChunkPosition,
}
pub enum LoadChunkResult {
    Ok(()),
    Err(()),
}
pub struct LoadChunk {
    args: LoadChunkArgs,
    callback: fn(LoadChunkResult),
}
impl LoadChunk {
    pub fn new(args: LoadChunkArgs, callback: Option<fn(LoadChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for LoadChunk {
    fn execute(&self, world: &mut World) {
        if world.query::<&Chunk>().iter(world).any(|chunk| chunk.position() == self.args.chunk_position) {
            (self.callback)(LoadChunkResult::Err(()));
            return;
        }

        let serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                (self.callback)(LoadChunkResult::Err(()));
                return;
            },
        };

        if !serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            (self.callback)(LoadChunkResult::Err(()));
            return;
        }

        let serialized_chunk = match serialized_chunk_storage.get(&self.args.chunk_position) {
            Some(serialized_chunk) => serialized_chunk.clone(),
            None => {
                (self.callback)(LoadChunkResult::Err(()));
                return;
            },
        };

        let chunk_entity = super::utilities::deserialize_chunk(world, serialized_chunk);

        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(LoadChunkResult::Err(()));
                return;
            },
        };

        let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                (self.callback)(LoadChunkResult::Err(()));
                return;
            },
        };

        let chunk_id = match world.get::<Chunk>(chunk_entity) {
            Some(chunk) => chunk.id(),
            None => {
                (self.callback)(LoadChunkResult::Err(()));
                return;
            },
        };

        if !chunk_instance_registry.is_registered(chunk_id) {
            (self.callback)(LoadChunkResult::Err(()));
            return;
        }

        if !chunk_instance_registry.is_managed(chunk_id) {
            (self.callback)(LoadChunkResult::Err(()));
            return;
        }
    }
}

pub struct UnloadChunkArgs {
    pub chunk_position: ChunkPosition,
}
pub enum UnloadChunkResult {
    Ok(()),
    Err(()),
}
pub struct UnloadChunk {
    args: UnloadChunkArgs,
    callback: fn(UnloadChunkResult),
}
impl UnloadChunk {
    pub fn new(args: UnloadChunkArgs, callback: Option<fn(UnloadChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UnloadChunk {
    fn execute(&self, world: &mut World) {
        let (chunk_entity, chunk) = match world.query::<(Entity, &Chunk)>().iter(world).find(|(_, chunk)| chunk.position() == self.args.chunk_position) {
            Some((chunk_entity, chunk)) => (chunk_entity, chunk),
            None => {
                (self.callback)(UnloadChunkResult::Err(()));
                return;
            },
        };

        let serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                (self.callback)(UnloadChunkResult::Err(()));
                return;
            },
        };

        if !serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            (self.callback)(UnloadChunkResult::Err(()));
            return;
        }

        let chunk_entity_raw = match world.get_entity(chunk_entity) {
            Some(chunk_raw) => chunk_raw,
            None => {
                (self.callback)(UnloadChunkResult::Err(()));
                return;
            },
        };

        if !chunk_entity_raw.contains::<Serialized>() {
            (self.callback)(UnloadChunkResult::Err(()));
            return;
        }

        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                (self.callback)(UnloadChunkResult::Err(()));
                return;
            },
        };

        let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
            Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
            None => {
                (self.callback)(UnloadChunkResult::Err(()));
                return;
            },
        };

        for registered_chunk_actor in chunk.registered_chunk_actors().clone() {
            let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                Some(chunk_actor_entity) => *chunk_actor_entity,
                None => {
                    (self.callback)(UnloadChunkResult::Err(()));
                    return;
                },
            };

            let chunk_actor_entity_raw = match world.get_entity(chunk_actor_entity) {
                Some(chunk_actor_raw) => chunk_actor_raw,
                None => {
                    (self.callback)(UnloadChunkResult::Err(()));
                    return;
                },
            };

            if !chunk_actor_entity_raw.contains::<Serialized>() {
                (self.callback)(UnloadChunkResult::Err(()));
                return;
            }

            if !world.despawn(chunk_actor_entity) {
                (self.callback)(UnloadChunkResult::Err(()));
                return;
            }
        }
        
        if !world.despawn(chunk_entity) {
            (self.callback)(UnloadChunkResult::Ok(()));
            return;
        }
    }
}

pub struct SaveChunkArgs {
    pub chunk_position: ChunkPosition,
    pub force: bool,
}
pub enum SaveChunkResult {
    Ok(()),
    Err(()),
}
pub struct SaveChunk {
    args: SaveChunkArgs,
    callback: fn(SaveChunkResult),
}
impl SaveChunk {
    pub fn new(args: SaveChunkArgs, callback: Option<fn(SaveChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for SaveChunk {
    fn execute(&self, world: &mut World) {
        let mut serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                (self.callback)(SaveChunkResult::Err(()));
                return;
            },
        };

        if !self.args.force && serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            (self.callback)(SaveChunkResult::Err(()));
            return;
        }

        match world.query::<(Entity, &Chunk)>().iter(world).find(|(_, chunk)| chunk.position() == self.args.chunk_position) {
            Some((chunk_entity, chunk)) => {
                let chunk_id = chunk.id();
                let registered_chunk_actors = chunk.registered_chunk_actors().clone();

                let mut chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
                    Some(chunk_raw) => chunk_raw,
                    None => {
                        (self.callback)(SaveChunkResult::Err(()));
                        return;
                    },
                };

                if !chunk_entity_raw.contains::<Serialized>() {
                    chunk_entity_raw.insert(Serialized);
                }

                let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                    Ok(main_type_registry) => main_type_registry,
                    Err(_) => {
                        (self.callback)(SaveChunkResult::Err(()));
                        return;
                    },
                };

                let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
                    Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
                    None => {
                        (self.callback)(SaveChunkResult::Err(()));
                        return;
                    },
                };

                for registered_chunk_actor in registered_chunk_actors {
                    let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                        Some(chunk_actor_entity) => *chunk_actor_entity,
                        None => {
                            (self.callback)(SaveChunkResult::Err(()));
                            return;
                        },
                    };

                    let mut chunk_actor_entity_raw = match world.get_entity_mut(chunk_actor_entity) {
                        Some(chunk_actor_raw) => chunk_actor_raw,
                        None => {
                            (self.callback)(SaveChunkResult::Err(()));
                            return;
                        },
                    };

                    if !chunk_actor_entity_raw.contains::<Serialized>() {
                        chunk_actor_entity_raw.insert(Serialized);
                    }
                }

                let serialized_chunk = super::utilities::serialize_chunk(world, chunk_id);

                serialized_chunk_storage.insert(self.args.chunk_position, serialized_chunk);
            },
            None => {
                (self.callback)(SaveChunkResult::Err(()));
                return;
            },
        }
    }
}

pub struct UnsaveChunkArgs {
    pub chunk_position: ChunkPosition,
}
pub enum UnsaveChunkResult {
    Ok(()),
    Err(()),
}
pub struct UnsaveChunk {
    args: UnsaveChunkArgs,
    callback: fn(UnsaveChunkResult),
}
impl UnsaveChunk {
    pub fn new(args: UnsaveChunkArgs, callback: Option<fn(UnsaveChunkResult)>) -> Self {
        Self {
            args,
            callback: callback.unwrap_or(|_| {}),
        }
    }
}
impl Operation for UnsaveChunk {
    fn execute(&self, world: &mut World) {
        let serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                (self.callback)(UnsaveChunkResult::Err(()));
                return;
            },
        };

        if !serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            (self.callback)(UnsaveChunkResult::Err(()));
            return;
        }
        
        match world.query::<(Entity, &Chunk)>().iter(world).find(|(_, chunk)| chunk.position() == self.args.chunk_position) {
            Some((chunk_entity, chunk)) => {
                let registered_chunk_actors = chunk.registered_chunk_actors().clone();

                let mut chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
                    Some(chunk_raw) => chunk_raw,
                    None => {
                        (self.callback)(UnsaveChunkResult::Err(()));
                        return;
                    },
                };

                if chunk_entity_raw.contains::<Serialized>() {
                    chunk_entity_raw.remove::<Serialized>();
                }

                let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                    Ok(main_type_registry) => main_type_registry,
                    Err(_) => {
                        (self.callback)(UnsaveChunkResult::Err(()));
                        return;
                    },
                };

                let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
                    Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
                    None => {
                        (self.callback)(UnsaveChunkResult::Err(()));
                        return;
                    },
                };

                for registered_chunk_actor in registered_chunk_actors {
                    let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                        Some(chunk_actor_entity) => *chunk_actor_entity,
                        None => {
                            (self.callback)(UnsaveChunkResult::Err(()));
                            return;
                        },
                    };

                    let mut chunk_actor_entity_raw = match world.get_entity_mut(chunk_actor_entity) {
                        Some(chunk_actor_raw) => chunk_actor_raw,
                        None => {
                            (self.callback)(UnsaveChunkResult::Err(()));
                            return;
                        },
                    };

                    if chunk_actor_entity_raw.contains::<Serialized>() {
                        chunk_actor_entity_raw.remove::<Serialized>();
                    }
                }
            },
            None => {
                (self.callback)(UnsaveChunkResult::Err(()));
                return;
            },
        };
    }
}
