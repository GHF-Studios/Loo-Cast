use syn::{Ident, LitStr};
use quote::quote;
use crate::commands::parsed_input::command_type::output_type::*;
use super::CommandOutputInfo;

#[derive(Clone)]
pub struct CommandOutputParametersInfo {
    pub field_declarations: proc_macro2::TokenStream,
    pub self_accesses: proc_macro2::TokenStream,
    pub interpolations: String
}

impl CommandOutputParametersInfo {
    pub fn extract(command_output_type: &CommandOutputType, _command_output_info: &CommandOutputInfo) -> Self {
        let output_parameter_infos: Vec<(LitStr, syn::Type)> = command_output_type.parameter_types.iter().map(|parameter_type| {
            (parameter_type.parameter_name.clone(), parameter_type.parameter_type.clone())
        }).collect();

        let mut field_declarations = quote! {};
        let mut self_accesses = quote! {};
        let mut interpolations = quote! {};
        let mut first = true;

        for (parameter_name, parameter_type) in output_parameter_infos.clone() {
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
                #parameter_name: ({})
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