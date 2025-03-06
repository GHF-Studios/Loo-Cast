use std::any::Any;

use crate::workflow::types::*;
use crate::config::statics::CONFIG;

pub enum WorkflowInstance {
    None(TypedWorkflowInstance),
    E(TypedWorkflowInstanceE),
    O(TypedWorkflowInstanceO),
    OE(TypedWorkflowInstanceOE),
    S(TypedWorkflowInstanceS),
    SE(TypedWorkflowInstanceSE),
    SO(TypedWorkflowInstanceSO),
    SOE(TypedWorkflowInstanceSOE),
    I(TypedWorkflowInstanceI),
    IE(TypedWorkflowInstanceIE),
    IO(TypedWorkflowInstanceIO),
    IOE(TypedWorkflowInstanceIOE),
    IS(TypedWorkflowInstanceIS),
    ISE(TypedWorkflowInstanceISE),
    ISO(TypedWorkflowInstanceISO),
    ISOE(TypedWorkflowInstanceISOE),
}
impl WorkflowInstance {
    pub fn has_input(&self) -> bool {
        match self {
            WorkflowInstance::None(_) => false,
            WorkflowInstance::E(_) => false,
            WorkflowInstance::O(_) => false,
            WorkflowInstance::OE(_) => false,
            WorkflowInstance::S(_) => false,
            WorkflowInstance::SE(_) => false,
            WorkflowInstance::SO(_) => false,
            WorkflowInstance::SOE(_) => false,
            WorkflowInstance::I(_) => true,
            WorkflowInstance::IE(_) => true,
            WorkflowInstance::IO(_) => true,
            WorkflowInstance::IOE(_) => true,
            WorkflowInstance::IS(_) => true,
            WorkflowInstance::ISE(_) => true,
            WorkflowInstance::ISO(_) => true,
            WorkflowInstance::ISOE(_) => true,
        }
    }

    pub fn has_output(&self) -> bool {
        match self {
            WorkflowInstance::None(_) => false,
            WorkflowInstance::E(_) => false,
            WorkflowInstance::O(_) => true,
            WorkflowInstance::OE(_) => true,
            WorkflowInstance::S(_) => false,
            WorkflowInstance::SE(_) => false,
            WorkflowInstance::SO(_) => true,
            WorkflowInstance::SOE(_) => true,
            WorkflowInstance::I(_) => false,
            WorkflowInstance::IE(_) => false,
            WorkflowInstance::IO(_) => true,
            WorkflowInstance::IOE(_) => true,
            WorkflowInstance::IS(_) => false,
            WorkflowInstance::ISE(_) => false,
            WorkflowInstance::ISO(_) => true,
            WorkflowInstance::ISOE(_) => true,
        }
    }

    pub fn has_error(&self) -> bool {
        match self {
            WorkflowInstance::None(_) => false,
            WorkflowInstance::E(_) => true,
            WorkflowInstance::O(_) => false,
            WorkflowInstance::OE(_) => true,
            WorkflowInstance::S(_) => false,
            WorkflowInstance::SE(_) => true,
            WorkflowInstance::SO(_) => false,
            WorkflowInstance::SOE(_) => true,
            WorkflowInstance::I(_) => false,
            WorkflowInstance::IE(_) => true,
            WorkflowInstance::IO(_) => false,
            WorkflowInstance::IOE(_) => true,
            WorkflowInstance::IS(_) => false,
            WorkflowInstance::ISE(_) => true,
            WorkflowInstance::ISO(_) => false,
            WorkflowInstance::ISOE(_) => true,
        }
    }
}

pub struct TypedWorkflowInstance {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub callback: Box<dyn FnOnce() + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceO {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceOE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceS {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub callback: Box<dyn FnOnce() + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceSE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceSO {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceSOE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceI {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce() + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceIE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceIO {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceIOE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceIS {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce() + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceISE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceISO {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
    pub num_stages: usize,
    pub timeout_frames: usize,
}
pub struct TypedWorkflowInstanceISOE {
    pub module_name: String,
    pub workflow_name: String,
    pub state: WorkflowState,
    pub data_buffer: Box<dyn Any + Send + Sync>,
    pub callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
impl std::fmt::Debug for TypedWorkflowInstanceS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceS(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceSE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceSE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceSO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceSO(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceSOE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceSOE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
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
impl std::fmt::Debug for TypedWorkflowInstanceIS {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceIS(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceISE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceISE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceISO {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceISO(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}
impl std::fmt::Debug for TypedWorkflowInstanceISOE {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "TypedWorkflowInstanceISOE(module_name: {}, workflow_name: {}, state: {}, num_stages: {}, timeout_frames: {})", 
            self.module_name, self.workflow_name, self.state, self.num_stages, self.timeout_frames)
    }
}

impl TypedWorkflowInstance {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
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
        module_name: String, 
        workflow_name: String, 
        callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
        module_name: String, 
        workflow_name: String, 
        callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
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
        module_name: String, 
        workflow_name: String, 
        callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
impl TypedWorkflowInstanceS {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
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
impl TypedWorkflowInstanceSE {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
        callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
impl TypedWorkflowInstanceSO {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
        callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
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
impl TypedWorkflowInstanceSOE {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
        callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
        module_name: String, 
        workflow_name: String, 
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
        module_name: String, 
        workflow_name: String, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
        module_name: String, 
        workflow_name: String, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
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
        module_name: String, 
        workflow_name: String, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
impl TypedWorkflowInstanceIS {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
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
impl TypedWorkflowInstanceISE {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(Result<(), Box<dyn Any + Send + Sync>>) + Send + Sync>,
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
impl TypedWorkflowInstanceISO {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(Box<dyn Any + Send + Sync>) + Send + Sync>,
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
impl TypedWorkflowInstanceISOE {
    pub(in super) fn new_request(
        module_name: String, 
        workflow_name: String, 
        input: Box<dyn Any + Send + Sync>, 
        callback: Box<dyn FnOnce(Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>) + Send + Sync>,
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