// Modules
pub mod data;
pub mod id;
pub mod metadata;
pub mod pos;

// Local imports
use data::*;
use id::*;
use metadata::*;
use pos::*;

// Internal imports
use crate::game::SimulationState;
use crate::AppState;
use crate::universe::chunk::*;

// External imports
use bevy::prelude::*;

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
}

#[derive(Debug)]
pub enum RegisterEntityError {
}

#[derive(Debug)]
pub enum UnregisterEntityError {
}

#[derive(Debug)]
pub enum LoadEntityMetadataError {
}

#[derive(Debug)]
pub enum UnloadEntityMetadataError {
}

#[derive(Debug)]
pub enum LoadEntityDataError {
}

#[derive(Debug)]
pub enum UnloadEntityDataError {
}

#[derive(Debug)]
pub enum SpawnEntityError {
}

#[derive(Debug)]
pub enum DespawnEntityError {
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

// Implementations
impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
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
        Entity::Registered {
            id,
        }
    }
}

// Module Functions
