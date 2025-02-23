use std::any::{Any, TypeId, type_name};
use bevy::prelude::*;

use super::types::RawVorkflowData;

pub struct InputState {
    input: RawVorkflowData,
}

pub struct OutputState {
    output: RawVorkflowData,
}

pub struct OutputStateBuilder;

pub struct CallbackState {
    callback_data: RawVorkflowData,
}

// TODO: Rename and/or move this damn file
// TODO: Rework this to allow more streamlined vorkflow io, with new concepts like data lanes, entry and exit points for lanes, etc. 
pub struct VorkflowIO<T> {
    state: T,
}

impl VorkflowIO<InputState> {
    pub fn new_input(input: RawVorkflowData) -> Self {
        Self {
            state: InputState {
                input
            },
        }
    }

    pub fn is_input_type<I: Any + Send + Sync>(&self) -> bool {
        self.state.input.data.is::<I>()
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

    pub fn get_input<I: Any + Send + Sync>(self) -> (I, VorkflowIO<OutputStateBuilder>) {
        let expected = self.state.input.data_type_name;
        let actual = type_name::<I>();

        let input = self.state.input.data.downcast::<I>().map(|boxed| *boxed).unwrap_or_else(|_| {
            panic!("Failed to get input: Correct type `{}`, provided type `{}`.", expected, actual)
        });

        (input, VorkflowIO { state: OutputStateBuilder {} })
    }

    pub(crate) fn consume_raw(self) -> RawVorkflowData {
        self.state.input
    }
}

impl VorkflowIO<OutputStateBuilder> {
    pub fn set_output(self, output: RawVorkflowData) -> VorkflowIO<OutputState> {
        VorkflowIO {
            state: OutputState {
                output
            },
        }
    }
}

impl VorkflowIO<OutputState> {
    pub fn consume_raw(self) -> RawVorkflowData {
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

impl VorkflowIO<CallbackState> {
    pub fn new_callback_data(callback_data: RawVorkflowData) -> Self {
        Self {
            state: CallbackState {
                callback_data
            },
        }
    }

    pub fn consume(self) -> RawVorkflowData {
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
