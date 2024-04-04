use proc_macro2::TokenStream;

pub trait Parse {
    fn parse_macro_input(input: TokenStream) -> Result<Self, syn::Error>
    where
        Self: Sized;
}