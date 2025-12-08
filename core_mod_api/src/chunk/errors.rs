use bevy::prelude::Reflect;

use crate::usf::pos::grid::types::GridVec;

#[derive(Debug, Reflect)]
pub enum SpawnError {
    AlreadySpawned { grid_coord: GridVec },
    NotSpawning { grid_coord: GridVec },
    AlreadyBeingDespawned { grid_coord: GridVec },
    AlreadyTransferingOwnership { grid_coord: GridVec },
}
impl std::fmt::Display for SpawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpawnError::AlreadySpawned { grid_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is already spawned", grid_coord)
            }
            SpawnError::NotSpawning { grid_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is not marked as being spawned", grid_coord)
            }
            SpawnError::AlreadyBeingDespawned { grid_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is already being despawned", grid_coord)
            }
            SpawnError::AlreadyTransferingOwnership { grid_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it's ownership is already being transfered", grid_coord)
            }
        }
    }
}
impl std::error::Error for SpawnError {}

#[derive(Debug, Reflect)]
pub enum DespawnError {
    AlreadyDespawned { grid_coord: GridVec },
    AlreadyBeingSpawned { grid_coord: GridVec },
    NotDespawning { grid_coord: GridVec },
    AlreadyTransferingOwnership { grid_coord: GridVec },
}
impl std::fmt::Display for DespawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DespawnError::AlreadyDespawned { grid_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is already despawned", grid_coord)
            }
            DespawnError::AlreadyBeingSpawned { grid_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is already being spawned", grid_coord)
            }
            DespawnError::NotDespawning { grid_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is not marked as being despawned", grid_coord)
            }
            DespawnError::AlreadyTransferingOwnership { grid_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it's ownership is already being transfered", grid_coord)
            }
        }
    }
}
impl std::error::Error for DespawnError {}
