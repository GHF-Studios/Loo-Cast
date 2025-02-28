use syn::bracketed;
use syn::{parse::Parse, Ident, Token, Result, LitStr};
use quote::quote;
use proc_macro2::TokenStream;
use super::stage::Stage;
use super::use_statement::UseStatements;
use super::user_item::UserItems;

pub struct WorkflowModule {
    pub name: Ident,  
    pub workflows: Vec<Workflow>,  
}

impl Parse for WorkflowModule {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name_label: Ident = input.parse()?;
        if name_label != "name" {
            return Err(syn::Error::new(name_label.span(), "Expected 'name'"));
        }
        input.parse::<Token![:]>()?;
        let name: LitStr = input.parse()?; 
        let name = Ident::new(&name.value(), name.span());

        input.parse::<Token![,]>()?;

        
        let workflows_label: Ident = input.parse()?;
        if workflows_label != "workflows" {
            return Err(syn::Error::new(workflows_label.span(), "Expected 'workflows'"));
        }
        input.parse::<Token![:]>()?;

        let content;
        bracketed!(content in input);

        let mut workflows = Vec::new();
        while !input.is_empty() {
            workflows.push(content.parse()?);
        }

        Ok(WorkflowModule { name, workflows })
    }
}

impl WorkflowModule {
    pub fn generate(self) -> TokenStream {
        let module_name = &self.name;
        let workflows = self.workflows.into_iter().map(|w| w.generate());

        quote! {
            pub mod #module_name {
                pub const NAME: &str = stringify!(#module_name);
                #(#workflows)*
            }
        }
    }
}

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

impl Workflow {
    pub fn generate(self) -> TokenStream {
        let workflow_name = &self.name;
        let imports = self.user_imports.generate();
        let user_items = self.user_items.generate();
        let stages = self.stages.into_iter().map(|s| s.generate());

        quote! {
            pub mod #workflow_name {
                pub const NAME: &str = stringify!(#workflow_name);
                
                pub mod workflow_imports {
                    // Automatic imports
                    pub use super::user_types::*;
                    pub use super::user_functions::*;
                    pub use crate::workflow::types::{Outcome, Outcome::Wait, Outcome::Done};
                    
                    // User imports
                    #imports
                }

                pub mod user_types {
                    #user_items
                }

                pub mod stages {
                    #(#stages)*
                }
            }
        }
    }
}
