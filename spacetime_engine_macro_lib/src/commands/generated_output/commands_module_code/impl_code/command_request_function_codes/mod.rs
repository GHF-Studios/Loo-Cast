pub mod command_request_function_code;

use crate::commands::parsed_input::commands_module_type::CommandsModuleType;
use quote::quote;
use self::command_request_function_code::CommandsModuleCommandRequestFunctionCode;

pub struct CommandsModuleCommandRequestFunctionCodes {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCommandRequestFunctionCodes {
    pub fn generate(commands_module_type: &CommandsModuleType) -> Self {
        let mut tokens = quote! {};
        let mut first = true;
        for command_type in &commands_module_type.command_types.values {
            let command_request_function_code = CommandsModuleCommandRequestFunctionCode::generate(command_type).tokens;

            if !first {
                tokens = quote! {
                    #tokens

                    #command_request_function_code
                }
            } else {
                first = false;

                tokens = quote! {
                    #command_request_function_code
                }
            }
        }

        Self {
            tokens
        }
    }
}