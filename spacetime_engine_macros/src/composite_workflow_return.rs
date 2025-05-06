use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, Token, Type};

pub struct CompositeWorkflowReturn {
    pub returns: Vec<VarReturn>,
}

pub struct VarReturn {
    pub is_mut: bool,
    pub ident: Ident,
    pub ty: Type,
}

impl Parse for VarReturn {
    fn parse(input: ParseStream) -> Result<Self> {
        let is_mut = input.peek(Token![mut]);

        if is_mut {
            input.parse::<Token![mut]>()?;
        }

        let ident: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(VarReturn { is_mut, ident, ty })
    }
}

impl Parse for CompositeWorkflowReturn {
    fn parse(input: ParseStream) -> Result<Self> {
        let punctuated: Punctuated<VarReturn, Token![,]> = Punctuated::parse_terminated(input)?;
        Ok(CompositeWorkflowReturn {
            returns: punctuated.into_iter().collect(),
        })
    }
}

impl CompositeWorkflowReturn {
    pub fn generate(&self) -> TokenStream2 {
        let return_contexts = self.returns.iter().map(|ret_var| {
            let VarReturn { is_mut, ident, ty } = ret_var;
            let mut_token = if *is_mut { quote!(mut) } else { quote!() };
            quote! {
                let #mut_token #ident: #ty = get_context::<#ty>();
            }
        });

        quote! {
            use crate::workflow::composite_workflow_context::get_context;
            use crate::workflow::composite_workflow_context::clear_all_context;

            #(#return_contexts)*
            clear_all_context();
        }
    }
}
