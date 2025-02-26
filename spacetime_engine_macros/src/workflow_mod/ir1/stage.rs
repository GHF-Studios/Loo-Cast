use syn::{parse::Parse, Result, Ident, Token};
use proc_macro2::TokenStream;
use quote::quote;
use super::core_type::CoreTypes;
use super::core_function::{CoreFunctions, CoreFunctionType};

pub struct Stages(pub Vec<Stage>);

impl Parse for Stages {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut stages = Vec::new();
        while !input.is_empty() {
            stages.push(input.parse()?);
        }
        Ok(Stages(stages))
    }
}

impl Stages {
    pub fn generate(self) -> TokenStream {
        let stages: Vec<TokenStream> = self.0.into_iter().map(Stage::generate).collect();
        quote! {
            #(#stages)*
        }
    }
}

pub struct Stage {
    pub name: Ident,
    pub stage_type: StageType,
    pub core_types: CoreTypes,
    pub core_functions: CoreFunctions,
}

impl Parse for Stage {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        // Parse stage name
        let name: Ident = input.parse()?;
        let _: Token![,] = input.parse()?;

        // Parse core types
        let core_types: CoreTypes = input.parse()?;
        let _: Token![,] = input.parse()?;

        // Parse core functions
        let core_functions: CoreFunctions = input.parse()?;

        // Validate function permutations against core type permutations
        core_functions.validate(&core_types)?;

        // Infer stage type based on core functions
        let stage_type = StageType::infer(&core_functions)?;

        Ok(Stage {
            name,
            stage_type,
            core_types,
            core_functions,
        })
    }
}

impl Stage {
    pub fn generate(self) -> TokenStream {
        let name = self.name;
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        quote! {
            pub mod #name {
                #core_types

                #core_functions
            }
        }
    }
}

/// Enum for the five possible stage types.
#[derive(Debug)]
pub enum StageType {
    Ecs,
    EcsWhile,
    Render,
    RenderWhile,
    Async,
}

impl StageType {
    /// Infers the `StageType` from provided core functions.
    fn infer(core_functions: &CoreFunctions) -> Result<Self> {
        match core_functions {
            CoreFunctions::Single(func) => match func.function_type {
                CoreFunctionType::RunEcs => Ok(StageType::Ecs),
                CoreFunctionType::RunRender => Ok(StageType::Render),
                CoreFunctionType::RunAsync => Ok(StageType::Async),
                _ => Err(syn::Error::new(
                    func.signature.name.span(),
                    "Invalid function type for a single-function stage. Expected RunEcs or RunRender or RunAsync.",
                )),
            },
            CoreFunctions::WhileFunctions { setup, run } => match (&setup.function_type, &run.function_type) {
                (CoreFunctionType::SetupEcsWhile, CoreFunctionType::RunEcsWhile) => Ok(StageType::EcsWhile),
                (CoreFunctionType::SetupRenderWhile, CoreFunctionType::RunRenderWhile) => Ok(StageType::RenderWhile),
                _ => Err(syn::Error::new(
                    run.signature.name.span(),
                    "Invalid setup-run function pair. Expected (SetupEcsWhile, RunEcsWhile) or (SetupRenderWhile, RunRenderWhile).",
                )),
            },
        }
    }
}
