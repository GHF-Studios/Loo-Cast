use proc_macro2::TokenStream;
use quote::quote;

/// Represents a collection of user-defined functions.
pub struct UserFunctions {
    pub functions: Vec<UserFunction>,
}

impl From<crate::workflow_mod::ir1::user_item::UserFunctions> for UserFunctions {
    fn from(ir1: crate::workflow_mod::ir1::user_item::UserFunctions) -> Self {
        Self {
            functions: ir1.0.into_iter().map(UserFunction::from).collect(),
        }
    }
}

impl UserFunctions {
    /// Generates Rust code for all user-defined functions.
    pub fn generate(&self) -> TokenStream {
        let functions: Vec<TokenStream> = self.functions.iter().map(|func| func.generate()).collect();

        quote! {
            #(#functions)*
        }
    }
}

/// Represents a user-defined function.
pub struct UserFunction {
    pub tokens: TokenStream, // Store the full function as a TokenStream
}

impl From<crate::workflow_mod::ir1::user_item::UserFunction> for UserFunction {
    fn from(ir1: crate::workflow_mod::ir1::user_item::UserFunction) -> Self {
        let signature = ir1.signature;
        let body = ir1.body;

        Self {
            tokens: quote!{#signature, #body},
        }
    }
}

impl UserFunction {
    /// Generates Rust code for the user-defined function.
    pub fn generate(&self) -> TokenStream {
        self.tokens.clone()
    }
}
