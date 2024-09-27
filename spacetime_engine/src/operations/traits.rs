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

// TODO: Implement derive-macro
pub trait OpArgs: 'static + Send + Sync {}

// TODO: Implement derive-macro
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
