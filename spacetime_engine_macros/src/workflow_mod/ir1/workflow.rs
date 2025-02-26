use super::core_function::*;
use super::core_type::*;
use super::stage::*;
use super::use_statement::*;
use super::user_item::*; // Replaces user_function & user_type
use proc_macro2::TokenStream;
use syn::{parse::Parse, Result, Ident, Token};
use quote::quote;

/// Represents the entire `vorkflow_mod! { ... }` macro input.
pub struct WorkflowModule {
    pub name: Ident,             // Name of the module (e.g., "Gpu", "Chunk")
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
    pub fn generate(self) -> TokenStream {
        let name = self.name;
        let workflows: Vec<TokenStream> = self.workflows.into_iter().map(Workflow::generate).collect();

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
    pub user_items: UserItems,         // Unified user-defined items (was user_types & user_functions)
    pub stages: Stages,                // Collection of stages
}

impl Parse for Workflow {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![,] = input.parse()?; // Expect a comma

        let user_imports: UseStatements = input.parse()?;
        let _: Token![,] = input.parse()?; // Expect a comma

        let user_items: UserItems = input.parse()?; // Replaces separate user_types and user_functions
        let _: Token![,] = input.parse()?; // Expect a comma

        let stages: Stages = input.parse()?; // Now parsing stages!

        Ok(Workflow {
            name,
            user_imports,
            user_items,
            stages,
        })
    }
}

impl Workflow {
    pub fn generate(self) -> TokenStream {
        let name = self.name;
        let imports = self.user_imports.generate();
        let user_items = self.user_items.generate();
        let stages = self.stages.generate();

        quote! {
            pub mod #name {
                #imports
                #user_items
                #stages
            }
        }
    }
}
