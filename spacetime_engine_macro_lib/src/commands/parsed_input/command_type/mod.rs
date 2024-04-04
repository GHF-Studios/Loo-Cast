pub mod input_type;
pub mod output_type;
pub mod error_type;
pub mod code_type;

use input_type::*;
use output_type::*;
use error_type::*;
use code_type::*;
use code_type::signature_type::return_type::*;
use code_type::signature_type::return_type::output_type::*;
use syn::{
    parse::{Parse, ParseStream}, spanned::Spanned, Ident, LitStr, Token
};

#[derive(Clone)]
pub struct CommandType {
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

        if input_type.parameter_types.is_empty() {
            if code_type.code_signature.input_type.is_some() {
                return Err(syn::Error::new(input.span(), "Expected Input Type Definition"));
            }
        } else if code_type.code_signature.input_type.is_none() {
            return Err(syn::Error::new(input.span(), "Expected Input Type Usage"));
        }

        if output_type.parameter_types.is_empty() {
            match code_type.code_signature.return_type {
                CommandCodeSignatureReturnType::OutputType(ref output_type) => {
                    if let CommandCodeSignatureOutputType::OutputType = output_type {
                        return Err(syn::Error::new(input.span(), "Expected Output Type Definition"));
                    }
                }
                CommandCodeSignatureReturnType::ResultType(ref result_type) => {
                    if let CommandCodeSignatureOutputType::OutputType = result_type.output_type {
                        return Err(syn::Error::new(input.span(), "Expected Output Type Definition"));
                    }
                }
            }
        } else {
            match code_type.code_signature.return_type {
                CommandCodeSignatureReturnType::OutputType(ref output_type) => {
                    if let CommandCodeSignatureOutputType::UnitType = output_type {
                        return Err(syn::Error::new(input.span(), "Expected Output Type Usage"));
                    }
                }
                CommandCodeSignatureReturnType::ResultType(ref result_type) => {
                    if let CommandCodeSignatureOutputType::UnitType = result_type.output_type {
                        return Err(syn::Error::new(input.span(), "Expected Output Type Usage"));
                    }
                }
            }
        }

        if error_type.variant_types.is_empty() {
            if let CommandCodeSignatureReturnType::ResultType(_) = code_type.code_signature.return_type {
                return Err(syn::Error::new(input.span(), "Expected Error Type Definition"));
            }
        } else if let CommandCodeSignatureReturnType::OutputType(_) = code_type.code_signature.return_type {
            return Err(syn::Error::new(input.span(), "Expected Error Type Usage"));
        }

        Ok(CommandType {
            command_id,
            input_type,
            output_type,
            error_type,
            code_type
        })
    }

}
