use syn::{Ident, Token, ExprStruct, parse::{Parse, ParseStream}};
use quote::quote;
use proc_macro2::TokenStream;

pub struct WorkflowInvocation {
    category: Ident,
    workflow: Ident,
    input: Option<ExprStruct>,
}

impl Parse for WorkflowInvocation {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let category: Ident = input.parse()?;
        input.parse::<Token![,]>()?;
        let workflow: Ident = input.parse()?;
        
        let input = if input.peek(Token![,]) {
            input.parse::<Token![,]>()?;
            let struct_lit: ExprStruct = input.parse()?;
            Some(struct_lit)
        } else {
            None
        };

        Ok(WorkflowInvocation { category, workflow, input })
    }
}

impl WorkflowInvocation {
    pub fn generate(&self) -> TokenStream {
        let category = &self.category;
        let workflow = &self.workflow;

        let type_path = quote! { crate::#category::workflows::#workflow };
        let type_name = quote! { #type_path::Type };

        let expanded = if let Some(input_struct) = &self.input {
            quote! {
                {
                    type T = #type_name;
                    type I = <T as crate::workflow::traits::WorkflowTypeIOE>::Input;
                    crate::workflow::functions::run_workflow_ioe::<T>(I #input_struct).await.map_err(Into::into)
                }
            }
        } else {
            quote! {
                {
                    type T = #type_name;
                    crate::workflow::functions::run_workflow::<T>().await;
                }
            }
        };

        quote!{ todo!() }
    }
}


