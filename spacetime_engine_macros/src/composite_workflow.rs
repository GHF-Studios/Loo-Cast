use syn::{Ident, Type, Token, Block, braced};
use syn::parse::{Parse, ParseStream, Result};
use quote::quote;
use proc_macro2::TokenStream as TokenStream2;

pub struct CompositeWorkflow {
    pub captures: Vec<VarCapture>,
    pub workflow_name: Ident,
    pub block: Block,
}

pub struct VarCapture {
    pub ident: Ident,
    pub ty: Type,
}

impl Parse for CompositeWorkflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut captures = Vec::new();

        while input.peek(Ident) && input.peek2(Token![:]) {
            let ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty: Type = input.parse()?;
            input.parse::<Token![,]>()?;
            captures.push(VarCapture { ident, ty });
        }

        let workflow_name: Ident = input.parse()?;
        let block: Block = input.parse()?;

        Ok(CompositeWorkflow {
            captures,
            workflow_name,
            block,
        })
    }
}

impl CompositeWorkflow {
    pub fn generate(&self) -> TokenStream2 {
        let set_contexts = self.captures.iter().map(|var| {
            let ident = &var.ident;
            let ty = &var.ty;
            quote! {
                set_context::<#ty>(#ident);
            }
        });

        let get_contexts = self.captures.iter().map(|var| {
            let ident = &var.ident;
            let ty = &var.ty;
            quote! {
                let #ident: #ty = get_context::<#ty>();
            }
        });

        let workflow_name = &self.workflow_name;
        let block = &self.block;

        quote! {{
            use crate::workflow::composite_workflow_context::set_context;
            use crate::workflow::composite_workflow_context::get_context;
            use crate::workflow::composite_workflow_context::clear_all_context;
            use crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME;

            define_composite_workflow_inner!(#workflow_name {
                #(#get_contexts)*
                #block
            });

            let handle = COMPOSITE_WORKFLOW_RUNTIME
                .lock()
                .unwrap()
                .spawn(Box::pin(async move {
                    #(#set_contexts)*
                    just_do_it().await;
                    clear_all_context();
                }));

            handle
        }}
    }
}
