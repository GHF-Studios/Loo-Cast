use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Ident;

/// Represents the fully expanded core types in a stage.
pub struct CoreTypes {
    pub input: Option<CoreStruct>,   // Fully formatted struct (or `None`)
    pub output: Option<CoreStruct>,  // Fully formatted struct (or `None`)
    pub error: Option<CoreEnum>,     // Fully formatted enum (or `None`)
    pub state: Option<CoreStruct>,   // Fully formatted struct (or `None`)
}

impl From<crate::workflow_mod::ir1::core_type::CoreTypes> for CoreTypes {
    fn from(ir1: crate::workflow_mod::ir1::core_type::CoreTypes) -> Self {
        Self {
            input: ir1.input.map(CoreStruct::from),
            output: ir1.output.map(CoreStruct::from),
            error: ir1.error.map(CoreEnum::from),
            state: ir1.state.map(CoreStruct::from),
        }
    }
}

impl CoreTypes {
    /// Generates Rust code for all core types.
    pub fn generate(&self) -> TokenStream {
        let input = self.input.as_ref().map(CoreStruct::generate);
        let output = self.output.as_ref().map(CoreStruct::generate);
        let error = self.error.as_ref().map(CoreEnum::generate);
        let state = self.state.as_ref().map(CoreStruct::generate);

        quote! {
            #input
            #output
            #error
            #state
        }
    }
}

/// Represents a parsed struct in core types.
pub struct CoreStruct {
    pub name: Ident,
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
    pub name: Ident,
    pub ty: String, // Simple String type
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
    pub name: Ident,
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
    pub name: Ident,
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
