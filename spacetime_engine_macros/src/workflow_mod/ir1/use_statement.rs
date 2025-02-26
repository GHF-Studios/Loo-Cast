use syn::{parse::Parse, Ident, Path, Token, Visibility, Result};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;

/// Represents a collection of parsed `use` statements.
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
    /// Generates Rust code for all `use` statements.
    pub fn generate(self) -> TokenStream {
        let imports: Vec<TokenStream> = self.0.into_iter().map(|stmt| stmt.generate()).collect();

        quote! {
            #(#imports)*
        }
    }
}

/// Represents a parsed Rust `use` statement.
#[derive(Debug)]
pub struct UseStatement {
    pub visibility: Option<Visibility>, // e.g., "pub", "pub(crate)", etc.
    pub path: Path,                     // Parsed Rust path (e.g., bevy::prelude::*)
}

impl Parse for UseStatement {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        // Try to parse an optional visibility modifier (e.g., "pub(crate)")
        let visibility: Option<Visibility> = input.parse().ok();

        // Expect the `use` keyword
        let _: Token![use] = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected `use` keyword to start an import statement.")
        })?;

        // Parse the full Rust path
        let path: Path = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected a valid Rust path after `use`.")
        })?;

        // Expect a `;` at the end
        let _: Token![;] = input.parse().map_err(|_| {
            syn::Error::new(input.span(), "Expected `;` at the end of `use` statement.")
        })?;

        Ok(UseStatement { visibility, path })
    }
}

impl UseStatement {
    /// Generates a single `use` statement.
    pub fn generate(self) -> TokenStream {
        let path = self.path;
        // TODO: Fix visibility being optional
        let visibility = self.visibility.map(|v| v.into_token_stream()).expect("DIGGAAAAAAAAAA VISIBILITY IS NICH OPTIONAL AMENA");

        quote! {
            #visibility use #path;
        }
    }
}
