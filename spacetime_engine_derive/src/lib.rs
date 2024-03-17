use proc_macro::TokenStream;
use quote::quote;
use syn::{
    Block,
    Ident,
    LitStr, 
    parse_macro_input,
    parse::{Parse, ParseStream}, 
    Path,
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

pub(crate) struct CommandModuleType {
    pub module_id: LitStr,
    pub module_path: Path,
    pub command_types: CommandTypes,
}

impl Parse for CommandModuleType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let module_id = input.parse::<Ident>()?;
        let module_id = module_id.to_string();
        let module_id = LitStr::new(&module_id, module_id.span());

        let content;
        syn::braced!(content in input);

        let module_path_label = content.parse::<Ident>()?;
        let span = module_path_label.span();
        let module_path_label = module_path_label.to_string();

        if module_path_label != "module_path" {
            return Err(syn::Error::new(span, "Expected 'module_path' Label"));
        }

        content.parse::<Token![:]>()?;

        let module_path = content.parse::<Path>()?;

        content.parse::<Token![,]>()?;

        let module_commands_label = content.parse::<Ident>()?;
        let span = module_commands_label.span();
        let module_commands_label = module_commands_label.to_string();

        if module_commands_label != "commands" {
            return Err(syn::Error::new(span, "Expected 'commands' Label"));
        }

        content.parse::<Token![:]>()?;

        let command_types = CommandTypes::parse(&content)?;

        Ok(CommandModuleType {
            module_id,
            module_path,
            command_types
        })
    }

}

#[derive(Clone)]
pub(crate) struct CommandTypes(Vec<CommandType>);

impl Parse for CommandTypes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        syn::bracketed!(content in input);

        let parsed_commands: Punctuated<CommandType, Token![,]> = Punctuated::parse_terminated(&content)?;

        Ok(CommandTypes(parsed_commands.into_iter().collect()))
    }
}

#[derive(Clone)]
pub(crate) struct CommandType {
    pub command_id: LitStr,
    pub input_type: CommandInputType,
    pub output_type: CommandOutputType,
    pub error_type: CommandErrorType,
    pub code_type: CommandCodeType
}

impl Parse for CommandType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let command_id = input.parse::<Ident>()?;
        let command_id = command_id.to_string();
        let command_id = LitStr::new(&command_id, command_id.span());

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
            command_id,
            input_type,
            output_type,
            error_type,
            code_type
        })
    }

}

#[derive(Clone)]
pub(crate) struct CommandInputType {
    pub parameter_types: Vec<CommandInputParameterType>
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

#[derive(Clone)]
pub(crate) struct CommandInputParameterType {
    pub parameter_name: LitStr,
    pub parameter_type: syn::Type
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

#[derive(Clone)]
pub(crate) struct CommandOutputType {
    pub parameter_types: Vec<CommandOutputParameterType>
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

#[derive(Clone)]
pub(crate) struct CommandOutputParameterType {
    pub parameter_name: LitStr,
    pub parameter_type: syn::Type
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

#[derive(Clone)]
pub(crate) struct CommandErrorType {
    pub error_variants: Vec<CommandErrorVariantType>
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

#[derive(Clone)]
pub(crate) struct CommandErrorVariantType {
    pub variant_name: LitStr
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

#[derive(Clone)]
pub(crate) struct CommandCodeType {
    pub code_block: Block
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
    let command_module_type = parse_macro_input!(tokens as CommandModuleType);
    let command_module_id = command_module_type.module_id.value().to_string();
    let command_module_name = Ident::new(&(command_module_id.clone() + "Commands"), command_module_id.span());

    let mut generated_command_request_functions = Vec::<proc_macro2::TokenStream>::new();
    for command_type in command_module_type.command_types.0 {
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

        let command_name = command_id.clone() + "Command";
        let mut command_name_snake_case = String::new();
        let mut prev_was_uppercase = false;
        for (i, c) in command_name.chars().enumerate() {
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
        

        let command_module_name = command_id.clone() + "Commands";

        let command_input_name = command_id.clone() + "CommandInput";

        let command_output_name = command_id.clone() + "CommandOutput";

        let command_error_name = command_id.clone() + "CommandError";

        let command_code_name = command_id.clone() + "CommandCode";
        let command_code_block = command_type.code_type.code_block;

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

        let command_id = Ident::new(&command_id, command_id.span());
        let command_id_snake_case = Ident::new(&command_id_snake_case, command_id.span());
        let command_name = Ident::new(&command_name, command_id.span());
        let command_name_snake_case = Ident::new(&command_name_snake_case, command_id.span());
        let command_input_name = Ident::new(&command_input_name, command_id.span());
        let command_output_name = Ident::new(&command_output_name, command_id.span());
        let command_error_name = Ident::new(&command_error_name, command_id.span());
        let command_code_name = Ident::new(&command_code_name, command_id.span());
        let command_result_name_snake_case = Ident::new(&command_result_name_snake_case, command_id.span());

        let input_parameter_infos: Vec<(LitStr, syn::Type)> = command_type.input_type.parameter_types.iter().map(|parameter_type| {
            (parameter_type.parameter_name.clone(), parameter_type.parameter_type.clone())
        }).collect();
        let mut generated_input_parameters = quote! {};
        for (parameter_name, parameter_type) in input_parameter_infos.clone() {
            let parameter_name = Ident::new(&parameter_name.value(), parameter_name.span());
            generated_input_parameters = quote! {
                #generated_input_parameters
                #parameter_name: #parameter_type,
            };
        }

        let mut generated_input_parameter_names = quote! {};
        for (parameter_name, _) in input_parameter_infos {
            let parameter_name = Ident::new(&parameter_name.value(), parameter_name.span());
            generated_input_parameter_names = quote! {
                #generated_input_parameter_names
                #parameter_name,
            };
        }

        let generated_command_request_function = quote! {
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

                if let Some(#command_result_name_snake_case) = #command_name_snake_case.finalize() {
                    #command_result_name_snake_case
                } else {
                    panic!("#command_name did not execute properly!");
                }
            }
        };

        generated_command_request_functions.push(generated_command_request_function);
    }

    let mut generated_command_module_impl_content = quote! {
    };
    for generated_command_request_function in generated_command_request_functions {
        println!("Generated Function: {}", generated_command_request_function);
        generated_command_module_impl_content = quote! {
            #generated_command_module_impl_content
            #generated_command_request_function
        };
    }

    let generated_command_module_struct = quote! {
        pub struct #command_module_name {
        }
        
        impl #command_module_name {
            #generated_command_module_impl_content
        }
    };

    let generated_code = quote! {
        #generated_command_module_struct

        pub(in crate::kernel::commands) enum HelloWorldCommand {
            Initialized {
                input: HelloWorldCommandInput,
                code: HelloWorldCommandCode,
            },
            Executed {
                result: Result<HelloWorldCommandOutput, HelloWorldCommandError>,
            },
        }
        
        impl HelloWorldCommand {
            fn initialize(input: HelloWorldCommandInput, code: HelloWorldCommandCode) -> Self {
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
        
            fn finalize(self) -> Option<Result<HelloWorldCommandOutput, HelloWorldCommandError>> {
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
        
        impl Display for HelloWorldCommandInput {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                write!(f, "HelloWorldCommandInput {{ value: {} }}", self.value)
            }
        }
        
        pub struct HelloWorldCommandOutput {
            pub value: i32,
        }
        
        impl Display for HelloWorldCommandOutput {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                write!(f, "HelloWorldCommandOutput {{ value: {} }}", self.value)
            }
        }
        
        pub enum HelloWorldCommandError {
            InvalidInput,
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
            closure: Box<dyn Fn(&HelloWorldCommandInput) -> Result<HelloWorldCommandOutput, HelloWorldCommandError>>,
        }
        
        impl Display for HelloWorldCommandCode {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                write!(f, "HelloWorldCommandCode {{ closure: No Display }}")
            }
        }
    };

    TokenStream::from(generated_code)
}

fn generate_command_module(command_module_type: CommandModuleType) -> TokenStream {
    todo!();
}

fn generate_command(command_type: CommandType) -> TokenStream {
    todo!();
}

fn generate_command_input(
    command_module_type: CommandModuleType, 
    command_type: CommandType, 
    command_input_type: CommandInputType
) -> TokenStream {
    let module_id = command_module_type.module_id.value().to_string();
    let module_command_input_trait_name = Ident::new(&(module_id.clone() + "CommandInput"), module_id.span());
    
    let command_id = command_type.command_id.value().to_string();
    let command_input_name = Ident::new(&(command_id.clone() + "CommandInput"), command_id.span());

    let command_input_parameter_types: Vec<CommandInputParameterType> = command_input_type.parameter_types;

    let generated_code = quote! {
        pub(in crate::kernel::commands) struct #command_input_name {
            pub value: i32,
        }
        
        impl #module_command_input_trait_name for #command_input_name {
        }
        
        impl Display for #command_input_name {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
                write!(f, "#command_input_name {{ value: {} }}", self.value)
            }
        }
    };

    generated_code.into()
}

fn generate_command_output(command_output_type: CommandOutputType) -> TokenStream {
    todo!();
}

fn generate_command_error(command_error_type: CommandErrorType) -> TokenStream {
    todo!();
}

fn generate_command_code(command_code_type: CommandCodeType) -> TokenStream {
    todo!();
}