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
    let mut buffer = vec![];
    let mut plain_buffer = TokenStream::new();

    let mut tokens = input.into_iter().peekable();

    while let Some(tt) = tokens.next() {
        match tt {
            TokenTree::Punct(ref p) if p.as_char() == '#' => {
                if let Some(TokenTree::Group(group)) = tokens.peek() {
                    if group.delimiter() == Delimiter::Bracket {
                        let group = match tokens.next() {
                            Some(TokenTree::Group(g)) => g,
                            _ => continue,
                        };
                        let group_stream = group.stream().to_string();

                        if group_stream.contains("WorkflowSignature") {
                            if let Ok(ident) = extract_signature_ident(group.stream()) {
                                buffer.push(InvocationPart::Signature(ident));
                                continue;
                            }
                        } else if group_stream.contains("WorkflowType") {
                            if let Ok(path) = extract_type_path(group.stream()) {
                                buffer.push(InvocationPart::Type(path));
                                continue;
                            }
                        } else if group_stream.contains("WorkflowInput") {
                            if let Ok(expr) = extract_input_struct(group.stream()) {
                                buffer.push(InvocationPart::Input(expr));
                                continue;
                            }
                        }

                        // If unrecognized, just flush everything and treat as plain
                        flush_buffer_as_plain(&mut segments, &mut plain_buffer, &mut buffer);
                        plain_buffer.extend(quote! { #tt #[#group] });
                    }
                }
            }

            TokenTree::Punct(p) if p.as_char() == ';' => {
                if let Some(invocation) = WorkflowInvocation::from_parts(buffer.clone()) {
                    segments.push(WorkflowSegment::Invocation(invocation));
                } else {
                    flush_buffer_as_plain(&mut segments, &mut plain_buffer, &mut buffer);
                }
                buffer.clear();
            }

            other => {
                if !buffer.is_empty() {
                    flush_buffer_as_plain(&mut segments, &mut plain_buffer, &mut buffer);
                }
                plain_buffer.extend(std::iter::once(other));
            }
        }
    }

    if !plain_buffer.is_empty() {
        merge_or_push_plain(&mut segments, plain_buffer);
    }

    segments
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
