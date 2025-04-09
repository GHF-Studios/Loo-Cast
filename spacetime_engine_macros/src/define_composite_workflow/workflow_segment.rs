use proc_macro2::{TokenStream, TokenTree, Group, Delimiter};
use quote::quote;
use syn::{parse2, ExprStruct, ExprPath, Attribute, Result, Ident};
use crate::define_composite_workflow::workflow_invocation::*;

#[derive(Debug, Clone)]
pub enum WorkflowSegment {
    Plain(TokenStream),
    Invocation(WorkflowInvocation),
}

pub fn extract_workflow_segments(input: TokenStream) -> Vec<WorkflowSegment> {
    let mut segments = Vec::new();
    let mut plain_buffer = TokenStream::new();
    let mut invocation_parts = Vec::new();

    let mut tokens = input.into_iter().peekable();

    while let Some(tt) = tokens.next() {
        match &tt {
            TokenTree::Punct(p) if p.as_char() == '#' => {
                // Check if this is an attribute group
                if let Some(TokenTree::Group(group)) = tokens.peek() {
                    if group.delimiter() == Delimiter::Bracket {
                        // Consume and match the next token as a Group
                        if let TokenTree::Group(group) = tokens.next().unwrap() {
                            let group_stream = group.stream();
                
                            let handled = if group_stream.to_string().contains("WorkflowSignature") {
                                extract_signature_ident(group_stream).map(InvocationPart::Signature)
                            } else if group_stream.to_string().contains("WorkflowType") {
                                extract_type_path(group_stream).map(InvocationPart::Type)
                            } else if group_stream.to_string().contains("WorkflowInput") {
                                extract_input_struct(group_stream).map(InvocationPart::Input)
                            } else {
                                Err(syn::Error::new_spanned(
                                    quote! { #[#group] },
                                    "Unrecognized attribute",
                                ))
                            };
                
                            match handled {
                                Ok(part) => {
                                    invocation_parts.push(part);
                
                                    if let Some(invocation) = WorkflowInvocation::from_parts(invocation_parts.clone()) {
                                        if !plain_buffer.is_empty() {
                                            segments.push(WorkflowSegment::Plain(plain_buffer.clone()));
                                            plain_buffer = TokenStream::new();
                                        }
                                        segments.push(WorkflowSegment::Invocation(invocation));
                                        invocation_parts.clear();
                                    }
                                }
                                Err(_) => {
                                    flush_to_plain(&mut segments, &mut plain_buffer, &mut invocation_parts);
                                    plain_buffer.extend(quote! { #[#group] });
                                }
                            }
                
                            continue;
                        }
                    }
                }

                // A lone `#` — treat as plain
                plain_buffer.extend(quote! { #tt });
            }

            _ => {
                // Any other token
                // If we had partial invocation parts, treat them as plain now
                if !invocation_parts.is_empty() {
                    flush_to_plain(&mut segments, &mut plain_buffer, &mut invocation_parts);
                }

                plain_buffer.extend(quote! { #tt });
            }
        }
    }

    // Final flush
    if !invocation_parts.is_empty() {
        flush_to_plain(&mut segments, &mut plain_buffer, &mut invocation_parts);
    }
    if !plain_buffer.is_empty() {
        segments.push(WorkflowSegment::Plain(plain_buffer));
    }

    segments
}

fn flush_to_plain(
    segments: &mut Vec<WorkflowSegment>,
    plain_buffer: &mut TokenStream,
    invocation_parts: &mut Vec<InvocationPart>,
) {
    if !invocation_parts.is_empty() {
        let dumped: TokenStream = invocation_parts
            .drain(..)
            .map(|p| match p {
                InvocationPart::Signature(s) => quote! { #[WorkflowSignature(#s)] },
                InvocationPart::Type(t) => quote! { #[WorkflowType(#t)] },
                InvocationPart::Input(i) => quote! { #[WorkflowInput #i] },
            })
            .collect();
        plain_buffer.extend(dumped);
    }

    if !plain_buffer.is_empty() {
        segments.push(WorkflowSegment::Plain(plain_buffer.clone()));
        *plain_buffer = TokenStream::new();
    }
}

fn merge_or_push_plain(segments: &mut Vec<WorkflowSegment>, tokens: TokenStream) {
    match segments.last_mut() {
        Some(WorkflowSegment::Plain(existing)) => {
            existing.extend(tokens);
        }
        _ => segments.push(WorkflowSegment::Plain(tokens)),
    }
}

fn flush_buffer_as_plain(
    segments: &mut Vec<WorkflowSegment>,
    plain_buffer: &mut TokenStream,
    buffer: &mut Vec<InvocationPart>,
) {
    if !buffer.is_empty() {
        let dumped: TokenStream = buffer
            .drain(..)
            .map(|p| match p {
                InvocationPart::Signature(s) => quote! { #[WorkflowSignature(#s)] },
                InvocationPart::Type(t) => quote! { #[WorkflowType(#t)] },
                InvocationPart::Input(i) => quote! { #[WorkflowInput #i] },
            })
            .collect();
        plain_buffer.extend(dumped);
    }

    if !plain_buffer.is_empty() {
        merge_or_push_plain(segments, plain_buffer.clone());
        *plain_buffer = TokenStream::new();
    }
}

fn extract_signature_ident(ts: TokenStream) -> Result<Ident> {
    let mut inner = ts.into_iter();
    match (inner.next(), inner.next()) {
        (Some(TokenTree::Ident(kw)), Some(TokenTree::Group(group))) if kw == "WorkflowSignature" && group.delimiter() == Delimiter::Parenthesis => {
            parse2::<Ident>(group.stream())
        }
        _ => Err(syn::Error::new_spanned(quote! { #(#inner)* }, "Expected WorkflowSignature(Ident)")),
    }
}

fn extract_type_path(ts: TokenStream) -> Result<ExprPath> {
    let mut inner = ts.into_iter();
    match (inner.next(), inner.next()) {
        (Some(TokenTree::Ident(kw)), Some(TokenTree::Group(group))) if kw == "WorkflowType" && group.delimiter() == Delimiter::Parenthesis => {
            parse2::<ExprPath>(group.stream())
        }
        _ => Err(syn::Error::new_spanned(quote! { #(#inner)* }, "Expected WorkflowType(path)")),
    }
}

fn extract_input_struct(ts: TokenStream) -> Result<ExprStruct> {
    let mut inner = ts.into_iter();
    match (inner.next(), inner.next()) {
        (Some(TokenTree::Ident(kw)), Some(TokenTree::Group(group))) if kw == "WorkflowInput" && group.delimiter() == Delimiter::Brace => {
            parse2::<ExprStruct>(quote! { #kw #group })
        }
        _ => Err(syn::Error::new_spanned(quote! { #(#inner)* }, "Expected WorkflowInput { ... }")),
    }
}
