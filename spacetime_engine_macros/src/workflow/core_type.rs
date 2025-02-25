use syn::{parse::Parse, Ident, Token, Result, braced, punctuated::Punctuated};
use syn::parse::ParseStream;
use quote::ToTokens;

/// Represents one permutation of the 4 available core types.
#[derive(Debug)]
pub struct CoreTypes {
    pub input: Option<CoreStruct>,
    pub output: Option<CoreStruct>,
    pub error: Option<CoreEnum>,
    pub state: Option<CoreStruct>,
}

impl Parse for CoreTypes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut core_types = CoreTypes {
            input: None,
            output: None,
            error: None,
            state: None,
        };

        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(Token![struct]) {
                let core_struct: CoreStruct = input.parse()?;
                match core_struct.name.as_str() {
                    "Input" => core_types.input = Some(core_struct),
                    "Output" => core_types.output = Some(core_struct),
                    "State" => core_types.state = Some(core_struct),
                    _ => {
                        return Err(syn::Error::new(
                            input.span(),
                            format!("Unexpected struct name `{}` in core types. Expected `Input`, `Output`, or `State`.", core_struct.name),
                        ))
                    }
                }
            } else if lookahead.peek(Token![enum]) {
                let core_enum: CoreEnum = input.parse()?;
                if core_enum.name == "Error" {
                    core_types.error = Some(core_enum);
                } else {
                    return Err(syn::Error::new(
                        input.span(),
                        format!("Unexpected enum name `{}` in core types. Expected `Error`.", core_enum.name),
                    ));
                }
            } else {
                return Err(syn::Error::new(
                    input.span(),
                    "Expected a `struct` or `enum` in core types.",
                ));
            }
        }

        Ok(core_types)
    }
}

/// Represents a parsed struct (with forced `pub` access).
#[derive(Debug)]
pub struct CoreStruct {
    pub name: String,
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
            name: name.to_string(),
            fields,
        })
    }
}

/// Represents a single field in a struct.
#[derive(Debug)]
pub struct CoreField {
    pub name: String,
    pub ty: String,
}

impl Parse for CoreField {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let colon: Token![:] = input.parse()?;
        let ty: syn::Type = input.parse()?;

        Ok(CoreField {
            name: name.to_string(),
            ty: ty.to_token_stream().to_string(),
        })
    }
}

/// Represents a parsed enum with struct-like variants.
#[derive(Debug)]
pub struct CoreEnum {
    pub name: String,
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
            name: name.to_string(),
            variants,
        })
    }
}

/// Represents a single variant in an enum.
#[derive(Debug)]
pub struct CoreEnumVariant {
    pub name: String,
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
            name: name.to_string(),
            fields,
        })
    }
}
