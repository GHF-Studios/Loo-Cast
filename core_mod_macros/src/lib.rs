extern crate proc_macro;

mod composite_workflow;
mod composite_workflow_return;
mod define_composite_workflow;
mod define_workflow_mod;
#[allow(non_snake_case)]
mod define_workflow_mod_OLD;
mod global_statics;
mod register_workflow_mods;
mod script;
mod usf;

use composite_workflow::CompositeWorkflow as OuterCompositeWorkflow;
use composite_workflow_return::CompositeWorkflowReturn;
use define_composite_workflow::CompositeWorkflow as InnerCompositeWorkflow;
use define_workflow_mod_OLD::WorkflowModule;
use register_workflow_mods::WorkflowMods;
use usf::scale::{
    configure_app_with_all_scales::AppConfigInput, scale_factor_exponent_dynamic_match::ScaleFactorExponentDynamicMatch,
    scale_type_generic_match::ScaleTypeGenericMatch,
};

use proc_macro::TokenStream;
use quote::quote;
use syn::parse_macro_input;

// Global statics

#[proc_macro]
pub fn export_static(input: TokenStream) -> TokenStream {
    global_statics::export_static(input)
}

#[proc_macro]
pub fn import_static(input: TokenStream) -> TokenStream {
    global_statics::import_static(input)
}

#[proc_macro]
pub fn api_initializer(input: TokenStream) -> TokenStream {
    global_statics::api_initializer(input)
}

// Workflow

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

// Script

// -Ecs

// --Components

// ---Internals

#[proc_macro_attribute]
pub fn component_ctor(attr: TokenStream, item: TokenStream) -> TokenStream {
    script::ecs::components::internals::types::component_ctor(attr, item)
}

// Usf

// -Scale

#[proc_macro]
pub fn configure_app_with_all_scales(input: TokenStream) -> TokenStream {
    let app_config_input = parse_macro_input!(input as AppConfigInput);
    app_config_input.generate().into()
}

#[proc_macro]
pub fn scale_type_generic_match(input: TokenStream) -> TokenStream {
    let scale_type_generic_match = parse_macro_input!(input as ScaleTypeGenericMatch);
    scale_type_generic_match.generate().into()
}

#[proc_macro]
pub fn scale_factor_exponent_dynamic_match(input: TokenStream) -> TokenStream {
    let scale_factor_exponent_dynamic_match = parse_macro_input!(input as ScaleFactorExponentDynamicMatch);
    scale_factor_exponent_dynamic_match.generate().into()
}
