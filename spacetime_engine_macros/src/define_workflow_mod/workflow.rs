use syn::{parse::Parse, Ident, Token, braced, bracketed, Result, LitStr};
use quote::quote;
use proc_macro2::TokenStream;
use heck::ToSnakeCase;
use super::stage::Stages;
use super::use_statement::UseStatements;
use super::user_item::UserItems;

pub struct WorkflowModule {
    pub name: Ident,
    pub workflows: Vec<Workflow>,
}

impl Parse for WorkflowModule {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let _: super::kw::name = input.parse()?;
        input.parse::<Token![:]>()?;
        let name: LitStr = input.parse()?; 
        let name = Ident::new(&name.value(), name.span());

        input.parse::<Token![,]>()?;

        let _: super::kw::workflows = input.parse()?;
        input.parse::<Token![:]>()?;
        let content;
        bracketed!(content in input);

        let mut workflows = Vec::new();
        while !content.is_empty() {
            workflows.push(content.parse()?);
        }

        Ok(WorkflowModule { name, workflows })
    }
}

impl WorkflowModule {
    pub fn generate(self) -> TokenStream {
        let module_ident = &self.name;
        let module_name = module_ident.to_string();
        let module_ident = Ident::new(module_name.as_str().to_snake_case().as_str(), module_ident.span());
        let workflows = self.workflows.into_iter().map(|w| w.generate());

        quote! {
            pub mod #module_ident {
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
        let user_imports_content;
        braced!(user_imports_content in content);
        let user_imports: UseStatements = user_imports_content.parse()?;

        content.parse::<Token![,]>()?;

        let _: super::kw::user_items = content.parse()?;
        content.parse::<Token![:]>()?;
        let user_items_content;
        braced!(user_items_content in content);
        let user_items: UserItems = user_items_content.parse()?; 

        content.parse::<Token![,]>()?;

        let _: super::kw::stages = content.parse()?;
        content.parse::<Token![:]>()?;
        let stages_content;
        bracketed!(stages_content in content);
        let stages: Stages = stages_content.parse()?;
        
        let lookahead = content.lookahead1();
        if lookahead.peek(Token![,]) {
            let _ = content.parse::<Token![,]>()?;
        }

        Ok(Workflow { name, user_imports, user_items, stages })
    }
}

impl Workflow {
    pub fn generate(self) -> TokenStream {
        let workflow_ident = &self.name;
        let workflow_name = workflow_ident.to_string();
        let workflow_ident = Ident::new(workflow_name.as_str().to_snake_case().as_str(), workflow_ident.span());
        let imports = self.user_imports.generate();
        let user_items = self.user_items.generate();
        let stages = self.stages.0.into_iter().map(|s| s.generate());

        quote! {
            pub mod #workflow_ident {
                pub const NAME: &str = stringify!(#workflow_name);
                
                pub mod workflow_imports {
                    #imports
                }

                pub mod user_items {
                    use super::workflow_imports::*;

                    #user_items
                }

                pub mod stages {
                    #(#stages)*
                }
            }
        }
    }
}
