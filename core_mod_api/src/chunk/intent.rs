use bevy::prelude::Reflect;
use std::marker::PhantomData;

use crate::usf::scale::Scale;

use super::types::ChunkOwnerId;

#[derive(Default, Debug, Clone, PartialEq, Eq, Reflect)]
pub enum State<S: Scale> {
    #[default]
    Absent,
    Owned(ChunkOwnerId<S>),
}
impl<S: Scale> State<S> {
    pub fn owner_id(&self) -> Option<ChunkOwnerId<S>> {
        match self {
            State::Absent => None,
            State::Owned(owner_id) => Some(owner_id.clone()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Reflect)]
pub enum ActionIntent<S: Scale> {
    Spawn {
        owner_id: ChunkOwnerId<S>,
        coord: (i32, i32),
        priority: ActionPriority,
    },
    Despawn {
        owner_id: ChunkOwnerId<S>,
        coord: (i32, i32),
        priority: ActionPriority,
    },
    TransferOwnership {
        new_owner_id: ChunkOwnerId<S>,
        coord: (i32, i32),
        priority: ActionPriority,
    },
}
impl<S: Scale> ActionIntent<S> {
    pub fn is_spawn(&self) -> bool {
        matches!(self, ActionIntent::Spawn { .. })
    }

    pub fn is_despawn(&self) -> bool {
        matches!(self, ActionIntent::Despawn { .. })
    }

    pub fn is_transfer_ownership(&self) -> bool {
        matches!(self, ActionIntent::TransferOwnership { .. })
    }

    pub fn owner_id(&self) -> ChunkOwnerId<S> {
        match self {
            ActionIntent::Spawn { owner_id, .. } | ActionIntent::Despawn { owner_id, .. } | ActionIntent::TransferOwnership { new_owner_id: owner_id, .. } => {
                owner_id.clone()
            }
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Reflect)]
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

#[derive(Debug, Reflect)]
pub enum ResolutionError<S: Scale> {
    IntentBufferNotFlushed(#[reflect(ignore)] PhantomData<S>),
    InvalidIntentCommitted(#[reflect(ignore)] PhantomData<S>),
    CurrentOwnerNotFoundInQuery(#[reflect(ignore)] PhantomData<S>),
}

#[derive(Debug, Reflect)]
pub enum ResolutionWarning<S: Scale> {
    RedundantIntent(#[reflect(ignore)] PhantomData<S>),
    IntentWithoutOwnership(#[reflect(ignore)] PhantomData<S>),
    IntentBufferUnavailable(#[reflect(ignore)] PhantomData<S>),
    SpawnIntentAfterCommittingToOwnershipTransfer(#[reflect(ignore)] PhantomData<S>),
    DespawnIntentAfterCommittingToOwnershipTransfer(#[reflect(ignore)] PhantomData<S>),
    OwnershipTransferIntentOfNonexistentChunk(#[reflect(ignore)] PhantomData<S>),
    OwnershipTransferItentOfDespawningChunk(#[reflect(ignore)] PhantomData<S>),
}

#[derive(Debug, Reflect)]
pub enum ResolvedActionIntent<S: Scale> {
    PushCommit(ActionIntent<S>),
    PushBuffer(ActionIntent<S>),
    CancelIntent,
    DiscardIncoming(ResolutionWarning<S>),
    Error(ResolutionError<S>),
}

pub fn resolve_intent<S: Scale>(
    chunk_state: &State<S>,
    committed: Option<&ActionIntent<S>>,
    buffered: Option<&ActionIntent<S>>,
    incoming: ActionIntent<S>,
) -> ResolvedActionIntent<S> {
    use ActionIntent::*;
    use ResolutionError::*;
    use ResolutionWarning::*;
    use ResolvedActionIntent::*;
    use State::*;

    match (chunk_state, committed, buffered, incoming.clone()) {
        (_, None, Some(_), _) => Error(IntentBufferNotFlushed(PhantomData)),
        (Absent, Some(TransferOwnership { .. }), _, _) => Error(InvalidIntentCommitted(PhantomData)),

        (Absent, None, None, Spawn { .. }) => PushCommit(incoming.clone()),
        (Absent, None, None, Despawn { .. }) => DiscardIncoming(RedundantIntent(PhantomData)),
        (Absent, None, None, TransferOwnership { .. }) => DiscardIncoming(OwnershipTransferIntentOfNonexistentChunk(PhantomData)),

        (Owned(_), None, None, Spawn { .. }) => DiscardIncoming(RedundantIntent(PhantomData)),
        (Owned(_), None, None, Despawn { .. }) => PushCommit(incoming.clone()),
        (Owned(current_owner), None, None, TransferOwnership { new_owner_id: new_owner, .. }) => {
            if *current_owner == new_owner {
                DiscardIncoming(RedundantIntent(PhantomData))
            } else {
                PushCommit(incoming.clone())
            }
        }

        (_, Some(Spawn { owner_id: committed_owner, .. }), None, incoming) => match incoming.clone() {
            Spawn { owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent(PhantomData))
                } else {
                    DiscardIncoming(IntentWithoutOwnership(PhantomData))
                }
            }
            Despawn { owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    PushBuffer(incoming)
                } else {
                    DiscardIncoming(IntentWithoutOwnership(PhantomData))
                }
            }
            TransferOwnership {
                new_owner_id: incoming_owner, ..
            } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent(PhantomData))
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
                    DiscardIncoming(IntentWithoutOwnership(PhantomData))
                }
            }
            Despawn { owner_id: incoming_owner, .. } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent(PhantomData))
                } else {
                    DiscardIncoming(IntentWithoutOwnership(PhantomData))
                }
            }
            TransferOwnership {
                new_owner_id: incoming_owner, ..
            } => {
                if incoming_owner == *committed_owner {
                    DiscardIncoming(RedundantIntent(PhantomData))
                } else {
                    DiscardIncoming(OwnershipTransferItentOfDespawningChunk(PhantomData))
                }
            }
        },
        (
            State::Owned(_current_owner),
            Some(TransferOwnership {
                new_owner_id: committed_owner, ..
            }),
            None,
            incoming,
        ) => match incoming.clone() {
            Spawn { .. } => DiscardIncoming(SpawnIntentAfterCommittingToOwnershipTransfer(PhantomData)),
            Despawn { .. } => DiscardIncoming(DespawnIntentAfterCommittingToOwnershipTransfer(PhantomData)),
            TransferOwnership { .. } => {
                if incoming.owner_id() == *committed_owner {
                    return DiscardIncoming(RedundantIntent(PhantomData));
                }

                PushBuffer(incoming)
            }
        },

        (
            State::Absent,
            Some(Spawn { owner_id: committed_owner, .. }),
            Some(Despawn { owner_id: buffered_owner, .. }),
            Spawn { owner_id: incoming_owner, .. },
        ) if buffered_owner == committed_owner && incoming_owner == *committed_owner => CancelIntent,

        (
            State::Absent,
            Some(Spawn { owner_id: committed_owner, .. }),
            Some(TransferOwnership {
                new_owner_id: buffered_owner, ..
            }),
            TransferOwnership {
                new_owner_id: incoming_owner, ..
            },
        ) if buffered_owner == committed_owner && incoming_owner == *committed_owner => CancelIntent,

        (
            State::Owned(_),
            Some(Despawn { owner_id: committed_owner, .. }),
            Some(Spawn { owner_id: buffered_owner, .. }),
            Despawn { owner_id: incoming_owner, .. },
        ) if buffered_owner == committed_owner && incoming_owner == *committed_owner => CancelIntent,

        (
            State::Owned(_),
            Some(TransferOwnership {
                new_owner_id: committed_owner, ..
            }),
            Some(Despawn { owner_id: buffered_owner, .. }),
            Spawn { owner_id: incoming_owner, .. },
        ) if buffered_owner == committed_owner && incoming_owner == *committed_owner => CancelIntent,

        (
            State::Owned(current_owner),
            Some(TransferOwnership {
                new_owner_id: committed_owner, ..
            }),
            Some(TransferOwnership {
                new_owner_id: buffered_owner, ..
            }),
            TransferOwnership {
                new_owner_id: incoming_owner, ..
            },
        ) if buffered_owner == current_owner && incoming_owner == *committed_owner => CancelIntent,

        (_, Some(_), Some(_), _) => DiscardIncoming(IntentBufferUnavailable(PhantomData)),
    }
}
