use proc_macro2::{Span, TokenStream};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse2, Attribute, ExprPath, ExprStruct, Ident, Result, Token,
};

#[derive(Debug, Clone)]
pub enum InvocationPart {
    Signature(Ident),
    Type(syn::ExprPath),
    Input(syn::ExprStruct),
}

// Used in `sub_macro.rs` expansion logic
#[derive(Debug, Clone)]
pub struct WorkflowInvocation {
    pub signature: Ident,
    pub workflow_type_path: ExprPath,
    pub input_struct: Option<ExprStruct>,
}

impl Parse for WorkflowInvocation {
    fn parse(input: ParseStream) -> Result<Self> {
        // First: pull out the #[WorkflowSignature(...)] attribute
        let attrs = input.call(Attribute::parse_outer)?;
        let signature_attr = attrs
            .iter()
            .find(|attr| attr.path().is_ident("WorkflowSignature"))
            .ok_or_else(|| {
                syn::Error::new(
                    Span::call_site(),
                    "Missing #[WorkflowSignature(...)] attribute",
                )
            })?;

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

impl WorkflowInvocation {
    pub fn from_parts(parts: Vec<InvocationPart>) -> Option<Self> {
        let mut signature: Option<Ident> = None;
        let mut workflow_type_path: Option<ExprPath> = None;
        let mut input_struct: Option<ExprStruct> = None;

        for part in parts {
            match part {
                InvocationPart::Signature(s) => signature = Some(s),
                InvocationPart::Type(p) => workflow_type_path = Some(p),
                InvocationPart::Input(i) => input_struct = Some(i),
            }
        }

        match (signature, workflow_type_path) {
            (Some(signature), Some(workflow_type_path)) => Some(Self {
                signature,
                workflow_type_path,
                input_struct,
            }),
            _ => None,
        }
    }
}
