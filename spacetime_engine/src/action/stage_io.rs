use std::any::{Any, TypeId, type_name};
use bevy::prelude::*;

use super::types::RawActionData;

pub struct InputState {
    input: RawActionData,
}

pub struct OutputState {
    output: RawActionData,
}

pub struct OutputStateBuilder;

pub struct CallbackState {
    callback_data: RawActionData,
}

pub struct ActionIO<T> {
    state: T,
}

impl ActionIO<InputState> {
    pub fn new_input(input: RawActionData) -> Self {
        Self {
            state: InputState {
                input
            },
        }
    }

    pub fn get_input_ref<I: Any + Send + Sync>(&self) -> &I {
        self.state.input.data.downcast_ref::<I>().unwrap_or_else(|| {
            panic!(
                "Failed to get input: Correct type `{}`, provided type `{}`.",
                self.state.input.data_type_name,
                type_name::<I>()
            )
        })
    }

    pub fn get_input_mut<I: Any + Send + Sync>(&mut self) -> &mut I {
        self.state.input.data.downcast_mut::<I>().unwrap_or_else(|| {
            panic!(
                "Failed to get input: Correct type `{}`, provided type `{}`.",
                self.state.input.data_type_name,
                type_name::<I>()
            )
        })
    }

    pub fn get_input<I: Any + Send + Sync>(self) -> (I, ActionIO<OutputStateBuilder>) {
        let expected = self.state.input.data_type_name;
        let actual = type_name::<I>();

        let input = self.state.input.data.downcast::<I>().map(|boxed| *boxed).unwrap_or_else(|_| {
            panic!("Failed to get input: Correct type `{}`, provided type `{}`.", expected, actual)
        });

        (input, ActionIO { state: OutputStateBuilder {} })
    }

    pub(crate) fn consume_raw(self) -> RawActionData {
        self.state.input
    }
}

impl ActionIO<OutputStateBuilder> {
    pub fn set_output(self, output: RawActionData) -> ActionIO<OutputState> {
        ActionIO {
            state: OutputState {
                output
            },
        }
    }
}

impl ActionIO<OutputState> {
    pub fn consume_raw(self) -> RawActionData {
        self.state.output
    }

    pub fn consume_cast<O: Any + Send + Sync>(self) -> Box<O> {
        let expected = self.state.output.data_type_name;
        let actual = type_name::<O>();

        self.state.output.data.downcast().unwrap_or_else(|_| {
            panic!(
                "Failed to consume and cast output: Correct type `{}`, provided type `{}`.",
                expected, actual
            )
        })
    }
}

impl ActionIO<CallbackState> {
    pub fn new_callback_data(callback_data: RawActionData) -> Self {
        Self {
            state: CallbackState {
                callback_data
            },
        }
    }

    pub fn consume(self) -> RawActionData {
        self.state.callback_data
    }

    pub fn consume_cast<D: Any + Send + Sync>(self) -> Box<D> {
        let expected = self.state.callback_data.data_type_name;
        let actual = type_name::<D>();

        self.state.callback_data.data.downcast().unwrap_or_else(|_| {
            panic!(
                "Failed to consume and cast callback data: Correct type `{}`, provided type `{}`.",
                expected, actual
            )
        })
    }
}
