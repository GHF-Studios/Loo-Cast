use super::structs::AbsoluteLockingPath;

#[derive(Debug)]
pub enum LockingHierarchyError {
    PathNotFound(AbsoluteLockingPath),
    PathAlreadyExists(AbsoluteLockingPath),
}