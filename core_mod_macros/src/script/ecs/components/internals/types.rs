use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use syn::parse::{Parse, ParseStream, Result};
use syn::{Item, Token, parse_macro_input};
use syn::{ItemStruct, ItemEnum};
use quote::quote;

pub enum ComponentItem {
    Struct(ItemStruct),
    Enum(ItemEnum),
}

pub fn component_ctor(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as Item);

    ComponentCtorInput::new(item)
        .generate()
        .into()
}

pub struct ComponentCtorInput {
    pub item: ComponentItem,
}
impl ComponentCtorInput {
    pub fn new(item: Item) -> Self {
        match item {
            Item::Struct(s) => Self {
                item: ComponentItem::Struct(s),
            },
            Item::Enum(e) => Self {
                item: ComponentItem::Enum(e),
            },
            _ => {
                panic!("`#[component_ctor]` can only be used on structs or enums");
            }
        }
    }
}

impl Parse for ComponentCtorInput {
    fn parse(input: ParseStream) -> Result<Self> {
        if input.peek(Token![struct]) {
            Ok(Self {
                item: ComponentItem::Struct(input.parse()?),
            })
        } else if input.peek(Token![enum]) {
            Ok(Self {
                item: ComponentItem::Enum(input.parse()?),
            })
        } else {
            Err(input.error("expected struct or enum"))
        }
    }
}
impl ComponentCtorInput {
    pub fn generate(self) -> TokenStream2 {
        let (item, ident) = match self.item {
            ComponentItem::Struct(s) => {
                let ident = &s.ident;
                (quote!(#s), ident.clone())
            }
            ComponentItem::Enum(e) => {
                let ident = &e.ident;
                (quote!(#e), ident.clone())
            }
        };

        let name = ident.to_string();

        quote! {
            #item

            // Compile-time contract enforcement
            const _: fn() = || {
                fn _assert<T: crate::script::core::internals::traits::InsertComponentFromDynamic>() {}
                _assert::<#ident>();
            };

            inventory::submit! {
                crate::script::ecs::component::internals::types::ComponentCtorEntry {
                    name: #name,
                    ctor: |entity, params| {
                        <#ident as crate::script::core::internals::traits::InsertComponentFromDynamic>::insert_component_from_dynamic(entity, params)
                    }
                }
            }
        }
    }
}
