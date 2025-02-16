use std::any::{Any, TypeId, type_name};
use bevy::prelude::*;

use super::types::RawWorkflowData;

pub struct InputState {
    input: RawWorkflowData,
}

pub struct OutputState {
    output: RawWorkflowData,
}

pub struct OutputStateBuilder;

pub struct CallbackState {
    callback_data: RawWorkflowData,
}

pub struct WorkflowIO<T> {
    state: T,
}

impl WorkflowIO<InputState> {
    pub fn new_input(input: RawWorkflowData) -> Self {
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

    pub fn get_input<I: Any + Send + Sync>(self) -> (I, WorkflowIO<OutputStateBuilder>) {
        let expected = self.state.input.data_type_name;
        let actual = type_name::<I>();

        let input = self.state.input.data.downcast::<I>().map(|boxed| *boxed).unwrap_or_else(|_| {
            panic!("Failed to get input: Correct type `{}`, provided type `{}`.", expected, actual)
        });

        (input, WorkflowIO { state: OutputStateBuilder {} })
    }

    pub(crate) fn consume_raw(self) -> RawWorkflowData {
        self.state.input
    }
}

impl WorkflowIO<OutputStateBuilder> {
    pub fn set_output(self, output: RawWorkflowData) -> WorkflowIO<OutputState> {
        WorkflowIO {
            state: OutputState {
                output
            },
        }
    }
}

impl WorkflowIO<OutputState> {
    pub fn consume_raw(self) -> RawWorkflowData {
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

impl WorkflowIO<CallbackState> {
    pub fn new_callback_data(callback_data: RawWorkflowData) -> Self {
        Self {
            state: CallbackState {
                callback_data
            },
        }
    }

    pub fn consume(self) -> RawWorkflowData {
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
