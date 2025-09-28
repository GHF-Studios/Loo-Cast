use bevy::prelude::Reflect;

use crate::chunk::types::ChunkCoord;

#[derive(Debug, Reflect)]
pub enum SpawnError {
    AlreadySpawned { chunk_coord: ChunkCoord },
    NotSpawning { chunk_coord: ChunkCoord },
    AlreadyBeingDespawned { chunk_coord: ChunkCoord },
    AlreadyTransferingOwnership { chunk_coord: ChunkCoord },
}
impl std::fmt::Display for SpawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpawnError::AlreadySpawned { chunk_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is already spawned", chunk_coord)
            }
            SpawnError::NotSpawning { chunk_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is not marked as being spawned", chunk_coord)
            }
            SpawnError::AlreadyBeingDespawned { chunk_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is already being despawned", chunk_coord)
            }
            SpawnError::AlreadyTransferingOwnership { chunk_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it's ownership is already being transfered", chunk_coord)
            }
        }
    }
}
impl std::error::Error for SpawnError {}

#[derive(Debug, Reflect)]
pub enum DespawnError {
    AlreadyDespawned { chunk_coord: ChunkCoord },
    AlreadyBeingSpawned { chunk_coord: ChunkCoord },
    NotDespawning { chunk_coord: ChunkCoord },
    AlreadyTransferingOwnership { chunk_coord: ChunkCoord },
}
impl std::fmt::Display for DespawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DespawnError::AlreadyDespawned { chunk_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is already despawned", chunk_coord)
            }
            DespawnError::AlreadyBeingSpawned { chunk_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is already being spawned", chunk_coord)
            }
            DespawnError::NotDespawning { chunk_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is not marked as being despawned", chunk_coord)
            }
            DespawnError::AlreadyTransferingOwnership { chunk_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it's ownership is already being transfered", chunk_coord)
            }
        }
    }
}
impl std::error::Error for DespawnError {}

#[derive(Debug, Reflect)]
pub enum TransferOwnershipError {
    AlreadyDespawned { chunk_coord: ChunkCoord },
    AlreadyTransferedOwnership { chunk_coord: ChunkCoord },
    AlreadyBeingDespawned { chunk_coord: ChunkCoord },
    AlreadyBeingSpawned { chunk_coord: ChunkCoord },
    NotTransferingOwnership { chunk_coord: ChunkCoord },
}
impl std::fmt::Display for TransferOwnershipError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransferOwnershipError::AlreadyDespawned { chunk_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it is already despawned", chunk_coord)
            }
            TransferOwnershipError::AlreadyTransferedOwnership { chunk_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it's ownership is already transfered", chunk_coord)
            }
            TransferOwnershipError::AlreadyBeingSpawned { chunk_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it is already being spawned", chunk_coord)
            }
            TransferOwnershipError::AlreadyBeingDespawned { chunk_coord } => {
                write!(f, "Cannot transfer ownership of chunk {:?}: it is already being despawned", chunk_coord)
            }
            TransferOwnershipError::NotTransferingOwnership { chunk_coord } => {
                write!(
                    f,
                    "Cannot transfer ownership of chunk {:?}: it's ownership is not marked as being transfered",
                    chunk_coord
                )
            }
        }
    }
}
