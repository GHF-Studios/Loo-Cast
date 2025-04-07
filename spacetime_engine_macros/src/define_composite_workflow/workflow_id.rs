use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream, Result},
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

pub struct IdMacro(pub TypedWorkflowId);

impl Parse for IdMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        let id = input.parse::<TypedWorkflowId>()?;
        Ok(IdMacro(id))
    }
}

impl IdMacro {
    pub fn generate(self) -> TokenStream {
        self.0.generate()
    }
}

pub enum TypedWorkflowId {
    None(WorkflowId),
    E(WorkflowId),
    O(WorkflowId),
    OE(WorkflowId),
    I(WorkflowId),
    IE(WorkflowId),
    IO(WorkflowId),
    IOE(WorkflowId),
}

pub struct WorkflowId {
    workflow_module_ident: Ident,
    workflow_ident: Ident,
}

impl Parse for TypedWorkflowId {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let fork = input.fork(); // Peek without consuming
        let lookahead = fork.lookahead1();

        // Parse the type keyword first
        let variant = if lookahead.peek(kw::E) {
            input.parse::<kw::E>()?;
            TypedWorkflowId::E
        } else if lookahead.peek(kw::O) {
            input.parse::<kw::O>()?;
            TypedWorkflowId::O
        } else if lookahead.peek(kw::OE) {
            input.parse::<kw::OE>()?;
            TypedWorkflowId::OE
        } else if lookahead.peek(kw::I) {
            input.parse::<kw::I>()?;
            TypedWorkflowId::I
        } else if lookahead.peek(kw::IE) {
            input.parse::<kw::IE>()?;
            TypedWorkflowId::IE
        } else if lookahead.peek(kw::IO) {
            input.parse::<kw::IO>()?;
            TypedWorkflowId::IO
        } else if lookahead.peek(kw::IOE) {
            input.parse::<kw::IOE>()?;
            TypedWorkflowId::IOE
        } else {
            // If no type keyword, it's the `None` variant (no keyword)
            // Fall back to parsing normally
            let workflow_module_name = input.parse()?;
            input.parse::<syn::Token![::]>()?;
            let workflow_name = input.parse()?;

            return Ok(TypedWorkflowId::None(WorkflowId {
                workflow_module_ident: workflow_module_name,
                workflow_ident: workflow_name,
            }));
        };

        // After the type keyword, expect a comma
        input.parse::<syn::Token![,]>()?;

        // Then parse the module::workflow id
        let workflow_module_name = input.parse()?;
        input.parse::<syn::Token![::]>()?;
        let workflow_name = input.parse()?;

        // Construct with the right variant
        Ok(variant(WorkflowId {
            workflow_module_ident: workflow_module_name,
            workflow_ident: workflow_name,
        }))
    }
}

impl TypedWorkflowId {
    pub fn generate(self) -> TokenStream {
        match self {
            TypedWorkflowId::None(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::Type 
                };
                WorkflowId::with_workflow_type_attribute(id, "None")
            }
            TypedWorkflowId::E(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeE
                };
                WorkflowId::with_workflow_type_attribute(id, "E")
            }
            TypedWorkflowId::O(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeO
                };
                WorkflowId::with_workflow_type_attribute(id, "O")
            }
            TypedWorkflowId::OE(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeOE
                };
                WorkflowId::with_workflow_type_attribute(id, "OE")
            }
            TypedWorkflowId::I(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeI
                };
                WorkflowId::with_workflow_type_attribute(id, "I")
            }
            TypedWorkflowId::IE(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIE
                };
                WorkflowId::with_workflow_type_attribute(id, "IE")
            }
            TypedWorkflowId::IO(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIO
                };
                WorkflowId::with_workflow_type_attribute(id, "IO")
            }
            TypedWorkflowId::IOE(id) => {
                let (workflow_module, workflow) = WorkflowId::convert_to_raw(id);
                let id = quote! {
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIOE
                };
                WorkflowId::with_workflow_type_attribute(id, "IOE")
            }
        }
    }
}

impl WorkflowId {
    fn convert_to_raw(self) -> (Ident, Ident) {
        let WorkflowId {
            workflow_module_ident,
            workflow_ident,
        } = self;

        let workflow_module_span = workflow_module_ident.span();
        let workflow_module_name = workflow_module_ident.to_string().to_snake_case();
        let workflow_module_ident = Ident::new(&workflow_module_name, workflow_module_span);

        let workflow_span = workflow_ident.span();
        let workflow_name = workflow_ident.to_string().to_snake_case();
        let workflow_ident = Ident::new(&workflow_name, workflow_span);

        (workflow_module_ident, workflow_ident)
    }
    
    fn with_workflow_type_attribute(id: TokenStream, type_name: &str) -> TokenStream {
        let attribute_ident = Ident::new(type_name, proc_macro2::Span::call_site());
        
        quote! {
            #[WorkflowSignature(#attribute_ident)]
            #id
        }
    }
}
