//! Runtime-crate resolution shared by generated macro output.

use proc_macro_crate::{FoundCrate, crate_name};
use proc_macro2::{Span, TokenStream};
use quote::quote;
use syn::{Ident, Result};

const SPACETIME_ENGINE_CRATE: &str = "spacetime-engine";

pub(super) fn path() -> Result<TokenStream> {
    match crate_name(SPACETIME_ENGINE_CRATE) {
        Ok(FoundCrate::Itself) => Ok(quote!(crate)),
        Ok(FoundCrate::Name(name)) => {
            let ident = Ident::new(&name, Span::call_site());
            Ok(quote!(::#ident))
        }
        Err(error) => Err(syn::Error::new(
            Span::call_site(),
            format!("could not resolve runtime crate `{SPACETIME_ENGINE_CRATE}`: {error}"),
        )),
    }
}
