use super::structs::AbsoluteLockingPath;

pub enum LockingHierarchyError {
    PathNotFound(AbsoluteLockingPath),
    PathAlreadyExists(AbsoluteLockingPath),
}