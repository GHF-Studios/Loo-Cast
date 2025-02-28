use syn::{parse::Parse, Ident, Token, braced, bracketed, Result, LitStr};
use quote::quote;
use proc_macro2::TokenStream;
use heck::ToSnakeCase;
use crate::workflow_mod::ir1::stage;

use super::stage::Stages;
use super::use_statement::UseStatements;
use super::user_item::UserItems;

pub struct WorkflowModule {
    pub pascal_case_name: Ident,
    pub snake_case_name: Ident,
    pub workflows: Vec<Workflow>,
}

impl Parse for WorkflowModule {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let _: super::kw::name = input.parse()?;
        input.parse::<Token![:]>()?;
        let name: LitStr = input.parse()?; 
        let pascal_case_name = Ident::new(&name.value(), name.span());
        let snake_case_name = name.value().to_snake_case();
        let snake_case_name = Ident::new(&snake_case_name, name.span());

        input.parse::<Token![,]>()?;

        let _: super::kw::workflows = input.parse()?;
        input.parse::<Token![:]>()?;
        let content;
        bracketed!(content in input);

        let mut workflows = Vec::new();
        while !input.is_empty() {
            workflows.push(content.parse()?);
        }

        Ok(WorkflowModule { pascal_case_name, snake_case_name, workflows })
    }
}

impl WorkflowModule {
    pub fn generate(self) -> TokenStream {
        let module_name = &self.pascal_case_name;
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
    pub stages: Stages,         
}

impl Parse for Workflow {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?; 
        
        let content;
        braced!(content in input);

        let _: super::kw::user_imports = content.parse()?;
        content.parse::<Token![:]>()?;
        let inner_content;
        braced!(inner_content in content);
        let user_imports: UseStatements = inner_content.parse()?;

        content.parse::<Token![,]>()?;

        let _: super::kw::user_items = content.parse()?;
        content.parse::<Token![:]>()?;
        let inner_content;
        braced!(inner_content in content);
        let user_items: UserItems = inner_content.parse()?; 

        content.parse::<Token![,]>()?;

        let _: super::kw::stages = content.parse()?;
        content.parse::<Token![:]>()?;
        let inner_content;
        bracketed!(inner_content in content);
        let stages: Stages = inner_content.parse()?;
        
        let lookahead = content.lookahead1();
        if lookahead.peek(Token![,]) {
            let _ = content.parse::<Token![,]>()?;
        }

        Ok(Workflow { name, user_imports, user_items, stages })
    }
}

impl Workflow {
    pub fn generate(self) -> TokenStream {
        let workflow_name = &self.name;
        let imports = self.user_imports.generate();
        let user_items = self.user_items.generate();
        let stages = self.stages.0.into_iter().map(|s| s.generate());

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
