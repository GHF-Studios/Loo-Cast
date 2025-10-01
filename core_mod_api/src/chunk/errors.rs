use bevy::prelude::Reflect;

use crate::chunk::types::GridCoord;

#[derive(Debug, Reflect)]
pub enum SpawnError {
    AlreadySpawned { grid_coord: GridCoord },
    NotSpawning { grid_coord: GridCoord },
    AlreadyBeingDespawned { grid_coord: GridCoord },
    AlreadyTransferingOwnership { grid_coord: GridCoord },
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
    AlreadyDespawned { grid_coord: GridCoord },
    AlreadyBeingSpawned { grid_coord: GridCoord },
    NotDespawning { grid_coord: GridCoord },
    AlreadyTransferingOwnership { grid_coord: GridCoord },
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

#[derive(Debug, Reflect)]
pub enum TransferOwnershipError {
    AlreadyDespawned { grid_coord: GridCoord },
    AlreadyTransferedOwnership { grid_coord: GridCoord },
    AlreadyBeingDespawned { grid_coord: GridCoord },
    AlreadyBeingSpawned { grid_coord: GridCoord },
    NotTransferingOwnership { grid_coord: GridCoord },
}
impl std::fmt::Display for TransferOwnershipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferOwnershipError::AlreadyDespawned { grid_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it is already despawned", grid_coord)
            }
            TransferOwnershipError::AlreadyTransferedOwnership { grid_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it's ownership is already transfered", grid_coord)
            }
            TransferOwnershipError::AlreadyBeingSpawned { grid_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it is already being spawned", grid_coord)
            }
            TransferOwnershipError::AlreadyBeingDespawned { grid_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it is already being despawned", grid_coord)
            }
            TransferOwnershipError::NotTransferingOwnership { grid_coord } => {
                write!(
                    f,
                    "Cannot transfer ownership of chunk {:?}: it's ownership is not marked as being transfered",
                    grid_coord
                )
            }
        }
    }
}
