use proc_macro2::TokenStream;
use syn::{parse::Parse, spanned::Spanned, Token, Result, ItemStruct, ItemEnum, ItemImpl};
use syn::parse::ParseStream;
use quote::ToTokens;

/// Represents a collection of user-defined types.
pub struct UserTypes(pub Vec<UserType>);

impl Parse for UserTypes {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut types = Vec::new();
        while !input.is_empty() {
            types.push(input.parse()?);
        }
        Ok(UserTypes(types))
    }
}

/// Represents a parsed user-defined type (struct or enum only).
#[derive(Debug)]
pub struct UserType {
    pub item: AllowedUserItem, // Struct, Enum, or Type Alias
    pub impls: Vec<TokenStream>,    // Allowed impl blocks (as strings)
}

/// The allowed user-defined Rust types.
#[derive(Debug)]
pub enum AllowedUserItem {
    Struct(TokenStream),
    Enum(TokenStream),
    TypeAlias(TokenStream),
}

impl Parse for UserType {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.lookahead1();
        let impls = Vec::new();

        if lookahead.peek(Token![struct]) {
            let item: ItemStruct = input.parse()?;
            let item_stream = item.to_token_stream();
            let mut user_type = UserType {
                item: AllowedUserItem::Struct(item_stream),
                impls,
            };

            // Parse potential associated impl blocks
            while input.peek(Token![impl]) {
                let parsed_impl: ItemImpl = input.parse()?;
                validate_impl(&parsed_impl, &user_type)?;
                user_type.impls.push(parsed_impl.to_token_stream());
            }

            Ok(user_type)
        } else if lookahead.peek(Token![enum]) {
            let item: ItemEnum = input.parse()?;
            let item_stream = item.to_token_stream();
            let mut user_type = UserType {
                item: AllowedUserItem::Enum(item_stream),
                impls,
            };

            // Parse potential associated impl blocks
            while input.peek(Token![impl]) {
                let parsed_impl: ItemImpl = input.parse()?;
                validate_impl(&parsed_impl, &user_type)?;
                user_type.impls.push(parsed_impl.to_token_stream());
            }

            Ok(user_type)
        } else if lookahead.peek(Token![type]) {
            let type_alias: syn::ItemType = input.parse()?;
            let item_stream = type_alias.to_token_stream();
            Ok(UserType {
                item: AllowedUserItem::TypeAlias(item_stream),
                impls,
            })
        } else {
            Err(syn::Error::new(
                input.span(),
                "Expected a `struct`, `enum`, or `type` alias. Other items (modules, functions, trait impls, etc.) are not allowed.",
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
        AllowedUserItem::Struct(name) => name.to_string(),
        AllowedUserItem::Enum(name) => name.to_string(),
        AllowedUserItem::TypeAlias(name) => name.to_string(),
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
