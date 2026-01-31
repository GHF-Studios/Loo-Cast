extern crate proc_macro;

mod global_statics;

use proc_macro::TokenStream;

// Global statics

#[proc_macro]
pub fn export_static(input: TokenStream) -> TokenStream {
    global_statics::export_static(input)
}

#[proc_macro]
pub fn import_static(input: TokenStream) -> TokenStream {
    global_statics::import_static(input)
}

#[proc_macro]
pub fn api_initializer(input: TokenStream) -> TokenStream {
    global_statics::api_initializer(input)
}
