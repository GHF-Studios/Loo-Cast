use crate::structs::*;

#[derive(Debug)]
pub enum LockingHierarchyError {
    PathNotFound(AbsoluteLockingPath),
    PathAlreadyExists(AbsoluteLockingPath),
}

#[derive(Debug)]
pub enum LockingNodeError {
    DavidJackson
}