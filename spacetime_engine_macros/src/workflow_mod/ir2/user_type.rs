use proc_macro2::TokenStream;
use quote::quote;

use crate::workflow_mod::ir1::user_type::AllowedUserItem;

/// Represents a collection of user-defined types.
pub struct UserTypes {
    pub items: Vec<UserType>,
}

impl From<crate::workflow_mod::ir1::user_type::UserTypes> for UserTypes {
    fn from(ir1: crate::workflow_mod::ir1::user_type::UserTypes) -> Self {
        Self {
            items: ir1.0.into_iter().map(UserType::from).collect(),
        }
    }
}

impl UserTypes {
    /// Generates Rust code for all user-defined types.
    pub fn generate(&self) -> TokenStream {
        let types: Vec<TokenStream> = self.items.iter().map(|ty| ty.generate()).collect();

        quote! {
            #(#types)*
        }
    }
}

/// Represents a user-defined type (struct, enum, or type alias).
pub struct UserType {
    pub tokens: TokenStream, // Just store the full raw Rust code
}

impl From<crate::workflow_mod::ir1::user_type::UserType> for UserType {
    fn from(ir1: crate::workflow_mod::ir1::user_type::UserType) -> Self {
        let item = match ir1.item {
            AllowedUserItem::Struct(item) => item,
            AllowedUserItem::Enum(item) => item,
            AllowedUserItem::TypeAlias(item) => item,
        };
        let impls = ir1.impls;

        Self {
            tokens: quote!{
                #item
                #(#impls)*
            },
        }
    }
}

impl UserType {
    /// Generates Rust code for the user-defined type.
    pub fn generate(&self) -> TokenStream {
        self.tokens.clone()
    }
}
