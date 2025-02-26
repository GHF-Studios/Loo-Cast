use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

/// Represents a collection of core functions in a stage.
pub struct CoreFunctions {
    pub functions: Vec<CoreFunction>,
}

impl From<crate::workflow_mod::ir1::core_function::CoreFunctions> for CoreFunctions {
    fn from(ir1: crate::workflow_mod::ir1::core_function::CoreFunctions) -> Self {
        let ir1_functions = match ir1 {
            crate::workflow_mod::ir1::core_function::CoreFunctions::Single(run) => vec![run],
            crate::workflow_mod::ir1::core_function::CoreFunctions::WhileFunctions { setup, run } => vec![setup, run]
        };

        Self {
            functions: ir1_functions.into_iter().map(CoreFunction::from).collect(),
        }
    }
}

impl CoreFunctions {
    /// Generates Rust code for all core functions.
    pub fn generate(&self) -> TokenStream {
        let functions: Vec<TokenStream> = self.functions.iter().map(|func| func.generate()).collect();

        quote! {
            #(#functions)*
        }
    }
}

/// Represents a fully expanded function inside a stage.
pub struct CoreFunction {
    pub function_type: CoreFunctionType, // Determines function behavior
    pub signature: CoreFunctionSignature, // Structured function signature
    pub body: TokenStream, // Function body as raw Rust code
}

impl From<crate::workflow_mod::ir1::core_function::CoreFunction> for CoreFunction {
    fn from(ir1: crate::workflow_mod::ir1::core_function::CoreFunction) -> Self {
        Self {
            function_type: ir1.function_type.into(),
            signature: ir1.signature.into(),
            body: ir1.body,
        }
    }
}

impl CoreFunction {
    /// Generates Rust code for the function.
    pub fn generate(&self) -> TokenStream {
        let fn_name = &self.signature.name;
        let params = self.signature.params.iter().map(|p| p.generate());
        let return_type = self.signature.return_type.as_ref().map(|r| quote! { -> #r });

        let body = &self.body;

        quote! {
            fn #fn_name(#(#params),*) #return_type {
                #body
            }
        }
    }
}

/// Represents the signature of a core function.
pub struct CoreFunctionSignature {
    pub name: Ident,
    pub params: Vec<CoreFunctionParam>,
    pub return_type: Option<TokenStream>, // Example: "Result<Output, Error>"
}

impl From<crate::workflow_mod::ir1::core_function::CoreFunctionSignature> for CoreFunctionSignature {
    fn from(ir1: crate::workflow_mod::ir1::core_function::CoreFunctionSignature) -> Self {
        Self {
            name: ir1.name,
            params: ir1.params.into_iter().map(CoreFunctionParam::from).collect(),
            return_type: ir1.return_type,
        }
    }
}

/// Represents a function parameter.
pub struct CoreFunctionParam {
    pub name: Ident,
    pub ty: TokenStream, // Example: "World &mut"
}

impl From<crate::workflow_mod::ir1::core_function::CoreFunctionParam> for CoreFunctionParam {
    fn from(ir1: crate::workflow_mod::ir1::core_function::CoreFunctionParam) -> Self {
        Self {
            name: ir1.name,
            ty: ir1.ty,
        }
    }
}

impl CoreFunctionParam {
    /// Generates Rust code for a function parameter.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;

        quote! {
            #name: #ty
        }
    }
}

/// Enum for function types inside a stage.
pub enum CoreFunctionType {
    RunEcs,
    RunRender,
    RunAsync,
    SetupEcsWhile,
    RunEcsWhile,
    SetupRenderWhile,
    RunRenderWhile,
}

impl From<crate::workflow_mod::ir1::core_function::CoreFunctionType> for CoreFunctionType {
    fn from(ir1: crate::workflow_mod::ir1::core_function::CoreFunctionType) -> Self {
        match ir1 {
            crate::workflow_mod::ir1::core_function::CoreFunctionType::RunEcs => CoreFunctionType::RunEcs,
            crate::workflow_mod::ir1::core_function::CoreFunctionType::RunRender => CoreFunctionType::RunRender,
            crate::workflow_mod::ir1::core_function::CoreFunctionType::RunAsync => CoreFunctionType::RunAsync,
            crate::workflow_mod::ir1::core_function::CoreFunctionType::SetupEcsWhile => CoreFunctionType::SetupEcsWhile,
            crate::workflow_mod::ir1::core_function::CoreFunctionType::RunEcsWhile => CoreFunctionType::RunEcsWhile,
            crate::workflow_mod::ir1::core_function::CoreFunctionType::SetupRenderWhile => CoreFunctionType::SetupRenderWhile,
            crate::workflow_mod::ir1::core_function::CoreFunctionType::RunRenderWhile => CoreFunctionType::RunRenderWhile,
        }
    }
}
