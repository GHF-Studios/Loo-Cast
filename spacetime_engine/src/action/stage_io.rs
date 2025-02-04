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
    pub(in super) fn new<I: Any + Send + Sync>(input: I) -> Self {
        Self {
            state: InputState {
                input: Box::new(input),
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
    fn consume<O: Any + Send + Sync>(self) -> O {
        self.state
            .output
            .downcast::<O>()
            .map(|boxed| *boxed)
            .unwrap_or_else(|_| {
                unreachable!(
                    "Failed to consume Output: expected type `{}`, but got something else.",
                    std::any::type_name::<O>()
                )
            })
    }
}
