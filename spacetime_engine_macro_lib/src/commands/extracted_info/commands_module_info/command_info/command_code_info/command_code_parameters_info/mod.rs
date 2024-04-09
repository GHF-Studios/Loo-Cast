use syn::{spanned::Spanned, Ident};
use quote::quote;
use crate::commands::parsed_input::command_type::code_type::*;
use super::CommandCodeInfo;

#[derive(Clone)]
pub struct CommandCodeParametersInfo {
    pub interpolations: String
}

impl CommandCodeParametersInfo {
    pub fn extract(_command_code_type: &CommandCodeType, command_code_info: &CommandCodeInfo) -> Self {
        let command_code_name = &command_code_info.name;
        let command_code_name = Ident::new(command_code_name, command_code_name.span());

        let interpolations = quote! {
            #command_code_name: {{ closure: No Display }}
        }.to_string().replace("{ { {", "{{ {").replace("} } }", "} }}").replace("{ {", "{{").replace("} }", "}}");

        Self {
            interpolations
        }
    }
}