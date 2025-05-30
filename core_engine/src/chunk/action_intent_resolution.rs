use std::f64::consts::E;

use crate::chunk::enums::ChunkAction;
use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Absent,
    Unowned,
    Owned(Entity),
}
impl ChunkState {
    pub fn current_owner(&self) -> Option<Entity> {
        match self {
            ChunkState::Absent | ChunkState::Unowned => None,
            ChunkState::Owned(owner) => Some(*owner),
        }
    }
}

#[derive(Debug)]
pub enum ResolutionError {
    IntentBufferNotFlushed,
    CommittedInvalidActionIntent,
}

#[derive(Debug)]
pub enum ResolutionWarning {
    OwnershipTransferIntentOfNonexistentChunk,
    RedundantActionIntent,
    IntentBufferUnavailable,
    DespawnIntentAfterCommittingToOwnershipTransfer,
    IntentWithoutOwnership
}

#[derive(Debug)]
pub enum Resolution {
    ReplaceIntent(ChunkAction),
    KeepExistingIntent,
    CancelIntent,
    DiscardIncoming(ResolutionWarning),
    PushCommit(ChunkAction),
    PushBuffer(ChunkAction),
    Error(ResolutionError),
}

pub fn resolve_chunk_intent(committed: Option<ChunkAction>, buffered: Option<ChunkAction>, chunk_state: ChunkState, incoming: ChunkAction) -> Resolution {
    use ChunkAction::*;
    use ChunkState::*;
    use Resolution::*;
    use ResolutionError::*;
    use ResolutionWarning::*;

    match (committed, buffered, chunk_state, incoming.clone()) {
        (None, None, Absent, Spawn { .. }) => PushCommit(incoming),
        (None, None, Absent, Despawn { .. }) => DiscardIncoming(RedundantActionIntent),
        (None, None, Absent, TransferOwnership { .. }) => DiscardIncoming(OwnershipTransferIntentOfNonexistentChunk),

        (None, None, Unowned, Spawn { .. }) => DiscardIncoming(RedundantActionIntent),
        (None, None, Unowned, Despawn { .. }) => PushCommit(incoming),
        (None, None, Unowned, TransferOwnership { .. }) => PushCommit(incoming),

        (None, None, Owned(_), Spawn { .. }) => DiscardIncoming(RedundantActionIntent),
        (None, None, Owned(_), Despawn { .. }) => PushCommit(incoming),
        (None, None, Owned(owner), TransferOwnership { new_owner, .. }) => {
            if owner == new_owner {
                DiscardIncoming(RedundantActionIntent)
            } else {
                PushCommit(incoming)
            }
        },

        (None, Some(_), _, _) => Error(IntentBufferNotFlushed),

        (Some(Spawn { .. }), None, chunk_state, incoming) => {
            if !matches!(chunk_state, ChunkState::Absent) {
                return Error(CommittedInvalidActionIntent);
            }

            // TODO: Add checks for Despawn and TransferOwnership
            match incoming {
                Spawn { .. } => DiscardIncoming(RedundantActionIntent),
                Despawn { .. } => {
                    // TODO: Stub
                    PushBuffer(incoming)
                },
                TransferOwnership { .. } => {
                    // TODO: Stub
                    PushBuffer(incoming)
                },
            }
        },
        (Some(Despawn { .. }), None, chunk_state, incoming) => {
            if matches!(chunk_state, ChunkState::Absent) {
                return Error(CommittedInvalidActionIntent);
            }

            // TODO: Add checks for Spawn and TransferOwnership
            match incoming {
                Spawn { .. } => {
                    // TODO: Stub
                    PushBuffer(incoming)
                },
                Despawn { .. } => DiscardIncoming(RedundantActionIntent),
                TransferOwnership { .. } => {
                    // TODO: Stub
                    PushBuffer(incoming)
                },
            }
        }
        (Some(TransferOwnership { new_owner: committed_owner, .. }), None, chunk_state, incoming) => {
            let current_owner = match chunk_state {
                ChunkState::Absent => {
                    return Error(CommittedInvalidActionIntent);
                },
                ChunkState::Unowned => None,
                ChunkState::Owned(current_owner) => Some(current_owner),
            };

            let current_requester_id = current_owner.map_or(0, |owner| owner.id()); // Assuming Entity has an id() method

            // TODO: Add checks for Despawn and TransferOwnership
            match incoming {
                Spawn { .. } => DiscardIncoming(RedundantActionIntent),
                Despawn { requester_id: incoming_requester_id, .. } => {
                    let incoming_owner = incoming_requester_id.entity(); // Assuming u32 has an entity() method
                    let incoming_owner_is_current_owner = incoming_requester_id == current_requester_id;
                    let incoming_owner_is_committed_owner = incoming_owner == committed_owner;

                    if incoming_owner_is_current_owner {
                        DiscardIncoming(DespawnIntentAfterCommittingToOwnershipTransfer)
                    } else if incoming_owner_is_committed_owner {
                        PushBuffer(incoming)
                    } else {
                        DiscardIncoming(IntentWithoutOwnership)
                    }
                },
                TransferOwnership { new_owner: incoming_owner, .. } => {
                    
                }
            }
        }

        (Some(_), Some(_), _, _) => DiscardIncoming(IntentBufferUnavailable)
    }
}
