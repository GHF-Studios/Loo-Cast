use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Ident,
};

mod kw {
    syn::custom_keyword!(E);
    syn::custom_keyword!(O);
    syn::custom_keyword!(OE);
    syn::custom_keyword!(I);
    syn::custom_keyword!(IE);
    syn::custom_keyword!(IO);
    syn::custom_keyword!(IOE);
}

pub enum TypedWorkflowPath {
    None(WorkflowPath),
    E(WorkflowPath),
    O(WorkflowPath),
    OE(WorkflowPath),
    I(WorkflowPath),
    IE(WorkflowPath),
    IO(WorkflowPath),
    IOE(WorkflowPath),
}

pub struct WorkflowPath {
    workflow_module_name: Ident,
    workflow_name: Ident,
}

impl Parse for TypedWorkflowPath {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let workflow_module_name = input.parse()?;
        input.parse::<syn::Token![::]>()?;
        let workflow_name = input.parse()?;
        input.parse::<syn::Token![,]>()?;

        if input.is_empty() {
            return Ok(TypedWorkflowPath::None(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }));
        }

        let lookahead = input.lookahead1();
        if lookahead.peek(kw::E) {
            input.parse::<kw::E>()?;
            Ok(TypedWorkflowPath::E(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }))
        } else if lookahead.peek(kw::O) {
            input.parse::<kw::O>()?;
            Ok(TypedWorkflowPath::O(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }))
        } else if lookahead.peek(kw::OE) {
            input.parse::<kw::OE>()?;
            Ok(TypedWorkflowPath::OE(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }))
        } else if lookahead.peek(kw::I) {
            input.parse::<kw::I>()?;
            Ok(TypedWorkflowPath::I(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }))
        } else if lookahead.peek(kw::IE) {
            input.parse::<kw::IE>()?;
            Ok(TypedWorkflowPath::IE(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }))
        } else if lookahead.peek(kw::IO) {
            input.parse::<kw::IO>()?;
            Ok(TypedWorkflowPath::IO(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }))
        } else if lookahead.peek(kw::IOE) {
            input.parse::<kw::IOE>()?;
            Ok(TypedWorkflowPath::IOE(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }))
        } else {
            Err(lookahead.error())
        }
    }
}

impl TypedWorkflowPath {
    fn convert_to_raw(path: WorkflowPath) -> (Ident, Ident) {
        let WorkflowPath {
            workflow_module_name: workflow_module_ident,
            workflow_name: workflow_ident,
        } = path;

        let workflow_module_span = workflow_module_ident.span();
        let workflow_module_name = workflow_module_ident.to_string().to_snake_case();
        let workflow_module_ident = Ident::new(&workflow_module_name, workflow_module_span);

        let workflow_span = workflow_ident.span();
        let workflow_name = workflow_ident.to_string().to_snake_case();
        let workflow_ident = Ident::new(&workflow_name, workflow_span);

        (workflow_module_ident, workflow_ident)
    }

    pub fn generate(self) -> TokenStream {
        match self {
            TypedWorkflowPath::None(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::Type }
            }
            TypedWorkflowPath::E(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::TypeE }
            }
            TypedWorkflowPath::O(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::TypeO }
            }
            TypedWorkflowPath::OE(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::TypeOE }
            }
            TypedWorkflowPath::I(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::TypeI }
            }
            TypedWorkflowPath::IE(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIE }
            }
            TypedWorkflowPath::IO(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIO }
            }
            TypedWorkflowPath::IOE(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                quote! { crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIOE }
            }
        }
    }
}
