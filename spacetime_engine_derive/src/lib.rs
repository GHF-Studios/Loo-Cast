use proc_macro::TokenStream;
use quote::quote;
use syn::{
    punctuated::Punctuated,
    LitStr, 
    token::Comma, 
    Token,
    parse::{Parse, ParseStream}, 
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
        let module_name = input.parse::<LitStr>()?;

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
}

impl Parse for CommandType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let command_name = input.parse::<LitStr>()?;

        let content;
        syn::braced!(content in input);

        let input_type = content.parse::<CommandInputType>()?;

        content.parse::<Token![,]>()?;

        let output_type = content.parse::<CommandOutputType>()?;

        content.parse::<Token![,]>()?;

        let error_type = content.parse::<CommandErrorType>()?;

        Ok(CommandType {
            command_name,
            input_type,
            output_type,
            error_type
        })
    }

}

struct CommandInputType {
    parameter_types: Vec<CommandInputParameterType>
}

impl Parse for CommandInputType {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        input.parse::<LitStr>()?;

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
        let parameter_name = input.parse()?;

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
        input.parse::<LitStr>()?;

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
        let parameter_name = input.parse()?;

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
        input.parse::<LitStr>()?;

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
        let variant_name = input.parse()?;

        Ok(CommandErrorVariantType {
            variant_name
        })
    }
}

#[proc_macro]
pub fn define_commands_module(tokens: TokenStream) -> TokenStream {
    let parsed_module = syn::parse_macro_input!(tokens as CommandModuleType);

    let generated_code = quote! {
        // Generated code will go here
    };

    TokenStream::from(generated_code)
}