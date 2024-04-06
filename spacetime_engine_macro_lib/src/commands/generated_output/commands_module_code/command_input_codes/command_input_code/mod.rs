pub mod impl_code;
pub mod struct_code;

use proc_macro2::TokenStream;
use quote::quote;
use syn::Result;
use crate::commands::parsed_input::command_type::input_type::CommandInputType;

pub struct CommandInputCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInputCode {
    pub fn generate(command_input_type: CommandInputType) -> Self {
        let tokens = quote! {
            todo!();
        };

        Self {
            tokens
        }
    }
}