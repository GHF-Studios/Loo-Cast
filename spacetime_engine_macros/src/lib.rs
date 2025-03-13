mod define_composite_workflow;
mod define_workflow_mod;
mod get_workflow_path;
mod run_workflow;

use define_workflow_mod::WorkflowModule;
//use define_composite_workflow::CompositeWorkflow;
//use run_workflow::TypedWorkflowInvocation;
use get_workflow_path::TypedWorkflowPath;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_workflow_mod(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
}

#[proc_macro]
pub fn run_workflow(input: TokenStream) -> TokenStream {
    //let workflow_invocation = parse_macro_input!(input as TypedWorkflowInvocation);
    //workflow_invocation.generate().into()

    //let placeholder_todo = quote! { todo!() };
    //placeholder_todo.into()

    input
}

#[proc_macro]
pub fn get_workflow_path(input: TokenStream) -> TokenStream {
    let workflow_path = parse_macro_input!(input as TypedWorkflowPath);
    workflow_path.generate().into()
}
