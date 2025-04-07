use proc_macro2::{TokenStream, TokenTree, Group};
use quote::quote;
use syn::parse2;

use super::workflow_id::TypedWorkflowId;
use super::workflow_invocation::WorkflowMacro;

pub struct SubMacroOutput {
    pub token_stream: TokenStream,
    pub invocations: Vec<WorkflowMacro>,
}

pub enum SubMacro {
    WorkflowId,
    WorkflowInvocation,
}

impl SubMacro {
    pub fn expand_in(&self, input: TokenStream) -> SubMacroOutput {
        match self {
            SubMacro::WorkflowId => Self::expand_ids(input),
            SubMacro::WorkflowInvocation => Self::expand_workflows(input),
        }
    }

    fn expand_ids(input: TokenStream) -> SubMacroOutput {
        let mut output = TokenStream::new();
        let mut tokens = input.into_iter().peekable();

        while let Some(tt) = tokens.next() {
            match &tt {
                TokenTree::Ident(ident) if ident == "id" => {
                    if let Some(TokenTree::Punct(p)) = tokens.peek() {
                        if p.as_char() == '!' {
                            tokens.next(); // consume '!'
                            if let Some(TokenTree::Group(group)) = tokens.next() {
                                let id_tokens = group.stream();

                                match parse2::<TypedWorkflowId>(id_tokens) {
                                    Ok(id) => {
                                        let expanded = id.generate();
                                        output.extend(expanded);
                                    }
                                    Err(e) => {
                                        unreachable!("Failed to parse ID: {}", e);
                                    }
                                }

                                continue;
                            }
                        }
                    }

                    output.extend(quote! { #tt });
                }

                TokenTree::Group(group) => {
                    let delimiter = group.delimiter();
                    let inner = Self::expand_ids(group.stream());
                    output.extend(std::iter::once(TokenTree::Group(Group::new(delimiter, inner.token_stream))));
                }

                _ => {
                    output.extend(std::iter::once(tt));
                }
            }
        }

        SubMacroOutput {
            token_stream: output,
            invocations: vec![], // not needed for ID phase
        }
    }

    fn expand_workflows(input: TokenStream) -> SubMacroOutput {
        let mut output = TokenStream::new();
        let mut invocations = vec![];
        let mut tokens = input.into_iter().peekable();

        while let Some(tt) = tokens.next() {
            match &tt {
                TokenTree::Ident(ident) if ident == "workflow" => {
                    if let Some(TokenTree::Punct(p)) = tokens.peek() {
                        if p.as_char() == '!' {
                            tokens.next(); // consume '!'
                            if let Some(TokenTree::Group(group)) = tokens.next() {
                                let inner_tokens = group.stream();

                                match parse2::<WorkflowMacro>(inner_tokens.clone()) {
                                    Ok(invocation) => {
                                        let generated = invocation.generate();
                                        output.extend(generated.clone());
                                        invocations.push(invocation);
                                    }
                                    Err(e) => {
                                        unreachable!("Failed to parse ID: {}", e);
                                    }
                                }

                                continue;
                            }
                        }
                    }

                    output.extend(quote! { #tt });
                }

                TokenTree::Group(group) => {
                    let delimiter = group.delimiter();
                    let inner = Self::expand_workflows(group.stream());
                    output.extend(std::iter::once(TokenTree::Group(Group::new(delimiter, inner.token_stream))));
                    invocations.extend(inner.invocations);
                }

                _ => {
                    output.extend(std::iter::once(tt));
                }
            }
        }

        SubMacroOutput {
            token_stream: output,
            invocations,
        }
    }
}
