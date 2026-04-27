//! Alpha bootstrap stub for legacy `core_engine_macros`.
//!
//! Legacy proc-macro module graph is intentionally not copied yet.

extern crate proc_macro;

use proc_macro::TokenStream;

#[proc_macro]
pub fn alpha_stub(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}
