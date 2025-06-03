use bevy::prelude::Entity;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum State {
    Absent,
    Owned(Entity),
}
impl State {
    pub fn owner(&self) -> Option<Entity> {
        match self {
            State::Absent => None,
            State::Owned(owner) => Some(*owner),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionContext {
    LoadChunks { loader: Entity, coords: Vec<(i32, i32)> },
    UnloadChunks { loader: Entity, coords: Vec<(i32, i32)> },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionIntent {
    Spawn {
        owner: Entity,
        coord: (i32, i32),
        priority: ActionPriority,
    },
    Despawn {
        owner: Entity,
        coord: (i32, i32),
        priority: ActionPriority,
    },
    TransferOwnership {
        owner: Entity,
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

    pub fn owner(&self) -> Entity {
        match self {
            ActionIntent::Spawn { owner, .. } | ActionIntent::Despawn { owner, .. } | ActionIntent::TransferOwnership { owner, .. } => *owner,
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

/// WIP
#[derive(Debug)]
pub enum ResolutionError {
    IntentBufferNotFlushed,
    InvalidIntentCommitted,
    CurrentOwnerNotFoundInQuery,
}

/// WIP
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

pub fn get_intent(chunk_state: State, committed: Option<ActionIntent>, buffered: Option<ActionIntent>, context: ActionContext) -> Option<ActionIntent> {
    use ActionContext::*;
    use ActionIntent::*;
    use State::*;

    match (chunk_state, committed, buffered, context) {
        (Absent, None, None, LoadChunks { loader, coords }) => {}
    }
}

pub fn resolve_intent(chunk_state: State, committed: Option<ActionIntent>, buffered: Option<ActionIntent>, incoming: ActionIntent) -> ResolvedActionIntent {
    use ActionIntent::*;
    use ResolutionError::*;
    use ResolutionWarning::*;
    use ResolvedActionIntent::*;
    use State::*;

    match (chunk_state, committed, buffered, incoming.clone()) {
        (_, None, Some(_), _) => Error(IntentBufferNotFlushed),
        (Absent, Some(TransferOwnership { .. }), _, _) => Error(InvalidIntentCommitted),

        (Absent, None, None, Spawn { .. }) => PushCommit(incoming),
        (Absent, None, None, Despawn { .. }) => DiscardIncoming(RedundantIntent),
        (Absent, None, None, TransferOwnership { .. }) => DiscardIncoming(OwnershipTransferIntentOfNonexistentChunk),

        (Owned(_), None, None, Spawn { .. }) => DiscardIncoming(RedundantIntent),
        (Owned(_), None, None, Despawn { .. }) => PushCommit(incoming),
        (Owned(current_owner), None, None, TransferOwnership { owner: new_owner, .. }) => {
            if current_owner == new_owner {
                DiscardIncoming(RedundantIntent)
            } else {
                PushCommit(incoming)
            }
        }

        (_, Some(Spawn { owner: committed_owner, .. }), None, incoming) => match incoming {
            Spawn { owner: incoming_owner, .. } => {
                if incoming_owner == committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            Despawn { owner: incoming_owner, .. } => {
                if incoming_owner == committed_owner {
                    PushBuffer(incoming)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            TransferOwnership { owner: incoming_owner, .. } => {
                if incoming_owner == committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    PushBuffer(incoming)
                }
            }
        },
        (_, Some(Despawn { owner: committed_owner, .. }), None, incoming) => match incoming {
            Spawn { owner: incoming_owner, .. } => {
                if incoming_owner == committed_owner {
                    PushBuffer(incoming)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            Despawn { owner: incoming_owner, .. } => {
                if incoming_owner == committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    DiscardIncoming(IntentWithoutOwnership)
                }
            }
            TransferOwnership { owner: incoming_owner, .. } => {
                if incoming_owner == committed_owner {
                    DiscardIncoming(RedundantIntent)
                } else {
                    DiscardIncoming(OwnershipTransferItentOfDespawningChunk)
                }
            }
        },
        (State::Owned(_current_owner), Some(TransferOwnership { owner: committed_owner, .. }), None, incoming) => match incoming {
            Spawn { .. } => DiscardIncoming(SpawnIntentAfterCommittingToOwnershipTransfer),
            Despawn { .. } => DiscardIncoming(DespawnIntentAfterCommittingToOwnershipTransfer),
            TransferOwnership { .. } => {
                if incoming.owner() == committed_owner {
                    return DiscardIncoming(RedundantIntent);
                }

                PushBuffer(incoming)
            }
        },

        (State::Absent, Some(Spawn { owner: committed_owner, .. }), Some(Despawn { owner: buffered_owner, .. }), Spawn { owner: incoming_owner, .. })
            if buffered_owner == committed_owner && incoming_owner == committed_owner =>
        {
            CancelIntent
        }

        (
            State::Absent,
            Some(Spawn { owner: committed_owner, .. }),
            Some(TransferOwnership { owner: buffered_owner, .. }),
            TransferOwnership { owner: incoming_owner, .. },
        ) if buffered_owner == committed_owner && incoming_owner == committed_owner => CancelIntent,

        (State::Owned(_), Some(Despawn { owner: committed_owner, .. }), Some(Spawn { owner: buffered_owner, .. }), Despawn { owner: incoming_owner, .. })
            if buffered_owner == committed_owner && incoming_owner == committed_owner =>
        {
            CancelIntent
        }

        (
            State::Owned(_),
            Some(TransferOwnership { owner: committed_owner, .. }),
            Some(Despawn { owner: buffered_owner, .. }),
            Spawn { owner: incoming_owner, .. },
        ) if buffered_owner == committed_owner && incoming_owner == committed_owner => CancelIntent,

        (
            State::Owned(current_owner),
            Some(TransferOwnership { owner: committed_owner, .. }),
            Some(TransferOwnership { owner: buffered_owner, .. }),
            TransferOwnership { owner: incoming_owner, .. },
        ) if buffered_owner == current_owner && incoming_owner == committed_owner => CancelIntent,

        (_, Some(_), Some(_), _) => DiscardIncoming(IntentBufferUnavailable),
    }
}

pub fn apply_intent(chunk_state: State, committed: Option<ActionIntent>, buffered: Option<ActionIntent>, resolved: ResolvedActionIntent) {}
