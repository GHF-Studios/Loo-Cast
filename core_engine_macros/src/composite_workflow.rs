use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::visit::Visit;
use syn::{Block, ExprMacro, Ident, Token, Type};

struct WorkflowMacroDetector {
    found: bool,
}

impl<'ast> Visit<'ast> for WorkflowMacroDetector {
    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        if let syn::Stmt::Macro(mac_stmt) = node {
            if mac_stmt.mac.path.is_ident("workflow") {
                if let Some(TokenTree::Ident(ident)) = mac_stmt.mac.tokens.clone().into_iter().next() {
                    if matches!(ident.to_string().as_str(), "E" | "OE" | "IE" | "IOE") {
                        self.found = true;
                    }
                }
            }
        }
        syn::visit::visit_stmt(self, node);
    }

    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        if node.mac.path.is_ident("workflow") {
            if let Some(TokenTree::Ident(ident)) = node.mac.tokens.clone().into_iter().next() {
                if matches!(ident.to_string().as_str(), "E" | "OE" | "IE" | "IOE") {
                    self.found = true;
                }
            }
        }
        syn::visit::visit_expr_macro(self, node);
    }
}

fn is_fallible(block: &Block) -> bool {
    let mut detector = WorkflowMacroDetector { found: false };
    syn::visit::visit_block(&mut detector, block);
    detector.found
}

#[derive(Debug, PartialEq)]
pub enum MoveMode {
    In,
    Out,
    Both,
}

pub struct VarCapture {
    pub move_mode: MoveMode,
    pub is_mut: bool,
    pub ident: Ident,
    pub ty: Type,
}

pub struct CompositeWorkflow {
    pub captures: Vec<VarCapture>,
    pub block: Block,
}

impl Parse for CompositeWorkflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut captures = Vec::new();

        while !input.is_empty() {
            if input.peek(syn::token::Brace) {
                break;
            }

            let mut move_mode = MoveMode::Both;
            if input.peek(Token![move]) {
                input.parse::<Token![move]>()?;

                if input.peek(Token![in]) {
                    input.parse::<Token![in]>()?;
                    move_mode = MoveMode::In;
                } else if input.peek(Ident) {
                    let direction: Ident = input.parse()?;
                    if direction == "out" {
                        move_mode = MoveMode::Out;
                    } else {
                        return Err(syn::Error::new(
                            direction.span(),
                            format!("Expected `in` or `out` after `move`, found `{}`", direction),
                        ));
                    }
                } else {
                    return Err(input.error("Expected `in` or `out` after `move`"));
                }
            }

            let is_mut = if input.peek(Token![mut]) {
                if move_mode == MoveMode::Out {
                    return Err(input.error("`move out mut` is not allowed"));
                }
                input.parse::<Token![mut]>()?;
                true
            } else {
                false
            };

            let ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty: Type = input.parse()?;
            input.parse::<Token![,]>()?;

            captures.push(VarCapture { move_mode, is_mut, ident, ty });
        }

        let block: Block = input.parse()?;

        Ok(CompositeWorkflow { captures, block })
    }
}

impl CompositeWorkflow {
    pub fn generate(&self) -> TokenStream2 {
        let pass_in_contexts = self.captures.iter().filter_map(|var| {
            if var.move_mode == MoveMode::Both || var.move_mode == MoveMode::In {
                let ident = &var.ident;
                let name = ident.to_string();
                let ty = &var.ty;
                Some(quote! {
                    set_context::<#ty>(#name, #ident);
                })
            } else {
                None
            }
        });

        let get_contexts = self.captures.iter().filter_map(|var| {
            if var.move_mode == MoveMode::Both || var.move_mode == MoveMode::In {
                let ident = &var.ident;
                let ty = &var.ty;
                let name = ident.to_string();
                if var.is_mut {
                    Some(quote! {
                        let mut #ident: #ty = get_context::<#ty>(#name);
                    })
                } else {
                    Some(quote! {
                        let #ident: #ty = get_context::<#ty>(#name);
                    })
                }
            } else {
                None
            }
        });

        let set_contexts = self.captures.iter().filter_map(|var| {
            if var.move_mode == MoveMode::Both || var.move_mode == MoveMode::Out {
                let ident = &var.ident;
                let name = ident.to_string();
                let ty = &var.ty;
                Some(quote! {
                    set_context::<#ty>(#name, #ident);
                })
            } else {
                None
            }
        });

        let return_contexts = self.captures.iter().filter_map(|var| {
            if var.move_mode == MoveMode::Both || var.move_mode == MoveMode::Out {
                let ident = &var.ident;
                let ty = &var.ty;
                let name = ident.to_string();
                Some(quote! {
                    let #ident: #ty = get_context::<#ty>(#name);
                    ctx.store_return(#name, #ident);
                })
            } else {
                None
            }
        });

        let composite_workflow_ident = Ident::new("composite_workflow", proc_macro2::Span::call_site());
        let block = &self.block;
        let is_fallible = is_fallible(block);

        if is_fallible {
            quote! {{
                use crate::workflow::composite_workflow_context::{set_context, get_context, ScopedCompositeWorkflowContext};
                use crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME;
                use core_engine_macros::define_composite_workflow;

                define_composite_workflow!(CompositeWorkflow {
                    #(#get_contexts)*
                    #block
                    #(#set_contexts)*
                    Ok(())
                });

                let handle = COMPOSITE_WORKFLOW_RUNTIME
                    .lock()
                    .unwrap()
                    .spawn_fallible(Box::pin(async move {
                        let scoped_ctx = ScopedCompositeWorkflowContext::default();
                        let (scoped_ctx, result) = scoped_ctx.run_fallible(|ctx: ScopedCompositeWorkflowContext| async {
                            #(#pass_in_contexts)*
                            let result = #composite_workflow_ident().await;
                            #(#return_contexts)*
                            (ctx, result)
                        }).await;
                        (scoped_ctx, result)
                    }));

                handle
            }}
        } else {
            quote! {{
                use crate::workflow::composite_workflow_context::{set_context, get_context, ScopedCompositeWorkflowContext};
                use crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME;
                use core_engine_macros::define_composite_workflow;

                define_composite_workflow!(CompositeWorkflow {
                    #(#get_contexts)*
                    #block
                    #(#set_contexts)*
                });

                let handle = COMPOSITE_WORKFLOW_RUNTIME
                    .lock()
                    .unwrap()
                    .spawn(Box::pin(async move {
                        let scoped_ctx = ScopedCompositeWorkflowContext::default();
                        let scoped_ctx = scoped_ctx.run(|ctx: ScopedCompositeWorkflowContext| async {
                            #(#pass_in_contexts)*
                            #composite_workflow_ident().await;
                            #(#return_contexts)*
                            ctx
                        }).await;
                        scoped_ctx
                    }));

                handle
            }}
        }
    }
}
