// Modules
pub mod data;
pub mod id;
pub mod metadata;
pub mod pos;

// Local imports
use data::*;
use id::*;
use metadata::*;

// Internal imports
use crate::engine::system::universe::chunk::*;

// External imports
use bevy::prelude::*;
use bevy::ecs::system::EntityCommands;
use std::sync::{Arc, Mutex};

// Static variables

// Constant variables

// Types

// Enums
#[derive(Debug)]
pub enum Entity {
    Registered {
        id: EntityID,
        bevy_entity: bevy::ecs::entity::Entity,
    },
    MetadataLoaded {
        id: EntityID,
        bevy_entity: bevy::ecs::entity::Entity,
        metadata: EntityMetadata,
    },
    DataLoaded {
        id: EntityID,
        bevy_entity: bevy::ecs::entity::Entity,
        metadata: EntityMetadata,
        data: EntityData,
    },
}

pub enum EntityOperation {
    Register {
        parent_chunk: Arc<Mutex<Chunk>>,
        local_entity_id: LocalEntityID,
        success_callback: Box<dyn FnOnce(RegisterEntitySuccess) + Send>,
        failure_callback: Box<dyn FnOnce(RegisterEntityError) + Send>,
    },
    Unregister {
        parent_chunk: Arc<Mutex<Chunk>>,
        entity: Arc<Mutex<Entity>>,
        success_callback: Box<dyn FnOnce(UnregisterEntitySuccess) + Send>,
        failure_callback: Box<dyn FnOnce(UnregisterEntityError) + Send>,
    },
    LoadMetadata {
        entity: Arc<Mutex<Entity>>,
        metadata: EntityMetadata,
        success_callback: Box<dyn FnOnce(LoadEntityMetadataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(LoadEntityMetadataError) + Send>,
    },
    UnloadMetadata {
        entity: Arc<Mutex<Entity>>,
        success_callback: Box<dyn FnOnce(UnloadEntityMetadataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(UnloadEntityMetadataError) + Send>,
    },
    LoadData {
        entity: Arc<Mutex<Entity>>,
        data: EntityData,
        success_callback: Box<dyn FnOnce(LoadEntityDataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(LoadEntityDataError) + Send>,
    },
    UnloadData {
        entity: Arc<Mutex<Entity>>,
        success_callback: Box<dyn FnOnce(UnloadEntityDataSuccess) + Send>,
        failure_callback: Box<dyn FnOnce(UnloadEntityDataError) + Send>,
    },
    Spawn {
        parent_chunk: Arc<Mutex<Chunk>>,
        entity: Arc<Mutex<Entity>>,
        success_callback: Box<dyn FnOnce(SpawnEntitySuccess) + Send>,
        failure_callback: Box<dyn FnOnce(SpawnEntityError) + Send>,
    },
    Despawn {
        entity: Arc<Mutex<Entity>>,
        success_callback: Box<dyn FnOnce(DespawnEntitySuccess) + Send>,
        failure_callback: Box<dyn FnOnce(DespawnEntityError) + Send>,
    },
    Command {
        entity_commands: Box<dyn FnOnce(EntityCommands) + Send>,
        entity: Arc<Mutex<Entity>>,
        success_callback: Box<dyn FnOnce(CommandEntitySuccess) + Send>,
        failure_callback: Box<dyn FnOnce(CommandEntityError) + Send>,
    },
}

#[derive(Debug)]
pub enum RegisterEntityError {
    ParentChunkDataNotLoaded,
    EntityAlreadyRegistered,
}

#[derive(Debug)]
pub enum UnregisterEntityError {
    ParentChunkDataNotLoaded,
    EntityMetadataStillLoaded,
    EntityDataStillLoaded,
    EntityAlreadyUnregistered,
}

#[derive(Debug)]
pub enum LoadEntityMetadataError {
    EntityMetadataAlreadyLoaded,
}

#[derive(Debug)]
pub enum UnloadEntityMetadataError {
    EntityMetadataAlreadyUnloaded,
    EntityDataStillLoaded,
}

#[derive(Debug)]
pub enum LoadEntityDataError {
    EntityMetadataNotLoaded,
    EntityDataAlreadyLoaded,
}

#[derive(Debug)]
pub enum UnloadEntityDataError {
    EntityDataAlreadyUnloaded,
    EntityStillSpawned,
}

#[derive(Debug)]
pub enum SpawnEntityError {
    ParentChunkDataNotLoaded,
    ParentChunkNotSpawned,
    EntityDataNotLoaded,
    EntityAlreadySpawned,
    WrongParentChunk
}

#[derive(Debug)]
pub enum DespawnEntityError {
    EntityDataNotLoaded,
    EntityAlreadyDespawned,
}

#[derive(Debug)]
pub enum CommandEntityError {
    EntityDataNotLoaded,
    EntityNotSpawned,
}

// Structs
pub struct EntityPlugin;

pub struct EntityOperationRequest {
    pub(in crate::engine::system::universe) operations: Vec<EntityOperation>,
}

#[derive(Debug)]
pub struct RegisterEntitySuccess;

#[derive(Debug)]
pub struct UnregisterEntitySuccess;

#[derive(Debug)]
pub struct LoadEntityMetadataSuccess;

#[derive(Debug)]
pub struct UnloadEntityMetadataSuccess;

#[derive(Debug)]
pub struct LoadEntityDataSuccess;

#[derive(Debug)]
pub struct UnloadEntityDataSuccess;

#[derive(Debug)]
pub struct SpawnEntitySuccess;

#[derive(Debug)]
pub struct DespawnEntitySuccess;

#[derive(Debug)]
pub struct CommandEntitySuccess;

#[derive(Component)]
pub struct EntityBevyComponent {
    pub entity: Arc<Mutex<Entity>>,
}

// Implementations
impl Plugin for EntityPlugin {
    fn build(&self, _app: &mut App) {}
}

impl EntityOperationRequest {
    pub fn new(operations: Vec<EntityOperation>) -> Self {
        EntityOperationRequest { operations }
    }
}

impl Default for Entity {
    fn default() -> Self {
        Entity::Registered {
            id: EntityID::default(),
            bevy_entity: bevy::ecs::entity::Entity::PLACEHOLDER
        }
    }
}

impl Entity {
    pub(in crate::engine::system::universe) fn new(id: EntityID, bevy_entity: bevy::ecs::entity::Entity) -> Self {
        Entity::Registered { id, bevy_entity }
    }
}

// Module Functions
