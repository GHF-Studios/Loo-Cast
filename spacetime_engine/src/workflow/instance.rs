use std::any::Any;

use crate::workflow::types::*;
use crate::config::statics::CONFIG;

use super::response::*;

#[derive(Debug)]
pub enum WorkflowInstance {
    None(TypedWorkflowInstance),
    E(TypedWorkflowInstanceE),
    O(TypedWorkflowInstanceO),
    OE(TypedWorkflowInstanceOE),
    I(TypedWorkflowInstanceI),
    IE(TypedWorkflowInstanceIE),
    IO(TypedWorkflowInstanceIO),
    IOE(TypedWorkflowInstanceIOE),
}
impl WorkflowInstance {
    pub fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        num_stages: usize,
        callback: Box<dyn FnOnce() + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::None(TypedWorkflowInstance::new_request(
            module_name,
            workflow_name,
            callback,
            num_stages
        ))
    }
    pub fn new_request_e(
        module_name: &'static str, 
        workflow_name: &'static str, 
        num_stages: usize,
        callback: Box<dyn FnOnce(TypedWorkflowResponseE) + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::E(TypedWorkflowInstanceE::new_request(
            module_name,
            workflow_name,
            callback,
            num_stages
        ))
    }
    pub fn new_request_o(
        module_name: &'static str, 
        workflow_name: &'static str, 
        num_stages: usize,
        callback: Box<dyn FnOnce(TypedWorkflowResponseO) + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::O(TypedWorkflowInstanceO::new_request(
            module_name,
            workflow_name,
            callback,
            num_stages
        ))
    }
    pub fn new_request_oe(
        module_name: &'static str, 
        workflow_name: &'static str, 
        num_stages: usize,
        callback: Box<dyn FnOnce(TypedWorkflowResponseOE) + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::OE(TypedWorkflowInstanceOE::new_request(
            module_name,
            workflow_name,
            callback,
            num_stages
        ))
    }
    pub fn new_request_i(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        num_stages: usize,
        callback: Box<dyn FnOnce() + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::I(TypedWorkflowInstanceI::new_request(
            module_name, 
            workflow_name,
            input,
            callback,
            num_stages
        ))
    }
    pub fn new_request_ie(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        num_stages: usize,
        callback: Box<dyn FnOnce(TypedWorkflowResponseE) + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::IE(TypedWorkflowInstanceIE::new_request(
            module_name, 
            workflow_name,
            input,
            callback,
            num_stages
        ))
    }
    pub fn new_request_io(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        num_stages: usize,
        callback: Box<dyn FnOnce(TypedWorkflowResponseO) + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::IO(TypedWorkflowInstanceIO::new_request(
            module_name, 
            workflow_name,
            input,
            callback,
            num_stages
        ))
    }
    pub fn new_request_ioe(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        num_stages: usize,
        callback: Box<dyn FnOnce(TypedWorkflowResponseOE) + Send + Sync>, 
    ) -> Self {
        WorkflowInstance::IOE(TypedWorkflowInstanceIOE::new_request(
            module_name, 
            workflow_name,
            input,
            callback,
            num_stages
        ))
    }

    pub fn has_input(&self) -> bool {
        match self {
            WorkflowInstance::None(_) => false,
            WorkflowInstance::E(_) => false,
            WorkflowInstance::O(_) => false,
            WorkflowInstance::OE(_) => false,
            WorkflowInstance::I(_) => true,
            WorkflowInstance::IE(_) => true,
            WorkflowInstance::IO(_) => true,
            WorkflowInstance::IOE(_) => true,
        }
    }

    pub fn has_output(&self) -> bool {
        match self {
            WorkflowInstance::None(_) => false,
            WorkflowInstance::E(_) => false,
            WorkflowInstance::O(_) => true,
            WorkflowInstance::OE(_) => true,
            WorkflowInstance::I(_) => false,
            WorkflowInstance::IE(_) => false,
            WorkflowInstance::IO(_) => true,
            WorkflowInstance::IOE(_) => true,
        }
    }

    pub fn has_error(&self) -> bool {
        match self {
            WorkflowInstance::None(_) => false,
            WorkflowInstance::E(_) => true,
            WorkflowInstance::O(_) => false,
            WorkflowInstance::OE(_) => true,
            WorkflowInstance::I(_) => false,
            WorkflowInstance::IE(_) => true,
            WorkflowInstance::IO(_) => false,
            WorkflowInstance::IOE(_) => true,
        }
    }

    pub fn module_name(&self) -> &'static str {
        match self {
            WorkflowInstance::None(instance) => instance.module_name,
            WorkflowInstance::E(instance) => instance.module_name,
            WorkflowInstance::O(instance) => instance.module_name,
            WorkflowInstance::OE(instance) => instance.module_name,
            WorkflowInstance::I(instance) => instance.module_name,
            WorkflowInstance::IE(instance) => instance.module_name,
            WorkflowInstance::IO(instance) => instance.module_name,
            WorkflowInstance::IOE(instance) => instance.module_name,
        }
    }

    pub fn workflow_name(&self) -> &'static str {
        match self {
            WorkflowInstance::None(instance) => instance.workflow_name,
            WorkflowInstance::E(instance) => instance.workflow_name,
            WorkflowInstance::O(instance) => instance.workflow_name,
            WorkflowInstance::OE(instance) => instance.workflow_name,
            WorkflowInstance::I(instance) => instance.workflow_name,
            WorkflowInstance::IE(instance) => instance.workflow_name,
            WorkflowInstance::IO(instance) => instance.workflow_name,
            WorkflowInstance::IOE(instance) => instance.workflow_name,
        }
    }

    pub fn state(&self) -> WorkflowState {
        match self {
            WorkflowInstance::None(instance) => instance.state,
            WorkflowInstance::E(instance) => instance.state,
            WorkflowInstance::O(instance) => instance.state,
            WorkflowInstance::OE(instance) => instance.state,
            WorkflowInstance::I(instance) => instance.state,
            WorkflowInstance::IE(instance) => instance.state,
            WorkflowInstance::IO(instance) => instance.state,
            WorkflowInstance::IOE(instance) => instance.state,
        }
    }

    pub fn state_mut(&mut self) -> &mut WorkflowState {
        match self {
            WorkflowInstance::None(instance) => &mut instance.state,
            WorkflowInstance::E(instance) => &mut instance.state,
            WorkflowInstance::O(instance) => &mut instance.state,
            WorkflowInstance::OE(instance) => &mut instance.state,
            WorkflowInstance::I(instance) => &mut instance.state,
            WorkflowInstance::IE(instance) => &mut instance.state,
            WorkflowInstance::IO(instance) => &mut instance.state,
            WorkflowInstance::IOE(instance) => &mut instance.state,
        }
    }

    pub fn num_stages(&self) -> usize {
        match self {
            WorkflowInstance::None(instance) => instance.num_stages,
            WorkflowInstance::E(instance) => instance.num_stages,
            WorkflowInstance::O(instance) => instance.num_stages,
            WorkflowInstance::OE(instance) => instance.num_stages,
            WorkflowInstance::I(instance) => instance.num_stages,
            WorkflowInstance::IE(instance) => instance.num_stages,
            WorkflowInstance::IO(instance) => instance.num_stages,
            WorkflowInstance::IOE(instance) => instance.num_stages,
        }
    }

    pub fn timeout_frames(&self) -> usize {
        match self {
            WorkflowInstance::None(instance) => instance.timeout_frames,
            WorkflowInstance::E(instance) => instance.timeout_frames,
            WorkflowInstance::O(instance) => instance.timeout_frames,
            WorkflowInstance::OE(instance) => instance.timeout_frames,
            WorkflowInstance::I(instance) => instance.timeout_frames,
            WorkflowInstance::IE(instance) => instance.timeout_frames,
            WorkflowInstance::IO(instance) => instance.timeout_frames,
            WorkflowInstance::IOE(instance) => instance.timeout_frames,
        }
    }
}

pub struct TypedWorkflowInstance {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub callback: Box<dyn FnOnce() + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub callback: Box<dyn FnOnce(TypedWorkflowResponseE) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(TypedWorkflowResponseO) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(TypedWorkflowResponseOE) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceI {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce() + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceIE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(TypedWorkflowResponseE) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceIO {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(TypedWorkflowResponseO) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceIOE {
    pub module_name: &'static str,
    pub workflow_name: &'static str,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(TypedWorkflowResponseOE) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}

impl std::fmt::Debug for TypedWorkflowInstance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstance(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceO(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceOE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceOE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceI(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceIE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceIE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceIO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceIO(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceIOE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceIOE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}

impl TypedWorkflowInstance {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        callback: Box<dyn FnOnce() + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            callback,
            num_stages,
            timeout_frames
        }
    }
}
impl TypedWorkflowInstanceE {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        callback: Box<dyn FnOnce(TypedWorkflowResponseE) + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            callback,
            num_stages,
            timeout_frames
        }
    }
}
impl TypedWorkflowInstanceO {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        callback: Box<dyn FnOnce(TypedWorkflowResponseO) + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            data_buffer: Box::new(()),
            callback,
            num_stages,
            timeout_frames
        }
    }
}
impl TypedWorkflowInstanceOE {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        callback: Box<dyn FnOnce(TypedWorkflowResponseOE) + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            data_buffer: Box::new(()),
            callback,
            num_stages,
            timeout_frames
        }
    }
}
impl TypedWorkflowInstanceI {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce() + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            data_buffer: input,
            callback,
            num_stages,
            timeout_frames
        }
    }
}
impl TypedWorkflowInstanceIE {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(TypedWorkflowResponseE) + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            data_buffer: input,
            callback,
            num_stages,
            timeout_frames
        }
    }
}
impl TypedWorkflowInstanceIO {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(TypedWorkflowResponseO) + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            data_buffer: input,
            callback,
            num_stages,
            timeout_frames
        }
    }
}
impl TypedWorkflowInstanceIOE {
    pub(in super) fn new_request(
        module_name: &'static str, 
        workflow_name: &'static str, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(TypedWorkflowResponseOE) + Send + Sync>,
        num_stages: usize,
    ) -> Self {
        let timeout_frames = num_stages * CONFIG.get::<usize>("workflow/timeout_frames_per_stage");

        Self {
            module_name,
            workflow_name,
            state: WorkflowState::Requested,
            data_buffer: input,
            callback,
            num_stages,
            timeout_frames
        }
    }
}
