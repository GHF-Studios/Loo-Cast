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

pub struct WorkflowID(pub TypedWorkflowPath);

impl Parse for WorkflowID {
    fn parse(input: ParseStream) -> Result<Self> {
        let inner = input.parse::<TypedWorkflowPath>()?;
        Ok(WorkflowID(inner))
    }
}

impl WorkflowID {
    pub fn generate(self) -> proc_macro2::TokenStream {
        self.0.generate()
    }
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
        let fork = input.fork(); // Peek without consuming
        let lookahead = fork.lookahead1();

        // Parse the type keyword first
        let variant = if lookahead.peek(kw::E) {
            input.parse::<kw::E>()?;
            TypedWorkflowPath::E
        } else if lookahead.peek(kw::O) {
            input.parse::<kw::O>()?;
            TypedWorkflowPath::O
        } else if lookahead.peek(kw::OE) {
            input.parse::<kw::OE>()?;
            TypedWorkflowPath::OE
        } else if lookahead.peek(kw::I) {
            input.parse::<kw::I>()?;
            TypedWorkflowPath::I
        } else if lookahead.peek(kw::IE) {
            input.parse::<kw::IE>()?;
            TypedWorkflowPath::IE
        } else if lookahead.peek(kw::IO) {
            input.parse::<kw::IO>()?;
            TypedWorkflowPath::IO
        } else if lookahead.peek(kw::IOE) {
            input.parse::<kw::IOE>()?;
            TypedWorkflowPath::IOE
        } else {
            // If no type keyword, it's the `None` variant (no keyword)
            // Fall back to parsing normally
            let workflow_module_name = input.parse()?;
            input.parse::<syn::Token![::]>()?;
            let workflow_name = input.parse()?;

            return Ok(TypedWorkflowPath::None(WorkflowPath {
                workflow_module_name,
                workflow_name,
            }));
        };

        // After the type keyword, expect a comma
        input.parse::<syn::Token![,]>()?;

        // Then parse the module::workflow path
        let workflow_module_name = input.parse()?;
        input.parse::<syn::Token![::]>()?;
        let workflow_name = input.parse()?;

        // Construct with the right variant
        Ok(variant(WorkflowPath {
            workflow_module_name,
            workflow_name,
        }))
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
    
    fn with_workflow_type_attribute(path: TokenStream, type_name: &str) -> TokenStream {
        let attribute_ident = Ident::new(type_name, proc_macro2::Span::call_site());
        
        quote! {
            #[WorkflowSignature(#attribute_ident)]
            #path
        }
    }

    pub fn generate(self) -> TokenStream {
        match self {
            TypedWorkflowPath::None(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::Type 
                };
                Self::with_workflow_type_attribute(path, "None")
            }
            TypedWorkflowPath::E(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeE
                };
                Self::with_workflow_type_attribute(path, "E")
            }
            TypedWorkflowPath::O(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeO
                };
                Self::with_workflow_type_attribute(path, "O")
            }
            TypedWorkflowPath::OE(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeOE
                };
                Self::with_workflow_type_attribute(path, "OE")
            }
            TypedWorkflowPath::I(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeI
                };
                Self::with_workflow_type_attribute(path, "I")
            }
            TypedWorkflowPath::IE(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIE
                };
                Self::with_workflow_type_attribute(path, "IE")
            }
            TypedWorkflowPath::IO(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! { 
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIO
                };
                Self::with_workflow_type_attribute(path, "IO")
            }
            TypedWorkflowPath::IOE(path) => {
                let (workflow_module, workflow) = TypedWorkflowPath::convert_to_raw(path);
                let path = quote! {
                    crate::#workflow_module::workflows::#workflow_module::#workflow::TypeIOE
                };
                Self::with_workflow_type_attribute(path, "IOE")
            }
        }
    }
}
