mod conflict;
mod inspect;

use proc_macro::TokenStream;
use conflict::Conflict;
use inspect::Inspect;

// ECS

// - Components

#[proc_macro_attribute]
pub fn conflict(attr: TokenStream, item: TokenStream) -> TokenStream {
    match Conflict::parse(attr.into(), item.into()) {
        Ok(conflict) => conflict.generate().into(),
        Err(error) => error.into_compile_error().into(),
    }
}

#[proc_macro_derive(Inspect, attributes(inspect))]
pub fn derive_inspect(input: TokenStream) -> TokenStream {
    match Inspect::parse(input.into()) {
        Ok(inspect) => inspect.generate().into(),
        Err(error) => error.into_compile_error().into(),
    }
}
