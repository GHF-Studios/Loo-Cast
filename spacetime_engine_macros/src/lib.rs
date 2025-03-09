mod define_workflow_mod;
mod define_composite_workflow;
mod run_workflow;

use define_workflow_mod::WorkflowModule;
use run_workflow::WorkflowInvocation;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

#[proc_macro]
pub fn define_workflow_mod(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
}

#[proc_macro]
pub fn run_workflow(input: TokenStream) -> TokenStream {
    let workflow_invocation = parse_macro_input!(input as WorkflowInvocation);
    workflow_invocation.generate().into()
}
