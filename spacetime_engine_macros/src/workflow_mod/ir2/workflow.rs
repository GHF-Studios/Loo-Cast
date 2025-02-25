use crate::workflow_mod::ir1::workflow;
use crate::workflow_mod::ir2::{
    stage::Stages, use_statement::UseStatements, user_type::UserTypes, user_function::UserFunctions
};
use syn::Ident;
use quote::quote;
use proc_macro2::TokenStream;

/// Represents the fully expanded workflow module, ready for Rust code generation.
pub struct WorkflowModule {
    pub name: Ident,
    pub workflows: Vec<Workflow>,
}

impl From<workflow::WorkflowModule> for WorkflowModule {
    fn from(ir1: workflow::WorkflowModule) -> Self {
        Self {
            name: ir1.name,
            workflows: ir1.workflows.into_iter().map(Workflow::from).collect(),
        }
    }
}

impl WorkflowModule {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let workflows: Vec<TokenStream> = self.workflows.iter().map(Workflow::generate).collect();

        quote! {
            mod #name {
                #(#workflows)*
            }
        }
    }
}

/// Represents a fully expanded workflow, ready for code generation.
pub struct Workflow {
    pub name: Ident,
    pub user_imports: UseStatements,
    pub user_types: UserTypes,
    pub user_functions: UserFunctions,
    pub stages: Stages, // Now a separate type!
}

impl From<workflow::Workflow> for Workflow {
    fn from(ir1: workflow::Workflow) -> Self {
        Self {
            name: ir1.name,
            user_imports: ir1.user_imports.into(),  // Direct conversion
            user_types: ir1.user_types.into(),      // Direct conversion
            user_functions: ir1.user_functions.into(), // Direct conversion
            stages: ir1.stages.into(),              // Now properly handled
        }
    }
}

impl Workflow {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let imports = self.user_imports.generate();
        let user_types = self.user_types.generate();
        let user_functions = self.user_functions.generate();
        let stages = self.stages.generate();

        quote! {
            pub mod #name {
                #imports
                #user_types
                #user_functions
                #stages
            }
        }
    }
}
