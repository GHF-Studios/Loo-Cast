use std::any::{Any, TypeId};

pub struct InputState {
    input: Box<dyn Any + Send + Sync>,
}

pub struct OutputState {
    output: Box<dyn Any + Send + Sync>,
}

pub struct OutputStateBuilder;

pub struct ActionIO<T> {
    state: T,
}

impl ActionIO<InputState> {
    pub fn new<I: Any + Send + Sync>(input: Box<I>) -> Self {
        Self {
            state: InputState {
                input,
            },
        }
    }

    pub fn get_input_ref<I: Any + Send + Sync>(&self) -> &I {
        let input = self
            .state
            .input
            .downcast_ref::<I>()
            .unwrap_or_else(|| {
                unreachable!(
                    "Failed to get Input: Expected type `{}`, but got something else.",
                    std::any::type_name::<I>()
                )
            });

        input
    }

    pub fn get_input<I: Any + Send + Sync>(self) -> (I, ActionIO<OutputStateBuilder>) {
        let input = self
            .state
            .input
            .downcast::<I>()
            .map(|boxed| *boxed)
            .unwrap_or_else(|_| {
                unreachable!(
                    "Failed to get Input: Expected type `{}`, but got something else.",
                    std::any::type_name::<I>()
                )
            });

        (input, ActionIO { state: OutputStateBuilder {} })
    }
}
impl ActionIO<OutputStateBuilder> {
    pub fn set_output<O: Any + Send + Sync>(self, output: O) -> ActionIO<OutputState> {
        ActionIO {
            state: OutputState {
                output: Box::new(output),
            },
        }
    }
}
impl ActionIO<OutputState> {
    pub fn consume(self) -> Box<dyn Any + Send + Sync> {
        self.state.output
    }

    pub fn consume_cast<T: Any + Send + Sync>(self) -> Box<T> {
        self.state.output.downcast().expect(&format!("Failed to consume and cast output: Type '{:?}' is not the correct type.", TypeId::of::<T>()))
    }
}
