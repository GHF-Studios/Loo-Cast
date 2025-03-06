use std::any::Any;

pub struct WorkflowResponse;
pub struct WorkflowResponseE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseS;
pub struct WorkflowResponseSE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseSO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseSOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseI;
pub struct WorkflowResponseIE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseIO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseIOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseIS;
pub struct WorkflowResponseISE(pub Result<(), Box<dyn Any + Send + Sync>>);
pub struct WorkflowResponseISO(pub Box<dyn Any + Send + Sync>);
pub struct WorkflowResponseISOE(pub Result<Box<dyn Any + Send + Sync>, Box<dyn Any + Send + Sync>>);