use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Item, Token, parse_macro_input};
use syn::{ItemStruct, ItemEnum};
use quote::quote;

pub fn reflect_top_level_module(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
pub fn reflect_sub_module(input: TokenStream) -> TokenStream {
    TokenStream::new()
}
pub fn reflect_trait(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
pub fn reflect_trait_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
pub fn reflect_type(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
pub fn reflect_inherent_impl(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
pub fn reflect_module_associated_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
pub fn reflect_type_associated_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
pub fn reflect_constructor_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
pub fn reflect_method_function(attr: TokenStream, item: TokenStream) -> TokenStream {
    item
}
