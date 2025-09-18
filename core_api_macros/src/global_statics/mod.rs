use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, spanned::Spanned, Expr, ExprPath, Ident, Token, Type};

pub fn export_static(input: TokenStream) -> TokenStream {
    let ExportInput {
        is_self,
        static_path,
        ty,
        init,
    } = parse_macro_input!(input as ExportInput);
    let ident = static_path.path.segments.last().unwrap().ident.clone();
    let mangled = Ident::new(&mangle_path(&static_path), static_path.span());

    let core_path = if is_self {
        quote! { crate }
    } else {
        quote! { ::core_api }
    };

    quote! {
        #[allow(non_upper_case_globals)]
        #[no_mangle]
        pub static #mangled: #core_path::once_cell::sync::Lazy<#ty> = #core_path::once_cell::sync::Lazy::new(|| #init);

        #[cfg(feature = "init_api")]
        paste::paste! {
            #[allow(non_snake_case)]
            #[no_mangle]
            pub extern "C" fn [<#mangled _init>]() {
                #core_path::once_cell::sync::Lazy::force(&#mangled);
            }
        }

        #[allow(non_snake_case)]
        pub fn #ident() -> &'static #ty {
            &#mangled
        }
    }
    .into()
}

pub fn import_static(input: TokenStream) -> TokenStream {
    let ImportInput { is_self, path, ty } = parse_macro_input!(input as ImportInput);
    let ident = path.path.segments.last().unwrap().ident.clone();
    let mangled = Ident::new(&mangle_path(&path), path.path.span());

    let core_path = if is_self {
        quote! { crate }
    } else {
        quote! { ::core_api }
    };

    quote! {
        extern "C" {
            #[allow(non_upper_case_globals)]
            #[no_mangle]
            static #mangled: #core_path::once_cell::sync::Lazy<#ty>;
        }

        #[allow(non_snake_case)]
        pub fn #ident() -> &'static #ty {
            unsafe { &#mangled }
        }
    }
    .into()
}

pub fn api_initializer(input: TokenStream) -> TokenStream {
    let paths = parse_macro_input!(input with Punctuated::<ExprPath, Token![,]>::parse_terminated);

    let init_calls = paths.iter().map(|path| {
        let segments = &path.path.segments;
        let mangled = Ident::new(&mangle_path(path), path.span());
        let module_path = segments.iter().take(segments.len() - 1);
        quote! {
            paste::paste! {
                #(#module_path)::* :: [<#mangled _init>]();
            }
        }
    });

    quote! {
        #[cfg(feature = "init_api")]
        #[no_mangle]
        pub extern "C" fn init_api() {
            unsafe {
                #(#init_calls)*
            }
        }
    }
    .into()
}

/// Converts a path like `core_api::foo::bar::BAZ` to `__STATIC__core_api__foo__bar__BAZ`
pub fn mangle_path(path: &ExprPath) -> String {
    let mut out = String::from("__STATIC__");
    out += &path.path.segments.iter().map(|seg| seg.ident.to_string()).collect::<Vec<_>>().join("__");
    out
}

struct ExportInput {
    is_self: bool,
    static_path: ExprPath,
    ty: Type,
    init: Expr,
}

impl syn::parse::Parse for ExportInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let is_self = if input.peek(Token![self]) && input.peek2(Token![,]) {
            input.parse::<Token![self]>()?;
            input.parse::<Token![,]>()?;
            true
        } else {
            false
        };

        let static_path: ExprPath = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        input.parse::<Token![=]>()?;
        let init: Expr = input.parse()?;
        Ok(Self {
            is_self,
            static_path,
            ty,
            init,
        })
    }
}

struct ImportInput {
    is_self: bool,
    path: ExprPath,
    ty: Type,
}

impl syn::parse::Parse for ImportInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let is_self = if input.peek(Token![self]) && input.peek2(Token![,]) {
            input.parse::<Token![self]>()?;
            input.parse::<Token![,]>()?;
            true
        } else {
            false
        };

        let path: ExprPath = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: Type = input.parse()?;
        Ok(Self { is_self, path, ty })
    }
}
