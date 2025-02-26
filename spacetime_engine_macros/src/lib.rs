mod workflow_mod;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use workflow_mod::ir1::workflow::WorkflowModule;

#[proc_macro]
pub fn workflow_mod(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
}
