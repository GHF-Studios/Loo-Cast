use super::stage::{Async, Ecs, EcsWhile, Render, RenderWhile, StageSignature};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::marker::PhantomData;
use syn::{
    parse::{Parse, ParseStream},
    token::Pub,
    Fields, ItemEnum, ItemStruct, Result, Visibility,
};

fn align_core_struct(item: &mut ItemStruct) {
    let span = item.ident.span();

    item.vis = Visibility::Public(Pub(span));

    match &mut item.fields {
        Fields::Named(named) => {
            for field in &mut named.named {
                field.vis = Visibility::Public(Pub(span));
            }
        }
        Fields::Unnamed(unnamed) => {
            for field in &mut unnamed.unnamed {
                field.vis = Visibility::Public(Pub(span));
            }
        }
        Fields::Unit => {}
    }
}

fn align_core_enum(item: &mut ItemEnum) {
    let span = item.ident.span();

    item.vis = Visibility::Public(Pub(span));
}

pub struct Input;
pub struct State;
pub struct Output;
pub struct Error;

pub enum CoreType<T> {
    Struct(ItemStruct, PhantomData<T>),
    Enum(ItemEnum, PhantomData<T>),
}

impl CoreType<Input> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => item.to_token_stream(),
            CoreType::Enum(item, _) => item.to_token_stream(),
        }
    }
}

impl CoreType<State> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => item.to_token_stream(),
            CoreType::Enum(item, _) => item.to_token_stream(),
        }
    }
}

impl CoreType<Output> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => item.to_token_stream(),
            CoreType::Enum(item, _) => item.to_token_stream(),
        }
    }
}

impl CoreType<Error> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(std::fmt::Debug, Error)]
                    #item
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
            }
            CoreType::Enum(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(std::fmt::Debug, Error)]
                    #item
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
            }
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
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref s)
                    if s.ident == "State"
                        || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") =>
                {
                    return Err(input.error("State is not allowed in Ecs stages"));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            phantom_data: PhantomData,
            input: input_type,
            state: None,
            output: output_type,
            error: error_type,
        })
    }
}

impl Parse for CoreTypes<EcsWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut state_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "State" => {
                    align_core_struct(s);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "State" => {
                    align_core_enum(e);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            phantom_data: PhantomData,
            input: input_type,
            state: state_type,
            output: output_type,
            error: error_type,
        })
    }
}

impl Parse for CoreTypes<Render> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref s)
                    if s.ident == "State"
                        || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") =>
                {
                    return Err(input.error("State is not allowed in Render stages"));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            phantom_data: PhantomData,
            input: input_type,
            state: None,
            output: output_type,
            error: error_type,
        })
    }
}

impl Parse for CoreTypes<RenderWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut state_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "State" => {
                    align_core_struct(s);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "State" => {
                    align_core_enum(e);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            phantom_data: PhantomData,
            input: input_type,
            state: state_type,
            output: output_type,
            error: error_type,
        })
    }
}

impl Parse for CoreTypes<Async> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref s)
                    if s.ident == "State"
                        || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") =>
                {
                    return Err(input.error("State is not allowed in Async stages"));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            phantom_data: PhantomData,
            input: input_type,
            state: None,
            output: output_type,
            error: error_type,
        })
    }
}

impl<T> CoreTypes<T> {
    pub fn generate(&self) -> TokenStream {
        let input = self.input.as_ref().map(|t| t.generate());
        let state = self.state.as_ref().map(|t| t.generate());
        let output = self.output.as_ref().map(|t| t.generate());
        let error = self.error.as_ref().map(|t| t.generate());

        if self.error.is_some() {
            quote! {
                use thiserror::Error;

                #input
                #state
                #output
                #error
            }
        } else {
            quote! {
                #input
                #state
                #output
            }
        }
    }

    pub fn has_input(&self) -> bool {
        self.input.is_some()
    }

    pub fn has_output(&self) -> bool {
        self.output.is_some()
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn get_signature(&self) -> StageSignature {
        let has_input = self.has_input();
        let has_output = self.has_output();
        let has_error = self.has_error();

        match (has_input, has_output, has_error) {
            (false, false, false) => StageSignature::None,
            (false, false, true) => StageSignature::E,
            (false, true, false) => StageSignature::O,
            (false, true, true) => StageSignature::OE,
            (true, false, false) => StageSignature::I,
            (true, false, true) => StageSignature::IE,
            (true, true, false) => StageSignature::IO,
            (true, true, true) => StageSignature::IOE,
        }
    }
}
