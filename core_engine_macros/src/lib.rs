extern crate proc_macro;

mod composite_workflow;
mod composite_workflow_return;
mod define_composite_workflow;
mod define_workflow_mod;
#[allow(non_snake_case)]
mod define_workflow_mod_OLD;
mod register_workflow_mods;
mod tracing_macros;
mod typed_dag_macros;

use composite_workflow::CompositeWorkflow as OuterCompositeWorkflow;
use composite_workflow_return::CompositeWorkflowReturn;
use define_composite_workflow::CompositeWorkflow as InnerCompositeWorkflow;
use define_workflow_mod_OLD::WorkflowModule;
use register_workflow_mods::WorkflowMods;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

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
pub fn define_composite_workflow(input: TokenStream) -> TokenStream {
    let inner_composite_workflow = parse_macro_input!(input as InnerCompositeWorkflow);
    inner_composite_workflow.generate().into()
}

#[proc_macro]
#[allow(non_snake_case)]
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
pub fn define_workflow_mod(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro]
pub fn define_workflow(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro]
pub fn define_worfklow_stages(_input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro_derive(DagNode, attributes(dag))]
pub fn derive_dag_node(input: TokenStream) -> TokenStream {
    typed_dag_macros::impl_dag_node(input)
}