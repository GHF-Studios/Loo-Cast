use bevy::prelude::debug;
use super::types::ChunkOwnerId;

#[derive(Default, Debug, Clone, PartialEq, Eq)]
pub enum State {
    #[default]
    Absent,
    Owned(ChunkOwnerId),
}
impl State {
    pub fn owner_id(&self) -> Option<ChunkOwnerId> {
        match self {
            State::Absent => None,
            State::Owned(owner_id) => Some(owner_id.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionIntent {
    Spawn {
        owner_id: ChunkOwnerId,
        coord: (i32, i32),
        priority: ActionPriority,
    },
    Despawn {
        owner_id: ChunkOwnerId,
        coord: (i32, i32),
        priority: ActionPriority,
    },
    TransferOwnership {
        new_owner_id: ChunkOwnerId,
        coord: (i32, i32),
        priority: ActionPriority,
    },
}
impl ActionIntent {
    pub fn is_spawn(&self) -> bool {
        matches!(self, ActionIntent::Spawn { .. })
    }

    pub fn is_despawn(&self) -> bool {
        matches!(self, ActionIntent::Despawn { .. })
    }

    pub fn is_transfer_ownership(&self) -> bool {
        matches!(self, ActionIntent::TransferOwnership { .. })
    }

    pub fn owner_id(&self) -> ChunkOwnerId {
        match self {
            ActionIntent::Spawn { owner_id, .. } | ActionIntent::Despawn { owner_id, .. } | ActionIntent::TransferOwnership { new_owner_id: owner_id, .. } => owner_id.clone(),
        }
    }

    pub fn coord(&self) -> (i32, i32) {
        match self {
            ActionIntent::Spawn { coord, .. } | ActionIntent::Despawn { coord, .. } | ActionIntent::TransferOwnership { coord, .. } => *coord,
        }
    }

    pub fn priority(&self) -> ActionPriority {
        match self {
            ActionIntent::Spawn { priority, .. } => *priority,
            ActionIntent::Despawn { priority, .. } => *priority,
            ActionIntent::TransferOwnership { priority, .. } => *priority,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ActionPriority {
    Deferred(i64),
    Realtime,
}
impl PartialOrd for ActionPriority {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ActionPriority {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (ActionPriority::Realtime, ActionPriority::Realtime) => std::cmp::Ordering::Equal,
            (ActionPriority::Realtime, _) => std::cmp::Ordering::Greater,
            (_, ActionPriority::Realtime) => std::cmp::Ordering::Less,
            (ActionPriority::Deferred(a), ActionPriority::Deferred(b)) => b.cmp(a),
        }
    }
}
impl Default for ActionPriority {
    fn default() -> Self {
        ActionPriority::Deferred(0)
    }
}

#[derive(Debug)]
pub enum ResolutionError {
    IntentBufferNotFlushed,
    InvalidIntentCommitted,
    CurrentOwnerNotFoundInQuery,
}

#[derive(Debug)]
pub enum ResolutionWarning {
    RedundantIntent,
    IntentWithoutOwnership,
    IntentBufferUnavailable,
    SpawnIntentAfterCommittingToOwnershipTransfer,
    DespawnIntentAfterCommittingToOwnershipTransfer,
    OwnershipTransferIntentOfNonexistentChunk,
    OwnershipTransferItentOfDespawningChunk,
}

#[derive(Debug)]
pub enum ResolvedActionIntent {
    PushCommit(ActionIntent),
    PushBuffer(ActionIntent),
    CancelIntent,
    DiscardIncoming(ResolutionWarning),
    Error(ResolutionError),
}

pub fn resolve_intent(
    chunk_state: &State,
    committed: Option<&ActionIntent>,
    buffered: Option<&ActionIntent>,
    incoming: ActionIntent,
) -> ResolvedActionIntent {
    use ActionIntent::*;
    use ResolutionError::*;
    use ResolutionWarning::*;
    use ResolvedActionIntent::*;
    use State::*;

    let result = match (chunk_state, committed, buffered, incoming.clone()) {
        (_, None, Some(_), _) => Error(IntentBufferNotFlushed),
        (Absent, Some(TransferOwnership { .. }), _, _) => Error(InvalidIntentCommitted),

        (Absent, None, None, Spawn { .. }) => PushCommit(incoming.clone()),
        (Absent, None, None, Despawn { .. }) => DiscardIncoming(RedundantIntent),
        (Absent, None, None, TransferOwnership { .. }) => DiscardIncoming(OwnershipTransferIntentOfNonexistentChunk),

        (Owned(_), None, None, Spawn { .. }) => DiscardIncoming(RedundantIntent),
        (Owned(_), None, None, Despawn { .. }) => PushCommit(incoming.clone()),
        (Owned(current_owner), None, None, TransferOwnership { new_owner_id: new_owner, .. }) => {
            if *current_owner == new_owner {
                DiscardIncoming(RedundantIntent)
            } else {
                PushCommit(incoming.clone())
            }
        }

        (_, Some(Spawn { owner_id: committed_owner, .. }), None, incoming) => match incoming.clone() {
            Spawn { owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            Despawn { owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    PushBuffer(incoming)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            TransferOwnership { new_owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    PushBuffer(incoming)
                }
            }
        },
        (_, Some(Despawn { owner_id: committed_owner, .. }), None, incoming) => match incoming.clone() {
            Spawn { owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    PushBuffer(incoming)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            Despawn { owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            TransferOwnership { new_owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    DiscardIncoming(OwnershipTransferItentOfDespawningChunk)
                }
            }
        },
        (State::Owned(_current_owner), Some(TransferOwnership { new_owner_id: committed_owner, .. }), None, incoming) => match incoming.clone() {
            Spawn { .. } => DiscardIncoming(SpawnIntentAfterCommittingToOwnershipTransfer),
            Despawn { .. } => DiscardIncoming(DespawnIntentAfterCommittingToOwnershipTransfer),
            TransferOwnership { .. } => {
                if incoming.owner_id() == *committed_owner {
                    return DiscardIncoming(RedundantIntent);
                }

                PushBuffer(incoming)
            }
        },

        (State::Absent, Some(Spawn { owner_id: committed_owner, .. }), Some(Despawn { owner_id: buffered_owner, .. }), Spawn { owner_id: incoming_owner, .. })
            if buffered_owner == committed_owner && incoming_owner == *committed_owner =>
        {
            CancelIntent
        }

        (
            State::Absent,
            Some(Spawn { owner_id: committed_owner, .. }),
            Some(TransferOwnership { new_owner_id: buffered_owner, .. }),
            TransferOwnership { new_owner_id: incoming_owner, .. },
        ) if buffered_owner == committed_owner && incoming_owner == *committed_owner => CancelIntent,

        (State::Owned(_), Some(Despawn { owner_id: committed_owner, .. }), Some(Spawn { owner_id: buffered_owner, .. }), Despawn { owner_id: incoming_owner, .. })
            if buffered_owner == committed_owner && incoming_owner == *committed_owner =>
        {
            CancelIntent
        }

        (
            State::Owned(_),
            Some(TransferOwnership { new_owner_id: committed_owner, .. }),
            Some(Despawn { owner_id: buffered_owner, .. }),
            Spawn { owner_id: incoming_owner, .. },
        ) if buffered_owner == committed_owner && incoming_owner == *committed_owner => CancelIntent,

        (
            State::Owned(current_owner),
            Some(TransferOwnership { new_owner_id: committed_owner, .. }),
            Some(TransferOwnership { new_owner_id: buffered_owner, .. }),
            TransferOwnership { new_owner_id: incoming_owner, .. },
        ) if buffered_owner == current_owner && incoming_owner == *committed_owner => CancelIntent,

        (_, Some(_), Some(_), _) => DiscardIncoming(IntentBufferUnavailable),
    };

    debug!(
        target: "intent::resolution",
        "ResolveIntent =>\n  State: {:?}\n  Committed: {:?}\n  Buffered: {:?}\n  Incoming: {:?}\n  Result: {:?}",
        chunk_state,
        committed,
        buffered,
        incoming,
        result
    );

    result
}
