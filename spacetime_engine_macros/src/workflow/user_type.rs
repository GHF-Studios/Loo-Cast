use syn::{parse::Parse, spanned::Spanned, Ident, Token, Result, ItemStruct, ItemEnum, ItemImpl};
use syn::parse::ParseStream;
use quote::ToTokens;

/// Represents a parsed user-defined type (struct or enum only).
#[derive(Debug)]
pub struct UserType {
    pub item: AllowedUserItem, // Struct or Enum
    pub impls: Vec<String>,    // Allowed impl blocks (as strings)
}

/// The allowed user-defined Rust types.
#[derive(Debug)]
pub enum AllowedUserItem {
    Struct(String),
    Enum(String),
}

impl Parse for UserType {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let impls = Vec::new();

        if lookahead.peek(Token![struct]) {
            let item: ItemStruct = input.parse()?;
            let item_str = item.to_token_stream().to_string();
            let mut user_type = UserType {
                item: AllowedUserItem::Struct(item_str),
                impls,
            };

            // Parse potential associated impl blocks
            while input.peek(Token![impl]) {
                let parsed_impl: ItemImpl = input.parse()?;
                validate_impl(&parsed_impl, &user_type)?;
                user_type.impls.push(parsed_impl.to_token_stream().to_string());
            }

            Ok(user_type)
        } else if lookahead.peek(Token![enum]) {
            let item: ItemEnum = input.parse()?;
            let item_str = item.to_token_stream().to_string();
            let mut user_type = UserType {
                item: AllowedUserItem::Enum(item_str),
                impls,
            };

            // Parse potential associated impl blocks
            while input.peek(Token![impl]) {
                let parsed_impl: ItemImpl = input.parse()?;
                validate_impl(&parsed_impl, &user_type)?;
                user_type.impls.push(parsed_impl.to_token_stream().to_string());
            }

            Ok(user_type)
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected a `struct` or `enum`. Other items (modules, functions, type aliases, trait impls, etc.) are not allowed.",
            ))
        }
    }
}

/// Ensures the impl block is for the correct type and is not a trait impl.
fn validate_impl(item_impl: &ItemImpl, user_type: &UserType) -> Result<()> {
    if item_impl.trait_.is_some() {
        return Err(syn::Error::new(
            item_impl.span(),
            "Trait implementations are not allowed. Only inherent impl blocks for the defined struct/enum are permitted.",
        ));
    }

    // Ensure the impl block matches the user-defined type
    let type_name = match &item_impl.self_ty.as_ref() {
        syn::Type::Path(type_path) => type_path.path.segments.last().map(|s| s.ident.to_string()),
        _ => None,
    };

    let expected_name = match &user_type.item {
        AllowedUserItem::Struct(name) => name.split_whitespace().nth(1).unwrap_or("").to_string(),
        AllowedUserItem::Enum(name) => name.split_whitespace().nth(1).unwrap_or("").to_string(),
    };

    if type_name.as_deref() != Some(&expected_name) {
        return Err(syn::Error::new(
            item_impl.span(),
            format!(
                "Impl block is for `{}`, but the defined type is `{}`. Only impl blocks for the defined type are allowed.",
                type_name.unwrap_or("unknown".to_string()),
                expected_name
            ),
        ));
    }

    Ok(())
}
