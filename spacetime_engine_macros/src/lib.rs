mod workflow_mod;

use proc_macro::TokenStream;
use syn::parse_macro_input;
use workflow_mod::ir1::workflow::WorkflowModule as IR1WorkflowModule;
use workflow_mod::ir2::workflow::WorkflowModule as IR2WorkflowModule;

#[proc_macro]
pub fn workflow_mod(input: TokenStream) -> TokenStream {
    let ir1 = parse_macro_input!(input as IR1WorkflowModule);
    let ir2 = IR2WorkflowModule::from(ir1);
    ir2.generate().into()
}
