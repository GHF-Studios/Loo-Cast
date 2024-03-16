use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Block,
    ExprClosure,
    Ident,
    LitStr, 
    parse::{Parse, ParseStream}, 
    punctuated::Punctuated,
    spanned::Spanned,
    Token,
};

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() {
                info!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

struct CommandModuleType {
    module_name: LitStr,
    command_types: Vec<CommandType>,
}

impl Parse for CommandModuleType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let module_name = input.parse::<Ident>()?;
        let module_name = module_name.to_string();
        let module_name = LitStr::new(&module_name, module_name.span());

        let content;
        syn::braced!(content in input);

        let parsed_commands: Punctuated<CommandType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandModuleType {
            module_name,
            command_types: parsed_commands.into_iter().collect()
        })
    }

}

struct CommandType {
    command_name: LitStr,
    input_type: CommandInputType,
    output_type: CommandOutputType,
    error_type: CommandErrorType,
    code_type: CommandCodeType
}

impl Parse for CommandType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let command_name = input.parse::<Ident>()?;
        let command_name = command_name.to_string();
        let command_name = LitStr::new(&command_name, command_name.span());

        let content;
        syn::braced!(content in input);

        let input_type = content.parse::<CommandInputType>()?;

        content.parse::<Token![,]>()?;

        let output_type = content.parse::<CommandOutputType>()?;

        content.parse::<Token![,]>()?;

        let error_type = content.parse::<CommandErrorType>()?;

        content.parse::<Token![,]>()?;

        let code_type = content.parse::<CommandCodeType>()?;

        Ok(CommandType {
            command_name,
            input_type,
            output_type,
            error_type,
            code_type
        })
    }

}

struct CommandInputType {
    parameter_types: Vec<CommandInputParameterType>
}

impl Parse for CommandInputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let input_label = input.parse::<Ident>()?;
        let span = input_label.span();
        let input_label = input_label.to_string();

        if input_label != "Input" {
            return Err(syn::Error::new(span, "Expected 'Input' Label"));
        }

        let content;
        syn::braced!(content in input);

        let parsed_parameters: Punctuated<CommandInputParameterType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandInputType {
            parameter_types: parsed_parameters.into_iter().collect()
        })
    }
}

struct CommandInputParameterType {
    parameter_name: LitStr,
    parameter_type: syn::Type
}

impl Parse for CommandInputParameterType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parameter_label = input.parse::<Ident>()?;
        let parameter_name = parameter_label.to_string();
        let parameter_name = LitStr::new(&parameter_name, parameter_name.span());

        input.parse::<Token![:]>()?;

        let parameter_type = input.parse()?;

        Ok(CommandInputParameterType {
            parameter_name,
            parameter_type
        })
    }

}
struct CommandOutputType {
    parameter_types: Vec<CommandOutputParameterType>
}

impl Parse for CommandOutputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let output_label = input.parse::<Ident>()?;
        let span = output_label.span();
        let output_label = output_label.to_string();

        if output_label != "Output" {
            return Err(syn::Error::new(span, "Expected 'Output' Label"));
        }

        let content;
        syn::braced!(content in input);

        let parsed_parameters: Punctuated<CommandOutputParameterType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandOutputType {
            parameter_types: parsed_parameters.into_iter().collect()
        })
    }
}

struct CommandOutputParameterType {
    parameter_name: LitStr,
    parameter_type: syn::Type
}

impl Parse for CommandOutputParameterType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parameter_label = input.parse::<Ident>()?;
        let parameter_name = parameter_label.to_string();
        let parameter_name = LitStr::new(&parameter_name, parameter_name.span());

        input.parse::<Token![:]>()?;

        let parameter_type = input.parse()?;

        Ok(CommandOutputParameterType {
            parameter_name,
            parameter_type
        })
    }

}

struct CommandErrorType {
    error_variants: Vec<CommandErrorVariantType>
}

impl Parse for CommandErrorType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let error_label = input.parse::<Ident>()?;
        let span = error_label.span();
        let error_label = error_label.to_string();

        if error_label != "Error" {
            return Err(syn::Error::new(span, "Expected 'Error' Label"));
        }

        let content;
        syn::braced!(content in input);

        let parsed_variants: Punctuated<CommandErrorVariantType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandErrorType {
            error_variants: parsed_variants.into_iter().collect()
        })
    }
}

struct CommandErrorVariantType {
    variant_name: LitStr
}

impl Parse for CommandErrorVariantType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variant_label = input.parse::<Ident>()?;
        let variant_name = variant_label.to_string();
        let variant_name = LitStr::new(&variant_name, variant_name.span());

        Ok(CommandErrorVariantType {
            variant_name
        })
    }
}

struct CommandCodeType {
    code_block: Block
}

impl Parse for CommandCodeType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let code_label = input.parse::<Ident>()?;
        let span = code_label.span();
        let code_label = code_label.to_string();

        if code_label != "Code" {
            return Err(syn::Error::new(span, "Expected 'Code' Label"));
        }

        input.parse::<Token![|]>()?;

        let input_parameter_label = input.parse::<Ident>()?;
        let span = input_parameter_label.span();
        let input_parameter_label = input_parameter_label.to_string();

        if input_parameter_label != "input" {
            return Err(syn::Error::new(span, "Expected 'input' Label"));
        }

        input.parse::<Token![|]>()?;

        input.parse::<Token![-]>()?;
        input.parse::<Token![>]>()?;

        let result_paramater_label = input.parse::<Ident>()?;
        let span = result_paramater_label.span();
        let result_paramater_label = result_paramater_label.to_string();

        if result_paramater_label != "Result" {
            return Err(syn::Error::new(span, "Expected 'Result' Label"));
        }

        input.parse::<Token![<]>()?;

        let output_parameter_label = input.parse::<Ident>()?;
        let span = output_parameter_label.span();
        let output_parameter_label = output_parameter_label.to_string();

        if output_parameter_label != "Output" {
            return Err(syn::Error::new(span, "Expected 'Output' Label"));
        }

        input.parse::<Token![,]>()?;

        let error_parameter_label = input.parse::<Ident>()?;
        let span = error_parameter_label.span();
        let error_parameter_label = error_parameter_label.to_string();

        if error_parameter_label != "Error" {
            return Err(syn::Error::new(span, "Expected 'Error' Label"));
        }

        input.parse::<Token![>]>()?;


        let code_block = input.parse::<Block>()?;

        Ok(CommandCodeType {
            code_block
        })
    }

}

#[proc_macro]
pub fn define_commands_module(tokens: TokenStream) -> TokenStream {
    let parsed_module = syn::parse_macro_input!(tokens as CommandModuleType);

    let generated_code = quote! {
        pub(in crate::kernel::commands) trait TestCommand {
            type Module: CommandsModule;
            type Input: TestCommandInput<Command = Self>;
            type Output: TestCommandOutput<Command = Self>;
            type Error: TestCommandError<Command = Self>;
            type Code: TestCommandCode<Command = Self>;
        
            fn initialize(input: Self::Input, code: Self::Code) -> Self;
            fn execute(&mut self);
            fn finalize(self) -> Option<Result<Self::Output, Self::Error>>;
        }
        
        pub(in crate::kernel::commands) trait TestCommandInput: Display {
            type Command: TestCommand;
        }
        
        pub(in crate::kernel::commands) trait TestCommandOutput: Display {
            type Command: TestCommand;
        }
        
        pub(in crate::kernel::commands) trait TestCommandError: Display {
            type Command: TestCommand;
        }
        
        pub(in crate::kernel::commands) trait TestCommandCode: Display {
            type Command: TestCommand;
        }
        
        pub struct TestCommands {
        }
        
        impl CommandsModule for TestCommands {
            fn module_name() -> &'static str {
                "TestCommands"
            }
        }
        
        impl TestCommands {
            pub fn hello_world(value: i32) -> Result<HelloWorldCommandOutput, HelloWorldCommandError> {
                let mut hello_world_command = HelloWorldCommand::initialize(
                    HelloWorldCommandInput {
                        value,
                    },
                    HelloWorldCommandCode {
                        closure: |input| -> Result<HelloWorldCommandOutput, HelloWorldCommandError> {
                            if input.value == 0 {
                                Ok(HelloWorldCommandOutput {
                                    value: input.value,
                                })
                            } else {
                                Err(HelloWorldCommandError::InvalidInput)
                            }
                        },
                    }
                );
        
                hello_world_command.execute();
        
                if let Some(hello_world_command_result) = hello_world_command.finalize() {
                    hello_world_command_result
                } else {
                    panic!("Command did not execute properly!");
                }
            }
        }
        
        pub(in crate::kernel::commands) enum HelloWorldCommand {
            Initialized {
                input: HelloWorldCommandInput,
                code: HelloWorldCommandCode,
            },
            Executed {
                result: Result<HelloWorldCommandOutput, HelloWorldCommandError>,
            },
        }
        
        impl TestCommand for HelloWorldCommand {
            type Module = TestCommands;
            type Input = HelloWorldCommandInput;
            type Output = HelloWorldCommandOutput;
            type Error = HelloWorldCommandError;
            type Code = HelloWorldCommandCode;
        
            fn initialize(input: Self::Input, code: Self::Code) -> Self {
                HelloWorldCommand::Initialized {
                    input,
                    code,
                }
            }
        
            fn execute(&mut self) {
                if let HelloWorldCommand::Initialized { input, code } = self {
                    *self = HelloWorldCommand::Executed {
                        result: (code.closure)(input),
                    };
                }
            }
        
            fn finalize(self) -> Option<Result<Self::Output, Self::Error>> {
                if let HelloWorldCommand::Executed { result } = self {
                    Some(result)
                } else {
                    None
                }
            }
        }
        
        pub(in crate::kernel::commands) struct HelloWorldCommandInput {
            pub value: i32,
        }
        
        impl TestCommandInput for HelloWorldCommandInput {
            type Command = HelloWorldCommand;
        }
        
        impl Display for HelloWorldCommandInput {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                write!(f, "HelloWorldCommandInput {{ value: {} }}", self.value)
            }
        }
        
        pub struct HelloWorldCommandOutput {
            pub value: i32,
        }
        
        impl TestCommandOutput for HelloWorldCommandOutput {
            type Command = HelloWorldCommand;
        }
        
        impl Display for HelloWorldCommandOutput {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                write!(f, "HelloWorldCommandOutput {{ value: {} }}", self.value)
            }
        }
        
        pub enum HelloWorldCommandError {
            InvalidInput,
        }
        
        impl TestCommandError for HelloWorldCommandError {
            type Command = HelloWorldCommand;
        }
        
        impl Display for HelloWorldCommandError {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                match *self {
                    HelloWorldCommandError::InvalidInput => {
                        return write!(f, "HelloWorldCommandError::InvalidInput");
                    },
                }
            }
        }
        
        pub(in crate::kernel::commands) struct HelloWorldCommandCode {
            closure: fn(&HelloWorldCommandInput) -> Result<HelloWorldCommandOutput, HelloWorldCommandError>,
        }
        
        impl TestCommandCode for HelloWorldCommandCode {
            type Command = HelloWorldCommand;
        }
        
        impl Display for HelloWorldCommandCode {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                write!(f, "HelloWorldCommandCode {{ closure: {:?} }}", self.closure)
            }
        }
    };

    TokenStream::from(generated_code)
}