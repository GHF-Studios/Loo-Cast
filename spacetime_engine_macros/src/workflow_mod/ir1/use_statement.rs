use syn::{parse::Parse, Path, Token, Visibility, Result};
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
            #(#imports)*
        }
    }
}

#[derive(Debug)]
pub struct UseStatement {
    pub visibility: Option<Visibility>,
    pub path: Path,
}

impl Parse for UseStatement {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let visibility: Option<Visibility> = input.parse().ok();

        let _: Token![use] = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected `use` keyword to start an import statement.")
        })?;

        let path: Path = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected a valid Rust path after `use`.")
        })?;

        let _: Token![;] = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected `;` at the end of `use` statement.")
        })?;

        Ok(UseStatement { visibility, path })
    }
}

impl UseStatement {
    pub fn generate(self) -> TokenStream {
        let path = self.path;
        // TODO: Fix visibility being optional
        let visibility = self.visibility.map(|v| v.into_token_stream()).expect("DIGGAAAAAAAAAA VISIBILITY IS NICH OPTIONAL AMENA");

        quote! {
            #visibility use #path;
        }
    }
}
