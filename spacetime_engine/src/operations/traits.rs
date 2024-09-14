use bevy::prelude::*;
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

pub trait Operation: 'static + Send + Sync {
    fn execute(&self, world: &mut World);
}
