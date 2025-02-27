use proc_macro2::TokenStream;
use syn::{parse::Parse, Ident, Token, Result, braced, punctuated::Punctuated};
use syn::parse::ParseStream;
use quote::{quote, ToTokens};

/// Represents all possible core types inside a stage.
pub enum CoreTypes {
    None,
    Input { input: CoreType },
    InputOutput { input: CoreType, output: CoreType },
    InputError { input: CoreType, error: CoreType },
    InputOutputError { input: CoreType, output: CoreType, error: CoreType },
    Output { output: CoreType },
    OutputError { output: CoreType, error: CoreType },
    Error { error: CoreType },

    While { state: CoreType },
    InputWhile { input: CoreType, state: CoreType },
    InputWhileOutput { input: CoreType, state: CoreType, output: CoreType },
    InputWhileError { input: CoreType, state: CoreType, error: CoreType },
    InputWhileOutputError { input: CoreType, state: CoreType, output: CoreType, error: CoreType },
    WhileOutput { state: CoreType, output: CoreType },
    WhileOutputError { state: CoreType, output: CoreType, error: CoreType },
    WhileError { state: CoreType, error: CoreType },
}

impl CoreTypes {
    /// Generates Rust code for the parsed core types.
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreTypes::None => quote! {},

            CoreTypes::Input { input } => input.generate(),
            CoreTypes::InputOutput { input, output } => {
                let input_tokens = input.generate();
                let output_tokens = output.generate();
                quote! { #input_tokens #output_tokens }
            }
            CoreTypes::InputError { input, error } => {
                let input_tokens = input.generate();
                let error_tokens = error.generate();
                quote! { #input_tokens #error_tokens }
            }
            CoreTypes::InputOutputError { input, output, error } => {
                let input_tokens = input.generate();
                let output_tokens = output.generate();
                let error_tokens = error.generate();
                quote! { #input_tokens #output_tokens #error_tokens }
            }
            CoreTypes::Output { output } => output.generate(),
            CoreTypes::OutputError { output, error } => {
                let output_tokens = output.generate();
                let error_tokens = error.generate();
                quote! { #output_tokens #error_tokens }
            }
            CoreTypes::Error { error } => error.generate(),

            CoreTypes::While { state } => state.generate(),
            CoreTypes::InputWhile { input, state } => {
                let input_tokens = input.generate();
                let state_tokens = state.generate();
                quote! { #input_tokens #state_tokens }
            }
            CoreTypes::InputWhileOutput { input, state, output } => {
                let input_tokens = input.generate();
                let state_tokens = state.generate();
                let output_tokens = output.generate();
                quote! { #input_tokens #state_tokens #output_tokens }
            }
            CoreTypes::InputWhileError { input, state, error } => {
                let input_tokens = input.generate();
                let state_tokens = state.generate();
                let error_tokens = error.generate();
                quote! { #input_tokens #state_tokens #error_tokens }
            }
            CoreTypes::InputWhileOutputError { input, state, output, error } => {
                let input_tokens = input.generate();
                let state_tokens = state.generate();
                let output_tokens = output.generate();
                let error_tokens = error.generate();
                quote! { #input_tokens #state_tokens #output_tokens #error_tokens }
            }
            CoreTypes::WhileOutput { state, output } => {
                let state_tokens = state.generate();
                let output_tokens = output.generate();
                quote! { #state_tokens #output_tokens }
            }
            CoreTypes::WhileOutputError { state, output, error } => {
                let state_tokens = state.generate();
                let output_tokens = output.generate();
                let error_tokens = error.generate();
                quote! { #state_tokens #output_tokens #error_tokens }
            }
            CoreTypes::WhileError { state, error } => {
                let state_tokens = state.generate();
                let error_tokens = error.generate();
                quote! { #state_tokens #error_tokens }
            }
        }
    }
}

/// Represents a struct or enum used as a core type.
pub enum CoreType {
    Struct(CoreStruct),
    Enum(CoreEnum),
}

impl CoreType {
    /// Generates Rust code for the core type.
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(core_struct) => core_struct.generate(),
            CoreType::Enum(core_enum) => core_enum.generate(),
        }
    }
}

/// Represents a parsed struct.
#[derive(Debug)]
pub struct CoreStruct {
    pub name: Ident,
    pub fields: Vec<CoreField>,
}

impl Parse for CoreStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![struct] = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let fields = Punctuated::<CoreField, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(CoreStruct { name, fields })
    }
}

impl CoreStruct {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let fields = self.fields.iter().map(|f| f.generate());

        quote! {
            pub struct #name {
                #(#fields),*
            }
        }
    }
}

/// Represents a struct field.
#[derive(Debug)]
pub struct CoreField {
    pub name: Ident,
    pub ty: TokenStream,
}

impl Parse for CoreField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let ty: syn::Type = input.parse()?;

        Ok(CoreField {
            name,
            ty: ty.to_token_stream(),
        })
    }
}

impl CoreField {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;

        quote! {
            pub #name: #ty
        }
    }
}

/// Represents a parsed enum.
#[derive(Debug)]
pub struct CoreEnum {
    pub name: Ident,
    pub variants: Vec<CoreEnumVariant>,
}

impl Parse for CoreEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let _: Token![enum] = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let variants = Punctuated::<CoreEnumVariant, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(CoreEnum { name, variants })
    }
}

impl CoreEnum {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let variants = self.variants.iter().map(|v| v.generate());

        quote! {
            pub enum #name {
                #(#variants),*
            }
        }
    }
}

/// Represents a single enum variant.
#[derive(Debug)]
pub struct CoreEnumVariant {
    pub name: Ident,
    pub fields: Vec<CoreField>,
}

impl Parse for CoreEnumVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let fields = Punctuated::<CoreField, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(CoreEnumVariant { name, fields })
    }
}

impl CoreEnumVariant {
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let fields = self.fields.iter().map(|f| f.generate());

        quote! {
            #name { #(#fields),* }
        }
    }
}
