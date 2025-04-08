mod sub_macro;
mod workflow_id;
mod workflow_invocation;
mod workflow_segment;

use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, Result, Ident, Token, braced, parse::{Parse, ParseStream}, spanned::Spanned};
use heck::ToSnakeCase;
use std::collections::HashSet;

use sub_macro::{SubMacro, SubMacroOutput};
use workflow_invocation::WorkflowMacro;
use workflow_segment::{extract_workflow_segments, WorkflowSegment};

#[derive(Debug)]
pub struct CompositeWorkflow {
    name: Ident,
    segments: Vec<WorkflowSegment>,
}

impl Parse for CompositeWorkflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;

        let content;
        braced!(content in input);
        let token_stream: TokenStream = content.parse()?;

        // TODO: Actually implement this and un-comment it
        // Step 1: expand id! and workflow!
        //let SubMacroOutput { token_stream, .. } = SubMacro::WorkflowId.expand_in(token_stream);
        //let SubMacroOutput { token_stream, .. } = SubMacro::WorkflowInvocation.expand_in(token_stream);

        // Step 2: segment the body into workflow blocks and plain tokens
        let segments = extract_workflow_segments(token_stream);

        Ok(Self {
            name,
            segments,
        })
    }
}

impl CompositeWorkflow {
    pub fn generate(self) -> TokenStream {
        let function_ident = &self.name;
        let error_enum_ident = Ident::new(&format!("{}Error", function_ident), function_ident.span());

        // --- Collect fallible invocations ---
        let fallible_invocations: Vec<_> = self.segments.iter().filter_map(|seg| {
            match seg {
                WorkflowSegment::Invocation(wf)
                    if matches!(
                        wf.signature.to_string().as_str(),
                        "E" | "OE" | "IE" | "IOE"
                    ) => Some(wf),
                _ => None,
            }
        }).collect();

        // --- Deduplicate error types by workflow_type_path ---
        let mut seen_paths = HashSet::new();
        let mut unique_errors = Vec::new();
        for wf in &fallible_invocations {
            let path = &wf.workflow_type_path;
            let key = path.to_token_stream().to_string();
            if seen_paths.insert(key) {
                unique_errors.push(path);
            }
        }

        let error_variants = unique_errors.iter().map(|path| {
            let name_str = path.to_token_stream().to_string().replace("::", "");
            let variant = Ident::new(&format!("{}Error", name_str), path.span());

            quote! {
                #[error(#name_str)]
                #variant(<#path as workflow::traits::WorkflowType>::Error)
            }
        });

        let from_impls = unique_errors.iter().map(|path| {
            let name_str = path.to_token_stream().to_string().replace("::", "");
            let variant = Ident::new(&format!("{}Error", name_str), path.span());

            quote! {
                impl From<<#path as workflow::traits::WorkflowType>::Error> for #error_enum_ident {
                    fn from(e: <#path as workflow::traits::WorkflowType>::Error) -> Self {
                        Self::#variant(e)
                    }
                }
            }
        });

        let error_enum_tokens = if unique_errors.is_empty() {
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

        // --- Build function body ---
        let mut body_segments = Vec::new();

        for segment in &self.segments {
            match segment {
                WorkflowSegment::Plain(tokens) => {
                    body_segments.push(quote! { #tokens });
                }

                WorkflowSegment::Invocation(wf) => {
                    let sig = wf.signature.to_string();
                    let path = &wf.workflow_type_path;

                    let block = match sig.as_str() {
                        "None" => quote! {
                            {
                                #path::run().await
                            }
                        },

                        "E" => quote! {
                            {
                                #path::run().await.map_err(Into::<#error_enum_ident>::into)?
                            }
                        },

                        "O" => quote! {
                            {
                                #path::run().await
                            }
                        },

                        "OE" => quote! {
                            {
                                #path::run().await.map_err(Into::<#error_enum_ident>::into)?
                            }
                        },

                        "I" | "IE" | "IO" | "IOE" => {
                            let input = wf.input_struct.as_ref().expect("Expected `Input { ... }` block for workflow with input");

                            let mut inner = quote! {
                                type T = #path;
                                type I = <T as workflow::traits::WorkflowType>::Input;
                                T::run(#input).await
                            };

                            if sig.contains('E') {
                                inner = quote! {
                                    #inner.map_err(Into::<#error_enum_ident>::into)?
                                };
                            }

                            quote! {
                                {
                                    #inner
                                }
                            }
                        }

                        _ => panic!("Unknown workflow signature: {}", sig),
                    };

                    body_segments.push(block);
                }
            }
        }

        let return_type = if fallible_invocations.is_empty() {
            quote! { () }
        } else {
            quote! { Result<(), #error_enum_ident> }
        };

        quote! {
            #error_enum_tokens

            pub async fn #function_ident() -> #return_type {
                #(#body_segments)*

                Ok(())
            }
        }
    }
}
