pub mod command_initialize_function_code;
pub mod command_execute_function_code;
pub mod command_finalize_function_code;

use syn::Ident;
use crate::commands::parsed_input::command_type::CommandType;
use quote::quote;
use self::{command_initialize_function_code::*, command_execute_function_code::*, command_finalize_function_code::*};

pub struct CommandsModuleCommandImplCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCommandImplCode {
    pub fn generate(
        command_type: &CommandType,
        command_name: Ident,
        command_input_name: Ident,
        command_output_name: Ident,
        command_error_name: Ident,
        command_code_name: Ident,
    ) -> Self {
        let command_initialize_function_code = CommandsModuleCommandInitializeFunctionCode::generate(
            command_type,
            command_name.clone(),
            command_input_name,
            command_code_name
        ).tokens;

        let command_execute_function_code = CommandsModuleCommandExecuteFunctionCode::generate(
            command_type,
            command_name.clone()
        ).tokens;

        let command_finalize_function_code = CommandsModuleCommandFinalizeFunctionCode::generate(
            command_type,
            command_name.clone(),
            command_output_name,
            command_error_name
        ).tokens;

        let tokens = quote! {
            impl #command_name {
                #command_initialize_function_code
                #command_execute_function_code
                #command_finalize_function_code
            }
        };

        Self {
            tokens
        }
    }
}