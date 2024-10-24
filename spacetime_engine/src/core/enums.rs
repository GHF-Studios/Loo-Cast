use std::any::Any;
use std::collections::HashMap;
use std::num::NonZero;
use std::fmt::{Debug, Display};
use std::sync::*;
use std::hash::Hash;
use super::structs::LockingPathSegment;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum LockingState {
    Unlocked,
    PartiallyLocked { child_lock_count: NonZero<usize>},
    FullyLocked,
}
impl Debug for LockingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingState::Unlocked => write!(f, "Unlocked"),
            LockingState::PartiallyLocked { child_lock_count } => write!(f, "PartiallyLocked({})", child_lock_count),
            LockingState::FullyLocked => write!(f, "FullyLocked"),
        }
    }
}
impl Display for LockingState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LockingState::Unlocked => write!(f, "Unlocked"),
            LockingState::PartiallyLocked { child_lock_count } => write!(f, "PartiallyLocked({})", child_lock_count),
            LockingState::FullyLocked => write!(f, "FullyLocked"),
        }
    }
}