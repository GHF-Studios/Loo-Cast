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

    quote! {
        #[allow(non_upper_case_globals)]
        #[unsafe(no_mangle)]
        pub static #mangled: once_cell::sync::Lazy<#ty> = once_cell::sync::Lazy::new(|| #init);

        paste::paste! {
            #[allow(non_snake_case)]
            #[unsafe(no_mangle)]
            pub extern "C" fn [<#mangled _init>]() {
                // println!("Calling init for {}", #mangled_string);
                once_cell::sync::Lazy::force(&#mangled);
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

    quote! {
        extern "C" {
            #[allow(non_upper_case_globals)]
            #[unsafe(no_mangle)]
            static #mangled: once_cell::sync::Lazy<#ty>;
        }

        #[allow(non_snake_case)]
        pub fn #ident() -> &'static #ty {
            unsafe { &#mangled }
        }
    }
    .into()
}

pub fn api_initializer(input: TokenStream) -> TokenStream {
    let mut tokens = input.into_iter();

    let custom_name = if let Some(first) = tokens.next() {
        if let proc_macro::TokenTree::Literal(lit) = &first {
            let symbol_name = lit.to_string().trim_matches('"').to_string();
            // Consume the comma after the name if present
            if let Some(proc_macro::TokenTree::Punct(p)) = tokens.next() {
                if p.as_char() != ',' {
                    panic!("Expected ',' after symbol name");
                }
            }
            Some(symbol_name)
        } else {
            panic!("Expected the caller's crate name as string literal as the first argument to api_initializer!");
        }
    } else {
        None
    };

    // Turn back into TokenStream for parsing the list of paths
    let tail: TokenStream = tokens.collect::<Vec<_>>().into_iter().collect();
    let paths = parse_macro_input!(tail with Punctuated::<ExprPath, Token![,]>::parse_terminated);

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

    let symbol_ident = Ident::new(
        &format!("__init_api{}", custom_name.as_ref().map(|s| format!("__{}", s)).unwrap_or_default()),
        proc_macro2::Span::call_site(),
    );

    let log_msg = custom_name
        .as_ref()
        .map(|s| format!("Calling __init_api__{}", s))
        .unwrap_or_else(|| "Calling init_api".to_string());

    quote! {
        #[cfg(feature = "init_api")]
        #[unsafe(no_mangle)]
        pub extern "C" fn #symbol_ident() {
            println!(#log_msg);
            unsafe {
                #(#init_calls)*
            }
        }
    }
    .into()
}

/// Converts a path like `core_mod_api::foo::bar::BAZ` to `__STATIC__core_mod_api__foo__bar__BAZ`
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
