use proc_macro2::TokenStream;
use syn::{parse::Parse, ItemFn, Result, braced};
use quote::ToTokens;

/// Represents a collection of user-defined functions.
pub struct UserFunctions(pub Vec<UserFunction>);

impl Parse for UserFunctions {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut functions = Vec::new();
        while !input.is_empty() {
            functions.push(input.parse()?);
        }
        Ok(UserFunctions(functions))
    }
}

/// Represents a parsed user-defined function.
pub struct UserFunction {
    pub signature: TokenStream, // Function signature (name, parameters, return type)
    pub body: TokenStream,      // Function body
}

impl Parse for UserFunction {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        // Parse the function as a standard Rust function
        let func: ItemFn = input.parse()?;

        // Extract function signature
        let signature = func.sig.to_token_stream();

        // Extract function body
        let body = func.block.to_token_stream();

        Ok(UserFunction {
            signature,
            body,
        })
    }
}
