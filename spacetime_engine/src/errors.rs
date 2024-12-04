use crate::structs::*;

#[derive(Debug)]
pub enum LockingHierarchyError {
    PathNotFound(AbsoluteLockingPath),
    PathAlreadyExists(AbsoluteLockingPath),
}

#[derive(Debug)]
pub enum LockingNodeError {
    CannotUnlockPartiallyLocked,
    AlreadyPartiallyLocked,
    AlreadyFullyLocked,
    AlreadyUnlocked,
    ParentPoisoned,
    ParentFullyLocked,
    ChildPoisoned,
    ChildFullyLocked,
    UnlockParentError(Box<LockingNodeError>),
    LockChildError(Box<LockingNodeError>),
    UnlockChildError(Box<LockingNodeError>),
}