use syn::Ident;
use crate::commands::parsed_input::command_type::CommandType;
use quote::quote;

pub struct CommandEnumCode {
    pub tokens: proc_macro2::TokenStream,
}

impl CommandEnumCode {
    pub fn generate(
        command_type: &CommandType,
        command_name: Ident,
        command_input_name: Ident,
        command_output_name: Ident,
        command_error_name: Ident,
        command_code_name: Ident,
    ) -> Self {
        if command_type.input_type.parameter_types.is_empty() {
            if command_type.output_type.parameter_types.is_empty() {
                if command_type.error_type.variant_types.is_empty() {
                    let tokens = quote! {
                        pub enum #command_name {
                            Initialized {
                                code: #command_code_name,
                            },
                            Executed {},
                        }
                    };

                    Self {
                        tokens
                    }
                } else {
                    let tokens = quote! {
                        pub enum #command_name {
                            Initialized {
                                code: #command_code_name,
                            },
                            Executed {
                                result: Result<(), #command_error_name>,
                            },
                        }
                    };

                    Self {
                        tokens
                    }
                }
            } else if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    pub enum #command_name {
                        Initialized {
                            code: #command_code_name,
                        },
                        Executed {
                            output: #command_output_name,
                        },
                    }
                };

                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    pub enum #command_name {
                        Initialized {
                            code: #command_code_name,
                        },
                        Executed {
                            result: Result<#command_output_name, #command_error_name>,
                        },
                    }
                };

                Self {
                    tokens
                }
            }
        } else if command_type.output_type.parameter_types.is_empty()  {
            if command_type.error_type.variant_types.is_empty() {
                let tokens = quote! {
                    pub enum #command_name {
                        Initialized {
                            input: #command_input_name,
                            code: #command_code_name,
                        },
                        Executed {},
                    }
                };

                Self {
                    tokens
                }
            } else {
                let tokens = quote! {
                    pub enum #command_name {
                        Initialized {
                            input: #command_input_name,
                            code: #command_code_name,
                        },
                        Executed {
                            result: Result<(), #command_error_name>,
                        },
                    }
                };

                Self {
                    tokens
                }
            }
        } else if command_type.error_type.variant_types.is_empty() {
            let tokens = quote! {
                pub enum #command_name {
                    Initialized {
                        input: #command_input_name,
                        code: #command_code_name,
                    },
                    Executed {
                        result: #command_output_name,
                    },
                }
            };

            Self {
                tokens
            }
        } else {
            let tokens = quote! {
                pub enum #command_name {
                    Initialized {
                        input: #command_input_name,
                        code: #command_code_name,
                    },
                    Executed {
                        result: Result<#command_output_name, #command_error_name>,
                    },
                }
            };

            Self {
                tokens
            }
        }
    }
}