pub mod command_input_code;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::commands::generated_output::commands_module_code::command_input_codes::command_input_code::CommandInputCode;

pub struct CommandInputCodes {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputCodes {
    pub fn generate(command_input_codes: Vec<CommandInputCode>) -> Self {
        let mut tokens = quote! {};
        for command_input_code in command_input_codes {
            let command_input_code_tokens = &command_input_code.tokens;
            
            tokens = quote! {
                #tokens
                #command_input_code_tokens
            };
        }

        Self {
            tokens
        }
    }
}