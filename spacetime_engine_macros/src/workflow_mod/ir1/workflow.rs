use syn::{parse::Parse, Ident, Token, Result};
use super::stage::Stage;
use super::use_statement::UseStatements;
use super::user_item::UserItems;

/// Represents the entire `workflow_mod! { ... }` macro input.
pub struct WorkflowModule {
    pub name: Ident,  
    pub workflows: Vec<Workflow>,  
}

impl Parse for WorkflowModule {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?; 
        input.parse::<Token![,]>()?;

        let mut workflows = Vec::new();
        while !input.is_empty() {
            workflows.push(input.parse()?);
        }

        Ok(WorkflowModule { name, workflows })
    }
}

/// Represents an individual workflow inside the module.
pub struct Workflow {
    pub name: Ident,                 
    pub user_imports: UseStatements,  
    pub user_items: UserItems,      
    pub stages: Vec<Stage>,         
}

impl Parse for Workflow {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?; 
        input.parse::<Token![,]>()?;

        let user_imports: UseStatements = input.parse()?;
        input.parse::<Token![,]>()?;

        let user_items: UserItems = input.parse()?; 
        input.parse::<Token![,]>()?;

        let mut stages = Vec::new();
        while !input.is_empty() {
            stages.push(input.parse()?);
        }

        Ok(Workflow { name, user_imports, user_items, stages })
    }
}
