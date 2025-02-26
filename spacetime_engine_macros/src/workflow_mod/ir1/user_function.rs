use proc_macro2::TokenStream;
use syn::{parse::Parse, ItemFn, Result};
use quote::{quote, ToTokens};

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

impl UserFunctions {
    pub fn generate(self) -> TokenStream {
        let functions: Vec<TokenStream> = self.0.into_iter().map(|func| func.generate()).collect();

        quote! {
            #(#functions)*
        }
    }
}

pub struct UserFunction {
    pub item_fn: TokenStream,
}

impl Parse for UserFunction {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let func: ItemFn = input.parse()?;
        let item_fn = func.to_token_stream();

        Ok(UserFunction {
            item_fn,
        })
    }
}

impl UserFunction {
    pub fn generate(self) -> TokenStream {
        self.item_fn
    }
}
