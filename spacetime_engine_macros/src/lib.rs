mod composite_workflow;
mod define_composite_workflow_inner;
mod composite_workflow_return;
mod define_workflow_mod;
mod define_workflow_mod_OLD;
mod register_workflow_mods;

use composite_workflow::CompositeWorkflow as OuterCompositeWorkflow;
use define_composite_workflow_inner::CompositeWorkflow as InnerCompositeWorkflow;
use composite_workflow_return::CompositeWorkflowReturn;
use define_workflow_mod_OLD::WorkflowModule;
use register_workflow_mods::WorkflowMods;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token};

#[proc_macro]
pub fn composite_workflow(input: TokenStream) -> TokenStream {
    let outer_composite_workflow = parse_macro_input!(input as OuterCompositeWorkflow);
    outer_composite_workflow.generate().into()
}

#[proc_macro]
pub fn composite_workflow_return(input: TokenStream) -> TokenStream {
    let composite_workflow_return = parse_macro_input!(input as CompositeWorkflowReturn);
    composite_workflow_return.generate().into()
}

#[proc_macro]
pub fn define_composite_workflow_inner(input: TokenStream) -> TokenStream {
    let inner_composite_workflow = parse_macro_input!(input as InnerCompositeWorkflow);
    inner_composite_workflow.generate().into()
}

#[proc_macro]
pub fn define_workflow_mod_OLD(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
}

#[proc_macro]
pub fn register_workflow_mods(input: TokenStream) -> TokenStream {
    let workflow_mods = parse_macro_input!(input as WorkflowMods);
    workflow_mods.generate().into()
}

#[proc_macro]
pub fn define_workflow_mod(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro]
pub fn define_workflow(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro]
pub fn define_worfklow_stages(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
