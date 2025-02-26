use super::core_function::*;
use super::core_type::*;
use super::stage::*;
use super::use_statement::*;
use super::user_function::*;
use super::user_type::*;
use proc_macro2::TokenStream;
use syn::{parse::Parse, Result, Ident, Token};
use quote::quote;

/// Represents the entire `vorkflow_mod! { ... }` macro input.
pub struct WorkflowModule {
    pub name: Ident,            // Name of the module (e.g., "Gpu", "Chunk")
    pub workflows: Vec<Workflow>, // Collection of workflows
}

impl Parse for WorkflowModule {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?; 
        let _: Token![,] = input.parse()?; // Expect a comma

        let mut workflows = Vec::new();
        while !input.is_empty() {
            workflows.push(input.parse()?);
        }

        Ok(WorkflowModule { name, workflows })
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

/// Represents an individual workflow inside the module.
pub struct Workflow {
    pub name: Ident,                  // Name of the workflow (e.g., "SpawnChunk")
    pub user_imports: UseStatements,  
    pub user_types: UserTypes,        
    pub user_functions: UserFunctions, 
    pub stages: Stages,                // Collection of stages
}

impl Parse for Workflow {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![,] = input.parse()?; // Expect a comma

        let user_imports: UseStatements = input.parse()?;
        let _: Token![,] = input.parse()?; // Expect a comma

        let user_types: UserTypes = input.parse()?;
        let _: Token![,] = input.parse()?; // Expect a comma

        let user_functions: UserFunctions = input.parse()?;
        let _: Token![,] = input.parse()?; // Expect a comma

        let stages: Stages = input.parse()?; // Now parsing stages!

        Ok(Workflow {
            name,
            user_imports,
            user_types,
            user_functions,
            stages,
        })
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