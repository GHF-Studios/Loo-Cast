use std::any::{Any, TypeId, type_name};

use super::types::RawActionData;

pub struct InputState {
    input: Box<dyn Any + Send + Sync>,
    input_type_name: &'static str,
}

pub struct OutputState {
    output: Box<dyn Any + Send + Sync>,
    output_type_name: &'static str,
}

pub struct OutputStateBuilder;

pub struct CallbackState {
    data: Box<dyn Any + Send + Sync>,
    data_type_name: &'static str,
}

pub struct ActionIO<T> {
    state: T,
}

impl ActionIO<InputState> {
    pub fn new_input(input: RawActionData) -> Self {
        Self {
            state: InputState {
                input: input.data,
                input_type_name: input.data_type_name,
            },
        }
    }

    pub fn get_input_ref<I: Any + Send + Sync>(&self) -> &I {
        self.state.input.downcast_ref::<I>().unwrap_or_else(|| {
            panic!(
                "Failed to get input: Expected `{}`, but got `{}`.",
                self.state.input_type_name,
                type_name::<I>()
            )
        })
    }

    pub fn get_input<I: Any + Send + Sync>(self) -> (I, ActionIO<OutputStateBuilder>) {
        let expected = self.state.input_type_name;
        let actual = type_name::<I>();

        let input = self.state.input.downcast::<I>().map(|boxed| *boxed).unwrap_or_else(|_| {
            panic!("Failed to get input: Expected `{}`, but got `{}`.", expected, actual)
        });

        (input, ActionIO { state: OutputStateBuilder {} })
    }
}

impl ActionIO<OutputStateBuilder> {
    pub fn set_output<O: Any + Send + Sync>(self, output: O) -> ActionIO<OutputState> {
        ActionIO {
            state: OutputState {
                output: Box::new(output),
                output_type_name: type_name::<O>(),
            },
        }
    }
}

impl ActionIO<OutputState> {
    pub fn consume(self) -> Box<dyn Any + Send + Sync> {
        self.state.output
    }

    pub fn consume_cast<O: Any + Send + Sync>(self) -> Box<O> {
        let expected = self.state.output_type_name;
        let actual = type_name::<O>();

        self.state.output.downcast().unwrap_or_else(|_| {
            panic!(
                "Failed to consume and cast output: Expected `{}`, but got `{}`.",
                expected, actual
            )
        })
    }
}

impl ActionIO<CallbackState> {
    pub fn new_callback_data(callback_data: RawActionData) -> Self {
        Self {
            state: CallbackState {
                data: callback_data.data,
                data_type_name: callback_data.data_type_name,
            },
        }
    }

    pub fn consume(self) -> Box<dyn Any + Send + Sync> {
        self.state.data
    }

    pub fn consume_cast<D: Any + Send + Sync>(self) -> Box<D> {
        let expected = self.state.data_type_name;
        let actual = type_name::<D>();

        self.state.data.downcast().unwrap_or_else(|_| {
            panic!(
                "Failed to consume and cast callback data: Expected `{}`, but got `{}`.",
                expected, actual
            )
        })
    }
}
