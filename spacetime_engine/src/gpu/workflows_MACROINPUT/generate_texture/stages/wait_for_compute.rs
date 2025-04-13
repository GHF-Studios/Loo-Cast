crate::workflow_stage_util!("WaitForCompute");

pub mod core_types {
    crate::workflow_stage_core_types_util!();

    #[derive(SystemParam)]
    pub struct MainAccess {}
    pub struct Input {
        request: GeneratorRequest<DispatchedCompute>,
    }
    pub struct State {
        request: GeneratorRequest<DispatchedCompute>,
    }
    pub struct Output {
        shader_name: &'static str,
        texture_handle: Handle<Image>,
    }
    pub enum Error {
        ComputePassReceiverDisconnected {
            shader_name: &'static str,
        },
    }
}

pub mod core_functions {
    crate::workflow_stage_core_functions_util!();
    crate::workflow_stage_core_function_util!(setup_ecs_while);
    crate::workflow_stage_core_function_util!(run_ecs_while);

    pub fn setup_ecs_while_inner(input: Input, ecs_access: RenderAccess) -> Result<State, Error> {
        Ok(State { request: input.request })
    }
    pub fn run_ecs_while_inner(state: State, ecs_access: RenderAccess) -> Result<Outcome<State, Output>, Error> {
        let receiver = &state.request.inner.receiver;

        match receiver.try_recv() {
            Ok(_) => {
                let (shader_name, texture_handle) = state.request.consume();
                Ok(Done(Output { shader_name, texture_handle }))
            },
            Err(crossbeam_channel::TryRecvError::Empty) => {
                Ok(Wait(state))
            },
            Err(crossbeam_channel::TryRecvError::Disconnected) => {
                Err(Error::ComputePassReceiverDisconnected { shader_name: state.request.inner.shader_name })
            },
        }
    }
}
