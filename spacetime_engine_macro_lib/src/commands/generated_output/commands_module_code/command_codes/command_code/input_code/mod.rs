pub mod struct_code;
pub mod impl_display_code;

use syn::spanned::Spanned;
use syn::Ident;
use syn::LitStr;
use quote::quote;
use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;
use self::struct_code::*;
use self::impl_display_code::*;

pub struct CommandInputCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputCode {
    pub fn generate(command_type: &CommandType) -> Self {
        let command_id = command_type.command_id.value().to_string();
        let command_input_name = command_id.clone() + "CommandInput";
        let command_input_name = Ident::new(&command_input_name, command_input_name.span());

        let command_input_type = &command_type.input_type;
        let input_parameter_infos: Vec<(LitStr, syn::Type)> = command_input_type.parameter_types.iter().map(|parameter_type| {
            (parameter_type.parameter_name.clone(), parameter_type.parameter_type.clone())
        }).collect();
        let mut command_input_parameter_declarations = quote! {};
        let mut command_input_parameter_self_accesses = quote! {};
        let mut command_input_parameters_interpolation = quote! {};
        let mut first = true;
        for (parameter_name, parameter_type) in input_parameter_infos.clone() {
            let parameter_name = Ident::new(&parameter_name.value(), parameter_name.span());

            if !first {
                command_input_parameter_declarations = quote! {
                    #command_input_parameter_declarations, 
                };
                command_input_parameter_self_accesses = quote! {
                    #command_input_parameter_self_accesses, 
                };
                command_input_parameters_interpolation = quote! {
                    #command_input_parameters_interpolation, 
                };
            } else {
                first = false;
            }

            command_input_parameter_declarations = quote! {
                #command_input_parameter_declarations
                pub #parameter_name: #parameter_type
            };
            command_input_parameter_self_accesses = quote! {
                #command_input_parameter_self_accesses
                self.#parameter_name
            };
            command_input_parameters_interpolation = quote! {
                #command_input_parameters_interpolation
                #parameter_name: ({})
            };
        }
        let command_input_parameters_interpolation = command_input_parameters_interpolation.to_string();

        let command_input_struct_code = CommandInputStructCode::generate(
            command_input_name.clone(),
            command_input_parameter_declarations
        ).tokens;

        let command_input_impl_display_code = CommandInputImplDisplayCode::generate(
            command_input_name,
            command_input_parameters_interpolation,
            command_input_parameter_self_accesses
        ).tokens;

        let tokens = quote! {
            #command_input_struct_code
            #command_input_impl_display_code
        };

        Self {
            tokens
        }
    }
}