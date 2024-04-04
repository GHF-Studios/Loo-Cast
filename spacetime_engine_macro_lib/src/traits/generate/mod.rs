use proc_macro2::TokenStream;

pub trait Generate {
    fn generate_code_output(&self) -> TokenStream;
}