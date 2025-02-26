use proc_macro2::{Span, TokenStream};
use syn::{parse::Parse, spanned::Spanned, Ident, Token, Result, Block, Signature, ItemFn, braced};
use quote::ToTokens;

use super::core_type::CoreTypes;

// TODO: This entire module is fucked. Rework it!

/// Represents all valid function sets for different stage types.
pub enum CoreFunctions {
    Single(CoreFunction),  // One function (RunEcs, RunRender, RunAsync)
    WhileFunctions { setup: CoreFunction, run: CoreFunction }, // Setup + Run functions for While stages
}

impl CoreFunctions {
    /// Returns a unique identifier for the CoreFunctions permutation.
    pub fn permutation(&self) -> &'static str {
        match self {
            CoreFunctions::Single(func) => match func.function_type {
                CoreFunctionType::RunEcs => "InputOutputError",  // Mirrors CoreTypes::InputOutputError
                CoreFunctionType::RunRender => "InputOutputError",
                CoreFunctionType::RunAsync => "InputOutputError",
                _ => "Invalid", // Should never happen
            },
            CoreFunctions::WhileFunctions { .. } => "While", // Mirrors CoreTypes::While*
        }
    }

    /// Validates that the provided functions match the expected core type pattern.
    pub fn validate(&self, core_types: &CoreTypes) -> Result<()> {
        let expected = core_types.permutation();
        let actual = self.permutation();

        if expected != actual {
            return Err(syn::Error::new(
                Span::call_site(),
                format!(
                    "Mismatch between CoreTypes ({}) and CoreFunctions ({}). Ensure function signatures align with expected input, output, and error types.",
                    expected, actual
                ),
            ));
        }

        Ok(())
    }
}

impl Parse for CoreFunctions {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut functions = Vec::new();

        while !input.is_empty() {
            functions.push(input.parse()?);
        }

        match functions.len() {
            1 => Ok(CoreFunctions::Single(functions.remove(0))),
            2 => {
                let setup = functions.remove(0);
                let run = functions.remove(0);
                
                // Ensure setup and run functions are of the same type (EcsWhile or RenderWhile)
                if !setup.function_type.is_setup_pair(&run.function_type) {
                    return Err(syn::Error::new(
                        run.signature.name.span(),
                        "Mismatched setup and run function types (e.g., SetupEcsWhile paired with RunRenderWhile is invalid)."
                    ));
                }
                
                Ok(CoreFunctions::WhileFunctions { setup, run })
            }
            0 => Err(syn::Error::new(
                input.span(),
                "Expected at least one core function, but found none."
            )),
            n => Err(syn::Error::new(
                input.span(),
                format!("Invalid number of core functions: expected 1 or 2, but found {}.", n),
            )),
        }
    }
}

/// Enum for function types within a stage.
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
    /// Checks if this function type is correctly paired with another (e.g., SetupEcsWhile → RunEcsWhile).
    fn is_setup_pair(&self, other: &CoreFunctionType) -> bool {
        matches!(
            (self, other),
            (CoreFunctionType::SetupEcsWhile, CoreFunctionType::RunEcsWhile)
                | (CoreFunctionType::SetupRenderWhile, CoreFunctionType::RunRenderWhile)
        )
    }
}

impl Parse for CoreFunctionType {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let func_name: Ident = input.parse()?; // Expect function name (e.g., "RunEcs")

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
    pub return_type: Option<TokenStream>, // Example: "Result<Output, Error>"
}

/// Represents a function parameter.
pub struct CoreFunctionParam {
    pub name: Ident,
    pub ty: TokenStream, // Example: "&mut World"
}

/// Represents a single function inside a stage.
pub struct CoreFunction {
    pub function_type: CoreFunctionType,   // Function type (RunEcs, SetupEcsWhile, etc.)
    pub signature: CoreFunctionSignature, // Parameters & return type
    pub body: TokenStream,                  // Raw Rust function body
}

impl Parse for CoreFunction {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        // Expect `fn`
        let _: Token![fn] = input.parse()?;

        // Parse function type (e.g., RunEcs, RunRender, etc.)
        let function_type: CoreFunctionType = input.parse()?;

        // Parse the function signature
        let func: ItemFn = input.parse()?;

        // Extract parameters
        let params: Vec<CoreFunctionParam> = func.sig.inputs.iter().map(|arg| {
            match arg {
                syn::FnArg::Typed(pat_type) => {
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

        // Extract return type
        let return_type = match &func.sig.output {
            syn::ReturnType::Type(_, ty) => Some(ty.to_token_stream()),
            syn::ReturnType::Default => None,
        };

        // Extract function body
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

        core_function.validate_signature()?;
        Ok(core_function)
    }
}

impl CoreFunction {
    /// Ensures that the function signature aligns with its function type.
    fn validate_signature(&self) -> Result<()> {
        let expects_input = matches!(
            self.function_type,
            CoreFunctionType::RunEcs | CoreFunctionType::RunRender | CoreFunctionType::RunAsync | CoreFunctionType::RunEcsWhile | CoreFunctionType::RunRenderWhile
        );

        let expects_output = matches!(
            self.function_type,
            CoreFunctionType::RunEcs | CoreFunctionType::RunRender | CoreFunctionType::RunAsync | CoreFunctionType::RunEcsWhile | CoreFunctionType::RunRenderWhile
        );

        let expects_error = matches!(
            self.function_type,
            CoreFunctionType::RunEcs | CoreFunctionType::RunRender | CoreFunctionType::RunAsync | CoreFunctionType::RunEcsWhile | CoreFunctionType::RunRenderWhile
        );

        let has_input = !self.signature.params.is_empty();
        let has_output = self.signature.return_type.is_some();
        let has_error = self.signature.return_type.as_ref().map(|r| r.contains("Result<")).unwrap_or(false);

        if expects_input != has_input {
            return Err(syn::Error::new(self.signature.name.span(), "Function signature mismatch: Incorrect input parameter presence."));
        }
        if expects_output != has_output {
            return Err(syn::Error::new(self.signature.name.span(), "Function signature mismatch: Incorrect output return presence."));
        }
        if expects_error != has_error {
            return Err(syn::Error::new(self.signature.name.span(), "Function signature mismatch: Expected Result<> return type but got something else."));
        }

        Ok(())
    }
}
