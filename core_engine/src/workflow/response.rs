use crate::debug::types::AnySendSyncNamedBox;
use std::any::{type_name, Any};

pub enum WorkflowResponse {
    E(TypedWorkflowResponseE),
    O(TypedWorkflowResponseO),
    OE(TypedWorkflowResponseOE),
}

pub struct TypedWorkflowResponseE(pub Result<(), AnySendSyncNamedBox>);
pub struct TypedWorkflowResponseO(pub AnySendSyncNamedBox);
pub struct TypedWorkflowResponseOE(pub Result<AnySendSyncNamedBox, AnySendSyncNamedBox>);

impl TypedWorkflowResponseE {
    pub fn unpack<E: 'static + Any + Send + Sync>(self) -> Result<(), E> {
        match self.0 {
            Ok(_) => Ok(()),
            Err(error) => Err(error.into_inner()),
        }
    }
}

impl TypedWorkflowResponseO {
    pub fn unpack<O: 'static + Any + Send + Sync>(self) -> O {
        self.0.into_inner()
    }
}

impl TypedWorkflowResponseOE {
    pub fn unpack<O: 'static + Any + Send + Sync, E: 'static + Any + Send + Sync>(
        self,
    ) -> Result<O, E> {
        match self.0 {
            Ok(output) => Ok(output.into_inner()),
            Err(error) => Err(error.into_inner()),
        }
    }
}
