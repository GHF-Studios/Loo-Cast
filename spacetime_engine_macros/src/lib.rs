mod define_composite_workflow;
mod define_workflow_mod;

use define_composite_workflow::CompositeWorkflow;
use define_workflow_mod::WorkflowModule;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use quote::quote;

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

#[proc_macro]
pub fn define_worfklow_stages(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
