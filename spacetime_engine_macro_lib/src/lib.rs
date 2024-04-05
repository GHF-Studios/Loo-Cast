pub mod commands;

pub trait Parse: Sized {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self>;
}

impl<T: syn::parse::Parse> Parse for T {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        T::parse(input)
    }
}

pub trait Generate<Input: Parse + Sized>: Sized {
    fn generate(parsed_macro_input: &Input) -> syn::Result<Self>;
}

pub trait Collect<TGenerated: Generate<TInput>, TInput: Parse + Sized>: Sized {
    fn collect(inputs: Vec<TGenerated>) -> syn::Result<Self>;
}