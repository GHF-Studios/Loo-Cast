pub mod commands;

pub trait MacroInputParser: Sized {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self>;
}

impl<T: syn::parse::Parse> MacroInputParser for T {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        T::parse(input)
    }
}

pub trait CodeOutputGenerator: Sized {
    fn generate(self) -> syn::Result<Self>;
}