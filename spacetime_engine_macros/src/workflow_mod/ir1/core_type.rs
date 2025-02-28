use std::marker::PhantomData;
use syn::{parse::Parse, ItemStruct, ItemEnum, Result, parse::ParseStream};
use quote::{quote, ToTokens};
use proc_macro2::TokenStream;
use super::stage::{Ecs, EcsWhile, Render, RenderWhile, Async};

pub struct Input;
pub struct State;
pub struct Output;
pub struct Error;

pub enum CoreType<T> {
    Struct(ItemStruct, PhantomData<T>),
    Enum(ItemEnum, PhantomData<T>),
}

impl<T> CoreType<T> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => item.to_token_stream(),
            CoreType::Enum(item, _) => item.to_token_stream(),
        }
    }
}

pub struct CoreTypes<T> {
    pub phantom_data: PhantomData<T>,
    pub input: Option<CoreType<Input>>,
    pub state: Option<CoreType<State>>,
    pub output: Option<CoreType<Output>>,
    pub error: Option<CoreType<Error>>,
}

impl Parse for CoreTypes<Ecs> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref s) if s.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "State" || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") => {
                    return Err(input.error("State is not allowed in Ecs stages"));
                },
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes { phantom_data: PhantomData, input: input_type, state: None, output: output_type, error: error_type })
    }
}

impl Parse for CoreTypes<EcsWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut state_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref s) if s.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "State" => {
                    if state_type.is_some() { return Err(input.error("Duplicate State type")); }
                    state_type = Some(CoreType::<State>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "State" => {
                    if state_type.is_some() { return Err(input.error("Duplicate State type")); }
                    state_type = Some(CoreType::<State>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                },
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes { phantom_data: PhantomData, input: input_type, state: state_type, output: output_type, error: error_type })
    }
}

impl Parse for CoreTypes<Render> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref s) if s.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "State" || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") => {
                    return Err(input.error("State is not allowed in Render stages"));
                },
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes { phantom_data: PhantomData, input: input_type, state: None, output: output_type, error: error_type })
    }
}

impl Parse for CoreTypes<RenderWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut state_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref s) if s.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "State" => {
                    if state_type.is_some() { return Err(input.error("Duplicate State type")); }
                    state_type = Some(CoreType::<State>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "State" => {
                    if state_type.is_some() { return Err(input.error("Duplicate State type")); }
                    state_type = Some(CoreType::<State>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                },
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes { phantom_data: PhantomData, input: input_type, state: state_type, output: output_type, error: error_type })
    }
}

impl Parse for CoreTypes<Async> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref s) if s.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Input" => {
                    if input_type.is_some() { return Err(input.error("Duplicate Input type")); }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Output" => {
                    if output_type.is_some() { return Err(input.error("Duplicate Output type")); }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                },
                syn::Item::Enum(ref e) if e.ident == "Error" => {
                    if error_type.is_some() { return Err(input.error("Duplicate Error type")); }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                },
                syn::Item::Struct(ref s) if s.ident == "State" || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") => {
                    return Err(input.error("State is not allowed in Async stages"));
                },
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes { phantom_data: PhantomData, input: input_type, state: None, output: output_type, error: error_type })
    }
}

impl<T> CoreTypes<T> {
    pub fn generate(&self) -> TokenStream {
        let input = self.input.as_ref().map(|t| t.generate());
        let state = self.state.as_ref().map(|t| t.generate());
        let output = self.output.as_ref().map(|t| t.generate());
        let error = self.error.as_ref().map(|t| t.generate());

        quote! {
            #input
            #state
            #output
            #error
        }
    }
}
