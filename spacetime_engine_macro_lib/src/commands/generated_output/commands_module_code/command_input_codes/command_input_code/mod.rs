pub mod impl_code;
pub mod struct_code;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::commands::parsed_input::command_type::input_type::CommandInputType;

pub enum CommandInputCode {
    Setup {
        command_input_type: CommandInputType,
    },
    Generation {
        tokens: proc_macro2::TokenStream,
    }	
}

impl CommandInputCode {
    pub fn new(command_input_type: CommandInputType) -> Self {
        CommandInputCode::Setup { command_input_type }
    }
}

impl crate::CodeOutputGenerator for CommandInputCode {
    fn generate(self) -> Result<Self> {
        match self {
            CommandInputCode::Setup { command_input_type } => {
                todo!();
            }
            CommandInputCode::Generation { tokens } => {}
        }
    }
}