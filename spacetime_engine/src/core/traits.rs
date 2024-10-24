use bevy::prelude::*;
use std::any::Any;
use std::fmt::{Debug, Display};
use std::hash::Hash;
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

pub trait Singleton: Any + Send + Sync {}
impl LockingNodePartialData for Box<dyn Singleton> {}
impl LockingNodeData for Box<dyn Singleton> {}

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

pub trait LockingNodePartialData {}
pub trait LockingNodeData: LockingNodePartialData {}