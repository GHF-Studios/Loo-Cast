use super::{core_type::CoreTypes, core_function::CoreFunctions};
use syn::Ident;
use quote::quote;
use proc_macro2::TokenStream;

/// Represents a collection of stages in a workflow.
pub struct Stages {
    pub stages: Vec<Stage>,
}

impl From<crate::workflow_mod::ir1::stage::Stages> for Stages {
    fn from(ir1: crate::workflow_mod::ir1::stage::Stages) -> Self {
        Self {
            stages: ir1.0.into_iter().map(Stage::from).collect(),
        }
    }
}

impl Stages {
    /// Generates Rust code for all stages.
    pub fn generate(&self) -> TokenStream {
        let stages: Vec<TokenStream> = self.stages.iter().map(Stage::generate).collect();
        quote! {
            #(#stages)*
        }
    }
}

/// Represents a single stage inside a workflow.
pub struct Stage {
    pub name: Ident,               
    pub stage_type: StageType,      
    pub core_types: CoreTypes,      
    pub core_functions: CoreFunctions, 
}

impl From<crate::workflow_mod::ir1::stage::Stage> for Stage {
    fn from(ir1: crate::workflow_mod::ir1::stage::Stage) -> Self {
        Self {
            name: ir1.name,
            stage_type: ir1.stage_type.into(),
            core_types: ir1.core_types.into(),
            core_functions: ir1.core_functions.into(),
        }
    }
}

impl Stage {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        quote! {
            pub mod #name {
                #core_types

                #core_functions
            }
        }
    }
}

/// Enum for the five possible stage types.
#[derive(Debug)]
pub enum StageType {
    Ecs,
    EcsWhile,
    Render,
    RenderWhile,
    Async,
}

impl From<crate::workflow_mod::ir1::stage::StageType> for StageType {
    fn from(ir1: crate::workflow_mod::ir1::stage::StageType) -> Self {
        match ir1 {
            crate::workflow_mod::ir1::stage::StageType::Ecs => StageType::Ecs,
            crate::workflow_mod::ir1::stage::StageType::EcsWhile => StageType::EcsWhile,
            crate::workflow_mod::ir1::stage::StageType::Render => StageType::Render,
            crate::workflow_mod::ir1::stage::StageType::RenderWhile => StageType::RenderWhile,
            crate::workflow_mod::ir1::stage::StageType::Async => StageType::Async,
        }
    }
}