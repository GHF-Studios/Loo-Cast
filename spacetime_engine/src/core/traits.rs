use bevy::prelude::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;
use std::sync::MutexGuard;

pub trait RegistryKey: 'static + Clone + Copy + Debug + PartialEq + Eq + Hash + Send + Sync {
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

pub trait LockingNodeParent {}
pub trait LockingNodeChild {}
pub trait LockingNodeParentChild: LockingNodeParent + LockingNodeChild {}

pub trait LockingHierarchy<R: LockingRootNode<CK, C>, CK: RegistryKey, C: LockingNodeChild + 'static + Send + Sync> {
    fn root(&self) -> &R;
    fn root_mut(&mut self) -> &mut R;
    fn insert(&mut self, )
}

pub trait LockingRootNode<CK: RegistryKey, C: LockingNodeChild>: LockingNodeParent {
    fn children(&self) -> MutexGuard<HashMap<CK, C>>;
}
pub trait LockingBranchNode<PK: RegistryKey, P: LockingNodeParent, CK: RegistryKey, C: LockingNodeChild>: LockingNodeParentChild {
    fn parent(&self) -> MutexGuard<(PK, P)>;
    fn children(&self) -> MutexGuard<HashMap<CK, C>>;
}
pub trait LockingLeafNode<PK: RegistryKey, P: LockingNodeParent>: LockingNodeChild {
    fn parent(&self) -> MutexGuard<(PK, P)>;
}