use syn::Ident;
use crate::commands::parsed_input::command_type::CommandType;
use quote::quote;

pub struct CommandInitializeFunctionCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandInitializeFunctionCode {
    pub fn generate(
        command_type: &CommandType,
        command_name: Ident,
        command_input_name: Ident,
        command_code_name: Ident,
    ) -> Self {
        if command_type.input_type.parameter_types.is_empty() {
            let tokens = quote! {
                fn initialize(code: #command_code_name) -> Self {
                    #command_name::Initialized {
                        code,
                    }
                }
            };

            Self {
                tokens
            }
        } else {
            let tokens = quote! {
                fn initialize(input: #command_input_name, code: #command_code_name) -> Self {
                    #command_name::Initialized {
                        input,
                        code,
                    }
                }
            };

            Self {
                tokens
            }
        }
    }
}