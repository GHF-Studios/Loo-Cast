use syn::Ident;
use quote::quote;
use crate::commands::generated_output::commands_module_code::command_codes::command_code::CommandType;

pub struct CommandCodeStructCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandCodeStructCode {
    pub fn generate(
        command_type: &CommandType,
        command_input_name: Ident,
        command_output_name: Ident,
        command_error_name: Ident,
        command_code_name: Ident,
    ) -> Self {
        if command_type.input_type.parameter_types.is_empty() {
            if command_type.output_type.parameter_types.is_empty() {
                if command_type.error_type.variant_types.is_empty() {
                    let tokens = quote! {
                        pub struct #command_code_name {
                            closure: Box<dyn Fn()>,
                        }
                    };

                    Self {
                        tokens
                    }
                } else {
                    let tokens = quote! {
                        pub struct #command_code_name {
                            closure: Box<dyn Fn() -> Result<(), #command_error_name>>,
                        }
                    };

                    Self {
                        tokens
                    }
                }
            } else if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    pub struct #command_code_name {
                        closure: Box<dyn Fn() -> #command_output_name>,
                    }
                };

                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    pub struct #command_code_name {
                        closure: Box<dyn Fn() -> Result<#command_output_name, #command_error_name>>,
                    }
                };

                Self {
                    tokens
                }
            }
        } else if command_type.output_type.parameter_types.is_empty()  {
            if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    pub struct #command_code_name {
                        closure: Box<dyn Fn(&#command_input_name)>,
                    }
                };

                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    pub struct #command_code_name {
                        closure: Box<dyn Fn(&#command_input_name) -> Result<(), #command_error_name>>,
                    }
                };

                Self {
                    tokens
                }
            }
        } else if command_type.error_type.variant_types.is_empty() {
            let tokens = quote! {
                pub struct #command_code_name {
                    closure: Box<dyn Fn(&#command_input_name) -> #command_output_name>,
                }
            };

            Self {
                tokens
            }
        } else {
            let tokens = quote! {
                pub struct #command_code_name {
                    closure: Box<dyn Fn(&#command_input_name) -> Result<#command_output_name, #command_error_name>>,
                }
            };

            Self {
                tokens
            }
        }
    }
}