use proc_macro2::{TokenStream, Span};
use quote::quote;
use syn::{
    Attribute, ExprPath, Ident, Result, Token, parse::{Parse, ParseStream}, ExprStruct, braced, parse2,
};

// Used in `sub_macro.rs` expansion logic
#[derive(Debug)]
pub struct WorkflowMacro {
    pub signature: Ident,
    pub workflow_type_path: ExprPath,
    pub input_struct: Option<ExprStruct>,
}

impl Parse for WorkflowMacro {
    fn parse(input: ParseStream) -> Result<Self> {
        // First: pull out the #[WorkflowSignature(...)] attribute
        let attrs = input.call(Attribute::parse_outer)?;
        let signature_attr = attrs
            .iter()
            .find(|attr| attr.path().is_ident("WorkflowSignature"))
            .ok_or_else(|| syn::Error::new(Span::call_site(), "Missing #[WorkflowSignature(...)] attribute"))?;

        let signature: Ident = signature_attr.parse_args()?;

        // Next: the workflow type path
        let workflow_type_path: ExprPath = input.parse()?;

        // Optional: input block
        let input_struct = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;

            let lookahead = input.lookahead1();
            if lookahead.peek(Ident) && input.peek2(syn::token::Brace) {
                let input_keyword: Ident = input.parse()?;
                if input_keyword != "Input" {
                    return Err(syn::Error::new_spanned(input_keyword, "Expected `Input`"));
                }

                let content;
                braced!(content in input);
                let content: TokenStream = content.parse()?;
                let input_expr: ExprStruct = parse2(content)?;

                Some(input_expr)
            } else {
                return Err(syn::Error::new(input.span(), "Expected `Input { ... }`"));
            }
        } else {
            None
        };

        Ok(Self {
            signature,
            workflow_type_path,
            input_struct,
        })
    }
}

impl WorkflowMacro {
    pub fn generate(&self) -> TokenStream {
        let sig = &self.signature;
        let path = &self.workflow_type_path;

        let signature_attr = quote! { #[WorkflowSignature(#sig)] };
        let type_attr = quote! { WorkflowType(#path) };

        let input_attr = self.input_struct.as_ref().map(|input| {
            quote! { WorkflowInput #input }
        });

        let attrs = match input_attr {
            Some(i) => quote! { #[#signature_attr, #type_attr, #i] },
            None => quote! { #[#signature_attr, #type_attr] },
        };

        quote! {
            #attrs
        }
    }
}
