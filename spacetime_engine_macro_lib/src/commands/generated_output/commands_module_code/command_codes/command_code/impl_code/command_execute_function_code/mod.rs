use syn::Ident;
use crate::commands::parsed_input::command_type::CommandType;
use quote::quote;

pub struct CommandExecuteFunctionCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandExecuteFunctionCode {
    pub fn generate(
        command_type: &CommandType,
        command_name: Ident,
    ) -> Self {
        if command_type.input_type.parameter_types.is_empty() {
            if command_type.output_type.parameter_types.is_empty() {
                if command_type.error_type.variant_types.is_empty() {
                    let tokens = quote! {
                        fn execute(&mut self) {
                            if let #command_name::Initialized { code } = self {
                                (code.closure)();
                                *self = #command_name::Executed {};
                            }
                        }
                    };
                    
                    Self {
                        tokens
                    }
                } else {
                    let tokens = quote! {
                        fn execute(&mut self) {
                            if let #command_name::Initialized { code } = self {
                                *self = #command_name::Executed {
                                    result: (code.closure)(),
                                };
                            }
                        }
                    };
                    
                    Self {
                        tokens
                    }
                }
            } else if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    fn execute(&mut self) {
                        if let #command_name::Initialized { code } = self {
                            *self = #command_name::Executed {
                                output: (code.closure)(),
                            };
                        }
                    }
                };
                
                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    fn execute(&mut self) {
                        if let #command_name::Initialized { code } = self {
                            *self = #command_name::Executed {
                                result: (code.closure)(),
                            };
                        }
                    }
                };
                
                Self {
                    tokens
                }
            }
        } else if command_type.output_type.parameter_types.is_empty()  {
            if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    fn execute(&mut self) {
                        if let #command_name::Initialized { input, code } = self {
                            (code.closure)(&input);
                            *self = #command_name::Executed {};
                        }
                    }
                };
                
                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    fn execute(&mut self) {
                        if let #command_name::Initialized { input, code } = self {
                            *self = #command_name::Executed {
                                result: (code.closure)(&input),
                            };
                        }
                    }
                };
                
                Self {
                    tokens
                }
            }
        } else if command_type.error_type.variant_types.is_empty() {
            let tokens = quote! {
                fn execute(&mut self) {
                    if let #command_name::Initialized { input, code } = self {
                        *self = #command_name::Executed {
                            output: (code.closure)(&input),
                        };
                    }
                }
            };
            
            Self {
                tokens
            }
        } else {
            let tokens = quote! {
                fn execute(&mut self) {
                    if let #command_name::Initialized { input, code } = self {
                        *self = #command_name::Executed {
                            result: (code.closure)(&input),
                        };
                    }
                }
            };
            
            Self {
                tokens
            }
        }
    }
}