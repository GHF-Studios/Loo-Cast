use std::fmt::{Debug, Display};
use std::hash::Hash;
use super::structs::LockingPathSegment;

#[derive(Clone, PartialEq, Eq, Hash)]
pub enum LockingState {
    Unlocked,
    PartiallyLocked { locked_children: Vec<LockingPathSegment> },
    FullyLocked,
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