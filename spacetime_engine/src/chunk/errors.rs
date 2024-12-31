#[derive(Debug)]
pub enum SpawnError {
    AlreadyBeingDespawned { chunk_coord: (i32, i32) },
    AlreadyBeingSpawned { chunk_coord: (i32, i32) },
}

impl std::fmt::Display for SpawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpawnError::AlreadyBeingDespawned { chunk_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is already being despawned", chunk_coord)
            }
            SpawnError::AlreadyBeingSpawned { chunk_coord } => {
                write!(f, "Cannot spawn chunk {:?}: it is already being spawned", chunk_coord)
            }
        }
    }
}

impl std::error::Error for SpawnError {}

#[derive(Debug)]
pub enum DespawnError {
    StillBeingSpawned { chunk_coord: (i32, i32) },
    AlreadyBeingDespawned { chunk_coord: (i32, i32) },
}

impl std::fmt::Display for DespawnError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DespawnError::StillBeingSpawned { chunk_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is still being spawned", chunk_coord)
            }
            DespawnError::AlreadyBeingDespawned { chunk_coord } => {
                write!(f, "Cannot despawn chunk {:?}: it is already being despawned", chunk_coord)
            }
        }
    }
}

impl std::error::Error for DespawnError {}