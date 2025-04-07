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

        // --- Collect fallible workflows ---
        let fallible: Vec<_> = self.invocations.clone().into_iter().filter(|inv| {
            matches!(
                inv.signature.to_string().as_str(),
                "E" | "OE" | "IE" | "IOE"
            )
        }).collect();

        // --- Error Enum Variants ---
        let error_variants = fallible.iter().map(|wf| {
            let path = &wf.workflow_type_path;
            let name_str = path.to_token_stream().to_string().replace("::", "");
            let variant = Ident::new(&format!("{name_str}Error"), path.span());

            quote! {
                #[error(#name_str)]
                #variant(<#path as workflow::traits::WorkflowType>::Error)
            }
        });

        // --- From impls for error conversion ---
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

        // --- Function return type ---
        let fn_return_type = if fallible.is_empty() {
            quote! { () }
        } else {
            quote! { Result<(), #error_enum_ident> }
        };

        // --- Generate function body from WorkflowMacro entries ---
        let body_blocks = self.invocations.into_iter().map(|wf| {
            let path = &wf.workflow_type_path;
            let sig = wf.signature.to_string();
            let input = wf.input_struct;

            let trait_path = match sig.as_str() {
                "E" => quote! { workflow::traits::WorkflowTypeE },
                "O" => quote! { workflow::traits::WorkflowTypeO },
                "OE" => quote! { workflow::traits::WorkflowTypeOE },
                "I" => quote! { workflow::traits::WorkflowTypeI },
                "IE" => quote! { workflow::traits::WorkflowTypeIE },
                "IO" => quote! { workflow::traits::WorkflowTypeIO },
                "IOE" => quote! { workflow::traits::WorkflowTypeIOE },
                _ => quote! { workflow::traits::WorkflowType },
            };

            let run_call = match sig.as_str() {
                "None" => quote! {
                    #path::run().await
                },
                "E" => quote! {
                    #path::run().await.map_err(Into::<#error_enum_ident>::into)
                },
                "O" => quote! {
                    let output = #path::run().await;
                },
                "OE" => quote! {
                    let output = #path::run().await.map_err(Into::<#error_enum_ident>::into)?;
                },
                "I" | "IE" | "IO" | "IOE" => {
                    let input_block = input.expect("Expected Input { ... } for workflow with input");

                    let result_expr = quote! {
                        #path::run(#input_block).await
                    };

                    let wrapped_expr = if sig.contains('E') {
                        quote! {
                            #result_expr.map_err(Into::<#error_enum_ident>::into)?
                        }
                    } else {
                        quote! {
                            #result_expr
                        }
                    };

                    if sig.contains('O') {
                        quote! {
                            let output = #wrapped_expr;
                        }
                    } else {
                        quote! {
                            #wrapped_expr;
                        }
                    }
                }
                _ => panic!("Unhandled workflow signature type: {}", sig),
            };

            quote! {
                {
                    type T = #path;
                    #run_call;
                }
            }
        });

        // --- Conditionally emit error enum ---
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

        // --- Final output ---
        quote! {
            #error_enum

            pub async fn #function_ident() -> #fn_return_type {
                #(#body_blocks)*
                Ok(())
            }

            crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME
                .lock()
                .unwrap()
                .spawn_fallible(Box::pin(#function_ident()));
        }
    }
}
