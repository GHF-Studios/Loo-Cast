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
    IntentWithoutOwnership,
    OwnershipTransferItentOfDespawningChunk,
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

pub fn resolve_intent(
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

        (Some(Spawn { requester_id: committed_id, .. }), None, _, incoming) => {
            match incoming {
                Spawn { requester_id, .. } => {
                    if requester_id == committed_id {
                        DiscardIncoming(RedundantIntent)
                    } else {
                        DiscardIncoming(IntentWithoutOwnership)
                    }
                }
                Despawn { requester_id, .. } => {
                    if requester_id == committed_id {
                        PushBuffer(incoming)
                    } else {
                        DiscardIncoming(IntentWithoutOwnership)
                    }
                }
                TransferOwnership { requester_id, .. } => {
                    if requester_id == committed_id {
                        PushBuffer(incoming)
                    } else {
                        DiscardIncoming(IntentWithoutOwnership)
                    }
                }
            }
        }
        (Some(Despawn { requester_id: committed_id, .. }), None, _, incoming) => {
            match incoming {
                Spawn { requester_id, .. } => {
                    if requester_id == committed_id {
                        PushBuffer(incoming)
                    } else {
                        DiscardIncoming(IntentWithoutOwnership)
                    }
                }
                Despawn { requester_id, .. } => {
                    if requester_id == committed_id {
                        DiscardIncoming(RedundantIntent)
                    } else {
                        DiscardIncoming(IntentWithoutOwnership)
                    }
                }
                TransferOwnership { requester_id, .. } => {
                    if requester_id == committed_id {
                        DiscardIncoming(OwnershipTransferItentOfDespawningChunk)
                    } else {
                        DiscardIncoming(IntentWithoutOwnership)
                    }
                }
            }
        }
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

            match incoming_ownership {
                IsCurrent => match incoming {
                    Spawn { .. } => DiscardIncoming(RedundantIntent),
                    Despawn { .. } => DiscardIncoming(IntentAfterCommittingToOwnershipTransfer),
                    TransferOwnership { .. } => DiscardIncoming(RedundantIntent),
                },
                IsCommitted => match incoming {
                    Spawn { .. } => DiscardIncoming(RedundantIntent),
                    Despawn { .. } | TransferOwnership { .. } => PushBuffer(incoming),
                },
                IsUnrelated => DiscardIncoming(IntentWithoutOwnership),
            }
        }

        (
            Some(Spawn { requester_id: committed_id, .. }), 
            Some(Despawn { requester_id: buffered_id, .. }), 
            _, 
            Spawn { requester_id: incoming_id, .. }
        ) if committed_id == buffered_id && committed_id == incoming_id => CancelIntent,

        (
            Some(Despawn { requester_id: committed_id, .. }), 
            Some(Despawn { requester_id: buffered_id, .. }), 
            _, 
            Despawn { requester_id: incoming_id, .. }
        ) if committed_id == buffered_id && committed_id == incoming_id => CancelIntent,

        (
            Some(TransferOwnership { requester_id: committed_id, new_owner: committed_owner, .. }), 
            Some(TransferOwnership { requester_id: buffered_id, new_owner: buffered_owner, .. }), 
            _, 
            TransferOwnership { requester_id: incoming_id, new_owner: incoming_owner, .. }
        ) if committed_id ==  => CancelIntent,

        (Some(_), Some(_), _, _) => DiscardIncoming(IntentBufferUnavailable)
    }
}
