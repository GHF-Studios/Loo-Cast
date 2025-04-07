mod sub_macro;
mod workflow_id;
mod workflow_invocation;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Result, Ident, Token, braced, parse::{Parse, ParseStream}, spanned::Spanned};

use sub_macro::{SubMacro, SubMacroOutput};
use workflow_invocation::WorkflowMacro;

#[derive(Debug)]
pub struct CompositeWorkflow {
    name: Ident,
    body: TokenStream,
    invocations: Vec<WorkflowMacro>,
}

impl Parse for CompositeWorkflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![!]>().ok(); // Allow optional !
        let content;
        braced!(content in input);
        let raw: TokenStream = content.parse()?;

        // -- Run ID expansion pass --
        let SubMacroOutput { token_stream, .. } = SubMacro::WorkflowId.expand_in(raw);

        // -- Run workflow! expansion + tracking --
        let SubMacroOutput { token_stream, invocations } =
            SubMacro::WorkflowInvocation.expand_in(token_stream);

        Ok(Self {
            name,
            body: token_stream,
            invocations,
        })
    }
}

impl CompositeWorkflow {
    pub fn generate(self) -> TokenStream {
        let function_ident = self.name;
        let error_enum_ident = Ident::new(&format!("{function_ident}Error"), function_ident.span());

        // -- Filter fallible workflows into a vec --
        let fallible: Vec<_> = self.invocations.iter().filter(|inv| {
            matches!(
                inv.signature.to_string().as_str(),
                "E" | "OE" | "IE" | "IOE"
            )
        }).collect();

        // -- Error variants --
        let error_variants = fallible.iter().map(|wf| {
            let path = &wf.workflow_type_path;
            let name_str = path.to_token_stream().to_string().replace("::", "");
            let variant = Ident::new(&format!("{name_str}Error"), path.span());

            quote! {
                #[error(#name_str)]
                #variant(<#path as workflow::traits::WorkflowType>::Error)
            }
        });

        // -- From impls --
        let from_impls = fallible.iter().map(|wf| {
            let path = &wf.workflow_type_path;
            let name_str = path.to_token_stream().to_string().replace("::", "");
            let variant = Ident::new(&format!("{name_str}Error"), path.span());

            quote! {
                impl From<<#path as workflow::traits::WorkflowType>::Error> for #error_enum_ident {
                    fn from(e: <#path as workflow::traits::WorkflowType>::Error) -> Self {
                        Self::#variant(e)
                    }
                }
            }
        });

        // -- Return type --
        let fn_return_type = if fallible.is_empty() {
            quote! { () }
        } else {
            quote! { Result<(), #error_enum_ident> }
        };

        // -- Error enum (only if needed) --
        let error_enum = if fallible.is_empty() {
            quote! {}
        } else {
            quote! {
                #[derive(Debug, thiserror::Error)]
                pub enum #error_enum_ident {
                    #(#error_variants),*
                }

                #(#from_impls)*
            }
        };

        let fn_body = self.body;

        quote! {
            #error_enum

            pub async fn #function_ident() -> #fn_return_type {
                #fn_body
            }

            crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
                .lock()
                .unwrap()
                .spawn_fallible(Box::pin(#function_ident()));
        }
    }
}
