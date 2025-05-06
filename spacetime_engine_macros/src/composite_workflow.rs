use heck::ToSnakeCase;
use proc_macro2::TokenStream as TokenStream2;
use proc_macro2::TokenTree;
use quote::quote;
use syn::parse::{Parse, ParseStream, Result};
use syn::visit::Visit;
use syn::{Block, ExprMacro, Ident, Token, Type};

struct WorkflowMacroDetector {
    found: bool,
}

impl<'ast> Visit<'ast> for WorkflowMacroDetector {
    #[allow(clippy::collapsible_match)]
    fn visit_stmt(&mut self, node: &'ast syn::Stmt) {
        if let syn::Stmt::Macro(mac_stmt) = node {
            let mac_path = &mac_stmt.mac.path;
            if mac_path.is_ident("workflow") {
                if let Some(first_token) = mac_stmt.mac.tokens.clone().into_iter().next() {
                    if let TokenTree::Ident(ident) = first_token {
                        let id = ident.to_string();
                        if id == "E" || id == "OE" || id == "IE" || id == "IOE" {
                            self.found = true;
                        }
                    }
                }
            }
        }

        // Continue descending into the rest
        syn::visit::visit_stmt(self, node);
    }

    #[allow(clippy::collapsible_match)]
    fn visit_expr_macro(&mut self, node: &'ast ExprMacro) {
        let mac_path = &node.mac.path;
        if mac_path.is_ident("workflow") {
            if let Some(first_token) = node.mac.tokens.clone().into_iter().next() {
                if let TokenTree::Ident(ident) = first_token {
                    let id = ident.to_string();
                    if id == "E" || id == "OE" || id == "IE" || id == "IOE" {
                        self.found = true;
                    }
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

pub struct CompositeWorkflow {
    pub captures: Vec<VarCapture>,
    pub name: Ident,
    pub block: Block,
}

pub struct VarCapture {
    pub is_mut: bool,
    pub ident: Ident,
    pub ty: Type,
}

impl Parse for CompositeWorkflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut captures = Vec::new();

        while input.peek(Token![mut]) || (input.peek(Ident) && input.peek2(Token![:])) {
            let is_mut = input.peek(Token![mut]);

            if is_mut {
                input.parse::<Token![mut]>()?;
            }

            let ident: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let ty: Type = input.parse()?;
            input.parse::<Token![,]>()?;

            captures.push(VarCapture { is_mut, ident, ty });
        }

        let composite_workflow_name: Ident = input.parse()?;
        let block: Block = input.parse()?;

        Ok(CompositeWorkflow {
            captures,
            name: composite_workflow_name,
            block,
        })
    }
}

impl CompositeWorkflow {
    pub fn generate(&self) -> TokenStream2 {
        let pass_in_contexts = self.captures.iter().map(|var| {
            let ident = &var.ident;
            let name = ident.to_string();
            let ty = &var.ty;
            quote! {
                set_context::<#ty>(#name, #ident);
            }
        });

        let get_contexts = self.captures.iter().map(|var| {
            let is_mut = var.is_mut;
            let ident = &var.ident;
            let name = ident.to_string();
            let ty = &var.ty;
        
            if is_mut {
                quote! {
                    let mut #ident: #ty = get_context::<#ty>(#name);
                }
            } else {
                quote! {
                    let #ident: #ty = get_context::<#ty>(#name);
                }
            }
        });
        
        let set_contexts = self.captures.iter().map(|var| {
            let ident = &var.ident;
            let name = ident.to_string();
            let ty = &var.ty;
            quote! {
                set_context::<#ty>(#name, #ident);
            }
        });

        let return_contexts = self.captures.iter().map(|ret_var| {
            let ident = &ret_var.ident;
            let name = ident.to_string();
            quote! {
                ctx.store_return(#name, #ident);
            }
        });

        let composite_workflow_name = &self.name;
        let composite_workflow_name_snake_case = format!("{}", composite_workflow_name).as_str().to_snake_case();
        let composite_workflow_ident = Ident::new(&composite_workflow_name_snake_case, composite_workflow_name.span());
        let block = &self.block;
        let is_fallible = is_fallible(block);

        if is_fallible {
            quote! {{
                use crate::workflow::composite_workflow_context::{set_context, get_context, ScopedCompositeWorkflowContext};
                use crate::workflow::statics::COMPOSITE_WORKFLOW_RUNTIME;
                use spacetime_engine_macros::define_composite_workflow;
                
                define_composite_workflow!(#composite_workflow_name {
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
                use spacetime_engine_macros::define_composite_workflow;
            
                define_composite_workflow!(#composite_workflow_name {
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
