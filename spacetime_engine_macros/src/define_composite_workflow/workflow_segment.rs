use crate::define_composite_workflow::workflow_invocation::*;
use proc_macro2::{Delimiter, TokenStream, TokenTree};
use quote::quote;
use syn::{parse2, ExprPath, ExprStruct, Ident, Result};

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
                if let Some(TokenTree::Group(group)) = tokens.peek() {
                    if group.delimiter() == Delimiter::Bracket {
                        if let TokenTree::Group(group) = tokens.next().unwrap() {
                            let tokens = group.stream();

                            let handled = if let Ok(ident) = extract_signature_ident(tokens.clone())
                            {
                                invocation_parts.push(InvocationPart::Signature(ident));
                                true
                            } else if let Ok(path) = extract_type_path(tokens.clone()) {
                                invocation_parts.push(InvocationPart::Type(path));
                                true
                            } else if let Ok(input) = extract_input_struct(tokens.clone()) {
                                invocation_parts.push(InvocationPart::Input(input));
                                true
                            } else {
                                false
                            };

                            if handled {
                                try_finalize_invocation(
                                    &mut segments,
                                    &mut plain_buffer,
                                    &mut invocation_parts,
                                );
                                continue;
                            } else {
                                flush_to_plain(
                                    &mut segments,
                                    &mut plain_buffer,
                                    &mut invocation_parts,
                                );
                                plain_buffer.extend(quote! { #[#group] });
                                continue;
                            }
                        }
                    }
                }

                plain_buffer.extend(quote! { #tt });
            }

            _ => {
                if !invocation_parts.is_empty() {
                    flush_to_plain(&mut segments, &mut plain_buffer, &mut invocation_parts);
                }

                plain_buffer.extend(quote! { #tt });
            }
        }
    }

    if !invocation_parts.is_empty() {
        flush_to_plain(&mut segments, &mut plain_buffer, &mut invocation_parts);
    }

    if !plain_buffer.is_empty() {
        segments.push(WorkflowSegment::Plain(plain_buffer));
    }

    segments
}

fn try_finalize_invocation(
    segments: &mut Vec<WorkflowSegment>,
    plain_buffer: &mut TokenStream,
    invocation_parts: &mut Vec<InvocationPart>,
) {
    if let Some(invocation) = WorkflowInvocation::from_parts(invocation_parts.clone()) {
        let sig = invocation.signature.to_string();

        let input_required = matches!(sig.as_str(), "I" | "IE" | "IO" | "IOE");
        let has_input = invocation.input_struct.is_some();

        if input_required && !has_input {
            return; // Don't finalize yet
        }

        // Finalize invocation
        if !plain_buffer.is_empty() {
            segments.push(WorkflowSegment::Plain(plain_buffer.clone()));
            *plain_buffer = TokenStream::new();
        }

        segments.push(WorkflowSegment::Invocation(invocation));
        invocation_parts.clear();
    }
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

fn extract_signature_ident(ts: TokenStream) -> Result<Ident> {
    let mut inner = ts.into_iter();
    match (inner.next(), inner.next()) {
        (Some(TokenTree::Ident(kw)), Some(TokenTree::Group(group)))
            if kw == "WorkflowSignature" && group.delimiter() == Delimiter::Parenthesis =>
        {
            parse2::<Ident>(group.stream())
        }
        _ => Err(syn::Error::new_spanned(
            quote! { #(#inner)* },
            "Expected WorkflowSignature(Ident)",
        )),
    }
}

fn extract_type_path(ts: TokenStream) -> Result<ExprPath> {
    let mut inner = ts.into_iter();
    match (inner.next(), inner.next()) {
        (Some(TokenTree::Ident(kw)), Some(TokenTree::Group(group)))
            if kw == "WorkflowType" && group.delimiter() == Delimiter::Parenthesis =>
        {
            parse2::<ExprPath>(group.stream())
        }
        _ => Err(syn::Error::new_spanned(
            quote! { #(#inner)* },
            "Expected WorkflowType(path)",
        )),
    }
}

fn extract_input_struct(ts: TokenStream) -> Result<ExprStruct> {
    let mut inner = ts.into_iter();
    match (inner.next(), inner.next()) {
        (Some(TokenTree::Ident(kw)), Some(TokenTree::Group(group)))
            if kw == "WorkflowInput" && group.delimiter() == Delimiter::Brace =>
        {
            parse2::<ExprStruct>(quote! { #kw #group })
        }
        _ => Err(syn::Error::new_spanned(
            quote! { #(#inner)* },
            "Expected WorkflowInput { ... }",
        )),
    }
}
