use syn::{parse::Parse, ItemUse, Path, Result, Token, Visibility};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;

pub struct UseStatements(pub Vec<UseStatement>);

impl Parse for UseStatements {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut imports = Vec::new();
        while !input.is_empty() {
            imports.push(input.parse()?);
        }
        Ok(UseStatements(imports))
    }
}

impl UseStatements {
    pub fn generate(self) -> TokenStream {
        let imports: Vec<TokenStream> = self.0.into_iter().map(|stmt| stmt.generate()).collect();

        quote! {
            // Automatic imports
            pub use super::user_items::*;
            pub use crate::workflow::types::{Outcome, Outcome::Wait, Outcome::Done};
            pub use bevy::prelude::World;
            
            // User imports
            #(#imports)*
        }
    }
}

#[derive(Debug)]
pub struct UseStatement {
    pub use_statement: ItemUse,
}

impl Parse for UseStatement {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let use_statement: ItemUse = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected a valid Rust path after `use`.")
        })?;

        Ok(UseStatement { use_statement })
    }
}

impl UseStatement {
    pub fn generate(self) -> TokenStream {
        self.use_statement.into_token_stream()
    }
}
