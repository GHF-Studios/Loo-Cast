pub mod struct_code;
pub mod impl_display_code;

use syn::spanned::Spanned;
use syn::Ident;
use quote::quote;

use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;
use self::struct_code::*;
use self::impl_display_code::*;

pub struct CommandCodeCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandCodeCode {
    pub fn generate(command_type: &CommandType) -> Self {
        let command_id = command_type.command_id.value().to_string();
        let command_input_name = command_id.clone() + "CommandInput";
        let command_input_name = Ident::new(&command_input_name, command_input_name.span());

        let command_output_name = command_id.clone() + "CommandOutput";
        let command_output_name = Ident::new(&command_output_name, command_output_name.span());

        let command_error_name = command_id.clone() + "CommandError";
        let command_error_name = Ident::new(&command_error_name, command_error_name.span());

        let command_code_name = command_id.clone() + "CommandCode";
        let command_code_name = Ident::new(&command_code_name, command_code_name.span());

        let code_parameters_interpolation = quote! {
            #command_code_name: {{ closure: No Display }}
        }.to_string().replace("{ { {", "{{ {").replace("} } }", "} }}").replace("{ {", "{{").replace("} }", "}}");

        let command_code_struct_code = CommandCodeStructCode::generate(
            command_type,
            command_input_name,
            command_output_name,
            command_error_name,
            command_code_name.clone()
        ).tokens;

        let command_code_impl_display_code = CommandCodeImplDisplayCode::generate(
            command_code_name,
            code_parameters_interpolation
        ).tokens;

        let tokens = quote! {
            #command_code_struct_code
            #command_code_impl_display_code
        };

        Self {
            tokens
        }
    }
}