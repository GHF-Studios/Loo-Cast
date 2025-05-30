use crate::{chunk::enums::ChunkAction, chunk_loader::components::ChunkLoaderComponent};
use bevy::prelude::*;

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
    CommittedInvalidIntent,
    CurrentOwnerNotFoundInQuery,
}

#[derive(Debug)]
pub enum ResolutionWarning {
    OwnershipTransferIntentOfNonexistentChunk,
    RedundantIntent,
    IntentBufferUnavailable,
    IntentAfterCommittingToOwnershipTransfer,
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

#[allow(clippy::enum_variant_names)]
enum IncomingOwnership {
    IsCurrent,
    IsCommitted,
    IsUnrelated
}

pub fn resolve_chunk_intent(
    committed: Option<ChunkAction>, 
    buffered: Option<ChunkAction>, 
    chunk_state: ChunkState, 
    incoming: ChunkAction,
    chunk_loader_query: &Query<(Entity, &ChunkLoaderComponent)>
) -> Resolution {
    use ChunkAction::*;
    use ChunkState::*;
    use Resolution::*;
    use ResolutionError::*;
    use ResolutionWarning::*;
    use IncomingOwnership::*;

    match (committed, buffered, chunk_state, incoming.clone()) {
        (None, None, Absent, Spawn { .. }) => PushCommit(incoming),
        (None, None, Absent, Despawn { .. }) => DiscardIncoming(RedundantIntent),
        (None, None, Absent, TransferOwnership { .. }) => DiscardIncoming(OwnershipTransferIntentOfNonexistentChunk),

        (None, None, Unowned, Spawn { .. }) => DiscardIncoming(RedundantIntent),
        (None, None, Unowned, Despawn { .. }) => PushCommit(incoming),
        (None, None, Unowned, TransferOwnership { .. }) => PushCommit(incoming),

        (None, None, Owned(_), Spawn { .. }) => DiscardIncoming(RedundantIntent),
        (None, None, Owned(_), Despawn { .. }) => PushCommit(incoming),
        (None, None, Owned(owner), TransferOwnership { new_owner, .. }) => {
            if owner == new_owner {
                DiscardIncoming(RedundantIntent)
            } else {
                PushCommit(incoming)
            }
        },

        (None, Some(_), _, _) => Error(IntentBufferNotFlushed),

        // TODO: Completely rework this case
        (Some(Spawn { .. }), None, chunk_state, incoming) => {
            if !matches!(chunk_state, ChunkState::Absent) {
                return Error(CommittedInvalidIntent);
            }

            match incoming {
                Spawn { .. } => DiscardIncoming(RedundantIntent),
                Despawn { .. } => {
                    PushBuffer(incoming)
                },
                TransferOwnership { .. } => {
                    PushBuffer(incoming)
                },
            }
        },
        // TODO: Completely rework this case
        (Some(Despawn { .. }), None, chunk_state, incoming) => {
            if matches!(chunk_state, ChunkState::Absent) {
                return Error(CommittedInvalidIntent);
            }

            match incoming {
                Spawn { .. } => {
                    PushBuffer(incoming)
                },
                Despawn { .. } => DiscardIncoming(RedundantIntent),
                TransferOwnership { .. } => {
                    PushBuffer(incoming)
                },
            }
        }
        // Completely reworked this case already!
        (Some(TransferOwnership { requester_id: committed_requester_id, .. }), None, chunk_state, incoming) => {
            let (current_owner, incoming_owner_is_current_owner) = match chunk_state {
                ChunkState::Absent => {
                    return Error(CommittedInvalidIntent);
                },
                ChunkState::Unowned => (None, false),
                ChunkState::Owned(current_owner) => {
                    let value = chunk_loader_query
                        .iter()
                        .find(|(entity, _)| *entity == current_owner);

                    let (current_owner, current_requester_id) = if let Some((entity, loader)) = value {
                        (entity, loader.id)
                    } else { 
                        return Error(CurrentOwnerNotFoundInQuery)
                    };

                    if current_requester_id != incoming.requester_id() {
                        if committed_requester_id != incoming.requester_id() {
                            return DiscardIncoming(IntentWithoutOwnership);
                        } else {
                            (Some(current_owner), false)
                        }
                    } else {
                        (Some(current_owner), true)
                    }
                },
            };

            let incoming_ownership = if incoming_owner_is_current_owner {
                IsCurrent
            } else if current_owner.is_some() {
                IsCommitted
            } else {
                IsUnrelated
            };

            match incoming {
                Spawn { .. } => DiscardIncoming(RedundantIntent),
                Despawn { .. } => match incoming_ownership {
                    IsCurrent => DiscardIncoming(IntentAfterCommittingToOwnershipTransfer),
                    IsCommitted => PushBuffer(incoming),
                    IsUnrelated => DiscardIncoming(IntentWithoutOwnership),
                },
                TransferOwnership { .. } => match incoming_ownership {
                    IsCurrent => DiscardIncoming(RedundantIntent),
                    IsCommitted => PushBuffer(incoming),
                    IsUnrelated => DiscardIncoming(IntentWithoutOwnership),
                }
            }
        }

        (Some(_), Some(_), _, _) => DiscardIncoming(IntentBufferUnavailable)
    }
}
