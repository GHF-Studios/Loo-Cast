use std::any::{type_name, Any};
use bevy::prelude::*;

use crate::config::statics::CONFIG;

use super::{stage::VorkflowStage, io::{VorkflowIO, CallbackState, InputState, OutputState}};

pub enum Outcome<S, O> {
    Wait(S),
    Done(O)
}

pub struct EcsFn(pub fn(world: &mut World));
pub struct EcsInFn<Input>(pub fn(input: Input, world: &mut World));
pub struct EcsOutFn<Output>(pub fn(world: &mut World) -> Output);
pub struct EcsErrFn<Error>(pub fn(world: &mut World) -> Result<(), Error>);
pub struct EcsInOutFn<Input, Output>(pub fn(input: Input, world: &mut World) -> Output);
pub struct EcsInErrFn<Input, Error>(pub fn(input: Input, world: &mut World) -> Result<(), Error>);
pub struct EcsOutErrFn<Output, Error>(pub fn(world: &mut World) -> Result<Output, Error>);
pub struct EcsInOutErrFn<Input, Output, Error>(pub fn(input: Input, world: &mut World) -> Result<Output, Error>);

pub struct EcsWhileFn<State>(pub fn(world: &mut World) -> Outcome<State, ()>);
pub struct EcsWhileInFn<Input, State>(pub fn(input: Input, world: &mut World) -> Outcome<State, ()>);
pub struct EcsWhileOutFn<State, Output>(pub fn(world: &mut World) -> Outcome<State, Output>);
pub struct EcsWhileErrFn<State, Error>(pub fn(world: &mut World) -> Outcome<State, Result<(), Error>>);
pub struct EcsWhileInOutFn<Input, State, Output>(pub fn(input: Input, world: &mut World) -> Outcome<State, Output>);
pub struct EcsWhileInErrFn<Input, State, Error>(pub fn(input: Input, world: &mut World) -> Outcome<State, Result<(), Error>>);
pub struct EcsWhileOutErrFn<State, Output, Error>(pub fn(world: &mut World) -> Outcome<State, Result<Output, Error>>);
pub struct EcsWhileInOutErrFn<Input, State, Output, Error>(pub fn(input: Input, world: &mut World) -> Outcome<State, Result<Output, Error>>);

pub struct RenderFn(pub fn(world: &mut World));
pub struct RenderInFn<Input>(pub fn(input: Input, world: &mut World));
pub struct RenderOutFn<Output>(pub fn(world: &mut World) -> Output);
pub struct RenderErrFn<Error>(pub fn(world: &mut World) -> Result<(), Error>);
pub struct RenderInOutFn<Input, Output>(pub fn(input: Input, world: &mut World) -> Output);
pub struct RenderInErrFn<Input, Error>(pub fn(input: Input, world: &mut World) -> Result<(), Error>);
pub struct RenderOutErrFn<Output, Error>(pub fn(world: &mut World) -> Result<Output, Error>);
pub struct RenderInOutErrFn<Input, Output, Error>(pub fn(input: Input, world: &mut World) -> Result<Output, Error>);

pub struct RenderWhileFn<State>(pub fn(world: &mut World) -> Outcome<State, ()>);
pub struct RenderWhileInFn<Input, State>(pub fn(input: Input, world: &mut World) -> Outcome<State, ()>);
pub struct RenderWhileOutFn<State, Output>(pub fn(world: &mut World) -> Outcome<State, Output>);
pub struct RenderWhileErrFn<State, Error>(pub fn(world: &mut World) -> Outcome<State, Result<(), Error>>);
pub struct RenderWhileInOutFn<Input, State, Output>(pub fn(input: Input, world: &mut World) -> Outcome<State, Output>);
pub struct RenderWhileInErrFn<Input, State, Error>(pub fn(input: Input, world: &mut World) -> Outcome<State, Result<(), Error>>);
pub struct RenderWhileOutErrFn<State, Output, Error>(pub fn(world: &mut World) -> Outcome<State, Result<Output, Error>>);
pub struct RenderWhileInOutErrFn<Input, State, Output, Error>(pub fn(input: Input, world: &mut World) -> Outcome<State, Result<Output, Error>>);

pub struct RawVorkflowData {
    pub data: Box<dyn Any + Send + Sync>,
    pub data_type_name: &'static str,
}
impl RawVorkflowData {
    pub fn new<D: Any + Send + Sync>(value: D) -> Self {
        let wrapped_value = Self {
            data: Box::new(value),
            data_type_name: type_name::<D>(),
        };

        // TODO: Inefficient! Cache the type name.
        if wrapped_value.data_type_name == type_name::<RawVorkflowData>() {
            panic!("Attempted to create a RawVorkflowData with a RawVorkflowData data type.")
        }

        wrapped_value
    }
}

#[derive(Debug)]
pub enum VorkflowState {
    Requested,
    Processing {
        current_stage: usize,
        stage_completed: bool,
    },
}
impl std::fmt::Display for VorkflowState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Requested => write!(f, "VorkflowState::Requested"),
            Self::Processing { current_stage, stage_completed: completed } => write!(f, "VorkflowState::Processing(current_stage: {}, completed: {})", current_stage, completed),
        }
    }
}
impl VorkflowState {
    pub fn is_requested(&self) -> bool {
        matches!(self, Self::Requested)
    }

    pub fn current_stage(&self) -> usize {
        match self {
            Self::Requested => 0,
            Self::Processing { current_stage, .. } => *current_stage,
        }
    }
}

pub struct VorkflowTypeModule {
    pub name: String,
    pub vorkflow_types: Vec<VorkflowType>
}

pub struct VorkflowType {
    pub name: String,
    pub primary_validation: Box<dyn Fn(VorkflowIO<InputState>) -> Result<VorkflowIO<InputState>, String> + Send + Sync>,
    pub secondary_validation: Box<dyn Fn(VorkflowIO<InputState>, &mut World) -> Result<VorkflowIO<InputState>, String> + Send + Sync>,
    pub stages: Vec<VorkflowStage>
}

pub struct Vorkflow {
    pub vorkflow_type: VorkflowType,
}

pub struct VorkflowInstance {
    pub module_name: String,
    pub vorkflow_name: String,
    pub state: VorkflowState,
    pub data_buffer: RawVorkflowData,
    pub callback: Option<Box<dyn FnOnce(&mut World, VorkflowIO<CallbackState>) + Send + Sync>>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
impl std::fmt::Debug for VorkflowInstance{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "VorkflowInstance(module_name: {}, vorkflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.vorkflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl VorkflowInstance {
    pub(in super) fn new_request(
        module_name: String, 
        vorkflow_name: String, 
        input_params: RawVorkflowData, 
        output_callback: Option<Box<dyn FnOnce(&mut World, VorkflowIO<CallbackState>) + Send + Sync>>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("vorkflow/timeout_frames_per_stage");

        Self {
            module_name,
            vorkflow_name,
            state: VorkflowState::Requested,
            data_buffer: input_params,
            callback: output_callback,
            num_stages,
            timeout_frames
        }
    }
}
