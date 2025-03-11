use syn::{parse::Parse, punctuated::Punctuated, spanned::Spanned, token::{In, Paren, Pub}, Ident, ItemUse, Path, PathSegment, Result, Token, VisRestricted, Visibility};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;

pub struct UseStatements(pub Vec<UseStatement>);

#[derive(Debug)]
pub struct UseStatement {
    pub use_statement: ItemUse,
}

impl Parse for UseStatements {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut imports = Vec::new();
        while !input.is_empty() {
            imports.push(input.parse()?);
        }
        Ok(UseStatements(imports))
    }
}

impl Parse for UseStatement {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let use_statement: ItemUse = input.parse()?;

        match use_statement.vis {
            Visibility::Inherited => {
                let use_statement = quote!(#use_statement).to_string();
                let use_statement = format!("pub(super) {}", use_statement);
                let use_statement: ItemUse = syn::parse_str(&use_statement)?;

                Ok(UseStatement { use_statement })
            }
            Visibility::Restricted(vis) => {
                Err(syn::Error::new(
                    vis.span(), 
                        "Use statements may not have an explicit visibility, 
                        because the visibility is automatically set to `pub(super)`."
                    )
                )
            },
            Visibility::Public(vis) => {
                Err(syn::Error::new(
                    vis.span(), 
                        "Use statements may not have an explicit visibility, 
                        because the visibility is automatically set to `pub(super)`."
                    )
                )
            }
        }
    }
}

impl UseStatements {
    pub fn generate(self) -> TokenStream {
        let imports: Vec<TokenStream> = self.0.into_iter().map(|stmt| stmt.generate()).collect();

        quote! {
            // Automatic imports
            pub use super::user_items::*;
            pub use crate::workflow::types::{WorkflowStageOutcome, WorkflowStageOutcome::Wait, WorkflowStageOutcome::Done};
            pub use bevy::prelude::World;
            
            // User imports
            #(#imports)*
        }
    }
}

impl UseStatement {
    pub fn generate(self) -> TokenStream {
        self.use_statement.into_token_stream()
    }
}
