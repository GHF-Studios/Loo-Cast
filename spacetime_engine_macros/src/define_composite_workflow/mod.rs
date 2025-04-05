mod workflow_id;
mod workflow_invocation;

use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{parse_macro_input, parse_quote, Expr, Ident, Token, Result, braced};
use syn::parse::{Parse, ParseStream};
use syn::token::Comma;
use syn::{Attribute, ExprPath, ExprStruct};

pub struct CompositeWorkflow {
    pub name: Ident,
    pub body: Vec<WorkflowExpr>,
}

impl Parse for CompositeWorkflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let composite_workflow_ident: Ident = input.parse()?;

        let code_block_content;
        braced!(code_block_content in input);

        // TODO: Find 

        Ok(CompositeWorkflow)
    }
}

impl CompositeWorkflow {
    pub fn generate(self) -> proc_macro2::TokenStream {
        let expanded_body = expand_pseudo_macros(self.body);
        quote! {
            async fn #startup_fn_name() -> Result<(), #error_type> {
                #expanded_body
            }
        }
    }
}

fn expand_pseudo_macros(tokens: TokenStream) -> TokenStream {
    let mut output = proc_macro2::TokenStream::new();
    let mut iter = tokens.into_iter().peekable();

    while let Some(tt) = iter.next() {
        match &tt {
            proc_macro2::TokenTree::Punct(punct) if punct.as_char() == '#' => {
                if let Some(proc_macro2::TokenTree::Group(group)) = iter.peek() {
                    let group_clone = group.clone();

                    // Try to parse as an `id!(...)`
                    if let Ok(parsed_id) = syn::parse2::<IdMacro>(group_clone.stream()) {
                        let expanded = parsed_id.generate();
                        output.extend(expanded);
                        iter.next(); // consume the group
                        continue;
                    }

                    // Try to parse as a `workflow!(...)`
                    if let Ok(parsed_workflow) = syn::parse2::<WorkflowMacro>(group_clone.stream()) {
                        let expanded = parsed_workflow.generate();
                        output.extend(expanded);
                        iter.next(); // consume the group
                        continue;
                    }
                }

                // Default case: just add the '#'
                output.extend(std::iter::once(tt));
            }

            proc_macro2::TokenTree::Group(group) => {
                let delim = group.delimiter();
                let inner = expand_pseudo_macros(group.stream());
                output.extend(std::iter::once(proc_macro2::TokenTree::Group(proc_macro2::Group::new(delim, inner))));
            }

            _ => {
                output.extend(std::iter::once(tt));
            }
        }
    }

    output
}
