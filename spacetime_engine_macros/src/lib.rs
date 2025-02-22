use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Ident, ItemStruct, ItemFn};

#[proc_macro]
pub fn workflow_mod(input: TokenStream) -> TokenStream {
    // Parse the input as Rust tokens
    let input = parse_macro_input!(input as syn::ItemMod);
    let mod_name = &input.ident;
    let mut workflow_types = Vec::new();

    // Iterate over items inside the module
    for item in input.content.iter().flat_map(|c| c.1.iter()) {
        if let syn::Item::Struct(ItemStruct { ident, .. }) = item {
            workflow_types.push(ident);
        }
    }

    // Generate workflow type registration
    let expanded = quote! {
        pub mod #mod_name {
            use bevy::prelude::*;
            use crate::workflow::{WorkflowTypeModule, WorkflowTypeModuleRegistry};

            pub fn register_workflows(workflow_registry: &mut WorkflowTypeModuleRegistry) {
                workflow_registry.register(WorkflowTypeModule {
                    name: stringify!(#mod_name).to_owned(),
                    workflow_types: vec![
                        #(#workflow_types::create_workflow_type(),)*
                    ],
                });
            }
        }
    };

    expanded.into()
}
