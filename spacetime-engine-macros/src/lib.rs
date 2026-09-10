mod conflict;

use proc_macro::TokenStream;
use conflict::Conflict;

// ECS

// - Components

#[proc_macro_attribute]
pub fn conflict(attr: TokenStream, item: TokenStream) -> TokenStream {
    match Conflict::parse(attr.into(), item.into()) {
        Ok(conflict) => conflict.generate().into(),
        Err(error) => error.into_compile_error().into(),
    }
}