use syn::Ident;
use crate::commands::parsed_input::command_type::CommandType;
use quote::quote;

pub struct CommandsModuleCommandFinalizeFunctionCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCommandFinalizeFunctionCode {
    pub fn generate(
        command_type: &CommandType,
        command_name: Ident,
        command_output_name: Ident,
        command_error_name: Ident,
    ) -> Self {
        if command_type.output_type.parameter_types.is_empty() {
            if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    fn finalize(self) -> Option<()> {
                        if let #command_name::Executed {} = self {
                            Some(())
                        } else {
                            None
                        }
                    }
                };

                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    fn finalize(self) -> Option<Result<(), #command_error_name>> {
                        if let #command_name::Executed { result } = self {
                            Some(result)
                        } else {
                            None
                        }
                    }	
                };

                Self {
                    tokens
                }
            }
        } else if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    fn finalize(self) -> Option<#command_output_name> {
                        if let #command_name::Executed { output } = self {
                            Some(output)
                        } else {
                            None
                        }
                    }
                };

                Self {
                    tokens
                }
        } else {
            let tokens = quote! {
                fn finalize(self) -> Option<Result<#command_output_name, #command_error_name>> {
                    if let #command_name::Executed { result } = self {
                        Some(result)
                    } else {
                        None
                    }
                }
            };

            Self {
                tokens
            }
        }
    }
}