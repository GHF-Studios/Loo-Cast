use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

/// Represents the fully expanded core types in a stage.
pub struct CoreTypes {
    pub input: Option<CoreType>,
    pub state: Option<CoreType>,
    pub output: Option<CoreType>,
    pub error: Option<CoreType>,
}

impl From<crate::workflow_mod::ir1::core_type::CoreTypes> for CoreTypes {
    fn from(ir1: crate::workflow_mod::ir1::core_type::CoreTypes) -> Self {
        let (input, output, error, state) = match ir1 {
            // Standard stages
            crate::workflow_mod::ir1::core_type::CoreTypes::None => (None, None, None, None),
            crate::workflow_mod::ir1::core_type::CoreTypes::Input { input } => (Some(input.into()), None, None, None),
            crate::workflow_mod::ir1::core_type::CoreTypes::InputOutput { input, output } => (Some(input.into()), None, Some(output.into()), None),
            crate::workflow_mod::ir1::core_type::CoreTypes::InputError { input, error } => (Some(input.into()), None, None, Some(error.into())),
            crate::workflow_mod::ir1::core_type::CoreTypes::InputOutputError { input, output, error } => (Some(input.into()), None, Some(output.into()), Some(error.into())),
            crate::workflow_mod::ir1::core_type::CoreTypes::Output { output } => (None, None, Some(output.into()), None),
            crate::workflow_mod::ir1::core_type::CoreTypes::OutputError { output, error } => (None, None, Some(output.into()), Some(error.into())),
            crate::workflow_mod::ir1::core_type::CoreTypes::Error { error } => (None, None, None, Some(error.into())),
        
            // While stages (must include `state`)
            crate::workflow_mod::ir1::core_type::CoreTypes::While { state } => (None, Some(state.into()), None, None),
            crate::workflow_mod::ir1::core_type::CoreTypes::InputWhile { input, state } => (Some(input.into()), Some(state.into()), None, None),
            crate::workflow_mod::ir1::core_type::CoreTypes::InputWhileOutput { input, state, output } => (Some(input.into()), Some(state.into()), Some(output.into()), None),
            crate::workflow_mod::ir1::core_type::CoreTypes::InputWhileError { input, state, error } => (Some(input.into()), Some(state.into()), None, Some(error.into())),
            crate::workflow_mod::ir1::core_type::CoreTypes::InputWhileOutputError { input, state, output, error } => (Some(input.into()), Some(state.into()), Some(output.into()), Some(error.into())),
            crate::workflow_mod::ir1::core_type::CoreTypes::WhileOutput { state, output } => (None, Some(state.into()), Some(output.into()), None),
            crate::workflow_mod::ir1::core_type::CoreTypes::WhileOutputError { state, output, error } => (None, Some(state.into()), Some(output.into()), Some(error.into())),
            crate::workflow_mod::ir1::core_type::CoreTypes::WhileError { state, error } => (None, Some(state.into()), None, Some(error.into())),
        };

        Self {
            input,
            output,
            error,
            state,
        }
    }
}

impl CoreTypes {
    /// Generates Rust code for all core types.
    pub fn generate(&self) -> TokenStream {
        let input = self.input.as_ref().map(CoreType::generate);
        let output = self.output.as_ref().map(CoreType::generate);
        let error = self.error.as_ref().map(CoreType::generate);
        let state = self.state.as_ref().map(CoreType::generate);

        quote! {
            #input
            #output
            #error
            #state
        }
    }
}


/// Represents a parsed core type.
pub enum CoreType {
    Struct(CoreStruct),
    Enum(CoreEnum),
}

/// Represents a parsed struct in core types.
pub struct CoreStruct {
    pub name: TokenStream,
    pub fields: Vec<CoreField>,
}

impl From<crate::workflow_mod::ir1::core_type::CoreStruct> for CoreStruct {
    fn from(ir1: crate::workflow_mod::ir1::core_type::CoreStruct) -> Self {
        Self {
            name: ir1.name,
            fields: ir1.fields.into_iter().map(CoreField::from).collect(),
        }
    }
}

impl CoreStruct {
    /// Generates Rust code for a core struct.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let fields: Vec<TokenStream> = self.fields.iter().map(CoreField::generate).collect();

        quote! {
            pub struct #name {
                #(#fields),*
            }
        }
    }
}

/// Represents a single field in a core struct.
pub struct CoreField {
    pub name: TokenStream,
    pub ty: TokenStream, // Simple String type
}

impl From<crate::workflow_mod::ir1::core_type::CoreField> for CoreField {
    fn from(ir1: crate::workflow_mod::ir1::core_type::CoreField) -> Self {
        Self {
            name: ir1.name,
            ty: ir1.ty,
        }
    }
}

impl CoreField {
    /// Generates Rust code for a struct field.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let ty = &self.ty;

        quote! {
            pub #name: #ty
        }
    }
}

/// Represents a parsed enum in core types.
pub struct CoreEnum {
    pub name: TokenStream,
    pub variants: Vec<CoreEnumVariant>,
}

impl From<crate::workflow_mod::ir1::core_type::CoreEnum> for CoreEnum {
    fn from(ir1: crate::workflow_mod::ir1::core_type::CoreEnum) -> Self {
        Self {
            name: ir1.name,
            variants: ir1.variants.into_iter().map(CoreEnumVariant::from).collect(),
        }
    }
}

impl CoreEnum {
    /// Generates Rust code for a core enum.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let variants: Vec<TokenStream> = self.variants.iter().map(CoreEnumVariant::generate).collect();

        quote! {
            pub enum #name {
                #(#variants),*
            }
        }
    }
}

/// Represents a single variant in a core enum.
pub struct CoreEnumVariant {
    pub name: TokenStream,
    pub fields: Vec<CoreField>,
}

impl From<crate::workflow_mod::ir1::core_type::CoreEnumVariant> for CoreEnumVariant {
    fn from(ir1: crate::workflow_mod::ir1::core_type::CoreEnumVariant) -> Self {
        Self {
            name: ir1.name,
            fields: ir1.fields.into_iter().map(CoreField::from).collect(),
        }
    }
}

impl CoreEnumVariant {
    /// Generates Rust code for an enum variant.
    pub fn generate(&self) -> TokenStream {
        let name = &self.name;
        let fields: Vec<TokenStream> = self.fields.iter().map(CoreField::generate).collect();

        quote! {
            #name { #(#fields),* }
        }
    }
}
