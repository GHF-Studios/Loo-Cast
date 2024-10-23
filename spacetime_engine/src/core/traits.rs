use bevy::prelude::*;
use std::any::Any;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use std::sync::*;
use super::enums::LockingNodeInfo;
use super::errors::LockingHierarchyError;
use super::structs::*;

pub trait RegistryKey: 'static + Send + Sync + Debug + Display + Clone + Copy + PartialEq + Eq + Hash {
    type ID;

    fn new(id: Self::ID) -> Self;
    fn get(&self) -> Self::ID;
}

pub trait StaticInstanceRegistryKey: RegistryKey<ID = &'static str> {}

pub trait DynamicInstanceRegistryKey: RegistryKey<ID = u64> {}

pub trait InstanceRegistryValue: 'static + PartialEq + Send + Sync {
}
impl InstanceRegistryValue for Entity {
}

pub trait LockingPath: 'static + Send + Sync + Debug + Display + Clone + PartialEq + Eq + Hash {
    fn segments(&self) -> &Vec<LockingPathSegment>;
    fn segments_mut(&mut self) -> &mut Vec<LockingPathSegment>;
    fn push(&mut self, segment: LockingPathSegment) -> Result<(), String>;
    fn pop(&mut self) -> Result<LockingPathSegment, String>;
}

pub trait LockingNode {
    fn node_info(&self) -> LockingNodeInfo;
}
pub trait LockingNodeParent: LockingNode {}
pub trait LockingNodeChild: LockingNode {}
pub trait LockingNodeParentChild: LockingNodeParent + LockingNodeChild {}

pub trait LockingHierarchy<R: LockingRootNode> {
    fn root(&self) -> &R;
    fn root_mut(&mut self) -> &mut R;
    fn insert(&mut self, path: AbsoluteLockingPath, entry: Box<dyn Any>) -> Result<(), LockingHierarchyError>;
    fn remove(&mut self, path: AbsoluteLockingPath) -> Result<Box<dyn Any>, LockingHierarchyError>;
    fn get(&self, path: AbsoluteLockingPath) -> Result<&Box<dyn Any>, LockingHierarchyError>;
    fn get_mut(&mut self, path: AbsoluteLockingPath) -> Result<&mut Box<dyn Any>, LockingHierarchyError>;
    fn contains(&self, path: AbsoluteLockingPath) -> Result<bool, LockingHierarchyError>;
    fn lock(&self, path: AbsoluteLockingPath) -> Result<MutexGuard<dyn Any>, LockingHierarchyError>;
    fn unlock(&self, path: AbsoluteLockingPath, entry_guard: MutexGuard<dyn Any>) -> Result<(), LockingHierarchyError>;
    fn is_locked(&self, path: AbsoluteLockingPath) -> Result<bool, LockingHierarchyError>;
}

pub trait LockingRootNode: LockingNodeParent {
    fn children(&self) -> MutexGuard<HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>>>;
}
pub trait LockingBranchNode: LockingNodeParentChild {
    fn parent(&self) -> MutexGuard<(LockingPathSegment, Arc<Mutex<dyn Any>>)>;
    fn children(&self) -> MutexGuard<HashMap<LockingPathSegment, Arc<Mutex<dyn Any>>>>;
}
pub trait LockingLeafNode: LockingNodeChild {
    fn parent(&self) -> MutexGuard<(LockingPathSegment, Arc<Mutex<dyn Any>>)>;
}