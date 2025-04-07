use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    Ident, Result, Token,
};
use heck::ToSnakeCase;

// --- Supported Signature Types ---
mod kw {
    syn::custom_keyword!(None);
    syn::custom_keyword!(E);
    syn::custom_keyword!(O);
    syn::custom_keyword!(OE);
    syn::custom_keyword!(I);
    syn::custom_keyword!(IE);
    syn::custom_keyword!(IO);
    syn::custom_keyword!(IOE);
}

#[derive(Debug, Clone)]
pub enum SignatureType {
    None,
    E,
    O,
    OE,
    I,
    IE,
    IO,
    IOE,
}

impl SignatureType {
    pub fn from_kw_optional(input: ParseStream) -> Result<Self> {
        let fork = input.fork();

        let lookahead = fork.lookahead1();
        if lookahead.peek(kw::E) {
            input.parse::<kw::E>()?;
            Ok(Self::E)
        } else if lookahead.peek(kw::O) {
            input.parse::<kw::O>()?;
            Ok(Self::O)
        } else if lookahead.peek(kw::OE) {
            input.parse::<kw::OE>()?;
            Ok(Self::OE)
        } else if lookahead.peek(kw::I) {
            input.parse::<kw::I>()?;
            Ok(Self::I)
        } else if lookahead.peek(kw::IE) {
            input.parse::<kw::IE>()?;
            Ok(Self::IE)
        } else if lookahead.peek(kw::IO) {
            input.parse::<kw::IO>()?;
            Ok(Self::IO)
        } else if lookahead.peek(kw::IOE) {
            input.parse::<kw::IOE>()?;
            Ok(Self::IOE)
        } else {
            Ok(Self::None)
        }
    }

    fn to_string(&self) -> &'static str {
        match self {
            Self::None => "None",
            Self::E => "E",
            Self::O => "O",
            Self::OE => "OE",
            Self::I => "I",
            Self::IE => "IE",
            Self::IO => "IO",
            Self::IOE => "IOE",
        }
    }

    fn to_suffix(&self) -> Option<&'static str> {
        match self {
            Self::None => None,
            _ => Some(self.to_string()),
        }
    }
}

#[derive(Debug)]
pub struct TypedWorkflowId {
    pub signature: SignatureType,
    pub namespace: Ident,
    pub workflow: Ident,
}

impl Parse for TypedWorkflowId {
    fn parse(input: ParseStream) -> Result<Self> {
        // Try parsing a signature keyword, fallback to None
        let signature = SignatureType::from_kw_optional(input)?;

        let namespace: Ident = input.parse()?;
        input.parse::<Token![::]>()?;
        let workflow: Ident = input.parse()?;

        Ok(Self {
            signature,
            namespace,
            workflow,
        })
    }
}

impl TypedWorkflowId {
    pub fn generate(&self) -> TokenStream2 {
        let workflow_module_name = self.namespace.to_string().to_snake_case();
        let workflow_name = self.workflow.to_string().to_snake_case();

        let sig_attr = self.signature.to_string();

        let type_suffix = self.signature.to_suffix().map_or_else(
            || quote! { Type },
            |suffix| {
                let ident = Ident::new(&format!("Type{}", suffix), Span::call_site());
                quote! { #ident }
            },
        );

        let namespace_mod = Ident::new(&workflow_module_name, self.namespace.span());
        let workflow_mod = Ident::new(&workflow_name, self.namespace.span());

        quote! {
            #[WorkflowSignature(#sig_attr)]
            crate::#namespace_mod::workflows::#namespace_mod::#workflow_mod::#type_suffix
        }
    }
}
