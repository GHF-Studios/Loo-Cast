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
    },
    MetadataLoaded {
        id: EntityID,
        metadata: EntityMetadata,
    },
    DataLoaded {
        id: EntityID,
        metadata: EntityMetadata,
        data: EntityData,
    },
}

pub enum EntityLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

pub enum EntityOperation {
    Register {
        id: EntityID,
        success_callback: Box<dyn Fn(RegisterEntitySuccess) + Send>,
        failure_callback: Box<dyn Fn(RegisterEntityError, EntityID) + Send>,
    },
    Unregister {
        id: EntityID,
        success_callback: Box<dyn Fn(UnregisterEntitySuccess) + Send>,
        failure_callback: Box<dyn Fn(UnregisterEntityError, EntityID) + Send>,
    },
    LoadMetadata {
        id: EntityID,
        metadata: EntityMetadata,
        success_callback: Box<dyn Fn(LoadEntityMetadataSuccess) + Send>,
        failure_callback: Box<dyn Fn(LoadEntityMetadataError, EntityID, EntityMetadata) + Send>,
    },
    UnloadMetadata {
        id: EntityID,
        success_callback: Box<dyn Fn(UnloadEntityMetadataSuccess) + Send>,
        failure_callback: Box<dyn Fn(UnloadEntityMetadataError, EntityID) + Send>,
    },
    LoadData {
        id: EntityID,
        data: EntityData,
        success_callback: Box<dyn Fn(LoadEntityDataSuccess) + Send>,
        failure_callback: Box<dyn Fn(LoadEntityDataError, EntityID, EntityData) + Send>,
    },
    UnloadData {
        id: EntityID,
        success_callback: Box<dyn Fn(UnloadEntityDataSuccess) + Send>,
        failure_callback: Box<dyn Fn(UnloadEntityDataError, EntityID) + Send>,
    },
    Spawn {
        id: EntityID,
        success_callback: Box<dyn Fn(SpawnEntitySuccess) + Send>,
        failure_callback: Box<dyn Fn(SpawnEntityError, EntityID) + Send>,
    },
    Despawn {
        id: EntityID,
        success_callback: Box<dyn Fn(DespawnEntitySuccess) + Send>,
        failure_callback: Box<dyn Fn(DespawnEntityError, EntityID) + Send>,
    },
    Command {
        id: EntityID,
        command: Box<dyn FnOnce(&mut EntityCommands) + Send>,
        success_callback: Box<dyn Fn(CommandEntitySuccess) + Send>,
        failure_callback: Box<dyn Fn(CommandEntityError, EntityID) + Send>,
    },
}

#[derive(Debug)]
pub enum RegisterEntityError {
    ParentChunkMutexPoisoned,

    ParentChunkNotRegistered,
    ParentChunkMetadataNotLoaded,
    EntityAlreadyRegistered,

    FailedToGetParentChunk,
}

#[derive(Debug)]
pub enum UnregisterEntityError {
    ParentChunkMutexPoisoned,

    ParentChunkNotRegistered,
    ParentChunkMetadataNotLoaded,
    EntityAlreadyUnregistered,

    FailedToGetParentChunk,
}

#[derive(Debug)]
pub enum LoadEntityMetadataError {
    ParentChunkMutexPoisoned,
    EntityMutexPoisoned,

    ParentChunkNotRegistered,
    EntityNotRegistered,
    EntityMetadataAlreadyLoaded,

    FailedToGetParentChunk,
    FailedToGetEntity,
    FatalUnexpectedError,
}

#[derive(Debug)]
pub enum UnloadEntityMetadataError {
    ParentChunkMutexPoisoned,
    EntityMutexPoisoned,

    ParentChunkNotRegistered,
    EntityNotRegistered,
    EntityMetadataAlreadyUnloaded,
    EntityDataStillLoaded,

    FailedToGetParentChunk,
    FailedToGetEntity,
}

#[derive(Debug)]
pub enum LoadEntityDataError {
    ParentChunkMutexPoisoned,
    EntityMutexPoisoned,

    ParentChunkNotRegistered,
    ParentChunkDataNotLoaded,
    EntityNotRegistered,
    EntityMetadataNotLoaded,
    EntityDataAlreadyLoaded,

    FailedToGetParentChunk,
    FailedToGetEntity,
}

#[derive(Debug)]
pub enum UnloadEntityDataError {
    ParentChunkMutexPoisoned,
    EntityMutexPoisoned,

    ParentChunkNotRegistered,
    EntityNotRegistered,
    EntityDataAlreadyUnloaded,

    FailedToGetParentChunk,
    FailedToGetEntity,
}

#[derive(Debug)]
pub enum SpawnEntityError {
    ParentChunkMutexPoisoned,
    EntityMutexPoisoned,

    ParentChunkNotRegistered,
    ParentChunkDataNotLoaded,
    ParentChunkNotSpawned,
    EntityNotRegistered,
    EntityDataNotLoaded,
    EntityAlreadySpawned,

    FailedToGetParentChunk,
    FailedToGetEntity,
}

#[derive(Debug)]
pub enum DespawnEntityError {
    ParentChunkMutexPoisoned,
    EntityMutexPoisoned,

    ParentChunkNotRegistered,
    EntityNotRegistered,
    EntityDataNotLoaded,
    EntityAlreadyDespawned,

    FailedToGetParentChunk,
    FailedToGetEntity,
}

#[derive(Debug)]
pub enum CommandEntityError {
    ParentChunkMutexPoisoned,
    EntityMutexPoisoned,

    ParentChunkNotRegistered,
    EntityNotRegistered,
    EntityDataNotLoaded,
    EntityNotSpawned,
    
    FailedToGetParentChunk,
    FailedToGetEntity,
}

// Structs
pub struct EntityPlugin;

pub struct EntityOperationRequest {
    pub(in crate::universe) operations: Vec<EntityOperation>,
}

pub struct RegisterEntitySuccess;
pub struct UnregisterEntitySuccess;
pub struct LoadEntityMetadataSuccess;
pub struct UnloadEntityMetadataSuccess;
pub struct LoadEntityDataSuccess;
pub struct UnloadEntityDataSuccess;
pub struct SpawnEntitySuccess;
pub struct DespawnEntitySuccess;
pub struct CommandEntitySuccess;

#[derive(Component)]
pub struct EntityBevyComponent {
    pub entity: Arc<Mutex<Entity>>,
}

// Implementations
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {}
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
        }
    }
}

impl Entity {
    pub(in crate::universe) fn new(id: EntityID) -> Self {
        Entity::Registered { id }
    }
}

// Module Functions
