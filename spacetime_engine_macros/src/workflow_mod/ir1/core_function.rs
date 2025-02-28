use std::marker::PhantomData;
use proc_macro2::TokenStream;
use syn::{parse::Parse, spanned::Spanned, Ident, Token, Result};
use quote::{quote, ToTokens};
use super::stage::{Ecs, EcsWhile, Render, RenderWhile, Async};

pub enum CoreFunctionType {
    RunEcs,
    RunRender,
    RunAsync,
    SetupEcsWhile,
    RunEcsWhile,
    SetupRenderWhile,
    RunRenderWhile,
}

impl std::fmt::Display for CoreFunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CoreFunctionType::RunEcs => write!(f, "run_ecs"),
            CoreFunctionType::RunRender => write!(f, "run_render"),
            CoreFunctionType::RunAsync => write!(f, "run_async"),
            CoreFunctionType::SetupEcsWhile => write!(f, "setup_ecs_while"),
            CoreFunctionType::RunEcsWhile => write!(f, "run_ecs_while"),
            CoreFunctionType::SetupRenderWhile => write!(f, "setup_render_while"),
            CoreFunctionType::RunRenderWhile => write!(f, "run_render_while"),
        }
    }
}

impl CoreFunctionType {
    fn is_valid_setup_pair(&self, other: &CoreFunctionType) -> bool {
        matches!(
            (self, other),
            (CoreFunctionType::SetupEcsWhile, CoreFunctionType::RunEcsWhile)
                | (CoreFunctionType::SetupRenderWhile, CoreFunctionType::RunRenderWhile)
        )
    }
}

impl Parse for CoreFunctionType {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let func_name: Ident = input.parse()?;

        match func_name.to_string().as_str() {
            "RunEcs" => Ok(CoreFunctionType::RunEcs),
            "RunRender" => Ok(CoreFunctionType::RunRender),
            "RunAsync" => Ok(CoreFunctionType::RunAsync),
            "SetupEcsWhile" => Ok(CoreFunctionType::SetupEcsWhile),
            "RunEcsWhile" => Ok(CoreFunctionType::RunEcsWhile),
            "SetupRenderWhile" => Ok(CoreFunctionType::SetupRenderWhile),
            "RunRenderWhile" => Ok(CoreFunctionType::RunRenderWhile),
            _ => Err(syn::Error::new(
                func_name.span(),
                "Invalid function type. Expected one of: RunEcs, RunRender, RunAsync, SetupEcsWhile, RunEcsWhile, SetupRenderWhile, RunRenderWhile."
            )),
        }
    }
}

pub struct CoreFunctionSignature {
    pub function_name: Ident,
    pub function_type: CoreFunctionType,
    pub has_input: bool,
    pub has_state: bool,
    pub has_world: bool,
    pub has_output: bool,
    pub has_error: bool,
    pub has_outcome: bool,
}

pub struct CoreFunctionParam {
    pub name: Ident,
    pub ty: TokenStream,
}

pub struct CoreFunction {
    pub signature: CoreFunctionSignature,
    pub body: TokenStream,
}

impl Parse for CoreFunction {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let _: Token![fn] = input.parse()?;
        let function_type: CoreFunctionType = input.parse()?;
        let function_name: Ident = input.parse()?;

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
            CoreFunctionType::RunEcsWhile | CoreFunctionType::RunRenderWhile
        );

        if input.peek(Token![->]) {
            let _: Token![->] = input.parse()?;
        
            #[derive(Clone, PartialEq, Eq)]
            enum Expected {
                Any,           // First value in return type
                ResultFirst,   // First value inside Result<T, E>
                ResultSecond,  // Second value inside Result<T, E>
                OutcomeFirst,  // First value inside Outcome<S, O>
                OutcomeSecond, // Second value inside Outcome<S, O>
                Done,          // Parsing is complete
            }
        
            let mut parse_state = Expected::Any;
            let mut parse_state_stack = Vec::new();
        
            while parse_state != Expected::Done {
                let first: Ident = input.parse()?;
            
                match (parse_state, first.to_string().as_str()) {
                    // Expect any
                    (Expected::Any, "Result") => {
                        has_error = true;
                        parse_state_stack.push(Expected::Any);
                        parse_state = Expected::ResultFirst;
                        let _ = input.parse::<Token![<]>()?;
                    }
                    (Expected::Any, "Outcome") => {
                        has_outcome = true;
                        parse_state_stack.push(Expected::Any);
                        parse_state = Expected::OutcomeFirst;
                        let _ = input.parse::<Token![<]>()?;
                    }
                    (Expected::Any, "State") => {
                        has_state = true;
                        parse_state = parse_state_stack.pop().unwrap_or(Expected::Done);
                    }
                    (Expected::Any, "Output") => {
                        has_output = true;
                        parse_state = parse_state_stack.pop().unwrap_or(Expected::Done);
                    }
                    (Expected::Any, "_" | "()") => {
                        parse_state = parse_state_stack.pop().unwrap_or(Expected::Done);
                    }

                    // Expect first result value
                    (Expected::ResultFirst, "Outcome") => {
                        has_outcome = true;
                        parse_state_stack.push(Expected::ResultFirst);
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

                    // Expect second result value
                    (Expected::ResultSecond, "Error") => {
                        parse_state = parse_state_stack.pop().unwrap_or(Expected::Done);
                        let _ = input.parse::<Token![>]>()?;
                    }
                    (Expected::ResultSecond, _) => {
                        return Err(syn::Error::new(first.span(), format!("Unexpected return type: `{}`. Expected: `Error`", first)));
                    }

                    // Expect first outcome value
                    (Expected::OutcomeFirst, "State") => {
                        has_state = true;
                        parse_state = Expected::OutcomeSecond;
                        let _ = input.parse::<Token![,]>()?;
                    }
                    (Expected::OutcomeFirst, "_" | "()") => {
                        parse_state = Expected::OutcomeSecond;
                        let _ = input.parse::<Token![,]>()?;
                    }

                    // Expect second outcome value
                    (Expected::OutcomeSecond, "Output") => {
                        has_output = true;
                        parse_state = parse_state_stack.pop().unwrap_or(Expected::Done);
                        let _ = input.parse::<Token![>]>()?;
                    }
                    (Expected::OutcomeSecond, "_" | "()") => {
                        parse_state = parse_state_stack.pop().unwrap_or(Expected::Done);
                        let _ = input.parse::<Token![>]>()?;
                    }

                    // Expect the unexpected
                    _ => {
                        return Err(syn::Error::new(first.span(), format!("Unexpected return type: `{}`", first)));
                    }
                }
            }
        }

        match (requires_outcome, has_outcome) {
            (false, true) => {
                return Err(syn::Error::new(function_name.span(), format!("Outcome is forbidden by function type `{}`.", function_name)));
            },
            (true, false) => {
                return Err(syn::Error::new(function_name.span(), format!("Outcome is required by function type `{}`.", function_name)));
            },
            _ => {}
        };

        // --- Finish parsing ---
        let signature = CoreFunctionSignature {
            function_name,
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
        let function_name = format!("{}", self.signature.function_type);
        let function_name: Ident = Ident::new(&function_name, self.signature.function_name.span());
        let has_input = self.signature.has_input;
        let has_state = self.signature.has_state;
        let has_output = self.signature.has_output;
        let has_error = self.signature.has_error;
        let body = &self.body;

        match self.signature.function_type {
            CoreFunctionType::RunEcs | CoreFunctionType::RunRender | CoreFunctionType::RunAsync => {
                match (has_input, has_output, has_error) {
                    (false, false, false) => {
                        quote!{
                            pub fn #function_name(world: &mut World) {
                                #body
                            }
                        }
                    }
                }
            },
            CoreFunctionType::SetupEcsWhile | CoreFunctionType::SetupRenderWhile => {
                match (has_input, has_state, has_output, has_error) {
                    (false, false, false, false) => {
                        quote!{
                            pub fn #function_name(world: &mut World) {
                                #body
                            }
                        }
                    }
                }
            },
            CoreFunctionType::RunEcsWhile | CoreFunctionType::RunRenderWhile => {
                match (has_input, has_state, has_output, has_error) {
                    (false, false, false, false) => {
                        quote!{
                            pub fn #function_name(world: &mut World) -> Outcome<(), ()> {
                                #body
                            }
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
        if !matches!(run.signature.function_type, CoreFunctionType::RunEcs) {
            return Err(syn::Error::new(run.signature.function_name.span(), "Expected a `RunEcs` function in Ecs stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
    }
}

impl Parse for CoreFunctions<EcsWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(setup.signature.function_type, CoreFunctionType::SetupEcsWhile) {
            return Err(syn::Error::new(setup.signature.function_name.span(), "Expected a `SetupEcsWhile` function as the first function in EcsWhile stage."));
        }
        if !matches!(run.signature.function_type, CoreFunctionType::RunEcsWhile) {
            return Err(syn::Error::new(run.signature.function_name.span(), "Expected a `RunEcsWhile` function as the second function in EcsWhile stage."));
        }

        Ok(CoreFunctions::While { phantom_data: PhantomData, setup, run })
    }
}

impl Parse for CoreFunctions<Render> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.signature.function_type, CoreFunctionType::RunRender) {
            return Err(syn::Error::new(run.signature.function_name.span(), "Expected a `RunRender` function in Render stage."));
        }
        Ok(CoreFunctions::Default { phantom_data: PhantomData, run })
    }
}

impl Parse for CoreFunctions<RenderWhile> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let setup: CoreFunction = input.parse()?;
        let run: CoreFunction = input.parse()?;

        if !matches!(setup.signature.function_type, CoreFunctionType::SetupRenderWhile) {
            return Err(syn::Error::new(setup.signature.function_name.span(), "Expected a `SetupRenderWhile` function as the first function in RenderWhile stage."));
        }
        if !matches!(run.signature.function_type, CoreFunctionType::RunRenderWhile) {
            return Err(syn::Error::new(run.signature.function_name.span(), "Expected a `RunRenderWhile` function as the second function in RenderWhile stage."));
        }

        Ok(CoreFunctions::While { phantom_data: PhantomData, setup, run })
    }
}

impl Parse for CoreFunctions<Async> {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let run: CoreFunction = input.parse()?;
        if !matches!(run.signature.function_type, CoreFunctionType::RunAsync) {
            return Err(syn::Error::new(run.signature.function_name.span(), "Expected a `RunAsync` function in Async stage."));
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