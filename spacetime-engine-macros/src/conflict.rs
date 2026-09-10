use proc_macro2::{Span, TokenStream};
use proc_macro_crate::{FoundCrate, crate_name};
use quote::{quote, quote_spanned};
use syn::{
    Ident, Item, Result, Token, Type,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    spanned::Spanned,
};

/// The Cargo package name of the runtime crate.
///
/// This must match `[package].name` in the runtime crate's Cargo.toml.
const SPACETIME_ENGINE_CRATE: &str = "spacetime-engine";

struct ConflictArguments {
    components: Punctuated<Type, Token![,]>,
}

impl Parse for ConflictArguments {
    fn parse(input: ParseStream) -> Result<Self> {
        let components =
            Punctuated::<Type, Token![,]>::parse_terminated(input)?;

        if components.is_empty() {
            return Err(input.error(
                "expected at least one conflicting component type",
            ));
        }

        Ok(Self { components })
    }
}

/// Parsed representation of `#[conflict(...)]`.
///
/// The macro itself performs no runtime conflict handling. It only emits
/// inventory registrations consumed by the spacetime engine's component-conflict
/// subsystem.
pub struct Conflict {
    item: Item,
    component_ident: Ident,
    conflicting_components: Vec<Type>,
}

impl Conflict {
    pub fn parse(
        attr: TokenStream,
        item: TokenStream,
    ) -> Result<Self> {
        let arguments = syn::parse2::<ConflictArguments>(attr)?;
        let item = syn::parse2::<Item>(item)?;

        let component_ident = match &item {
            Item::Struct(item_struct) => {
                ensure_non_generic(&item_struct.generics)?;
                item_struct.ident.clone()
            }

            Item::Enum(item_enum) => {
                ensure_non_generic(&item_enum.generics)?;
                item_enum.ident.clone()
            }

            _ => {
                return Err(syn::Error::new_spanned(
                    &item,
                    "#[conflict(...)] can only be applied to a struct or enum",
                ));
            }
        };

        Ok(Self {
            item,
            component_ident,
            conflicting_components:
            arguments.components.into_iter().collect(),
        })
    }

    pub fn generate(self) -> TokenStream {
        let Self {
            item,
            component_ident,
            conflicting_components,
        } = self;

        let spacetime_engine = match spacetime_engine_path() {
            Ok(path) => path,
            Err(error) => return error.into_compile_error(),
        };

        let registrations =
            conflicting_components.iter().map(|conflicting_component| {
                let span = conflicting_component.span();

                quote_spanned! {span=>
                    #spacetime_engine::ecs::component_conflict::__macro_support::inventory::submit! {
                        #spacetime_engine::ecs::component_conflict::__macro_support::ConflictRegistration::new::<
                            #component_ident,
                            #conflicting_component
                        >()
                    }
                }
            });

        quote! {
            #item

            #(#registrations)*
        }
    }
}

fn ensure_non_generic(
    generics: &syn::Generics,
) -> Result<()> {
    if generics.params.is_empty() {
        return Ok(());
    }

    Err(syn::Error::new_spanned(
        generics,
        "#[conflict(...)] cannot be applied to a generic component definition; \
         inventory registrations require concrete component types",
    ))
}

/// Resolves the runtime crate from the consuming crate's Cargo manifest.
///
/// This allows the generated code to work both from within `spacetime_engine`
/// itself and when `spacetime_engine` has been renamed as a dependency.
fn spacetime_engine_path() -> Result<TokenStream> {
    match crate_name(SPACETIME_ENGINE_CRATE) {
        Ok(FoundCrate::Itself) => Ok(quote!(crate)),

        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident))
        }

        Err(error) => Err(syn::Error::new(
            Span::call_site(),
            format!(
                "could not resolve runtime crate `{SPACETIME_ENGINE_CRATE}`: {error}"
            ),
        )),
    }
}