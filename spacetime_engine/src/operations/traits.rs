use bevy::prelude::*;
use std::any::TypeId;
use std::fmt::Debug;
use std::hash::Hash;

pub trait InstanceRegistryKey: 'static + Clone + Copy + Debug + PartialEq + Eq + Hash + Send + Sync {
    fn new(id: u64) -> Self;
    fn get(&self) -> u64;
}

pub trait InstanceRegistryValue: 'static + PartialEq + Send + Sync {
}
impl InstanceRegistryValue for Entity {
}

pub trait OpArgs: 'static + Send + Sync {}

pub trait OpResult: 'static + Send + Sync {}

pub trait Operation: 'static + Send + Sync {
    type Args: OpArgs;
    type Result: OpResult;

    fn execute(&self, world: &mut World);
}

pub trait DynOperation: 'static + Send + Sync {
    fn execute(&self, world: &mut World);
}

impl<T> DynOperation for T
where
    T: Operation,
{
    fn execute(&self, world: &mut World) {
        <T as Operation>::execute(self, world);
    }
}
