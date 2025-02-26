use proc_macro2::TokenStream;
use syn::{parse::Parse, Ident, Token, Result, braced, punctuated::Punctuated};
use syn::parse::ParseStream;
use quote::ToTokens;

/// Represents all valid permutations of core types in a stage.
pub enum CoreTypes {
    // Standard stages
    None,
    Input { input: CoreType },
    InputOutput { input: CoreType, output: CoreType },
    InputError { input: CoreType, error: CoreType },
    InputOutputError { input: CoreType, output: CoreType, error: CoreType },
    Output { output: CoreType },
    OutputError { output: CoreType, error: CoreType },
    Error { error: CoreType },

    // While stages (must include `state`)
    While { state: CoreType },
    InputWhile { input: CoreType, state: CoreType },
    InputWhileOutput { input: CoreType, state: CoreType, output: CoreType },
    InputWhileError { input: CoreType, state: CoreType, error: CoreType },
    InputWhileOutputError { input: CoreType, state: CoreType, output: CoreType, error: CoreType },
    WhileOutput { state: CoreType, output: CoreType },
    WhileOutputError { state: CoreType, output: CoreType, error: CoreType },
    WhileError { state: CoreType, error: CoreType },
}

/// Represents either a struct or an enum.
pub enum CoreType {
    Struct(CoreStruct),
    Enum(CoreEnum),
}

impl CoreTypes {
    /// Returns a unique identifier for the CoreTypes permutation.
    pub fn permutation(&self) -> &'static str {
        match self {
            CoreTypes::None => "None",
            CoreTypes::Input { .. } => "Input",
            CoreTypes::InputOutput { .. } => "InputOutput",
            CoreTypes::InputError { .. } => "InputError",
            CoreTypes::InputOutputError { .. } => "InputOutputError",
            CoreTypes::Output { .. } => "Output",
            CoreTypes::OutputError { .. } => "OutputError",
            CoreTypes::Error { .. } => "Error",

            CoreTypes::While { .. } => "While",
            CoreTypes::InputWhile { .. } => "InputWhile",
            CoreTypes::InputWhileOutput { .. } => "InputWhileOutput",
            CoreTypes::InputWhileError { .. } => "InputWhileError",
            CoreTypes::InputWhileOutputError { .. } => "InputWhileOutputError",
            CoreTypes::WhileOutput { .. } => "WhileOutput",
            CoreTypes::WhileOutputError { .. } => "WhileOutputError",
            CoreTypes::WhileError { .. } => "WhileError",
        }
    }
}

impl Parse for CoreTypes {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let mut input_type: Option<CoreType> = None;
        let mut state_type: Option<CoreType> = None;
        let mut output_type: Option<CoreType> = None;
        let mut error_type: Option<CoreType> = None;

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![struct]) {
                let core_struct: CoreStruct = input.parse()?;
                let core_type = CoreType::Struct(core_struct);
                match core_type {
                    CoreType::Struct(ref s) if s.name.to_string() == "Input" => input_type = Some(core_type),
                    CoreType::Struct(ref s) if s.name.to_string() == "State" => state_type = Some(core_type),
                    CoreType::Struct(ref s) if s.name.to_string() == "Output" => output_type = Some(core_type),
                    CoreType::Struct(ref s) if s.name.to_string() == "Error" => error_type = Some(core_type),
                    _ => {
                        return Err(syn::Error::new(
                            input.span(),
                            "Unexpected struct name. Expected `Input`, `State`, `Output`, or `Error`.",
                        ))
                    }
                }
            } else if lookahead.peek(Token![enum]) {
                let core_enum: CoreEnum = input.parse()?;
                let core_type = CoreType::Enum(core_enum);
                match core_type {
                    CoreType::Enum(ref e) if e.name.to_string() == "Input" => input_type = Some(core_type),
                    CoreType::Enum(ref e) if e.name.to_string() == "State" => state_type = Some(core_type),
                    CoreType::Enum(ref e) if e.name.to_string() == "Output" => output_type = Some(core_type),
                    CoreType::Enum(ref e) if e.name.to_string() == "Error" => error_type = Some(core_type),
                    _ => {
                        return Err(syn::Error::new(
                            input.span(),
                            "Unexpected enum name. Expected `Input`, `State`, `Output`, or `Error`.",
                        ))
                    }
                }
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "Expected a `struct` or `enum` in core types.",
                ));
            }
        }

        // Construct the correct variant based on what was found
        match (input_type, output_type, error_type, state_type) {
            // Standard stages
            (None, None, None, None) => Ok(CoreTypes::None),
            (Some(input), None, None, None) => Ok(CoreTypes::Input { input }),
            (Some(input), Some(output), None, None) => Ok(CoreTypes::InputOutput { input, output }),
            (Some(input), None, Some(error), None) => Ok(CoreTypes::InputError { input, error }),
            (Some(input), Some(output), Some(error), None) => Ok(CoreTypes::InputOutputError { input, output, error }),
            (None, Some(output), None, None) => Ok(CoreTypes::Output { output }),
            (None, Some(output), Some(error), None) => Ok(CoreTypes::OutputError { output, error }),
            (None, None, Some(error), None) => Ok(CoreTypes::Error { error }),

            // While stages
            (None, None, None, Some(state)) => Ok(CoreTypes::While { state }),
            (Some(input), None, None, Some(state)) => Ok(CoreTypes::InputWhile { input, state }),
            (Some(input), None, Some(error), Some(state)) => Ok(CoreTypes::InputWhileError { input, state, error }),
            (Some(input), Some(output), None, Some(state)) => Ok(CoreTypes::InputWhileOutput { input, state, output }),
            (Some(input), Some(output), Some(error), Some(state)) => Ok(CoreTypes::InputWhileOutputError { input, state, output, error }),
            (None, Some(output), None, Some(state)) => Ok(CoreTypes::WhileOutput { state, output }),
            (None, Some(output), Some(error), Some(state)) => Ok(CoreTypes::WhileOutputError { state, output, error }),
            (None, None, Some(error), Some(state)) => Ok(CoreTypes::WhileError { state, error }),
        }
    }
}


/// Represents a parsed struct (with forced `pub` access).
#[derive(Debug)]
pub struct CoreStruct {
    pub name: TokenStream,
    pub fields: Vec<CoreField>,
}

impl Parse for CoreStruct {
    fn parse(input: ParseStream) -> Result<Self> {
        let struct_token: Token![struct] = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let fields = Punctuated::<CoreField, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(CoreStruct {
            name: name.to_token_stream(),
            fields,
        })
    }
}

/// Represents a single field in a struct.
#[derive(Debug)]
pub struct CoreField {
    pub name: TokenStream,
    pub ty: TokenStream,
}

impl Parse for CoreField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let colon: Token![:] = input.parse()?;
        let ty: syn::Type = input.parse()?;

        Ok(CoreField {
            name: name.to_token_stream(),
            ty: ty.to_token_stream(),
        })
    }
}

/// Represents a parsed enum with struct-like variants.
#[derive(Debug)]
pub struct CoreEnum {
    pub name: TokenStream,
    pub variants: Vec<CoreEnumVariant>,
}

impl Parse for CoreEnum {
    fn parse(input: ParseStream) -> Result<Self> {
        let enum_token: Token![enum] = input.parse()?;
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let variants = Punctuated::<CoreEnumVariant, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(CoreEnum {
            name: name.into_token_stream(),
            variants,
        })
    }
}

/// Represents a single variant in an enum.
#[derive(Debug)]
pub struct CoreEnumVariant {
    pub name: TokenStream,
    pub fields: Vec<CoreField>,
}

impl Parse for CoreEnumVariant {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let fields = Punctuated::<CoreField, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(CoreEnumVariant {
            name: name.into_token_stream(),
            fields,
        })
    }
}
