use std::marker::PhantomData;

use proc_macro2::TokenStream;
use syn::{parse::Parse, spanned::Spanned, Ident, Token, Result, ItemFn, FnArg, ReturnType};
use quote::{quote, ToTokens};
use super::stage::{Ecs, EcsWhile, Render, RenderWhile, Async};

/// Enum representing the type of a core function.
pub enum CoreFunctionType {
    RunEcs,
    RunRender,
    RunAsync,
    SetupEcsWhile,
    RunEcsWhile,
    SetupRenderWhile,
    RunRenderWhile,
}

impl CoreFunctionType {
    /// Ensures that a setup function is correctly paired with a run function.
    fn is_valid_setup_pair(&self, other: &CoreFunctionType) -> bool {
        matches!(
            (self, other),
            (CoreFunctionType::SetupEcsWhile, CoreFunctionType::RunEcsWhile)
                | (CoreFunctionType::SetupRenderWhile, CoreFunctionType::RunRenderWhile)
        )
    }
}

impl Parse for CoreFunctionType {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let func_name: Ident = input.parse()?;

        match func_name.to_string().as_str() {
            "RunEcs" => Ok(CoreFunctionType::RunEcs),
            "RunRender" => Ok(CoreFunctionType::RunRender),
            "RunAsync" => Ok(CoreFunctionType::RunAsync),
            "SetupEcsWhile" => Ok(CoreFunctionType::SetupEcsWhile),
            "RunEcsWhile" => Ok(CoreFunctionType::RunEcsWhile),
            "SetupRenderWhile" => Ok(CoreFunctionType::SetupRenderWhile),
            "RunRenderWhile" => Ok(CoreFunctionType::RunRenderWhile),
            _ => Err(syn::Error::new(
                func_name.span(),
                "Invalid function type. Expected one of: RunEcs, RunRender, RunAsync, SetupEcsWhile, RunEcsWhile, SetupRenderWhile, RunRenderWhile."
            )),
        }
    }
}

/// Represents a parsed function signature.
pub struct CoreFunctionSignature {
    pub name: Ident,
    pub params: Vec<CoreFunctionParam>,
    pub return_type: Option<TokenStream>,
}

/// Represents a function parameter.
pub struct CoreFunctionParam {
    pub name: Ident,
    pub ty: TokenStream,
}

/// Represents a single function inside a stage.
pub struct CoreFunction {
    pub function_type: CoreFunctionType,
    pub signature: CoreFunctionSignature,
    pub body: TokenStream,
}

/// Represents all functions inside a stage
pub enum CoreFunctions<T> {
    Default { phantom_data: PhantomData<T>, run: CoreFunction },
    While { phantom_data: PhantomData<T>, setup: CoreFunction, run: CoreFunction }
}

impl Parse for CoreFunctions<Ecs> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.function_type, CoreFunctionType::RunEcs) {
            return Err(syn::Error::new(run.signature.name.span(), "Expected a `RunEcs` function in Ecs stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
    }
}

impl Parse for CoreFunctions<EcsWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(setup.function_type, CoreFunctionType::SetupEcsWhile) {
            return Err(syn::Error::new(setup.signature.name.span(), "Expected a `SetupEcsWhile` function as the first function in EcsWhile stage."));
        }
        if !matches!(run.function_type, CoreFunctionType::RunEcsWhile) {
            return Err(syn::Error::new(run.signature.name.span(), "Expected a `RunEcsWhile` function as the second function in EcsWhile stage."));
        }

        Ok(CoreFunctions::While { phantom_data: PhantomData, setup, run })
    }
}

impl Parse for CoreFunctions<Render> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.function_type, CoreFunctionType::RunRender) {
            return Err(syn::Error::new(run.signature.name.span(), "Expected a `RunRender` function in Render stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
    }
}

impl Parse for CoreFunctions<RenderWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(setup.function_type, CoreFunctionType::SetupRenderWhile) {
            return Err(syn::Error::new(setup.signature.name.span(), "Expected a `SetupRenderWhile` function as the first function in RenderWhile stage."));
        }
        if !matches!(run.function_type, CoreFunctionType::RunRenderWhile) {
            return Err(syn::Error::new(run.signature.name.span(), "Expected a `RunRenderWhile` function as the second function in RenderWhile stage."));
        }

        Ok(CoreFunctions::While { phantom_data: PhantomData, setup, run })
    }
}

impl Parse for CoreFunctions<Async> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.function_type, CoreFunctionType::RunAsync) {
            return Err(syn::Error::new(run.signature.name.span(), "Expected a `RunAsync` function in Async stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
    }
}

impl Parse for CoreFunction {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let _: Token![fn] = input.parse()?;
        let function_type: CoreFunctionType = input.parse()?;
        let func: ItemFn = input.parse()?;

        let params: Vec<CoreFunctionParam> = func.sig.inputs.iter().map(|arg| {
            match arg {
                FnArg::Typed(pat_type) => {
                    let name = match &*pat_type.pat {
                        syn::Pat::Ident(ident) => ident.ident.clone(),
                        _ => return Err(syn::Error::new(pat_type.span(), "Unexpected function parameter format.")),
                    };
                    let ty = pat_type.ty.to_token_stream();
                    Ok(CoreFunctionParam { name, ty })
                }
                _ => Err(syn::Error::new(arg.span(), "Unexpected function parameter format.")),
            }
        }).collect::<Result<Vec<_>>>()?;

        let return_type = match &func.sig.output {
            ReturnType::Type(_, ty) => Some(ty.to_token_stream()),
            ReturnType::Default => None,
        };

        let body = func.block.to_token_stream();

        let signature = CoreFunctionSignature {
            name: func.sig.ident.clone(),
            params,
            return_type,
        };

        let core_function = CoreFunction {
            function_type,
            signature,
            body,
        };

        core_function.validate()?;
        Ok(core_function)
    }
}

impl CoreFunction {
    fn validate(&self) -> Result<()> {
        use CoreFunctionType::*;

        let expects_input = matches!(
            self.function_type,
            RunEcs | RunRender | RunAsync | RunEcsWhile | RunRenderWhile
        );

        let expects_world_param = matches!(
            self.function_type,
            RunEcs | RunRender | RunEcsWhile | RunRenderWhile
        );

        let is_async = matches!(self.function_type, RunAsync);

        let has_input = self.signature.params.iter().any(|param| param.name.to_string() == "input");
        let has_world_param = self.signature.params.iter().any(|param| param.ty.to_string().contains("&mut World"));
        
        if expects_world_param && !has_world_param {
            return Err(syn::Error::new(
                self.signature.name.span(),
                "Expected `world: &mut World` parameter, but it is missing."
            ));
        }

        if is_async && has_world_param {
            return Err(syn::Error::new(
                self.signature.name.span(),
                "Async functions cannot take `world: &mut World` as a parameter."
            ));
        }

        if expects_input && !has_input {
            return Err(syn::Error::new(
                self.signature.name.span(),
                "Function signature mismatch: Expected an `input` parameter."
            ));
        }

        Ok(())
    }

    /// Generates the Rust function based on the function type.
    pub fn generate(&self) -> TokenStream {
        let name = &self.signature.name;
        let body = &self.body;
        
        let params = self.signature.params.iter().map(|p| {
            let name = &p.name;
            let ty = &p.ty;
            quote! { #name: #ty }
        });

        let return_type = self.signature.return_type.as_ref().map(|r| quote! { -> #r }).unwrap_or(quote! {});

        quote! {
            pub fn #name(#(#params),*) #return_type {
                #body
            }
        }
    }
}
