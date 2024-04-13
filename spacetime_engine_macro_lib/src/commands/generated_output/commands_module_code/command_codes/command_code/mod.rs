pub mod enum_code;
pub mod impl_code;

use syn::{spanned::Spanned, Ident};
use quote::quote;
use crate::commands::{generated_output::commands_module_code::command_codes::command_code::{enum_code::CommandsModuleCommandEnumCode, impl_code::CommandsModuleCommandImplCode}, parsed_input::command_type::CommandType};

pub struct CommandsModuleCommandCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCommandCode {
    pub fn generate(command_type: &CommandType) -> Self {
        let command_id = command_type.command_id.value().to_string();
        let command_name = command_id.clone() + "Command";
        let command_name = Ident::new(&command_name, command_id.span());

        let command_input_name = command_id.clone() + "CommandInput";
        let command_input_name = Ident::new(&command_input_name, command_id.span());

        let command_output_name = command_id.clone() + "CommandOutput";
        let command_output_name = Ident::new(&command_output_name, command_id.span());

        let command_error_name = command_id.clone() + "CommandError";
        let command_error_name = Ident::new(&command_error_name, command_id.span());

        let command_code_name = command_id.clone() + "CommandCode";
        let command_code_name = Ident::new(&command_code_name, command_id.span());

        let enum_code = CommandsModuleCommandEnumCode::generate(
            command_type,
            command_name.clone(),
            command_input_name.clone(),
            command_output_name.clone(),
            command_error_name.clone(),
            command_code_name.clone()
        ).tokens;
        let impl_code = CommandsModuleCommandImplCode::generate(
            command_type,
            command_name.clone(),
            command_input_name.clone(),
            command_output_name.clone(),
            command_error_name.clone(),
            command_code_name.clone()
        ).tokens;

        let tokens = quote! {
            #enum_code
            #impl_code
        };

        Self {
            tokens
        }
    }
}