use std::any::Any;

pub struct InputState {
    input: Box<dyn Any + Send + Sync>,
}

pub struct OutputState {
    output: Box<dyn Any + Send + Sync>,
}

pub struct OutputStateBuilder;

pub struct ActionStageIO<T> {
    state: T,
}

impl ActionStageIO<InputState> {
    pub(in super) fn new<I: Any + Send + Sync>(input: Box<I>) -> Self {
        Self {
            state: InputState {
                input,
            },
        }
    }

    pub fn get_input<I: Any + Send + Sync>(self) -> (I, ActionStageIO<OutputStateBuilder>) {
        let input = self
            .state
            .input
            .downcast::<I>()
            .map(|boxed| *boxed)
            .unwrap_or_else(|_| {
                unreachable!(
                    "Failed to get Input: expected type `{}`, but got something else.",
                    std::any::type_name::<I>()
                )
            });

        (input, ActionStageIO { state: OutputStateBuilder {} })
    }
}
impl ActionStageIO<OutputStateBuilder> {
    pub fn set_output<O: Any + Send + Sync>(self, output: O) -> ActionStageIO<OutputState> {
        ActionStageIO {
            state: OutputState {
                output: Box::new(output),
            },
        }
    }
}
impl ActionStageIO<OutputState> {
    pub(in super) fn consume(self) -> Box<dyn Any + Send + Sync> {
        self.state.output
    }
}
