use std::any::{Any, TypeId};

use super::types::{ActionState, ActionType};

pub struct ActionTargetType {
    pub name: String,
    pub type_id: TypeId,
    pub action_types: Vec<ActionType>
}

pub struct ActionTargetRef<'a>(Option<&'a dyn Any>);
impl<'a> ActionTargetRef<'a> {
    pub fn new(value: Option<&'a dyn Any>) -> Self {
        Self(value)
    }

    pub fn resolve<T: Any>(&self) -> &T {
        self.0.expect("ActionTargetRef: Can not resolve 'None' reference").downcast_ref::<T>().unwrap_or_else(|| {
            unreachable!(
                "ActionTargetRef: Expected `{}`, but found something else.",
                std::any::type_name::<T>()
            )
        })
    }
}

pub enum ActionTargetState {
    Idle,
    Busy(ActionState),
}