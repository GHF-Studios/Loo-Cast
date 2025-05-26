use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse::Parse, Item, Result};

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
    pub fn generate(self) -> TokenStream {
        let items: Vec<TokenStream> = self.0.into_iter().map(|item| item.generate()).collect();

        quote! {
            #(#items)*
        }
    }
}

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
