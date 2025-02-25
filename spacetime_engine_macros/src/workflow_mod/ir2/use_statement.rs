use syn::{Path, Visibility};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;

/// Represents a collection of parsed `use` statements.
pub struct UseStatements {
    pub statements: Vec<UseStatement>,
}

impl From<crate::workflow_mod::ir1::use_statement::UseStatements> for UseStatements {
    fn from(ir1: crate::workflow_mod::ir1::use_statement::UseStatements) -> Self {
        Self {
            statements: ir1.0.into_iter().map(UseStatement::from).collect(),
        }
    }
}

impl UseStatements {
    /// Generates Rust code for all `use` statements.
    pub fn generate(&self) -> TokenStream {
        let imports: Vec<TokenStream> = self.statements.iter().map(|stmt| stmt.generate()).collect();

        quote! {
            #(#imports)*
        }
    }
}

/// Represents a parsed Rust `use` statement.
pub struct UseStatement {
    pub visibility: Option<Visibility>, // Example: "pub(crate)"
    pub path: Path,                     // Example: "bevy::prelude::*"
}

impl From<crate::workflow_mod::ir1::use_statement::UseStatement> for UseStatement {
    fn from(ir1: crate::workflow_mod::ir1::use_statement::UseStatement) -> Self {
        Self {
            visibility: ir1.visibility,
            path: ir1.path,
        }
    }
}

impl UseStatement {
    /// Generates a single `use` statement.
    pub fn generate(&self) -> TokenStream {
        let path = &self.path;
        let visibility = self.visibility.as_ref().map(ToTokens::to_token_stream);

        quote! {
            #visibility use #path;
        }
    }
}
