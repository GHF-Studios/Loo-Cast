mod define_composite_workflow;
mod define_workflow_mod;
mod define_workflow_mod_OLD;

use define_composite_workflow::CompositeWorkflow;
use define_workflow_mod_OLD::WorkflowModule;

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn define_composite_workflow(input: TokenStream) -> TokenStream {
    let composite_workflow = parse_macro_input!(input as CompositeWorkflow);
    composite_workflow.generate().into()
}

#[proc_macro]
pub fn define_workflow_mod_OLD(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
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
