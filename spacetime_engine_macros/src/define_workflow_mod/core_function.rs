use super::stage::{Async, Ecs, EcsWhile, Render, RenderWhile};
use proc_macro2::{Span, TokenStream};
use quote::{quote, ToTokens};
use std::marker::PhantomData;
use syn::{parenthesized, parse::Parse, spanned::Spanned, Ident, Result, Token};

pub enum CoreFunctionType {
    RunEcs { span: Span },
    RunRender { span: Span },
    RunAsync { span: Span },
    SetupEcsWhile { span: Span },
    RunEcsWhile { span: Span },
    SetupRenderWhile { span: Span },
    RunRenderWhile { span: Span },
}

impl std::fmt::Display for CoreFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreFunctionType::RunEcs { .. } => write!(f, "run_ecs"),
            CoreFunctionType::RunRender { .. } => write!(f, "run_render"),
            CoreFunctionType::RunAsync { .. } => write!(f, "run_async"),
            CoreFunctionType::SetupEcsWhile { .. } => write!(f, "setup_ecs_while"),
            CoreFunctionType::RunEcsWhile { .. } => write!(f, "run_ecs_while"),
            CoreFunctionType::SetupRenderWhile { .. } => write!(f, "setup_render_while"),
            CoreFunctionType::RunRenderWhile { .. } => write!(f, "run_render_while"),
        }
    }
}

impl CoreFunctionType {
    pub fn span(&self) -> Span {
        match self {
            CoreFunctionType::RunEcs { span } => *span,
            CoreFunctionType::RunRender { span } => *span,
            CoreFunctionType::RunAsync { span } => *span,
            CoreFunctionType::SetupEcsWhile { span } => *span,
            CoreFunctionType::RunEcsWhile { span } => *span,
            CoreFunctionType::SetupRenderWhile { span } => *span,
            CoreFunctionType::RunRenderWhile { span } => *span,
        }
    }
}

impl Parse for CoreFunctionType {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let func_name: Ident = input.parse()?;
        let span = func_name.span();

        match func_name.to_string().as_str() {
            "RunEcs" => Ok(CoreFunctionType::RunEcs { span }),
            "RunRender" => Ok(CoreFunctionType::RunRender { span }),
            "RunAsync" => Ok(CoreFunctionType::RunAsync { span }),
            "SetupEcsWhile" => Ok(CoreFunctionType::SetupEcsWhile { span }),
            "RunEcsWhile" => Ok(CoreFunctionType::RunEcsWhile { span }),
            "SetupRenderWhile" => Ok(CoreFunctionType::SetupRenderWhile { span }),
            "RunRenderWhile" => Ok(CoreFunctionType::RunRenderWhile { span }),
            _ => Err(syn::Error::new(
                func_name.span(),
                "Invalid function type. Expected one of: RunEcs, RunRender, RunAsync, SetupEcsWhile, RunEcsWhile, SetupRenderWhile, RunRenderWhile."
            )),
        }
    }
}

pub struct CoreFunctionSignature {
    pub function_type: CoreFunctionType,
    pub has_input: bool,
    pub has_state: bool,
    pub has_output: bool,
    pub has_error: bool,
}

pub struct CoreFunction {
    pub signature: CoreFunctionSignature,
    pub body: TokenStream,
}

impl Parse for CoreFunction {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let _: Token![fn] = input.parse()?;
        let function_type: CoreFunctionType = input.parse()?;

        // --- Parse `| ... |` parameter list ---
        let _ = input.parse::<Token![|]>()?;

        let mut has_input = false;
        let mut has_state = false;
        let mut has_main_access = false;
        let mut has_render_access = false;

        while !input.peek(Token![|]) {
            let param_name: Ident = input.parse()?;

            if !has_input && !has_state && (has_main_access || has_render_access) {
                return Err(syn::Error::new(
                    param_name.span(),
                    "Parameters cannot appear after `main_access` nor `render_access`",
                ));
            }
            if has_input && !has_state && (has_main_access || has_render_access) {
                return Err(syn::Error::new(
                    param_name.span(),
                    "Parameters cannot appear after `main_access` nor `render_access`",
                ));
            }
            if !has_input && has_state && (has_main_access || has_render_access) {
                return Err(syn::Error::new(
                    param_name.span(),
                    "Parameters cannot appear after `main_access` nor `render_access`",
                ));
            }

            match param_name.to_string().as_str() {
                "input" => {
                    if has_input {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Duplicate parameter: `input`",
                        ));
                    }
                    if has_state {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Parameter `input` cannot be used with `state`",
                        ));
                    }
                    has_input = true;
                }
                "state" => {
                    if has_state {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Duplicate parameter: `state`",
                        ));
                    }
                    if has_input {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Parameter `state` cannot be used with `input`",
                        ));
                    }
                    has_state = true;
                }
                "main_access" => {
                    if has_main_access {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Duplicate parameter: `main_access`",
                        ));
                    }
                    if has_render_access {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Parameter `main_access` cannot be used with `render_access`",
                        ));
                    }
                    has_main_access = true;
                }
                "render_access" => {
                    if has_render_access {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Duplicate parameter: `render_access`",
                        ));
                    }
                    if has_main_access {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Parameter `render_access` cannot be used with `main_access`",
                        ));
                    }
                    has_render_access = true;
                }
                _ => match (has_input, has_state) {
                    (false, false) => {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Unexpected parameter name. Expected: `input`, `state`, or `main_access` or `render_access`",
                        ));
                    }
                    (true, false) => {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Unexpected parameter name. Expected: `main_access` or `render_access`",
                        ))
                    }
                    (false, true) => {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Unexpected parameter name. Expected: `main_access` or `render_access`",
                        ))
                    }
                    _ => {
                        return Err(syn::Error::new(
                            param_name.span(),
                            "Unexpected parameter name. Expected: `input`, `state`, or `main_access` or `render_access`",
                        ))
                    }
                },
            }

            if input.peek(Token![,]) {
                let comma: Token![,] = input.parse()?;

                if has_main_access || has_render_access {
                    return Err(syn::Error::new(
                        comma.span(),
                        "Parameters cannot appear after `main_access` nor `render_access`",
                    ));
                }
            }
        }

        let _ = input.parse::<Token![|]>()?;

        // --- Parse return type ---
        let mut has_output = false;
        let mut has_error = false;
        let mut has_state = false;
        let mut has_outcome = false;
        let requires_outcome = matches!(
            function_type,
            CoreFunctionType::RunEcsWhile { .. } | CoreFunctionType::RunRenderWhile { .. }
        );

        if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;

            #[derive(Clone, PartialEq, Eq)]
            enum Expected {
                Any,
                ResultFirst,
                ResultSecond,
                OutcomeFirst,
                OutcomeSecond,
                Done,
            }

            let mut parse_state = Expected::Any;

            while parse_state != Expected::Done {
                let (ident, ident_span) = if input.peek(Token![_]) {
                    let underscore = input.parse::<Token![_]>()?;
                    ("_".to_string(), underscore.span())
                } else if input.peek(syn::token::Paren) {
                    let content;
                    parenthesized!(content in input);
                    if !content.is_empty() {
                        let content_str = content.to_string();
                        return Err(syn::Error::new(
                            content.span(),
                            format!(
                                "Expected no content in unit type parantheses! Found content: `{}`",
                                content_str
                            ),
                        ));
                    }
                    ("()".to_string(), content.span())
                } else {
                    let ident = input.parse::<Ident>()?;
                    (ident.to_string(), ident.span())
                };

                match (parse_state, ident.as_str()) {
                    (Expected::Any, "Result") => {
                        has_error = true;
                        parse_state = Expected::ResultFirst;
                        let _ = input.parse::<Token![<]>()?;
                    }
                    (Expected::Any, "Outcome") => {
                        has_outcome = true;
                        parse_state = Expected::OutcomeFirst;
                        let _ = input.parse::<Token![<]>()?;
                    }
                    (Expected::Any, "State") => {
                        has_state = true;
                        parse_state = Expected::Done;
                    }
                    (Expected::Any, "Output") => {
                        has_output = true;
                        parse_state = Expected::Done;
                    }
                    (Expected::Any, "_" | "()") => {
                        parse_state = Expected::Done;
                    }

                    (Expected::ResultFirst, "Outcome") => {
                        has_outcome = true;
                        parse_state = Expected::OutcomeFirst;
                        let _ = input.parse::<Token![<]>()?;
                    }
                    (Expected::ResultFirst, "State") => {
                        has_state = true;
                        parse_state = Expected::ResultSecond;
                        let _ = input.parse::<Token![,]>()?;
                    }
                    (Expected::ResultFirst, "Output") => {
                        has_output = true;
                        parse_state = Expected::ResultSecond;
                        let _ = input.parse::<Token![,]>()?;
                    }
                    (Expected::ResultFirst, "_" | "()") => {
                        parse_state = Expected::ResultSecond;
                        let _ = input.parse::<Token![,]>()?;
                    }

                    (Expected::ResultSecond, "Error") => {
                        parse_state = Expected::Done;
                        let _ = input.parse::<Token![>]>()?;
                    }
                    (Expected::ResultSecond, _) => {
                        return Err(syn::Error::new(
                            ident_span,
                            format!("Unexpected return type: `{}`. Expected: `Error`", ident),
                        ));
                    }

                    (Expected::OutcomeFirst, "State") => {
                        has_state = true;
                        parse_state = Expected::OutcomeSecond;
                        let _ = input.parse::<Token![,]>()?;
                    }
                    (Expected::OutcomeFirst, "_" | "()") => {
                        parse_state = Expected::OutcomeSecond;
                        let _ = input.parse::<Token![,]>()?;
                    }

                    (Expected::OutcomeSecond, "Output") => {
                        has_output = true;
                        if has_error {
                            let _ = input.parse::<Token![>]>()?;
                            let _ = input.parse::<Token![,]>()?;
                            parse_state = Expected::ResultSecond;
                        } else {
                            let _ = input.parse::<Token![>]>()?;
                            parse_state = Expected::Done;
                        }
                    }
                    (Expected::OutcomeSecond, "_" | "()") => {
                        if has_error {
                            let _ = input.parse::<Token![>]>()?;
                            let _ = input.parse::<Token![,]>()?;
                            parse_state = Expected::ResultSecond;
                        } else {
                            let _ = input.parse::<Token![>]>()?;
                            parse_state = Expected::Done;
                        }
                    }

                    _ => {
                        return Err(syn::Error::new(
                            ident_span,
                            format!("Unexpected return type: `{}`", ident),
                        ));
                    }
                }
            }
        }

        match (requires_outcome, has_outcome) {
            (false, true) => {
                return Err(syn::Error::new(
                    function_type.span(),
                    format!("Outcome is forbidden by function type `{}`.", function_type),
                ));
            }
            (true, false) => {
                return Err(syn::Error::new(
                    function_type.span(),
                    format!("Outcome is required by function type `{}`.", function_type),
                ));
            }
            _ => {}
        };

        // --- Finish parsing ---
        let signature = CoreFunctionSignature {
            function_type,
            has_input,
            has_state,
            has_output,
            has_error,
        };

        let body: syn::Block = input.parse()?;

        Ok(CoreFunction {
            signature,
            body: body.to_token_stream(),
        })
    }
}

impl CoreFunction {
    pub fn generate(&self) -> TokenStream {
        let has_input = self.signature.has_input;
        let has_state = self.signature.has_state;
        let has_output = self.signature.has_output;
        let has_error = self.signature.has_error;
        let body = &self.body;

        match self.signature.function_type {
            CoreFunctionType::RunEcs { .. } => match (has_input, has_output, has_error) {
                (false, false, false) => {
                    quote! {
                        pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            run_ecs_inner(*main_access);
                            None
                        }

                        fn run_ecs_inner(mut main_access: MainAccess) #body
                    }
                }
                (false, false, true) => {
                    quote! {
                        pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = run_ecs_inner(*main_access);
                            Some(Box::new(result))
                        }

                        fn run_ecs_inner(mut main_access: MainAccess) -> Result<(), Error> #body
                    }
                }
                (false, true, false) => {
                    quote! {
                        pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let output = run_ecs_inner(*main_access);
                            Some(Box::new(output))
                        }

                        fn run_ecs_inner(mut main_access: MainAccess) -> Output #body
                    }
                }
                (false, true, true) => {
                    quote! {
                        pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = run_ecs_inner(*main_access);
                            Some(Box::new(result))
                        }

                        fn run_ecs_inner(mut main_access: MainAccess) -> Result<Output, Error> #body
                    }
                }
                (true, false, false) => {
                    quote! {
                        pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            run_ecs_inner(*input, *main_access);
                            None
                        }

                        fn run_ecs_inner(input: Input, mut main_access: MainAccess) #body
                    }
                }
                (true, false, true) => {
                    quote! {
                        pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = run_ecs_inner(*input, *main_access);
                            Some(Box::new(result))
                        }

                        fn run_ecs_inner(input: Input, mut main_access: MainAccess) -> Result<(), Error> #body
                    }
                }
                (true, true, false) => {
                    quote! {
                        pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let output = run_ecs_inner(*input, *main_access);
                            Some(Box::new(output))
                        }

                        fn run_ecs_inner(input: Input, mut main_access: MainAccess) -> Output #body
                    }
                }
                (true, true, true) => {
                    quote! {
                        pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = run_ecs_inner(*input, *main_access);
                            Some(Box::new(result))
                        }

                        fn run_ecs_inner(input: Input, mut main_access: MainAccess) -> Result<Output, Error> #body
                    }
                }
            },
            CoreFunctionType::RunRender { .. } => match (has_input, has_output, has_error) {
                (false, false, false) => {
                    quote! {
                        pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            run_render_inner(*render_access);
                            None
                        }

                        fn run_render_inner(mut render_access: RenderAccess) #body
                    }
                }
                (false, false, true) => {
                    quote! {
                        pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = run_render_inner(*render_access);
                            Some(Box::new(result))
                        }

                        fn run_render_inner(mut render_access: RenderAccess) -> Result<(), Error> #body
                    }
                }
                (false, true, false) => {
                    quote! {
                        pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let output = run_render_inner(*render_access);
                            Some(Box::new(output))
                        }

                        fn run_render_inner(mut render_access: RenderAccess) -> Output #body
                    }
                }
                (false, true, true) => {
                    quote! {
                        pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = run_render_inner(*render_access);
                            Some(Box::new(result))
                        }

                        fn run_render_inner(mut render_access: RenderAccess) -> Result<Output, Error> #body
                    }
                }
                (true, false, false) => {
                    quote! {
                        pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            run_render_inner(*input, *render_access);
                            None
                        }

                        fn run_render_inner(input: Input, mut render_access: RenderAccess) #body
                    }
                }
                (true, false, true) => {
                    quote! {
                        pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = run_render_inner(*input, *render_access);
                            Some(Box::new(result))
                        }

                        fn run_render_inner(input: Input, mut render_access: RenderAccess) -> Result<(), Error> #body
                    }
                }
                (true, true, false) => {
                    quote! {
                        pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let output = run_render_inner(*input, *render_access);
                            Some(Box::new(output))
                        }

                        fn run_render_inner(input: Input, mut render_access: RenderAccess) -> Output #body
                    }
                }
                (true, true, true) => {
                    quote! {
                        pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = run_render_inner(*input, *render_access);
                            Some(Box::new(result))
                        }

                        fn run_render_inner(input: Input, mut render_access: RenderAccess) -> Result<Output, Error> #body
                    }
                }
            },
            CoreFunctionType::RunAsync { .. } => match (has_input, has_output, has_error) {
                (false, false, false) => {
                    quote! {
                        pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            run_async_inner();
                            None
                        }

                        fn run_async_inner() #body
                    }
                }
                (false, false, true) => {
                    quote! {
                        pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let result = run_async_inner();
                            Some(Box::new(result))
                        }

                        fn run_async_inner() -> Result<(), Error> #body
                    }
                }
                (false, true, false) => {
                    quote! {
                        pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let output = run_async_inner();
                            Some(Box::new(output))
                        }

                        fn run_async_inner() -> Output #body
                    }
                }
                (false, true, true) => {
                    quote! {
                        pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let result = run_async_inner();
                            Some(Box::new(result))
                        }

                        fn run_async_inner() -> Result<Output, Error> #body
                    }
                }
                (true, false, false) => {
                    quote! {
                        pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            run_async_inner(*input);
                            None
                        }

                        fn run_async_inner(input: Input) #body
                    }
                }
                (true, false, true) => {
                    quote! {
                        pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let result = run_async_inner(*input);
                            Some(Box::new(result))
                        }

                        fn run_async_inner(input: Input) -> Result<(), Error> #body
                    }
                }
                (true, true, false) => {
                    quote! {
                        pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let output = run_async_inner(*input);
                            Some(Box::new(output))
                        }

                        fn run_async_inner(input: Input) -> Output #body
                    }
                }
                (true, true, true) => {
                    quote! {
                        pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let result = run_async_inner(*input);
                            Some(Box::new(result))
                        }

                        fn run_async_inner(input: Input) -> Result<Output, Error> #body
                    }
                }
            },
            CoreFunctionType::SetupEcsWhile { .. } => match (has_input, has_state, has_error) {
                (false, false, false) => {
                    quote! {
                        pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            setup_ecs_while_inner(*main_access);
                            None
                        }

                        fn setup_ecs_while_inner(mut main_access: MainAccess) #body
                    }
                }
                (false, false, true) => {
                    quote! {
                        pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = setup_ecs_while_inner(*main_access);
                            Some(Box::new(result))
                        }

                        fn setup_ecs_while_inner(mut main_access: MainAccess) -> Result<(), Error> #body
                    }
                }
                (false, true, false) => {
                    quote! {
                        pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let state = setup_ecs_while_inner(*main_access);
                            Some(Box::new(state))
                        }

                        fn setup_ecs_while_inner(mut main_access: MainAccess) -> State #body
                    }
                }
                (false, true, true) => {
                    quote! {
                        pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = setup_ecs_while_inner(*main_access);
                            Some(Box::new(result))
                        }

                        fn setup_ecs_while_inner(mut main_access: MainAccess) -> Result<State, Error> #body
                    }
                }
                (true, false, false) => {
                    quote! {
                        pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            setup_ecs_while_inner(*input, *main_access);
                            None
                        }

                        fn setup_ecs_while_inner(input: Input, mut main_access: MainAccess) #body
                    }
                }
                (true, false, true) => {
                    quote! {
                        pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = setup_ecs_while_inner(*input, *main_access);
                            Some(Box::new(result))
                        }

                        fn setup_ecs_while_inner(input: Input, mut main_access: MainAccess) -> Result<(), Error> #body
                    }
                }
                (true, true, false) => {
                    quote! {
                        pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let state = setup_ecs_while_inner(*input, *main_access);
                            Some(Box::new(state))
                        }

                        fn setup_ecs_while_inner(input: Input, mut main_access: MainAccess) -> State #body
                    }
                }
                (true, true, true) => {
                    quote! {
                        pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let result = setup_ecs_while_inner(*input, *main_access);
                            Some(Box::new(result))
                        }

                        fn setup_ecs_while_inner(input: Input, mut main_access: MainAccess) -> Result<State, Error> #body
                    }
                }
            },
            CoreFunctionType::SetupRenderWhile { .. } => match (has_input, has_state, has_error) {
                (false, false, false) => {
                    quote! {
                        pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            setup_render_while_inner(*render_access);
                            None
                        }

                        fn setup_render_while_inner(mut render_access: RenderAccess) #body
                    }
                }
                (false, false, true) => {
                    quote! {
                        pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = setup_render_while_inner(*render_access);
                            Some(Box::new(result))
                        }

                        fn setup_render_while_inner(mut render_access: RenderAccess) -> Result<(), Error> #body
                    }
                }
                (false, true, false) => {
                    quote! {
                        pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let state = setup_render_while_inner(*render_access);
                            Some(Box::new(state))
                        }

                        fn setup_render_while_inner(mut render_access: RenderAccess) -> State #body
                    }
                }
                (false, true, true) => {
                    quote! {
                        pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = setup_render_while_inner(*render_access);
                            Some(Box::new(result))
                        }

                        fn setup_render_while_inner(mut render_access: RenderAccess) -> Result<State, Error> #body
                    }
                }
                (true, false, false) => {
                    quote! {
                        pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            setup_render_while_inner(*input, *render_access);
                            None
                        }

                        fn setup_render_while_inner(input: Input, mut render_access: RenderAccess) #body
                    }
                }
                (true, false, true) => {
                    quote! {
                        pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = setup_render_while_inner(*input, *render_access);
                            Some(Box::new(result))
                        }

                        fn setup_render_while_inner(input: Input, mut render_access: RenderAccess) -> Result<(), Error> #body
                    }
                }
                (true, true, false) => {
                    quote! {
                        pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let state = setup_render_while_inner(*input, *render_access);
                            Some(Box::new(state))
                        }

                        fn setup_render_while_inner(input: Input, mut render_access: RenderAccess) -> State #body
                    }
                }
                (true, true, true) => {
                    quote! {
                        // TODO: MINOR: Setup functions' responses (and data in general) do not need to be optional at all
                        pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                            let input = input.unwrap().downcast::<Input>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let result = setup_render_while_inner(*input, *render_access);
                            Some(Box::new(result))
                        }

                        fn setup_render_while_inner(input: Input, mut render_access: RenderAccess) -> Result<State, Error> #body
                    }
                }
            },
            CoreFunctionType::RunEcsWhile { .. } => match (has_state, has_output, has_error) {
                (false, false, false) => {
                    quote! {
                        pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome = run_ecs_while_inner(*main_access);
                            Box::new(outcome)
                        }

                        fn run_ecs_while_inner(mut main_access: MainAccess) -> Outcome<(), ()> #body
                    }
                }
                (false, false, true) => {
                    quote! {
                        pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome_result = run_ecs_while_inner(*main_access);
                            Box::new(outcome_result)
                        }

                        fn run_ecs_while_inner(mut main_access: MainAccess) -> Result<Outcome<(), ()>, Error> #body
                    }
                }
                (false, true, false) => {
                    quote! {
                        pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome = run_ecs_while_inner(*main_access);
                            Box::new(outcome)
                        }

                        fn run_ecs_while_inner(mut main_access: MainAccess) -> Outcome<(), Output> #body
                    }
                }
                (false, true, true) => {
                    quote! {
                        pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome_result = run_ecs_while_inner(*main_access);
                            Box::new(outcome_result)
                        }

                        fn run_ecs_while_inner(mut main_access: MainAccess) -> Result<Outcome<(), Output>, Error> #body
                    }
                }
                (true, false, false) => {
                    quote! {
                        pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome = run_ecs_while_inner(*state, *main_access);
                            Box::new(outcome)
                        }

                        fn run_ecs_while_inner(state: State, mut main_access: MainAccess) -> Outcome<State, ()> #body
                    }
                }
                (true, false, true) => {
                    quote! {
                        pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome_result = run_ecs_while_inner(*state, *main_access);
                            Box::new(outcome_result)
                        }

                        fn run_ecs_while_inner(state: State, mut main_access: MainAccess) -> Result<Outcome<State, ()>, Error> #body
                    }
                }
                (true, true, false) => {
                    quote! {
                        pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome = run_ecs_while_inner(*state, *main_access);
                            Box::new(outcome)
                        }

                        fn run_ecs_while_inner(state: State, mut main_access: MainAccess) -> Outcome<State, Output> #body
                    }
                }
                (true, true, true) => {
                    quote! {
                        pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, main_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let main_access = main_access.downcast::<MainAccess>().unwrap();
                            let outcome_result = run_ecs_while_inner(*state, *main_access);
                            Box::new(outcome_result)
                        }

                        fn run_ecs_while_inner(state: State, mut main_access: MainAccess) -> Result<Outcome<State, Output>, Error> #body
                    }
                }
            },
            CoreFunctionType::RunRenderWhile { .. } => match (has_state, has_output, has_error) {
                (false, false, false) => {
                    quote! {
                        pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome = run_render_while_inner(*render_access);
                            Box::new(outcome)
                        }

                        fn run_render_while_inner(mut render_access: RenderAccess) -> Outcome<(), ()> #body
                    }
                }
                (false, false, true) => {
                    quote! {
                        pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome_result = run_render_while_inner(*render_access);
                            Box::new(outcome_result)
                        }

                        fn run_render_while_inner(mut render_access: RenderAccess) -> Result<Outcome<(), ()>, Error> #body
                    }
                }
                (false, true, false) => {
                    quote! {
                        pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome = run_render_while_inner(*render_access);
                            Box::new(outcome)
                        }

                        fn run_render_while_inner(mut render_access: RenderAccess) -> Outcome<(), Output> #body
                    }
                }
                (false, true, true) => {
                    quote! {
                        pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome_result = run_render_while_inner(*render_access);
                            Box::new(outcome_result)
                        }

                        fn run_render_while_inner(mut render_access: RenderAccess) -> Result<Outcome<(), Output>, Error> #body
                    }
                }
                (true, false, false) => {
                    quote! {
                        pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome = run_render_while_inner(*state, *render_access);
                            Box::new(outcome)
                        }

                        fn run_render_while_inner(state: State, mut render_access: RenderAccess) -> Outcome<State, ()> #body
                    }
                }
                (true, false, true) => {
                    quote! {
                        pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome_result = run_render_while_inner(*state, *render_access);
                            Box::new(outcome_result)
                        }

                        fn run_render_while_inner(state: State, mut render_access: RenderAccess) -> Result<Outcome<State, ()>, Error> #body
                    }
                }
                (true, true, false) => {
                    quote! {
                        pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome = run_render_while_inner(*state, *render_access);
                            Box::new(outcome)
                        }

                        fn run_render_while_inner(state: State, mut render_access: RenderAccess) -> Outcome<State, Output> #body
                    }
                }
                (true, true, true) => {
                    quote! {
                        pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, render_access: Box<dyn std::any::Any + Send + Sync>) -> Box<dyn std::any::Any + Send + Sync> {
                            let state = state.unwrap().downcast::<State>().unwrap();
                            let render_access = render_access.downcast::<RenderAccess>().unwrap();
                            let outcome_result = run_render_while_inner(*state, *render_access);
                            Box::new(outcome_result)
                        }

                        fn run_render_while_inner(state: State, mut render_access: RenderAccess) -> Result<Outcome<State, Output>, Error> #body
                    }
                }
            },
        }
    }
}

pub enum CoreFunctions<T> {
    Default {
        _phantom_data: PhantomData<T>,
        run: CoreFunction,
    },
    While {
        _phantom_data: PhantomData<T>,
        setup: CoreFunction,
        run: CoreFunction,
    },
}

impl Parse for CoreFunctions<Ecs> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.signature.function_type, CoreFunctionType::RunEcs { .. }) {
            return Err(syn::Error::new(
                run.signature.function_type.span(),
                "Expected a `RunEcs` function in Ecs stage.",
            ));
        }
        Ok(CoreFunctions::Default {
            _phantom_data: PhantomData,
            run,
        })
    }
}

impl Parse for CoreFunctions<EcsWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(
            setup.signature.function_type,
            CoreFunctionType::SetupEcsWhile { .. }
        ) {
            return Err(syn::Error::new(
                setup.signature.function_type.span(),
                "Expected a `SetupEcsWhile` function as the first function in EcsWhile stage.",
            ));
        }
        if !matches!(
            run.signature.function_type,
            CoreFunctionType::RunEcsWhile { .. }
        ) {
            return Err(syn::Error::new(
                run.signature.function_type.span(),
                "Expected a `RunEcsWhile` function as the second function in EcsWhile stage.",
            ));
        }

        Ok(CoreFunctions::While {
            _phantom_data: PhantomData,
            setup,
            run,
        })
    }
}

impl Parse for CoreFunctions<Render> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(
            run.signature.function_type,
            CoreFunctionType::RunRender { .. }
        ) {
            return Err(syn::Error::new(
                run.signature.function_type.span(),
                "Expected a `RunRender` function in Render stage.",
            ));
        }
        Ok(CoreFunctions::Default {
            _phantom_data: PhantomData,
            run,
        })
    }
}

impl Parse for CoreFunctions<RenderWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(
            setup.signature.function_type,
            CoreFunctionType::SetupRenderWhile { .. }
        ) {
            return Err(syn::Error::new(setup.signature.function_type.span(), "Expected a `SetupRenderWhile` function as the first function in RenderWhile stage."));
        }
        if !matches!(
            run.signature.function_type,
            CoreFunctionType::RunRenderWhile { .. }
        ) {
            return Err(syn::Error::new(
                run.signature.function_type.span(),
                "Expected a `RunRenderWhile` function as the second function in RenderWhile stage.",
            ));
        }

        Ok(CoreFunctions::While {
            _phantom_data: PhantomData,
            setup,
            run,
        })
    }
}

impl Parse for CoreFunctions<Async> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(
            run.signature.function_type,
            CoreFunctionType::RunAsync { .. }
        ) {
            return Err(syn::Error::new(
                run.signature.function_type.span(),
                "Expected a `RunAsync` function in Async stage.",
            ));
        }
        Ok(CoreFunctions::Default {
            _phantom_data: PhantomData,
            run,
        })
    }
}

impl CoreFunctions<Ecs> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreFunctions::Default { run, .. } => {
                let run_fn = run.generate();

                quote! {
                    #run_fn
                }
            }
            _ => unreachable!(),
        }
    }
}

impl CoreFunctions<EcsWhile> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreFunctions::While { setup, run, .. } => {
                let setup_fn = setup.generate();
                let run_fn = run.generate();

                quote! {
                    #setup_fn
                    #run_fn
                }
            }
            _ => unreachable!(),
        }
    }
}

impl CoreFunctions<Render> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreFunctions::Default { run, .. } => {
                let run_fn = run.generate();

                quote! {
                    #run_fn
                }
            }
            _ => unreachable!(),
        }
    }
}

impl CoreFunctions<RenderWhile> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreFunctions::While { setup, run, .. } => {
                let setup_fn = setup.generate();
                let run_fn = run.generate();

                quote! {
                    #setup_fn
                    #run_fn
                }
            }
            _ => unreachable!(),
        }
    }
}

impl CoreFunctions<Async> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreFunctions::Default { run, .. } => {
                let run_fn = run.generate();

                quote! {
                    #run_fn
                }
            }
            _ => unreachable!(),
        }
    }
}
