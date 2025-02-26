use proc_macro2::TokenStream;
use syn::{parse::Parse, Item, Result};
use quote::{quote, ToTokens};

/// Represents a collection of user-defined Rust items (structs, enums, functions, etc.).
pub struct UserItems(pub Vec<UserItem>);

impl Parse for UserItems {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut items = Vec::new();
        while !input.is_empty() {
            items.push(input.parse()?);
        }
        Ok(UserItems(items))
    }
}

impl UserItems {
    /// Generates Rust code for all user-defined items.
    pub fn generate(self) -> TokenStream {
        let items: Vec<TokenStream> = self.0.into_iter().map(|item| item.generate()).collect();

        quote! {
            #(#items)*
        }
    }
}

/// Represents any valid Rust item that can appear inside a module.
pub struct UserItem {
    pub tokens: TokenStream,
}

impl Parse for UserItem {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let item: Item = input.parse()?;
        Ok(UserItem {
            tokens: item.to_token_stream(),
        })
    }
}

impl UserItem {
    pub fn generate(self) -> TokenStream {
        self.tokens
    }
}
