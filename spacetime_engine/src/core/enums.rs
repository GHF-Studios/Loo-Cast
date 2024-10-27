use std::fmt::{Debug, Display};
use std::hash::Hash;
use super::structs::LockingPathSegment;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LockingState {
    Unlocked,
    PartiallyLocked { locked_children: Vec<LockingPathSegment> },
    FullyLocked,
}
impl LockingState {
    pub fn is_unlocked(&self) -> bool {
        match self {
            LockingState::Unlocked => true,
            _ => false,
        }
    }

    pub fn is_locked(&self) -> bool {
        match self {
            LockingState::PartiallyLocked { .. } | LockingState::FullyLocked => true,
            _ => false,
        }
    }

    pub fn is_partially_locked(&self) -> bool {
        match self {
            LockingState::PartiallyLocked { .. } => true,
            _ => false,
        }
    }

    pub fn is_fully_locked(&self) -> bool {
        match self {
            LockingState::FullyLocked => true,
            _ => false,
        }
    }
}
impl Debug for LockingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingState::Unlocked => write!(f, "Unlocked"),
            LockingState::PartiallyLocked { locked_children } => write!(f, "PartiallyLocked({:?})", locked_children),
            LockingState::FullyLocked => write!(f, "FullyLocked"),
        }
    }
}
impl Display for LockingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingState::Unlocked => write!(f, "Unlocked"),
            LockingState::PartiallyLocked { locked_children } => write!(f, "PartiallyLocked({:?})", locked_children),
            LockingState::FullyLocked => write!(f, "FullyLocked"),
        }
    }
}