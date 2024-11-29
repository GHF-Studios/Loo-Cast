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

pub trait InstanceRegistryValue: 'static + PartialEq + Send + Sync {}

pub trait LockingNodeData: Any + Send + Sync {
    fn pre_startup(&mut self, hierarchy: &mut LockingHierarchy);
    fn startup(&mut self, hierarchy: &mut LockingHierarchy);
    fn post_startup(&mut self, hierarchy: &mut LockingHierarchy);
    fn pre_update(&mut self, hierarchy: &mut LockingHierarchy);
    fn update(&mut self, hierarchy: &mut LockingHierarchy);
    fn post_update(&mut self, hierarchy: &mut LockingHierarchy);
}

pub trait LockingTypeDataTrait: Any + Send + Sync {}

pub trait LockingPath: 'static + Send + Sync + Debug + Display + Clone + PartialEq + Eq + Hash {
    fn segments(&self) -> &Vec<LockingPathSegment>;
    fn segments_mut(&mut self) -> &mut Vec<LockingPathSegment>;
    fn push(self, segment: LockingPathSegment) -> Result<Self, String>;
    fn pop(self) -> Result<(Self, LockingPathSegment), String>;
}