use std::marker::PhantomData;
use proc_macro2::{Span, TokenStream};
use syn::{parenthesized, parse::Parse, spanned::Spanned, Ident, Result, Token};
use quote::{quote, ToTokens};
use super::stage::{Ecs, EcsWhile, Render, RenderWhile, Async};

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
    pub has_world: bool,
    pub has_output: bool,
    pub has_error: bool,
    pub has_outcome: bool,
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
        let mut has_world = false;

        while !input.peek(Token![|]) {
            let param_name: Ident = input.parse()?;
            
            if !has_input && !has_state && has_world {
                return Err(syn::Error::new(param_name.span(), "Parameters cannot appear after `world`"))
            }
            if has_input && !has_state && has_world {
                return Err(syn::Error::new(param_name.span(), "Parameters cannot appear after `input` and `world`"))
            }
            if !has_input && has_state && has_world {
                return Err(syn::Error::new(param_name.span(), "Parameters cannot appear after `state` and `world`"))
            }

            match param_name.to_string().as_str() {
                "input" => {
                    if has_input {
                        return Err(syn::Error::new(param_name.span(), "Duplicate parameter: `input`"));
                    }
                    if has_state {
                        return Err(syn::Error::new(param_name.span(), "Parameter `input` cannot be used with `state`"));
                    }
                    has_input = true; 
                }
                "state" => {
                    if has_state {
                        return Err(syn::Error::new(param_name.span(), "Duplicate parameter: `state`"));
                    }
                    if has_input {
                        return Err(syn::Error::new(param_name.span(), "Parameter `state` cannot be used with `input`"));
                    }
                    has_state = true; 
                }
                "world" => {
                    has_world = true; 
                }
                _ => {
                    match (has_input, has_state) {
                        (false, false) => {
                            return Err(syn::Error::new(param_name.span(), "Unexpected parameter name. Expected: `input`, `state`, or `world`"));
                        },
                        (true, false) => {
                            return Err(syn::Error::new(param_name.span(), "Unexpected parameter name. Expected: `world`"))
                        },
                        (false, true) => {
                            return Err(syn::Error::new(param_name.span(), "Unexpected parameter name. Expected: `world`"))
                        },
                        _ => {
                            return Err(syn::Error::new(param_name.span(), "Unexpected parameter name. Expected: `input`, `state`, or `world`"))
                        }
                    }
                },
            }

            if input.peek(Token![,]) {
                let comma: Token![,] = input.parse()?;

                if has_world {
                    return Err(syn::Error::new(comma.span(), "Parameters cannot appear after `world`"))
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
                        return Err(syn::Error::new(content.span(), format!("Expected no content in parantheses! Found content: `{}`", content_str)));
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
                        return Err(syn::Error::new(ident_span, format!("Unexpected return type: `{}`. Expected: `Error`", ident)));
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
                        return Err(syn::Error::new(ident_span, format!("Unexpected return type: `{}`", ident)));
                    }
                }
            }
        }

        match (requires_outcome, has_outcome) {
            (false, true) => {
                return Err(syn::Error::new(function_type.span(), format!("Outcome is forbidden by function type `{}`.", function_type)));
            },
            (true, false) => {
                return Err(syn::Error::new(function_type.span(), format!("Outcome is required by function type `{}`.", function_type)));
            },
            _ => {}
        };

        // --- Finish parsing ---
        let signature = CoreFunctionSignature {
            function_type,
            has_input,
            has_state,
            has_world,
            has_output,
            has_error,
            has_outcome,
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
            CoreFunctionType::RunEcs { .. } => {
                match (has_input, has_output, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                run_ecs_inner(world);
                                None
                            }

                            fn run_ecs_inner(world: &mut World) #body
                        }
                    }
                    (false, false, true) => {
                        quote!{
                            pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = run_ecs_inner(world);
                                Some(Box::new(result))
                            }

                            fn run_ecs_inner(world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (false, true, false) => {
                        quote!{
                            pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let output = run_ecs_inner(world);
                                Some(Box::new(output))
                            }

                            fn run_ecs_inner(world: &mut World) -> Output #body
                        }
                    }
                    (false, true, true) => {
                        quote!{
                            pub fn run_ecs(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = run_ecs_inner(world);
                                Some(Box::new(result))
                            }
                            
                            fn run_ecs_inner(world: &mut World) -> Result<Output, Error> #body
                        }
                    }
                    (true, false, false) => {
                        quote!{
                            pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                run_ecs_inner(*input, world);
                                None
                            }

                            fn run_ecs_inner(input: Input, world: &mut World) #body
                        }
                    }
                    (true, false, true) => {
                        quote!{
                            pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = run_ecs_inner(*input, world);
                                Some(Box::new(result))
                            }

                            fn run_ecs_inner(input: Input, world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (true, true, false) => {
                        quote!{
                            pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let output = run_ecs_inner(*input, world);
                                Some(Box::new(output))
                            }

                            fn run_ecs_inner(input: Input, world: &mut World) -> Output #body
                        }
                    }
                    (true, true, true) => {
                        quote!{
                            pub fn run_ecs(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = run_ecs_inner(*input, world);
                                Some(Box::new(result))
                            }

                            fn run_ecs_inner(input: Input, world: &mut World) -> Result<Output, Error> #body
                        }
                    }
                }
            },
            CoreFunctionType::RunRender { .. } => {
                match (has_input, has_output, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                run_render_inner(world);
                                None
                            }

                            fn run_render_inner(world: &mut World) #body
                        }
                    }
                    (false, false, true) => {
                        quote!{
                            pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = run_render_inner(world);
                                Some(Box::new(result))
                            }

                            fn run_render_inner(world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (false, true, false) => {
                        quote!{
                            pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let output = run_render_inner(world);
                                Some(Box::new(output))
                            }

                            fn run_render_inner(world: &mut World) -> Output #body
                        }
                    }
                    (false, true, true) => {
                        quote!{
                            pub fn run_render(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = run_render_inner(world);
                                Some(Box::new(result))
                            }
                            
                            fn run_render_inner(world: &mut World) -> Result<Output, Error> #body
                        }
                    }
                    (true, false, false) => {
                        quote!{
                            pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                run_render_inner(*input, world);
                                None
                            }

                            fn run_render_inner(input: Input, world: &mut World) #body
                        }
                    }
                    (true, false, true) => {
                        quote!{
                            pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = run_render_inner(*input, world);
                                Some(Box::new(result))
                            }

                            fn run_render_inner(input: Input, world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (true, true, false) => {
                        quote!{
                            pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let output = run_render_inner(*input, world);
                                Some(Box::new(output))
                            }

                            fn run_render_inner(input: Input, world: &mut World) -> Output #body
                        }
                    }
                    (true, true, true) => {
                        quote!{
                            pub fn run_render(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = run_render_inner(*input, world);
                                Some(Box::new(result))
                            }

                            fn run_render_inner(input: Input, world: &mut World) -> Result<Output, Error> #body
                        }
                    }
                }
            },
            CoreFunctionType::RunAsync { .. } => {
                match (has_input, has_output, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                run_async_inner();
                                None
                            }

                            fn run_async_inner() #body
                        }
                    }
                    (false, false, true) => {
                        quote!{
                            pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = run_async_inner();
                                Some(Box::new(result))
                            }

                            fn run_async_inner() -> Result<(), Error> #body
                        }
                    }
                    (false, true, false) => {
                        quote!{
                            pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let output = run_async_inner();
                                Some(Box::new(output))
                            }

                            fn run_async_inner() -> Output #body
                        }
                    }
                    (false, true, true) => {
                        quote!{
                            pub fn run_async(_input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = run_async_inner();
                                Some(Box::new(result))
                            }
                            
                            fn run_async_inner() -> Result<Output, Error> #body
                        }
                    }
                    (true, false, false) => {
                        quote!{
                            pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                run_async_inner(*input);
                                None
                            }

                            fn run_async_inner(input: Input) #body
                        }
                    }
                    (true, false, true) => {
                        quote!{
                            pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = run_async_inner(*input);
                                Some(Box::new(result))
                            }

                            fn run_async_inner(input: Input) -> Result<(), Error> #body
                        }
                    }
                    (true, true, false) => {
                        quote!{
                            pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let output = run_async_inner(*input);
                                Some(Box::new(output))
                            }

                            fn run_async_inner(input: Input) -> Output #body
                        }
                    }
                    (true, true, true) => {
                        quote!{
                            pub fn run_async(input: Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = run_async_inner(*input);
                                Some(Box::new(result))
                            }

                            fn run_async_inner(input: Input) -> Result<Output, Error> #body
                        }
                    }
                }
            },
            CoreFunctionType::SetupEcsWhile { .. } => {
                match (has_input, has_state, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                setup_ecs_while_inner(world);
                                None
                            }

                            fn setup_ecs_while_inner(world: &mut World) #body
                        }
                    }
                    (false, false, true) => {
                        quote!{
                            pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = setup_ecs_while_inner(world);
                                Some(Box::new(result))
                            }

                            fn setup_ecs_while_inner(world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (false, true, false) => {
                        quote!{
                            pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let state = setup_ecs_while_inner(world);
                                Some(Box::new(state))
                            }
                            
                            fn setup_ecs_while_inner(world: &mut World) -> State #body
                        }
                    }
                    (false, true, true) => {
                        quote!{
                            pub fn setup_ecs_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = setup_ecs_while_inner(world);
                                Some(Box::new(result))
                            }
                            
                            fn setup_ecs_while_inner(world: &mut World) -> Result<State, Error> #body
                        }
                    }
                    (true, false, false) => {
                        quote!{
                            pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                setup_ecs_while_inner(*input, world);
                                None
                            }
                            
                            fn setup_ecs_while_inner(input: Input, world: &mut World) #body
                        }
                    }
                    (true, false, true) => {
                        quote!{
                            pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = setup_ecs_while_inner(*input, world);
                                Some(Box::new(result))
                            }

                            fn setup_ecs_while_inner(input: Input, world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (true, true, false) => {
                        quote!{
                            pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let state = setup_ecs_while_inner(*input, world);
                                Some(Box::new(state))
                            }
                            
                            fn setup_ecs_while_inner(input: Input, world: &mut World) -> State #body
                        }
                    }
                    (true, true, true) => {
                        quote!{
                            pub fn setup_ecs_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = setup_ecs_while_inner(*input, world);
                                Some(Box::new(result))
                            }
                            
                            fn setup_ecs_while_inner(input: Input, world: &mut World) -> Result<State, Error> #body
                        }
                    }
                }
            },
            CoreFunctionType::SetupRenderWhile { .. } => {
                match (has_input, has_state, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                setup_render_while_inner(world);
                                None
                            }

                            fn setup_render_while_inner(world: &mut World) #body
                        }
                    }
                    (false, false, true) => {
                        quote!{
                            pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = setup_render_while_inner(world);
                                Some(Box::new(result))
                            }

                            fn setup_render_while_inner(world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (false, true, false) => {
                        quote!{
                            pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let state = setup_render_while_inner(world);
                                Some(Box::new(state))
                            }
                            
                            fn setup_render_while_inner(world: &mut World) -> State #body
                        }
                    }
                    (false, true, true) => {
                        quote!{
                            pub fn setup_render_while(_input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let result = setup_render_while_inner(world);
                                Some(Box::new(result))
                            }
                            
                            fn setup_render_while_inner(world: &mut World) -> Result<State, Error> #body
                        }
                    }
                    (true, false, false) => {
                        quote!{
                            pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                setup_render_while_inner(*input, world);
                                None
                            }
                            
                            fn setup_render_while_inner(input: Input, world: &mut World) #body
                        }
                    }
                    (true, false, true) => {
                        quote!{
                            pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = setup_render_while_inner(*input, world);
                                Some(Box::new(result))
                            }

                            fn setup_render_while_inner(input: Input, world: &mut World) -> Result<(), Error> #body
                        }
                    }
                    (true, true, false) => {
                        quote!{
                            pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let state = setup_render_while_inner(*input, world);
                                Some(Box::new(state))
                            }
                            
                            fn setup_render_while_inner(input: Input, world: &mut World) -> State #body
                        }
                    }
                    (true, true, true) => {
                        quote!{
                            pub fn setup_render_while(input: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                let input = input.unwrap().downcast::<Input>().unwrap();
                                let result = setup_render_while_inner(*input, world);
                                Some(Box::new(result))
                            }
                            
                            fn setup_render_while_inner(input: Input, world: &mut World) -> Result<State, Error> #body
                        }
                    }
                }
            },
            CoreFunctionType::RunEcsWhile { .. } => {
                match (has_state, has_output, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome = run_ecs_while_inner(world);
                                let outcome = match outcome {
                                    Wait(_) => Wait(None),
                                    Done(_) => Done(None),
                                };
                                Box::new(outcome)
                            }

                            fn run_ecs_while_inner(world: &mut World) -> WorkflowStageOutcome<(), ()> #body
                        }
                    }
                    (false, false, true) => {
                        quote!{
                            pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome_result = run_ecs_while_inner(world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(_) => Wait(None),
                                            Done(_) => Done(None),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_ecs_while_inner(world: &mut World) -> Result<WorkflowStageOutcome<(), ()>, Error> #body
                        }
                    }
                    (false, true, false) => {
                        quote!{
                            pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome = run_ecs_while_inner(world);
                                let outcome = match outcome {
                                    Wait(_) => Wait(None),
                                    Done(output) => Done(Some(Box::new(output))),
                                };
                                Box::new(outcome)
                            }

                            fn run_ecs_while_inner(world: &mut World) -> WorkflowStageOutcome<(), Output> #body
                        }
                    }
                    (false, true, true) => {
                        quote!{
                            pub fn run_ecs_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome_result = run_ecs_while_inner(world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(_) => Wait(None),
                                            Done(output) => Done(Some(Box::new(output))),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_ecs_while_inner(world: &mut World) -> Result<WorkflowStageOutcome<(), Output>, Error> #body
                        }
                    }
                    (true, false, false) => {
                        quote!{
                            pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome = run_ecs_while_inner(*state, world);
                                let outcome = match outcome {
                                    Wait(state) => Wait(Some(Box::new(state))),
                                    Done(_) => Done(None),
                                };
                                Box::new(outcome)
                            }

                            fn run_ecs_while_inner(state: State, world: &mut World) -> WorkflowStageOutcome<State, ()> #body
                        }
                    }
                    (true, false, true) => {
                        quote!{
                            pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome_result = run_ecs_while_inner(*state, world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(state) => Wait(Some(Box::new(state))),
                                            Done(_) => Done(None),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_ecs_while_inner(state: State, world: &mut World) -> Result<WorkflowStageOutcome<State, ()>, Error> #body
                        }
                    }
                    (true, true, false) => {
                        quote!{
                            pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome = run_ecs_while_inner(*state, world);
                                let outcome = match outcome {
                                    Wait(state) => Wait(Some(Box::new(state))),
                                    Done(output) => Done(Some(Box::new(output))),
                                };
                                Box::new(outcome)
                            }

                            fn run_ecs_while_inner(state: State, world: &mut World) -> WorkflowStageOutcome<State, Output> #body
                        }
                    }
                    (true, true, true) => {
                        quote!{
                            pub fn run_ecs_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome_result = run_ecs_while_inner(*state, world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(state) => Wait(Some(Box::new(state))),
                                            Done(output) => Done(Some(Box::new(output))),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_ecs_while_inner(state: State, world: &mut World) -> Result<WorkflowStageOutcome<State, Output>, Error> #body
                        }
                    }
                }
            },
            CoreFunctionType::RunRenderWhile { .. } => {
                match (has_state, has_output, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome = run_render_while_inner(world);
                                let outcome = match outcome {
                                    Wait(_) => Wait(None),
                                    Done(_) => Done(None),
                                };
                                Box::new(outcome)
                            }

                            fn run_render_while_inner(world: &mut World) -> WorkflowStageOutcome<(), ()> #body
                        }
                    }
                    (false, false, true) => {
                        quote!{
                            pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome_result = run_render_while_inner(world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(_) => Wait(None),
                                            Done(_) => Done(None),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_render_while_inner(world: &mut World) -> Result<WorkflowStageOutcome<(), ()>, Error> #body
                        }
                    }
                    (false, true, false) => {
                        quote!{
                            pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome = run_render_while_inner(world);
                                let outcome = match outcome {
                                    Wait(_) => Wait(None),
                                    Done(output) => Done(Some(Box::new(output))),
                                };
                                Box::new(outcome)
                            }

                            fn run_render_while_inner(world: &mut World) -> WorkflowStageOutcome<(), Output> #body
                        }
                    }
                    (false, true, true) => {
                        quote!{
                            pub fn run_render_while(_state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let outcome_result = run_render_while_inner(world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(_) => Wait(None),
                                            Done(output) => Done(Some(Box::new(output))),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_render_while_inner(world: &mut World) -> Result<WorkflowStageOutcome<(), Output>, Error> #body
                        }
                    }
                    (true, false, false) => {
                        quote!{
                            pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome = run_render_while_inner(*state, world);
                                let outcome = match outcome {
                                    Wait(state) => Wait(Some(Box::new(state))),
                                    Done(_) => Done(None),
                                };
                                Box::new(outcome)
                            }

                            fn run_render_while_inner(state: State, world: &mut World) -> WorkflowStageOutcome<State, ()> #body
                        }
                    }
                    (true, false, true) => {
                        quote!{
                            pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome_result = run_render_while_inner(*state, world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(state) => Wait(Some(Box::new(state))),
                                            Done(_) => Done(None),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_render_while_inner(state: State, world: &mut World) -> Result<WorkflowStageOutcome<State, ()>, Error> #body
                        }
                    }
                    (true, true, false) => {
                        quote!{
                            pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome = run_render_while_inner(*state, world);
                                let outcome = match outcome {
                                    Wait(state) => Wait(Some(Box::new(state))),
                                    Done(output) => Done(Some(Box::new(output))),
                                };
                                Box::new(outcome)
                            }

                            fn run_render_while_inner(state: State, world: &mut World) -> WorkflowStageOutcome<State, Output> #body
                        }
                    }
                    (true, true, true) => {
                        quote!{
                            pub fn run_render_while(state: Option<Box<dyn std::any::Any + Send + Sync>>, world: &mut World) -> Box<dyn std::any::Any + Send + Sync> {
                                let state = state.unwrap().downcast::<State>().unwrap();
                                let outcome_result = run_render_while_inner(*state, world);
                                let outcome_result = match outcome_result {
                                    Ok(outcome) => {
                                        let outcome = match outcome {
                                            Wait(state) => Wait(Some(Box::new(state))),
                                            Done(output) => Done(Some(Box::new(output))),
                                        };
                                        Ok(outcome)
                                    },
                                    Err(error) => Err(error),
                                };
                                Box::new(outcome_result)
                            }

                            fn run_render_while_inner(state: State, world: &mut World) -> Result<WorkflowStageOutcome<State, Output>, Error> #body
                        }
                    }
                }
            },
        }
    }
}

pub enum CoreFunctions<T> {
    Default { phantom_data: PhantomData<T>, run: CoreFunction },
    While { phantom_data: PhantomData<T>, setup: CoreFunction, run: CoreFunction }
}

impl Parse for CoreFunctions<Ecs> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.signature.function_type, CoreFunctionType::RunEcs { .. }) {
            return Err(syn::Error::new(run.signature.function_type.span(), "Expected a `RunEcs` function in Ecs stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
    }
}

impl Parse for CoreFunctions<EcsWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(setup.signature.function_type, CoreFunctionType::SetupEcsWhile { .. }) {
            return Err(syn::Error::new(setup.signature.function_type.span(), "Expected a `SetupEcsWhile` function as the first function in EcsWhile stage."));
        }
        if !matches!(run.signature.function_type, CoreFunctionType::RunEcsWhile { .. }) {
            return Err(syn::Error::new(run.signature.function_type.span(), "Expected a `RunEcsWhile` function as the second function in EcsWhile stage."));
        }

        Ok(CoreFunctions::While { phantom_data: PhantomData, setup, run })
    }
}

impl Parse for CoreFunctions<Render> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.signature.function_type, CoreFunctionType::RunRender { .. }) {
            return Err(syn::Error::new(run.signature.function_type.span(), "Expected a `RunRender` function in Render stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
    }
}

impl Parse for CoreFunctions<RenderWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(setup.signature.function_type, CoreFunctionType::SetupRenderWhile { .. }) {
            return Err(syn::Error::new(setup.signature.function_type.span(), "Expected a `SetupRenderWhile` function as the first function in RenderWhile stage."));
        }
        if !matches!(run.signature.function_type, CoreFunctionType::RunRenderWhile { .. }) {
            return Err(syn::Error::new(run.signature.function_type.span(), "Expected a `RunRenderWhile` function as the second function in RenderWhile stage."));
        }

        Ok(CoreFunctions::While { phantom_data: PhantomData, setup, run })
    }
}

impl Parse for CoreFunctions<Async> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.signature.function_type, CoreFunctionType::RunAsync { .. }) {
            return Err(syn::Error::new(run.signature.function_type.span(), "Expected a `RunAsync` function in Async stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
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
            _ => unreachable!()
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
            _ => unreachable!()
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
            _ => unreachable!()
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
            _ => unreachable!()
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
            _ => unreachable!()
        }
    }
}