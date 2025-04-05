mod define_workflow_mod;
mod define_composite_workflow;

use define_workflow_mod::WorkflowModule;
use define_composite_workflow::CompositeWorkflow;

use proc_macro::TokenStream;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_workflow_mod(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
}

#[proc_macro]
pub fn define_composite_workflow(input: TokenStream) -> TokenStream {
    let composite_workflow = parse_macro_input!(input as CompositeWorkflow);
    composite_workflow.generate().into()
}
