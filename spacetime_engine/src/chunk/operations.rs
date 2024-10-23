use bevy::prelude::*;
use crate::{chunk_actor::{components::ChunkActor, wrappers::ChunkActorInstanceRegistry}, chunk_loader::components::ChunkLoader, core::{components::*, singletons::*}, entity::wrappers::EntityInstanceRegistry, operations::traits::*};
use crate::core::structs::*;
use super::{components::Chunk, singletons::SERIALIZED_CHUNK_STORAGE, structs::ChunkPosition, wrappers::ChunkInstanceRegistry};
use tokio::sync::oneshot;

pub struct UpgradeToChunkArgs {
    pub target_entity_id: DynamicID<Entity>,
    pub chunk_position: ChunkPosition,
    pub chunk_owner: Option<DynamicID<ChunkLoader>>,
}
impl OpArgs for UpgradeToChunkArgs {}
pub enum UpgradeToChunkResult {
    Ok{
        chunk_id: DynamicID<Chunk>,
    },
    Err(()),
}
impl OpResult for UpgradeToChunkResult {}
pub struct UpgradeToChunk {
    args: UpgradeToChunkArgs,
    callback: Option<oneshot::Sender<UpgradeToChunkResult>>,
}
impl Operation for UpgradeToChunk {
    type Args = UpgradeToChunkArgs;
    type Result = UpgradeToChunkResult;

    fn new(args: UpgradeToChunkArgs, callback: oneshot::Sender<UpgradeToChunkResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        if world.query::<&Chunk>().iter(world).any(|chunk| chunk.position() == self.args.chunk_position) {
            self.callback.send(UpgradeToChunkResult::Err(()));
            return;
        }

        match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => {
                if serialized_chunk_storage.contains_key(&self.args.chunk_position) {
                    self.callback.send(UpgradeToChunkResult::Err(()));
                    return;
                }
            },
            Err(_) => {
                self.callback.send(UpgradeToChunkResult::Err(()));
                return;
            },
        };

        let target_entity = {
            let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                Ok(main_type_registry) => main_type_registry,
                Err(_) => {
                    self.callback.send(UpgradeToChunkResult::Err(()));
                    return;
                },
            };

            let entity_instance_registry = match main_type_registry.get_data_mut::<Entity, EntityInstanceRegistry>() {
                Some(entity_instance_registry) => entity_instance_registry,
                None => {
                    self.callback.send(UpgradeToChunkResult::Err(()));
                    return;
                },
            };

            match entity_instance_registry.get(self.args.target_entity_id) {
                Some(entity) => *entity,
                None => {
                    self.callback.send(UpgradeToChunkResult::Err(()));
                    return;
                },
            }
        };

        let mut target_entity_raw = match world.get_entity_mut(target_entity) {
            Some(target_entity_raw) => target_entity_raw,
            None => {
                self.callback.send(UpgradeToChunkResult::Err(()));
                return;
            },
        };

        target_entity_raw.insert(Chunk::new(self.args.chunk_position, self.args.chunk_owner));

        let chunk_id = match target_entity_raw.get::<Chunk>() {
            Some(chunk) => chunk.id(),
            None => {
                self.callback.send(UpgradeToChunkResult::Err(()));
                return;
            },
        };

        self.callback.send(UpgradeToChunkResult::Ok {
            chunk_id,
        })
    }
}

pub struct DowngradeFromChunkArgs {
    pub chunk_entity_id: DynamicID<Entity>,
    pub chunk_id: DynamicID<Chunk>,
}
impl OpArgs for DowngradeFromChunkArgs {}
pub enum DowngradeFromChunkResult {
    Ok(()),
    Err(()),
}
impl OpResult for DowngradeFromChunkResult {}
pub struct DowngradeFromChunk {
    args: DowngradeFromChunkArgs,
    callback: Option<oneshot::Sender<DowngradeFromChunkResult>>,
}
impl Operation for DowngradeFromChunk {
    type Args = DowngradeFromChunkArgs;
    type Result = DowngradeFromChunkResult;

    fn new(args: DowngradeFromChunkArgs, callback: oneshot::Sender<DowngradeFromChunkResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        let chunk_entity = match chunk_instance_registry.get(self.args.chunk_id) {
            Some(chunk) => *chunk,
            None => {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        let chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
            Some(chunk_raw) => chunk_raw,
            None => {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        if !chunk_entity_raw.contains::<Chunk>() {
            self.callback.send(DowngradeFromChunkResult::Err(()));
            return;
        }

        if chunk_entity_raw.contains::<Serialized>() {
            self.callback.send(DowngradeFromChunkResult::Err(()));
            return;
        }

        match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => {
                if serialized_chunk_storage.contains_key(&chunk_entity_raw.get::<Chunk>().unwrap().position()) {
                    self.callback.send(DowngradeFromChunkResult::Err(()));
                    return;
                }
            },
            Err(_) => {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        if !chunk_instance_registry.is_managed(self.args.chunk_id) {
            self.callback.send(DowngradeFromChunkResult::Err(()));
            return;
        }

        if !chunk_instance_registry.is_registered(self.args.chunk_id) {
            self.callback.send(DowngradeFromChunkResult::Err(()));
            return;
        }

        let chunk = chunk_entity_raw.get::<Chunk>().unwrap();

        let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
            Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
            None => {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        for registered_chunk_actor in chunk.registered_chunk_actors().clone() {
            let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                Some(chunk_actor_entity) => *chunk_actor_entity,
                None => {
                    self.callback.send(DowngradeFromChunkResult::Err(()));
                    return;
                },
            };

            let mut chunk_actor_entity_raw = match world.get_entity_mut(chunk_actor_entity) {
                Some(chunk_actor_raw) => chunk_actor_raw,
                None => {
                    self.callback.send(DowngradeFromChunkResult::Err(()));
                    return;
                },
            };

            if !chunk_actor_entity_raw.contains::<ChunkActor>() {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            }

            if chunk_actor_entity_raw.contains::<Serialized>() {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            }

            if !chunk_actor_instance_registry.is_managed(registered_chunk_actor) {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            }

            if !chunk_actor_instance_registry.is_registered(registered_chunk_actor) {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            }

            chunk_actor_entity_raw.remove::<ChunkActor>();
        }

        let mut chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
            Some(chunk_raw) => chunk_raw,
            None => {
                self.callback.send(DowngradeFromChunkResult::Err(()));
                return;
            },
        };

        chunk_entity_raw.remove::<Chunk>();
        self.callback.send(DowngradeFromChunkResult::Ok(()));
    }
}

pub struct LoadChunkArgs {
    pub chunk_position: ChunkPosition,
}
impl OpArgs for LoadChunkArgs {}
pub enum LoadChunkResult {
    Ok(()),
    Err(()),
}
impl OpResult for LoadChunkResult {}
pub struct LoadChunk {
    args: LoadChunkArgs,
    callback: Option<oneshot::Sender<LoadChunkResult>>,
}
impl Operation for LoadChunk {
    type Args = LoadChunkArgs;
    type Result = LoadChunkResult;

    fn new(args: LoadChunkArgs, callback: oneshot::Sender<LoadChunkResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        if world.query::<&Chunk>().iter(world).any(|chunk| chunk.position() == self.args.chunk_position) {
            self.callback.send(LoadChunkResult::Err(()));
            return;
        }

        let serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                self.callback.send(LoadChunkResult::Err(()));
                return;
            },
        };

        if !serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            self.callback.send(LoadChunkResult::Err(()));
            return;
        }

        let serialized_chunk = match serialized_chunk_storage.get(&self.args.chunk_position) {
            Some(serialized_chunk) => serialized_chunk.clone(),
            None => {
                self.callback.send(LoadChunkResult::Err(()));
                return;
            },
        };

        let chunk_entity = super::utilities::deserialize_chunk(world, serialized_chunk);

        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                self.callback.send(LoadChunkResult::Err(()));
                return;
            },
        };

        let chunk_instance_registry = match main_type_registry.get_data_mut::<Chunk, ChunkInstanceRegistry>() {
            Some(chunk_instance_registry) => chunk_instance_registry,
            None => {
                self.callback.send(LoadChunkResult::Err(()));
                return;
            },
        };

        let chunk_id = match world.get::<Chunk>(chunk_entity) {
            Some(chunk) => chunk.id(),
            None => {
                self.callback.send(LoadChunkResult::Err(()));
                return;
            },
        };

        if !chunk_instance_registry.is_registered(chunk_id) {
            self.callback.send(LoadChunkResult::Err(()));
            return;
        }

        if !chunk_instance_registry.is_managed(chunk_id) {
            self.callback.send(LoadChunkResult::Err(()));
        }
    }
}

pub struct UnloadChunkArgs {
    pub chunk_position: ChunkPosition,
}
impl OpArgs for UnloadChunkArgs {}
pub enum UnloadChunkResult {
    Ok(()),
    Err(()),
}
impl OpResult for UnloadChunkResult {}
pub struct UnloadChunk {
    args: UnloadChunkArgs,
    callback: Option<oneshot::Sender<UnloadChunkResult>>,
}
impl Operation for UnloadChunk {
    type Args = UnloadChunkArgs;
    type Result = UnloadChunkResult;

    fn new(args: UnloadChunkArgs, callback: oneshot::Sender<UnloadChunkResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let (chunk_entity, chunk) = match world.query::<(Entity, &Chunk)>().iter(world).find(|(_, chunk)| chunk.position() == self.args.chunk_position) {
            Some((chunk_entity, chunk)) => (chunk_entity, chunk),
            None => {
                self.callback.send(UnloadChunkResult::Err(()));
                return;
            },
        };

        let serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                self.callback.send(UnloadChunkResult::Err(()));
                return;
            },
        };

        if !serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            self.callback.send(UnloadChunkResult::Err(()));
            return;
        }

        let chunk_entity_raw = match world.get_entity(chunk_entity) {
            Some(chunk_raw) => chunk_raw,
            None => {
                self.callback.send(UnloadChunkResult::Err(()));
                return;
            },
        };

        if !chunk_entity_raw.contains::<Serialized>() {
            self.callback.send(UnloadChunkResult::Err(()));
            return;
        }

        let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
            Ok(main_type_registry) => main_type_registry,
            Err(_) => {
                self.callback.send(UnloadChunkResult::Err(()));
                return;
            },
        };

        let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
            Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
            None => {
                self.callback.send(UnloadChunkResult::Err(()));
                return;
            },
        };

        for registered_chunk_actor in chunk.registered_chunk_actors().clone() {
            let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                Some(chunk_actor_entity) => *chunk_actor_entity,
                None => {
                    self.callback.send(UnloadChunkResult::Err(()));
                    return;
                },
            };

            let chunk_actor_entity_raw = match world.get_entity(chunk_actor_entity) {
                Some(chunk_actor_raw) => chunk_actor_raw,
                None => {
                    self.callback.send(UnloadChunkResult::Err(()));
                    return;
                },
            };

            if !chunk_actor_entity_raw.contains::<Serialized>() {
                self.callback.send(UnloadChunkResult::Err(()));
                return;
            }

            if !world.despawn(chunk_actor_entity) {
                self.callback.send(UnloadChunkResult::Err(()));
                return;
            }
        }
        
        if !world.despawn(chunk_entity) {
            self.callback.send(UnloadChunkResult::Ok(()));
        }
    }
}

pub struct SaveChunkArgs {
    pub chunk_position: ChunkPosition,
    pub force: bool,
}
impl OpArgs for SaveChunkArgs {}
pub enum SaveChunkResult {
    Ok(()),
    Err(()),
}
impl OpResult for SaveChunkResult {}
pub struct SaveChunk {
    args: SaveChunkArgs,
    callback: Option<oneshot::Sender<SaveChunkResult>>,
}
impl Operation for SaveChunk {
    type Args = SaveChunkArgs;
    type Result = SaveChunkResult;

    fn new(args: SaveChunkArgs, callback: oneshot::Sender<SaveChunkResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }

    fn execute(&mut self, world: &mut World) {
        let mut serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                self.callback.send(SaveChunkResult::Err(()));
                return;
            },
        };

        if !self.args.force && serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            self.callback.send(SaveChunkResult::Err(()));
            return;
        }

        match world.query::<(Entity, &Chunk)>().iter(world).find(|(_, chunk)| chunk.position() == self.args.chunk_position) {
            Some((chunk_entity, chunk)) => {
                let chunk_id = chunk.id();
                let registered_chunk_actors = chunk.registered_chunk_actors().clone();

                let mut chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
                    Some(chunk_raw) => chunk_raw,
                    None => {
                        self.callback.send(SaveChunkResult::Err(()));
                        return;
                    },
                };

                if !chunk_entity_raw.contains::<Serialized>() {
                    chunk_entity_raw.insert(Serialized);
                }

                let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                    Ok(main_type_registry) => main_type_registry,
                    Err(_) => {
                        self.callback.send(SaveChunkResult::Err(()));
                        return;
                    },
                };

                let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
                    Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
                    None => {
                        self.callback.send(SaveChunkResult::Err(()));
                        return;
                    },
                };

                for registered_chunk_actor in registered_chunk_actors {
                    let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                        Some(chunk_actor_entity) => *chunk_actor_entity,
                        None => {
                            self.callback.send(SaveChunkResult::Err(()));
                            return;
                        },
                    };

                    let mut chunk_actor_entity_raw = match world.get_entity_mut(chunk_actor_entity) {
                        Some(chunk_actor_raw) => chunk_actor_raw,
                        None => {
                            self.callback.send(SaveChunkResult::Err(()));
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
                self.callback.send(SaveChunkResult::Err(()));
            },
        }
    }
}

pub struct UnsaveChunkArgs {
    pub chunk_position: ChunkPosition,
}
impl OpArgs for UnsaveChunkArgs {}
pub enum UnsaveChunkResult {
    Ok(()),
    Err(()),
}
impl OpResult for UnsaveChunkResult {}
pub struct UnsaveChunk {
    args: UnsaveChunkArgs,
    callback: Option<oneshot::Sender<UnsaveChunkResult>>,
}
impl Operation for UnsaveChunk {
    type Args = UnsaveChunkArgs;
    type Result = UnsaveChunkResult;

    fn new(args: UnsaveChunkArgs, callback: oneshot::Sender<UnsaveChunkResult>) -> Self {
        Self {
            args,
            callback: Some(callback),
        }
    }
    
    fn execute(&mut self, world: &mut World) {
        let serialized_chunk_storage = match SERIALIZED_CHUNK_STORAGE.lock() {
            Ok(serialized_chunk_storage) => serialized_chunk_storage,
            Err(_) => {
                self.callback.send(UnsaveChunkResult::Err(()));
                return;
            },
        };

        if !serialized_chunk_storage.contains_key(&self.args.chunk_position) {
            self.callback.send(UnsaveChunkResult::Err(()));
            return;
        }
        
        match world.query::<(Entity, &Chunk)>().iter(world).find(|(_, chunk)| chunk.position() == self.args.chunk_position) {
            Some((chunk_entity, chunk)) => {
                let registered_chunk_actors = chunk.registered_chunk_actors().clone();

                let mut chunk_entity_raw = match world.get_entity_mut(chunk_entity) {
                    Some(chunk_raw) => chunk_raw,
                    None => {
                        self.callback.send(UnsaveChunkResult::Err(()));
                        return;
                    },
                };

                if chunk_entity_raw.contains::<Serialized>() {
                    chunk_entity_raw.remove::<Serialized>();
                }

                let mut main_type_registry = match MAIN_TYPE_REGISTRY.lock() {
                    Ok(main_type_registry) => main_type_registry,
                    Err(_) => {
                        self.callback.send(UnsaveChunkResult::Err(()));
                        return;
                    },
                };

                let chunk_actor_instance_registry = match main_type_registry.get_data_mut::<ChunkActor, ChunkActorInstanceRegistry>() {
                    Some(chunk_actor_instance_registry) => chunk_actor_instance_registry,
                    None => {
                        self.callback.send(UnsaveChunkResult::Err(()));
                        return;
                    },
                };

                for registered_chunk_actor in registered_chunk_actors {
                    let chunk_actor_entity = match chunk_actor_instance_registry.get(registered_chunk_actor) {
                        Some(chunk_actor_entity) => *chunk_actor_entity,
                        None => {
                            self.callback.send(UnsaveChunkResult::Err(()));
                            return;
                        },
                    };

                    let mut chunk_actor_entity_raw = match world.get_entity_mut(chunk_actor_entity) {
                        Some(chunk_actor_raw) => chunk_actor_raw,
                        None => {
                            self.callback.send(UnsaveChunkResult::Err(()));
                            return;
                        },
                    };

                    if chunk_actor_entity_raw.contains::<Serialized>() {
                        chunk_actor_entity_raw.remove::<Serialized>();
                    }
                }
            },
            None => {
                self.callback.send(UnsaveChunkResult::Err(()));
            },
        };
    }
}
