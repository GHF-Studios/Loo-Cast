use syn::{Ident, LitStr};
use quote::quote;

use crate::commands::parsed_input::command_type::input_type::CommandInputType;

pub struct CommandInputParametersInfo {
    pub field_declarations: proc_macro2::TokenStream,
    pub self_accesses: proc_macro2::TokenStream,
    pub interpolations: String
}

impl CommandInputParametersInfo {
    pub fn extract(command_input_type: &CommandInputType) -> Self {
        let input_parameter_details: Vec<(LitStr, syn::Type)> = command_input_type.parameter_types.iter().map(|parameter_type| {
            (parameter_type.parameter_name.clone(), parameter_type.parameter_type.clone())
        }).collect();
        let mut field_declarations = quote! {};
        let mut self_accesses = quote! {};
        let mut interpolations = quote! {};
        let mut first = true;
        for (parameter_name, parameter_type) in input_parameter_details.clone() {
            let parameter_name = Ident::new(&parameter_name.value(), parameter_name.span());

            if !first {
                field_declarations = quote! {
                    #field_declarations, 
                };
                self_accesses = quote! {
                    #self_accesses, 
                };
                interpolations = quote! {
                    #interpolations, 
                };
            } else {
                first = false;
            }

            field_declarations = quote! {
                #field_declarations
                pub #parameter_name: #parameter_type
            };
            self_accesses = quote! {
                #self_accesses
                self.#parameter_name
            };
            interpolations = quote! {
                #interpolations
                #parameter_name: {}
            };
        }
        let interpolations = interpolations.to_string();

        Self {
            field_declarations,
            self_accesses,
            interpolations
        }
    }
}