mod define_workflow_mod;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use define_workflow_mod::workflow::WorkflowModule;

#[proc_macro]
pub fn define_workflow_mod(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
}
