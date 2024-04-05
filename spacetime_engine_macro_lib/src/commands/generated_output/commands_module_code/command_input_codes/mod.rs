pub mod command_input_code;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::commands::parsed_input::command_type::input_type::CommandInputType;

pub struct CommandInputCodes {
    pub tokens: TokenStream,
}

impl crate::Collect<CommandInputCode> for CommandInputCodes {
    fn collect(parsed_macro_input: &CommandInputType) -> Result<Vec<Self>> {
        todo!();
    }
}
