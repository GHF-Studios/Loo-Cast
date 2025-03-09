mod define_workflow_mod;
mod define_composite_workflow;
mod run_workflow;

use define_workflow_mod::WorkflowModule;
use run_workflow::WorkflowInvocation;
use run_workflow_e::WorkflowInvocationE;
use run_workflow_o::WorkflowInvocationO;
use run_workflow_oe::WorkflowInvocationOE;
use run_workflow_i::WorkflowInvocationI;
use run_workflow_ie::WorkflowInvocationIE;
use run_workflow_io::WorkflowInvocationIO;
use run_workflow_ioe::WorkflowInvocationIOE;

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

#[proc_macro]
pub fn run_workflow_e(input: TokenStream) -> TokenStream {
    let workflow_e_invocation = parse_macro_input!(input as WorkflowInvocationE);
    workflow_e_invocation.generate().into()
}

#[proc_macro]
pub fn run_workflow_o(input: TokenStream) -> TokenStream {
    let workflow_o_invocation = parse_macro_input!(input as WorkflowInvocationO);
    workflow_o_invocation.generate().into()
}

#[proc_macro]
pub fn run_workflow_oe(input: TokenStream) -> TokenStream {
    let workflow_oe_invocation = parse_macro_input!(input as WorkflowInvocationOE);
    workflow_oe_invocation.generate().into()
}

#[proc_macro]
pub fn run_workflow_i(input: TokenStream) -> TokenStream {
    let workflow_i_invocation = parse_macro_input!(input as WorkflowInvocationI);
    workflow_i_invocation.generate().into()
}

#[proc_macro]
pub fn run_workflow_ie(input: TokenStream) -> TokenStream {
    let workflow_ie_invocation = parse_macro_input!(input as WorkflowInvocationIE);
    workflow_ie_invocation.generate().into()
}

#[proc_macro]
pub fn run_workflow_io(input: TokenStream) -> TokenStream {
    let workflow_io_invocation = parse_macro_input!(input as WorkflowInvocationIO);
    workflow_io_invocation.generate().into()
}

#[proc_macro]
pub fn run_workflow_ioe(input: TokenStream) -> TokenStream {
    let workflow_ioe_invocation = parse_macro_input!(input as WorkflowInvocationIOE);
    workflow_ioe_invocation.generate().into()
}
