use proc_macro2::{TokenStream, TokenTree, Group, Delimiter};
use quote::quote;
use syn::{parse2, ExprStruct, ExprPath, Attribute, Result, Ident};
use crate::define_composite_workflow::workflow_invocation::WorkflowMacro;

#[derive(Debug, Clone)]
pub enum WorkflowSegment {
    Plain(TokenStream),
    Invocation(WorkflowMacro),
}

pub fn extract_workflow_segments(input: TokenStream) -> Vec<WorkflowSegment> {
    let mut segments = Vec::new();
    let mut plain_buffer = TokenStream::new();

    let mut tokens = input.into_iter().peekable();

    while let Some(tt) = tokens.peek() {
        // Try to detect #[WorkflowSignature(...)]
        let is_signature_block = if let TokenTree::Punct(p) = tt {
            if p.as_char() == '#' {
                let mut clone = tokens.clone();
                clone.next(); // consume '#'
                if let Some(TokenTree::Group(group)) = clone.next() {
                    group.delimiter() == Delimiter::Bracket
                        && group.stream().to_string().contains("WorkflowSignature")
                } else {
                    false
                }
            } else {
                false
            }
        } else {
            false
        };

        if is_signature_block {
            // Flush any plain tokens we've accumulated
            if !plain_buffer.is_empty() {
                segments.push(WorkflowSegment::Plain(plain_buffer.clone()));
                plain_buffer = TokenStream::new();
            }

            // === Collect the full attribute set ===
            let mut wf_tokens = TokenStream::new();
            while let Some(tt) = tokens.peek() {
                if let TokenTree::Punct(p) = tt {
                    if p.as_char() == '#' {
                        wf_tokens.extend(tokens.next()); // #
                        if let Some(TokenTree::Group(group)) = tokens.peek() {
                            wf_tokens.extend(tokens.next()); // #[...]
                        }
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }

            // Collect the actual statement after the attributes
            while let Some(next) = tokens.peek() {
                if let TokenTree::Punct(p) = next {
                    if p.as_char() == ';' {
                        wf_tokens.extend(tokens.next()); // consume ';'
                        break;
                    }
                }
                wf_tokens.extend(tokens.next());
            }

            // Try parsing the collected chunk
            match parse2::<WorkflowMacro>(wf_tokens.clone()) {
                Ok(parsed) => segments.push(WorkflowSegment::Invocation(parsed)),
                Err(_) => segments.push(WorkflowSegment::Plain(wf_tokens)),
            }

            continue;
        }

        // Not a workflow marker, just accumulate normally
        plain_buffer.extend(std::iter::once(tokens.next().unwrap()));
    }

    if !plain_buffer.is_empty() {
        segments.push(WorkflowSegment::Plain(plain_buffer));
    }

    segments
}

