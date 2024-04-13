pub mod impl_code;
pub mod struct_code;

use quote::quote;
use syn::spanned::Spanned;
use syn::Ident;
use crate::commands::generated_output::commands_module_code::command_input_codes::command_input_code::struct_code::*;
use crate::commands::generated_output::commands_module_code::command_input_codes::command_input_code::impl_code::*;
use crate::commands::parsed_input::command_type::*;

pub struct CommandInputCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputCode {
    pub fn generate(command_type: &CommandType) -> Self {
        let command_id = command_type.command_id.value().to_string();
        let command_input_name = command_id.clone() + "CommandInput";
        let command_input_name = Ident::new(&command_input_name, command_id.span());

        let command_input_parameters_info = CommandInputParametersInfo::extract(&command_type.input_type);

        let command_input_struct_code = CommandInputStructCode::generate(command_input_name.clone(), &command_input_parameters_info);
        let command_input_impl_code = CommandInputImplCode::generate(command_input_name, &command_input_parameters_info);

        let command_input_struct_code_tokens = command_input_struct_code.tokens;
        let command_input_impl_code_tokens = command_input_impl_code.tokens;

        let tokens = quote! {
            #command_input_struct_code_tokens
            #command_input_impl_code_tokens
        };

        Self {
            tokens
        }
    }
}