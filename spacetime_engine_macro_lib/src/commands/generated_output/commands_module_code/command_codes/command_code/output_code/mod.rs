pub mod struct_code;
pub mod impl_display_code;

use syn::spanned::Spanned;
use syn::Ident;
use syn::LitStr;
use quote::quote;
use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;
use self::struct_code::*;
use self::impl_display_code::*;

pub struct CommandOutputCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandOutputCode {
    pub fn generate(command_type: &CommandType) -> Self {
        let command_id = command_type.command_id.value().to_string();
        let command_output_name = command_id.clone() + "CommandOutput";
        let command_output_name = Ident::new(&command_output_name, command_output_name.span());

        let command_output_type = &command_type.output_type;
        let output_parameter_infos: Vec<(LitStr, syn::Type)> = command_output_type.parameter_types.iter().map(|parameter_type| {
            (parameter_type.parameter_name.clone(), parameter_type.parameter_type.clone())
        }).collect();
        let mut command_output_parameter_declarations = quote! {};
        let mut command_output_parameter_self_accesses = quote! {};
        let mut command_output_parameters_interpolation = quote! {};
        let mut first = true;
        for (parameter_name, parameter_type) in output_parameter_infos.clone() {
            let parameter_name = Ident::new(&parameter_name.value(), parameter_name.span());

            if !first {
                command_output_parameter_declarations = quote! {
                    #command_output_parameter_declarations, 
                };
                command_output_parameter_self_accesses = quote! {
                    #command_output_parameter_self_accesses, 
                };
                command_output_parameters_interpolation = quote! {
                    #command_output_parameters_interpolation, 
                };
            } else {
                first = false;
            }

            command_output_parameter_declarations = quote! {
                #command_output_parameter_declarations
                pub #parameter_name: #parameter_type
            };
            command_output_parameter_self_accesses = quote! {
                #command_output_parameter_self_accesses
                self.#parameter_name
            };
            command_output_parameters_interpolation = quote! {
                #command_output_parameters_interpolation
                #parameter_name: ({})
            };
        }
        let command_output_parameters_interpolation = command_output_parameters_interpolation.to_string();

        let command_output_struct_code = CommandOutputStructCode::generate(
            command_output_name.clone(),
            command_output_parameter_declarations
        ).tokens;

        let command_output_impl_display_code = CommandOutputImplDisplayCode::generate(
            command_output_name,
            command_output_parameters_interpolation,
            command_output_parameter_self_accesses
        ).tokens;

        let tokens = quote! {
            #command_output_struct_code
            #command_output_impl_display_code
        };

        Self {
            tokens
        }
    }
}