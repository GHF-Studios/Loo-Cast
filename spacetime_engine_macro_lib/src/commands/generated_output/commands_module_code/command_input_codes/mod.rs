pub mod command_input_code;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::commands::generated_output::commands_module_code::command_input_codes::command_input_code::CommandInputCode;

pub enum CommandInputCodes {
    Setup {
        command_input_codes: Vec<CommandInputCode>,
    },
    Generation {
        tokens: proc_macro2::TokenStream,
    }
}

impl crate::CodeOutputGenerator for CommandInputCodes {
    fn generate(self) -> Result<Self> {
        match self {
            CommandInputCodes::Setup { command_input_codes } => {
                todo!();
            }
            CommandInputCodes::Generation { tokens } => {}
        }
    }
}