use crate::chunk::enums::ChunkAction;
use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChunkState {
    Absent,
    Unowned,
    Owned(Entity),
}

#[derive(Debug)]
pub enum ResolutionError {
    IntentBufferNotFlushedAtResolutionTime,
    CommittedRedundantActionIntent,
}

#[derive(Debug)]
pub enum ResolutionWarning {
    OwnershipTransferOfNonexistentChunk,
    RedundantActionIntent,
    IntentBufferUnavailable,
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
        (None, None, Absent, TransferOwnership { .. }) => DiscardIncoming(OwnershipTransferOfNonexistentChunk),

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

        (None, Some(_), _, _) => Error(IntentBufferNotFlushedAtResolutionTime),

        (Some(Spawn { .. }), None, Absent, Spawn { .. }) => DiscardIncoming(RedundantActionIntent),
        (Some(Spawn { .. }), None, Absent, incoming) if matches!(incoming, ChunkAction::Despawn { .. }) => PushBuffer(incoming),
        (Some(Spawn { .. }), None, Absent, incoming) if matches!(incoming, ChunkAction::TransferOwnership { .. }) => PushBuffer(incoming),

        // TODO: Make use of chunk_state
        (Some(Spawn { .. }), None, chunk_state, incoming) => match incoming {
            Spawn { .. } => DiscardIncoming(RedundantActionIntent),
            Despawn { .. } => PushBuffer(incoming),
            TransferOwnership { .. } => PushBuffer(incoming),
        },
        // TODO: Make use of chunk_state
        (Some(Despawn { .. }), None, chunk_state, incoming) => match incoming {
            Spawn { .. } => PushBuffer(incoming),
            Despawn { .. } => DiscardIncoming(RedundantActionIntent),
            TransferOwnership { .. } => PushBuffer(incoming),
        }
        (Some(TransferOwnership { new_owner: committed_owner, .. }), None, chunk_state, incoming) => {
            match chunk_state {
                Absent => {
                    // TODO: Implement
                },
                Unowned => {
                    // TODO: Implement
                },
                Owned(current_owner) => {
                    // TODO: Implement
                },
            }

            match incoming {
                Spawn { .. } => Error(CommittedRedundantActionIntent),
                Despawn { .. } => PushBuffer(incoming),
                TransferOwnership { new_owner: incoming_owner, .. } => {
                    if committed_owner == incoming_owner {
                        DiscardIncoming(RedundantActionIntent)
                    } else {
                        PushBuffer(incoming)
                    }
                }
            }
        }

        (Some(_), Some(_), _, _) => DiscardIncoming(IntentBufferUnavailable)
    }
}
