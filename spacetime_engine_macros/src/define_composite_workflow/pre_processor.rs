use proc_macro2::{TokenStream, TokenTree, Group, Delimiter, Ident, Span};
use quote::{quote, ToTokens};
use syn::parse::{Parse, ParseStream};
use syn::{braced, parse_macro_input};
use heck::ToSnakeCase;

pub fn pre_process_workflows(input: TokenStream) -> TokenStream {
    let mut output = TokenStream::new();
    let mut iter = input.into_iter();

    while let Some(token) = iter.next() {
        match &token {
            TokenTree::Ident(ident) if ident.to_string() == "workflow" => {
                if let Some(TokenTree::Punct(p)) = iter.next() {
                    if p.as_char() == '!' {
                        if let Some(TokenTree::Group(group)) = iter.next() {
                            let transformed = transform_workflow_group(group.clone());
                            output.extend(transformed);
                            continue;
                        }
                    }
                }
                // If something went wrong, just push the tokens back in untouched
                output.extend([token].into_iter());
            }
            _ => {
                output.extend([token].into_iter());
            }
        }
    }

    output
}

fn transform_workflow_group(group: Group) -> TokenStream {
    let tokens: Vec<TokenTree> = group.stream().into_iter().collect();
    let mut cursor = 0;

    // Optional signature (Ident or nothing)
    let signature = if let Some(TokenTree::Ident(sig)) = tokens.get(cursor) {
        cursor += 1;
        if let Some(TokenTree::Punct(p)) = tokens.get(cursor) {
            if p.as_char() == ',' {
                cursor += 1;
                Some(sig.to_string())
            } else {
                cursor -= 1; // not a sig
                None
            }
        } else {
            cursor -= 1;
            None
        }
    } else {
        None
    };

    // Expect ModuleName::WorkflowName
    let module = if let Some(TokenTree::Ident(module_ident)) = tokens.get(cursor) {
        cursor += 1;
        module_ident.to_string()
    } else {
        return quote! { /* malformed workflow macro: missing module */ };
    };

    // Expect ::
    if !matches!(tokens.get(cursor), Some(TokenTree::Punct(p)) if p.as_char() == ':') {
        return quote! { /* malformed workflow macro: expected :: */ };
    }
    cursor += 1;
    if !matches!(tokens.get(cursor), Some(TokenTree::Punct(p)) if p.as_char() == ':') {
        return quote! { /* malformed workflow macro: expected :: */ };
    }
    cursor += 1;

    // WorkflowName
    let workflow = if let Some(TokenTree::Ident(workflow_ident)) = tokens.get(cursor) {
        cursor += 1;
        workflow_ident.to_string()
    } else {
        return quote! { /* malformed workflow macro: missing workflow name */ };
    };

    // Optional trailing comma
    if matches!(tokens.get(cursor), Some(TokenTree::Punct(p)) if p.as_char() == ',') {
        cursor += 1;
    }

    // Optional Input block
    let mut input_block: Option<Group> = None;
    if let Some(TokenTree::Ident(input_ident)) = tokens.get(cursor) {
        if input_ident.to_string() == "Input" {
            cursor += 1;
            if let Some(TokenTree::Group(group)) = tokens.get(cursor) {
                if group.delimiter() == Delimiter::Brace {
                    input_block = Some(group.clone());
                    cursor += 1;
                }
            }
        }
    }

    // Snake-case identifiers
    let module_snake = module.to_snake_case();
    let workflow_snake = workflow.to_snake_case();
    let type_path = format!("crate::{0}::workflows::{0}::{1}::Type", module_snake, workflow_snake);

    let workflow_signature = match &signature {
        Some(sig) => quote! { #sig },
        None => quote! { None },
    };

    let workflow_type = match &signature {
        Some(sig) => {
            let full_type = syn::parse_str::<TokenStream>(&format!("{type_path}{}", sig)).unwrap();
            quote! { #full_type }
        }
        None => {
            let full_type = syn::parse_str::<TokenStream>(&type_path).unwrap();
            quote! { #full_type }
        }
    };

    let signature_attr = quote! { #[WorkflowSignature(#workflow_signature)] };
    let type_attr = quote! { #[WorkflowType(#workflow_type)] };

    let input_attr = if let Some(input) = input_block {
        let input_tokens = input.stream();
        quote! { #[WorkflowInput { #input_tokens }] }
    } else {
        quote! {}
    };

    quote! {
        #signature_attr #type_attr #input_attr
    }
}
