use syn::{spanned::Spanned, Ident, LitStr};
use crate::commands::parsed_input::command_type::CommandType;
use quote::quote;

pub struct CommandsModuleCommandRequestFunctionCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandsModuleCommandRequestFunctionCode {
    pub fn generate(command_type: &CommandType) -> Self {
        // Command ID Snake Case
        let command_id = command_type.command_id.value().to_string();
        let mut command_id_snake_case = String::new();
        let mut prev_was_uppercase = false;
        for (i, c) in command_id.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && !prev_was_uppercase {
                    command_id_snake_case.push('_');
                }
                command_id_snake_case.push(c.to_lowercase().next().unwrap());
                prev_was_uppercase = true;
            } else {
                command_id_snake_case.push(c);
                prev_was_uppercase = false;
            }
        }
        let command_id_snake_case = Ident::new(&command_id_snake_case, command_id.span());

        // Command Name
        let command_name = command_id.clone() + "Command";
        let command_name = Ident::new(&command_name, command_id.span());

        // Command Name Snake Case
        let mut command_name_snake_case = String::new();
        let mut prev_was_uppercase = false;
        for (i, c) in command_name.to_string().chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && !prev_was_uppercase {
                    command_name_snake_case.push('_');
                }
                command_name_snake_case.push(c.to_lowercase().next().unwrap());
                prev_was_uppercase = true;
            } else {
                command_name_snake_case.push(c);
                prev_was_uppercase = false;
            }
        }
        let command_name_snake_case = Ident::new(&command_name_snake_case, command_id.span());

        // Command Input Name
        let command_input_name = command_id.clone() + "CommandInput";
        let command_input_name = Ident::new(&command_input_name, command_id.span());

        // Command Output Name
        let command_output_name = command_id.clone() + "CommandOutput";
        let command_output_name = Ident::new(&command_output_name, command_id.span());

        // Command Output Name Snake Case
        let mut command_output_name_snake_case = String::new();
        let mut prev_was_uppercase = false;
        for (i, c) in command_output_name.to_string().chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && !prev_was_uppercase {
                    command_output_name_snake_case.push('_');
                }
                command_output_name_snake_case.push(c.to_lowercase().next().unwrap());
                prev_was_uppercase = true;
            } else {
                command_output_name_snake_case.push(c);
                prev_was_uppercase = false;
            }
        }
        let command_output_name_snake_case = Ident::new(&command_output_name_snake_case, command_id.span());

        // Command Error Name
        let command_error_name = command_id.clone() + "CommandError";
        let command_error_name = Ident::new(&command_error_name, command_id.span());

        // Command Code Name
        let command_code_name = command_id.clone() + "CommandCode";
        let command_code_name = Ident::new(&command_code_name, command_id.span());

        // Command Code Block
        let command_code_block = command_type.code_type.code_block.clone();

        // Command Result Name Snake Case
        let command_result_name = command_id.clone() + "CommandResult";
        let mut command_result_name_snake_case = String::new();
        let mut prev_was_uppercase = false;
        for (i, c) in command_result_name.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && !prev_was_uppercase {
                    command_result_name_snake_case.push('_');
                }
                command_result_name_snake_case.push(c.to_lowercase().next().unwrap());
                prev_was_uppercase = true;
            } else {
                command_result_name_snake_case.push(c);
                prev_was_uppercase = false;
            }
        }
        let command_result_name_snake_case = Ident::new(&command_result_name_snake_case, command_id.span());

        // Input Parameter Infos
        let input_parameter_infos: Vec<(LitStr, syn::Type)> = command_type.input_type.parameter_types.iter().map(|parameter_type| {
            (parameter_type.parameter_name.clone(), parameter_type.parameter_type.clone())
        }).collect();

        let mut generated_input_parameters = quote! {};
        let mut generated_input_parameter_names = quote! {};
        let mut first_inner = true;
        for (parameter_name, parameter_type) in input_parameter_infos.clone() {
            let parameter_name = Ident::new(&parameter_name.value(), parameter_name.span());

            if !first_inner {
                generated_input_parameters = quote! {
                    #generated_input_parameters, 
                };
                generated_input_parameter_names = quote! {
                    #generated_input_parameter_names, 
                };
            } else {
                first_inner = false;
            }

            generated_input_parameters = quote! {
                #generated_input_parameters
                #parameter_name: #parameter_type
            };
            generated_input_parameter_names = quote! {
                #generated_input_parameter_names
                #parameter_name
            };
        }

        // Interpolated Panic Message
        let generated_interpolated_panic_message = quote! {
            #command_name did not execute properly!
        }.to_string();
        let generated_interpolated_panic_message = LitStr::new(
            &generated_interpolated_panic_message, 
            generated_interpolated_panic_message.span()
        );

        // Code Generation
        if command_type.input_type.parameter_types.is_empty() {
            if command_type.output_type.parameter_types.is_empty() {
                if command_type.error_type.variant_types.is_empty() {
                    let tokens = quote! {
                        pub fn #command_id_snake_case(&self) {
                            let mut #command_name_snake_case = #command_name::initialize(
                                #command_code_name {
                                    closure: Box::new(|| #command_code_block),
                                }
                            );
            
                            #command_name_snake_case.execute();
            
                            match #command_name_snake_case.finalize() {
                                Some(_) => {},
                                None => panic!(#generated_interpolated_panic_message),
                            };
                        }
                    };

                    Self {
                        tokens
                    }
                } else {
                    let tokens = quote! {
                        pub fn #command_id_snake_case(&self) -> Result<(), #command_error_name> {
                            let mut #command_name_snake_case = #command_name::initialize(
                                #command_code_name {
                                    closure: Box::new(|| -> Result<(), #command_error_name> #command_code_block),
                                }
                            );
            
                            #command_name_snake_case.execute();
            
                            match #command_name_snake_case.finalize() {
                                Some(#command_result_name_snake_case) => return #command_result_name_snake_case,
                                None => panic!(#generated_interpolated_panic_message),
                            };
                        }
                    };

                    Self {
                        tokens
                    }
                }
            } else if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    pub fn #command_id_snake_case(&self) -> #command_output_name {
                        let mut #command_name_snake_case = #command_name::initialize(
                            #command_code_name {
                                closure: Box::new(|| -> #command_output_name #command_code_block),
                            }
                        );
        
                        #command_name_snake_case.execute();
        
                        match #command_name_snake_case.finalize() {
                            Some(#command_output_name_snake_case) => return #command_output_name_snake_case,
                            None => panic!(#generated_interpolated_panic_message),
                        };
                    }
                };

                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    pub fn #command_id_snake_case(&self) -> Result<#command_output_name, #command_error_name> {
                        let mut #command_name_snake_case = #command_name::initialize(
                            #command_code_name {
                                closure: Box::new(|| -> Result<#command_output_name, #command_error_name> #command_code_block),
                            }
                        );
        
                        #command_name_snake_case.execute();
        
                        match #command_name_snake_case.finalize() {
                            Some(#command_result_name_snake_case) => return #command_result_name_snake_case,
                            None => panic!(#generated_interpolated_panic_message),
                        };
                    }
                };

                Self {
                    tokens
                }
            }
        } else if command_type.output_type.parameter_types.is_empty()  {
            if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    pub fn #command_id_snake_case(&self, #generated_input_parameters) {
                        let mut #command_name_snake_case = #command_name::initialize(
                            #command_input_name {
                                #generated_input_parameter_names
                            },
                            #command_code_name {
                                closure: Box::new(|input: &#command_input_name| #command_code_block),
                            }
                        );
        
                        #command_name_snake_case.execute();
        
                        match #command_name_snake_case.finalize() {
                            Some(_) => {},
                            None => panic!(#generated_interpolated_panic_message),
                        };
                    }
                };

                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    pub fn #command_id_snake_case(&self, #generated_input_parameters) -> Result<(), #command_error_name> {
                        let mut #command_name_snake_case = #command_name::initialize(
                            #command_input_name {
                                #generated_input_parameter_names
                            },
                            #command_code_name {
                                closure: Box::new(|input: &#command_input_name| -> Result<(), #command_error_name> #command_code_block),
                            }
                        );
        
                        #command_name_snake_case.execute();
        
                        match #command_name_snake_case.finalize() {
                            Some(#command_result_name_snake_case) => return #command_result_name_snake_case,
                            None => panic!(#generated_interpolated_panic_message),
                        };
                    }
                };

                Self {
                    tokens
                }
            }
        } else if command_type.error_type.variant_types.is_empty() {
            let tokens = quote! {
                pub fn #command_id_snake_case(&self, #generated_input_parameters) -> #command_output_name {
                    let mut #command_name_snake_case = #command_name::initialize(
                        #command_input_name {
                            #generated_input_parameter_names
                        },
                        #command_code_name {
                            closure: Box::new(|input: &#command_input_name| -> #command_output_name #command_code_block),
                        }
                    );
    
                    #command_name_snake_case.execute();
    
                    match #command_name_snake_case.finalize() {
                        Some(#command_output_name_snake_case) => return #command_output_name_snake_case,
                        None => panic!(#generated_interpolated_panic_message),
                    };
                }
            };

            Self {
                tokens
            }
        } else {
            let tokens = quote! {
                pub fn #command_id_snake_case(&self, #generated_input_parameters) -> Result<#command_output_name, #command_error_name> {
                    let mut #command_name_snake_case = #command_name::initialize(
                        #command_input_name {
                            #generated_input_parameter_names
                        },
                        #command_code_name {
                            closure: Box::new(|input: &#command_input_name| -> Result<#command_output_name, #command_error_name> #command_code_block),
                        }
                    );
    
                    #command_name_snake_case.execute();
    
                    match #command_name_snake_case.finalize() {
                        Some(#command_result_name_snake_case) => return #command_result_name_snake_case,
                        None => panic!(#generated_interpolated_panic_message),
                    };
                }
            };

            Self {
                tokens
            }
        }
    }
}