use bevy::prelude::*;
use std::any::Any;
use std::fmt::{Debug, Display};
use std::hash::Hash;
use crate::structs::*;

// TODO: Implement derive-macro, because this is a marker trait
pub trait OpArgs: 'static + Send + Sync {}

// TODO: Implement derive-macro, because this is a marker trait
pub trait OpResult: 'static + Send + Sync {}

pub trait OpCallback<R: OpResult>: 'static + Send + Sync {
    fn send(&mut self, result: R);
}
impl<R: OpResult> OpCallback<R> for Option<tokio::sync::oneshot::Sender<R>> {
    fn send(&mut self, result: R) {
        let sender = match self.take() {
            Some(sender) => sender,
            None => {
                error!("Callback sender could not be found!");
                return;
            },
        };

        match sender.send(result) {
            Ok(_) => {},
            Err(_) => {
                error!("Callback receiver could not be found!");
            },
        };
    }
}

pub trait Operation: 'static + Send + Sync {
    type Args: OpArgs;
    type Result: OpResult;

    fn new(args: Self::Args, callback: tokio::sync::oneshot::Sender<Self::Result>) -> Self;
    fn execute(&mut self, world: &mut World);
}

pub trait DynOperation: 'static + Send + Sync {
    fn execute(&mut self, world: &mut World);
}

impl<T> DynOperation for T
where
    T: Operation,
{
    fn execute(&mut self, world: &mut World) {
        <T as Operation>::execute(self, world);
    }
}

pub type Command = Box<dyn Fn(Box<dyn Any>) -> Box<dyn Any> + Send>;

pub trait RegistryKey: 'static + Send + Sync + Debug + Display + Clone + Copy + PartialEq + Eq + Hash {
    type ID;

    fn new(id: Self::ID) -> Self;
    fn get(&self) -> Self::ID;
}

pub trait StaticInstanceRegistryKey: RegistryKey<ID = &'static str> {}

pub trait DynamicInstanceRegistryKey: RegistryKey<ID = u64> {}

pub trait InstanceRegistryValue: 'static + PartialEq + Send + Sync {}

pub trait LockingNodeData: Any + Send + Sync {
    fn on_insert(&mut self, hierarchy: &mut LockingHierarchy);
    fn on_remove(&mut self, hierarchy: &mut LockingHierarchy);
}

pub trait LockingTypeDataTrait: Any + Send + Sync {}

pub trait LockingPath: 'static + Send + Sync + Debug + Display + Clone + PartialEq + Eq + Hash {
    fn segments(&self) -> &Vec<LockingPathSegment>;
    fn segments_mut(&mut self) -> &mut Vec<LockingPathSegment>;
    fn push(self, segment: LockingPathSegment) -> Result<Self, String>;
    fn pop(self) -> Result<(Self, LockingPathSegment), String>;
}
