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

pub enum EntityLoadState {
    Registered,
    MetadataLoaded,
    DataLoaded,
}

pub enum EntityOperation {
    Register {
        id: EntityID,
        success_callback: Box<dyn Fn(RegisterEntitySuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(RegisterEntityError, EntityID) + Send>,
    },
    Unregister {
        id: EntityID,
        success_callback: Box<dyn Fn(UnregisterEntitySuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(UnregisterEntityError, EntityID) + Send>,
    },
    LoadMetadata {
        id: EntityID,
        metadata: EntityMetadata,
        success_callback: Box<dyn Fn(LoadEntityMetadataSuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(LoadEntityMetadataError, EntityID, EntityMetadata) + Send>,
    },
    UnloadMetadata {
        id: EntityID,
        success_callback: Box<dyn Fn(UnloadEntityMetadataSuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(UnloadEntityMetadataError, EntityID) + Send>,
    },
    LoadData {
        id: EntityID,
        data: EntityData,
        success_callback: Box<dyn Fn(LoadEntityDataSuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(LoadEntityDataError, EntityID, EntityData) + Send>,
    },
    UnloadData {
        id: EntityID,
        success_callback: Box<dyn Fn(UnloadEntityDataSuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(UnloadEntityDataError, EntityID) + Send>,
    },
    Spawn {
        id: EntityID,
        success_callback: Box<dyn Fn(SpawnEntitySuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(SpawnEntityError, EntityID) + Send>,
    },
    Despawn {
        id: EntityID,
        success_callback: Box<dyn Fn(DespawnEntitySuccess, EntityID) + Send>,
        failure_callback: Box<dyn Fn(DespawnEntityError, EntityID) + Send>,
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
    EntityMutexPoisoned,

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

// Structs
pub struct EntityPlugin;

pub struct EntityOperationRequest {
    pub(in crate::engine::kernel::universe) operations: Vec<EntityOperation>,
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
            bevy_entity: bevy::ecs::entity::Entity::PLACEHOLDER
        }
    }
}

impl Entity {
    pub(in crate::engine::kernel::universe) fn new(id: EntityID, bevy_entity: bevy::ecs::entity::Entity) -> Self {
        Entity::Registered { id, bevy_entity }
    }
}

// Module Functions
