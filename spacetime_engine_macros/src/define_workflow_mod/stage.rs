use super::core_function::CoreFunctions;
use super::core_type::CoreTypes;
use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{braced, bracketed, parse::Parse, parse::ParseStream, Ident, LitInt, Result, Token};

pub struct Ecs;
pub struct Render;
pub struct Async;
pub struct EcsWhile;
pub struct RenderWhile;

pub struct Stages(pub Vec<Stage>);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StageType {
    Ecs,
    Render,
    Async,
    EcsWhile,
    RenderWhile,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StageSignature {
    None,
    E,
    O,
    OE,
    I,
    IE,
    IO,
    IOE,
}
impl StageSignature {
    pub fn generate(self) -> TokenStream {
        match self {
            StageSignature::None => quote! { crate::workflow::stage::StageSignature::None },
            StageSignature::E => quote! { crate::workflow::stage::StageSignature::E },
            StageSignature::O => quote! { crate::workflow::stage::StageSignature::O },
            StageSignature::OE => quote! { crate::workflow::stage::StageSignature::OE },
            StageSignature::I => quote! { crate::workflow::stage::StageSignature::I },
            StageSignature::IE => quote! { crate::workflow::stage::StageSignature::IE },
            StageSignature::IO => quote! { crate::workflow::stage::StageSignature::IO },
            StageSignature::IOE => quote! { crate::workflow::stage::StageSignature::IOE },
        }
    }
}

pub enum Stage {
    Ecs(TypedStage<Ecs>),
    Render(TypedStage<Render>),
    Async(TypedStage<Async>),
    EcsWhile(TypedStage<EcsWhile>),
    RenderWhile(TypedStage<RenderWhile>),
}

pub struct TypedStage<T> {
    pub name: Ident,
    pub index: usize,
    pub core_types: CoreTypes<T>,
    pub core_functions: CoreFunctions<T>,
}

impl Stage {
    fn parse(input: ParseStream, index: usize) -> Result<Self> {
        let lookahead = input.fork();

        let _stage_name: Ident = lookahead.parse()?;
        let _: Token![:] = lookahead.parse()?;
        let stage_type: Ident = lookahead.parse()?;

        match stage_type.to_string().as_str() {
            "Ecs" => TypedStage::<Ecs>::parse(input, index).map(Stage::Ecs),
            "Render" => TypedStage::<Render>::parse(input, index).map(Stage::Render),
            "Async" => TypedStage::<Async>::parse(input, index).map(Stage::Async),
            "EcsWhile" => TypedStage::<EcsWhile>::parse(input, index).map(Stage::EcsWhile),
            "RenderWhile" => TypedStage::<RenderWhile>::parse(input, index).map(Stage::RenderWhile),
            _ => Err(input.error("Invalid stage type")),
        }
    }

    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_state_type_path: Option<&TokenStream>,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
    ) -> (TokenStream, TokenStream) {
        match self {
            Stage::Ecs(stage) => {
                let signature = stage.core_types.get_signature();
                stage.generate(
                    workflow_path,
                    this_stage_out_type_path,
                    this_stage_err_type_path,
                    next_stage_in_type_path,
                    is_last,
                    signature,
                )
            }
            Stage::Render(stage) => {
                let signature = stage.core_types.get_signature();
                stage.generate(
                    workflow_path,
                    this_stage_out_type_path,
                    this_stage_err_type_path,
                    next_stage_in_type_path,
                    is_last,
                    signature,
                )
            }
            Stage::Async(stage) => {
                let signature = stage.core_types.get_signature();
                stage.generate(
                    workflow_path,
                    this_stage_out_type_path,
                    this_stage_err_type_path,
                    next_stage_in_type_path,
                    is_last,
                    signature,
                )
            }
            Stage::EcsWhile(stage) => {
                let signature = stage.core_types.get_signature();
                stage.generate(
                    workflow_path,
                    this_stage_state_type_path,
                    this_stage_out_type_path,
                    this_stage_err_type_path,
                    next_stage_in_type_path,
                    is_last,
                    signature,
                )
            }
            Stage::RenderWhile(stage) => {
                let signature = stage.core_types.get_signature();
                stage.generate(
                    workflow_path,
                    this_stage_state_type_path,
                    this_stage_out_type_path,
                    this_stage_err_type_path,
                    next_stage_in_type_path,
                    is_last,
                    signature,
                )
            }
        }
    }

    pub fn name(&self) -> &Ident {
        match self {
            Stage::Ecs(stage) => stage.name(),
            Stage::Render(stage) => stage.name(),
            Stage::Async(stage) => stage.name(),
            Stage::EcsWhile(stage) => stage.name(),
            Stage::RenderWhile(stage) => stage.name(),
        }
    }

    pub fn has_input(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_input(),
            Stage::Async(stage) => stage.core_types.has_input(),
            Stage::Render(stage) => stage.core_types.has_input(),
            Stage::EcsWhile(stage) => stage.core_types.has_input(),
            Stage::RenderWhile(stage) => stage.core_types.has_input(),
        }
    }

    pub fn has_state(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_state(),
            Stage::Async(stage) => stage.core_types.has_state(),
            Stage::Render(stage) => stage.core_types.has_state(),
            Stage::EcsWhile(stage) => stage.core_types.has_state(),
            Stage::RenderWhile(stage) => stage.core_types.has_state(),
        }
    }

    pub fn has_output(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_output(),
            Stage::Render(stage) => stage.core_types.has_output(),
            Stage::Async(stage) => stage.core_types.has_output(),
            Stage::EcsWhile(stage) => stage.core_types.has_output(),
            Stage::RenderWhile(stage) => stage.core_types.has_output(),
        }
    }

    pub fn has_error(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_error(),
            Stage::Render(stage) => stage.core_types.has_error(),
            Stage::Async(stage) => stage.core_types.has_error(),
            Stage::EcsWhile(stage) => stage.core_types.has_error(),
            Stage::RenderWhile(stage) => stage.core_types.has_error(),
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Stage::Ecs(stage) => stage.get_index(),
            Stage::Render(stage) => stage.get_index(),
            Stage::Async(stage) => stage.get_index(),
            Stage::EcsWhile(stage) => stage.get_index(),
            Stage::RenderWhile(stage) => stage.get_index(),
        }
    }

    pub fn get_type(&self) -> StageType {
        match self {
            Stage::Ecs(_) => StageType::Ecs,
            Stage::Render(_) => StageType::Render,
            Stage::Async(_) => StageType::Async,
            Stage::EcsWhile(_) => StageType::EcsWhile,
            Stage::RenderWhile(_) => StageType::RenderWhile,
        }
    }

    pub fn get_in_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        match self {
            Stage::Ecs(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
            Stage::Render(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
            Stage::Async(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
            Stage::EcsWhile(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
            Stage::RenderWhile(stage) => {
                stage.get_in_type_path(workflow_module_ident, workflow_ident)
            }
        }
    }

    pub fn get_state_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        match self {
            Stage::Ecs(_) => None,
            Stage::Render(_) => None,
            Stage::Async(_) => None,
            Stage::EcsWhile(stage) => {
                stage.get_state_type_path(workflow_module_ident, workflow_ident)
            },
            Stage::RenderWhile(stage) => {
                stage.get_state_type_path(workflow_module_ident, workflow_ident)
            }
        }
    }

    pub fn get_out_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        match self {
            Stage::Ecs(stage) => stage.get_out_type_path(workflow_module_ident, workflow_ident),
            Stage::Render(stage) => stage.get_out_type_path(workflow_module_ident, workflow_ident),
            Stage::Async(stage) => stage.get_out_type_path(workflow_module_ident, workflow_ident),
            Stage::EcsWhile(stage) => {
                stage.get_out_type_path(workflow_module_ident, workflow_ident)
            }
            Stage::RenderWhile(stage) => {
                stage.get_out_type_path(workflow_module_ident, workflow_ident)
            }
        }
    }

    pub fn get_err_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        match self {
            Stage::Ecs(stage) => stage.get_err_type_path(workflow_module_ident, workflow_ident),
            Stage::Render(stage) => stage.get_err_type_path(workflow_module_ident, workflow_ident),
            Stage::Async(stage) => stage.get_err_type_path(workflow_module_ident, workflow_ident),
            Stage::EcsWhile(stage) => {
                stage.get_err_type_path(workflow_module_ident, workflow_ident)
            }
            Stage::RenderWhile(stage) => {
                stage.get_err_type_path(workflow_module_ident, workflow_ident)
            }
        }
    }
}

impl TypedStage<Ecs> {
    fn parse(input: ParseStream, index: usize) -> Result<Self> {
        let stage_name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _stage_type: Ident = input.parse()?;

        let stage_content;
        braced!(stage_content in input);

        let _: super::kw::core_types = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_types_content;
        bracketed!(core_types_content in stage_content);
        let core_types: CoreTypes<Ecs> = core_types_content.parse()?;

        let _: Token![,] = stage_content.parse()?;

        let _: super::kw::core_functions = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_functions_content;
        bracketed!(core_functions_content in stage_content);
        let core_functions: CoreFunctions<Ecs> = core_functions_content.parse()?;

        Ok(TypedStage {
            name: stage_name,
            index,
            core_types,
            core_functions,
        })
    }
}

impl TypedStage<Render> {
    fn parse(input: ParseStream, index: usize) -> Result<Self> {
        let stage_name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _stage_type: Ident = input.parse()?;

        let stage_content;
        braced!(stage_content in input);

        let _: super::kw::core_types = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_types_content;
        bracketed!(core_types_content in stage_content);
        let core_types: CoreTypes<Render> = core_types_content.parse()?;

        let _: Token![,] = stage_content.parse()?;

        let _: super::kw::core_functions = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_functions_content;
        bracketed!(core_functions_content in stage_content);
        let core_functions: CoreFunctions<Render> = core_functions_content.parse()?;

        Ok(TypedStage {
            name: stage_name,
            index,
            core_types,
            core_functions,
        })
    }
}

impl TypedStage<Async> {
    fn parse(input: ParseStream, index: usize) -> Result<Self> {
        let stage_name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _stage_type: Ident = input.parse()?;

        let stage_content;
        braced!(stage_content in input);

        let _: super::kw::core_types = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_types_content;
        bracketed!(core_types_content in stage_content);
        let core_types: CoreTypes<Async> = core_types_content.parse()?;

        let _: Token![,] = stage_content.parse()?;

        let _: super::kw::core_functions = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_functions_content;
        bracketed!(core_functions_content in stage_content);
        let core_functions: CoreFunctions<Async> = core_functions_content.parse()?;

        Ok(TypedStage {
            name: stage_name,
            index,
            core_types,
            core_functions,
        })
    }
}

impl TypedStage<EcsWhile> {
    fn parse(input: ParseStream, index: usize) -> Result<Self> {
        let stage_name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _stage_type: Ident = input.parse()?;

        let stage_content;
        braced!(stage_content in input);

        let _: super::kw::core_types = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_types_content;
        bracketed!(core_types_content in stage_content);
        let core_types: CoreTypes<EcsWhile> = core_types_content.parse()?;

        let _: Token![,] = stage_content.parse()?;

        let _: super::kw::core_functions = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_functions_content;
        bracketed!(core_functions_content in stage_content);
        let core_functions: CoreFunctions<EcsWhile> = core_functions_content.parse()?;

        Ok(TypedStage {
            name: stage_name,
            index,
            core_types,
            core_functions,
        })
    }
}

impl TypedStage<RenderWhile> {
    fn parse(input: ParseStream, index: usize) -> Result<Self> {
        let stage_name: Ident = input.parse()?;
        let _: Token![:] = input.parse()?;
        let _stage_type: Ident = input.parse()?;

        let stage_content;
        braced!(stage_content in input);

        let _: super::kw::core_types = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_types_content;
        bracketed!(core_types_content in stage_content);
        let core_types: CoreTypes<RenderWhile> = core_types_content.parse()?;

        let _: Token![,] = stage_content.parse()?;

        let _: super::kw::core_functions = stage_content.parse()?;
        stage_content.parse::<Token![:]>()?;
        let core_functions_content;
        bracketed!(core_functions_content in stage_content);
        let core_functions: CoreFunctions<RenderWhile> = core_functions_content.parse()?;

        Ok(TypedStage {
            name: stage_name,
            index,
            core_types,
            core_functions,
        })
    }
}

impl Parse for Stages {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut stages = Vec::new();
        let mut index = 0;
        while !input.is_empty() {
            stages.push(Stage::parse(input, index)?);

            let lookahead = input.lookahead1();
            if lookahead.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            }

            index += 1;
        }

        if stages.is_empty() {
            return Err(input.error("No stages found"));
        }

        Ok(Stages(stages))
    }
}

impl TypedStage<Ecs> {
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let signature = signature.generate();
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = stringify!(#stage_name);

                pub mod core_types {
                    use super::super::super::workflow_imports::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let stage_literal = if !is_last {
            let ecs_response_handler = match (
                this_stage_out_type_path,
                this_stage_err_type_path,
                next_stage_in_type_path,
            ) {
                (Some(this_out_path), Some(this_err_path), Some(next_in_path)) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcs)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcs
                        | {
                            response.map(|response| {
                                let result_data: Result<#this_out_path, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                match result_data {
                                    Ok(output) => {
                                        let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                        let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::Ecs,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                            stage_output: output,
                                        }) {
                                            unreachable!("Ecs response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::Ecs,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("Ecs response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(this_out_path), Some(this_err_path), None) => {
                    if is_last {
                        let stage_err_name = format!("{}Error", stage_name.as_str());
                        let stage_err_name =
                            Ident::new(stage_err_name.as_str(), stage_ident.span());

                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageEcs)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageEcs
                            | {
                                response.map(|response| {
                                    let result: Result<#this_out_path, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                    match result {
                                        Ok(output) => {
                                            let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)

                                            if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                ty: crate::workflow::stage::StageType::Ecs,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                                stage_output: output,
                                            }) {
                                                unreachable!("Ecs response handler error: Completion event send error: {}", send_err);
                                            }
                                        }
                                        Err(error) => {
                                            let error = #workflow_path::Error::#stage_err_name(error);
                                            let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                                ty: crate::workflow::stage::StageType::Ecs,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                                stage_error: error,
                                            }) {
                                                unreachable!("Ecs response handler error: Failure event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (Some(this_out_path), None, Some(next_in_path)) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcs)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcs
                        | {
                            response.map(|response| {
                                let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty: crate::workflow::stage::StageType::Ecs,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                    stage_output: output,
                                }) {
                                    unreachable!("Ecs response handler error: Completion event send error: {}", send_err);
                                }
                            });
                        })
                    })}
                }
                (Some(this_out_path), None, None) => {
                    if is_last {
                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageEcs)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageEcs,
                            | {
                                response.map(|response| {
                                    let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                    let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)

                                    if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                        ty: crate::workflow::stage::StageType::Ecs,
                                        module_name,
                                        workflow_name,
                                        current_stage: #index_literal,
                                        stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                        stage_output: output,
                                    }) {
                                        unreachable!("Ecs response handler error: Completion event send error: {}", send_err);
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (None, Some(_), Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, Some(this_err_path), None) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcs)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcs
                        | {
                            response.map(|response| {
                                let result: Result<(), #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                match result {
                                    Ok(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::Ecs,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                            stage_output: None,
                                        }) {
                                            unreachable!("Ecs response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::Ecs,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Ecs(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("Ecs response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, None, Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, None, None) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcs)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcs
                        | {
                            response.map(|response| {
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent(
                                    ty: crate::workflow::stage::StageType::Ecs,
                                    module_name,
                                    workflow_name,
                                    #index_literal,
                                    stage,
                                    None,
                                )) {
                                    unreachable!("Ecs response handler error: Completion event send error: {}", send_err);
                                }
                            });
                        })
                    })}
                }
            };

            quote! {
                crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    run_ecs: Box::new(self::stages::#stage_ident::core_functions::run_ecs) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    handle_ecs_response: #ecs_response_handler,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        } else {
            let ecs_response_handler_placeholder = quote! { Box::new(|
                _module_name: &'static str,
                _workflow_name: &'static str,
                _response: Option<Box<dyn std::any::Any + Send + Sync>>,
                _completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
            | -> Box<dyn FnOnce(crate::workflow::stage::StageEcs)> {
                Box::new(move |
                    _stage: crate::workflow::stage::StageEcs
                | {
                    unreachable!("Tried to call ecs response handler placeholder");
                })
            })};

            quote! {
                crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    run_ecs: Box::new(self::stages::#stage_ident::core_functions::run_ecs) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    handle_ecs_response: #ecs_response_handler_placeholder,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_in_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.input.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Input })
    }

    pub fn get_out_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
    }

    pub fn get_err_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.error.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error })
    }
}

impl TypedStage<Render> {
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let signature = signature.generate();
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = stringify!(#stage_name);

                pub mod core_types {
                    use super::super::super::workflow_imports::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let stage_literal = if !is_last {
            let render_response_handler = match (
                this_stage_out_type_path,
                this_stage_err_type_path,
                next_stage_in_type_path,
            ) {
                (Some(this_out_path), Some(this_err_path), Some(next_in_path)) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRender)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRender
                        | {
                            response.map(|response| {
                                let result_data: Result<#this_out_path, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                match result_data {
                                    Ok(output) => {
                                        let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                        let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::Render,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Render(stage),
                                            stage_output: output,
                                        }) {
                                            unreachable!("Render response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::Render,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Render(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("Render response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(this_out_path), Some(this_err_path), None) => {
                    if is_last {
                        let stage_err_name = format!("{}Error", stage_name.as_str());
                        let stage_err_name =
                            Ident::new(stage_err_name.as_str(), stage_ident.span());

                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageRender)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageRender
                            | {
                                response.map(|response| {
                                    let result: Result<#this_out_path, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                    match result {
                                        Ok(output) => {
                                            let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)

                                            if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                ty: crate::workflow::stage::StageType::Render,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::Render(stage),
                                                stage_output: output,
                                            }) {
                                                unreachable!("Render response handler error: Completion event send error: {}", send_err);
                                            }
                                        }
                                        Err(error) => {
                                            let error = #workflow_path::Error::#stage_err_name(error);
                                            let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                                ty: crate::workflow::stage::StageType::Render,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::Render(stage),
                                                stage_error: error,
                                            }) {
                                                unreachable!("Render response handler error: Failure event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (Some(this_out_path), None, Some(next_in_path)) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRender)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRender
                        | {
                            response.map(|response| {
                                let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty: crate::workflow::stage::StageType::Render,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::Render(stage),
                                    stage_output: output,
                                }) {
                                    unreachable!("Render response handler error: Completion event send error: {}", send_err);
                                }
                            });
                        })
                    })}
                }
                (Some(this_out_path), None, None) => {
                    if is_last {
                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageRender)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageRender,
                            | {
                                response.map(|response| {
                                    let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                    let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)

                                    if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                        ty: crate::workflow::stage::StageType::Render,
                                        module_name,
                                        workflow_name,
                                        current_stage: #index_literal,
                                        stage_return: crate::workflow::stage::Stage::Render(stage),
                                        stage_output: output,
                                    }) {
                                        unreachable!("Render response handler error: Completion event send error: {}", send_err);
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (None, Some(_), Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, Some(this_err_path), None) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRender)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRender
                        | {
                            response.map(|response| {
                                let result: Result<(), #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                match result {
                                    Ok(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::Render,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Render(stage),
                                            stage_output: None,
                                        }) {
                                            unreachable!("Render response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::Render,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Render(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("Render response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, None, Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, None, None) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRender)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRender
                        | {
                            response.map(|response| {
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty: crate::workflow::stage::StageType::Render,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::Render(stage),
                                    stage_output: None,
                                }) {
                                    unreachable!("Render response handler error: Completion event send error: {}", send_err);
                                }
                            });
                        })
                    })}
                }
            };

            quote! {
                crate::workflow::stage::Stage::Render(crate::workflow::stage::StageRender {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    run_render: Box::new(self::stages::#stage_ident::core_functions::run_render) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    handle_render_response: #render_response_handler,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        } else {
            let render_response_handler_placeholder = quote! { Box::new(|
                _module_name: &'static str,
                _workflow_name: &'static str,
                _response: Option<Box<dyn std::any::Any + Send + Sync>>,
                _completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
            | -> Box<dyn FnOnce(crate::workflow::stage::StageRender)> {
                Box::new(move |
                    _stage: crate::workflow::stage::StageRender
                | {
                    unreachable!("Tried to call render response handler placeholder");
                })
            })};

            quote! {
                crate::workflow::stage::Stage::Render(crate::workflow::stage::StageRender {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    run_render: Box::new(self::stages::#stage_ident::core_functions::run_render) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    handle_render_response: #render_response_handler_placeholder,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_in_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.input.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Input })
    }

    pub fn get_out_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
    }

    pub fn get_err_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.error.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error })
    }
}

impl TypedStage<Async> {
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let signature = signature.generate();
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = stringify!(#stage_name);

                pub mod core_types {
                    use super::super::super::workflow_imports::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let stage_literal = if !is_last {
            let async_response_handler = match (
                this_stage_out_type_path,
                this_stage_err_type_path,
                next_stage_in_type_path,
            ) {
                (Some(this_out_path), Some(this_err_path), Some(next_in_path)) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageAsync)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageAsync
                        | {
                            response.map(|response| {
                                let result_data: Result<#this_out_path, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                match result_data {
                                    Ok(output) => {
                                        let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                        let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::Async,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Async(stage),
                                            stage_output: output,
                                        }) {
                                            unreachable!("Async response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::Async,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Async(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("Async response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(this_out_path), Some(this_err_path), None) => {
                    if is_last {
                        let stage_err_name = format!("{}Error", stage_name.as_str());
                        let stage_err_name =
                            Ident::new(stage_err_name.as_str(), stage_ident.span());

                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageAsync)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageAsync
                            | {
                                response.map(|response| {
                                    let result: Result<#this_out_path, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                    match result {
                                        Ok(output) => {
                                            let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)

                                            if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                ty: crate::workflow::stage::StageType::Async,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::Async(stage),
                                                stage_output: output,
                                            }) {
                                                unreachable!("Async response handler error: Completion event send error: {}", send_err);
                                            }
                                        }
                                        Err(error) => {
                                            let error = #workflow_path::Error::#stage_err_name(error);
                                            let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                                ty: crate::workflow::stage::StageType::Async,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::Async(stage),
                                                stage_error: error,
                                            }) {
                                                unreachable!("Async response handler error: Failure event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (Some(this_out_path), None, Some(next_in_path)) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageAsync)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageAsync
                        | {
                            response.map(|response| {
                                let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty: crate::workflow::stage::StageType::Async,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::Async(stage),
                                    stage_output: output,
                                }) {
                                    unreachable!("Async response handler error: Completion event send error: {}", send_err);
                                }
                            });
                        })
                    })}
                }
                (Some(this_out_path), None, None) => {
                    if is_last {
                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageAsync)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageAsync,
                            | {
                                response.map(|response| {
                                    let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                    let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)

                                    if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                        ty: crate::workflow::stage::StageType::Async,
                                        module_name,
                                        workflow_name,
                                        current_stage: #index_literal,
                                        stage_return: crate::workflow::stage::Stage::Async(stage),
                                        stage_output: output,
                                    }) {
                                        unreachable!("Async response handler error: Completion event send error: {}", send_err);
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (None, Some(_), Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, Some(this_err_path), None) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageAsync)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageAsync
                        | {
                            response.map(|response| {
                                let result: Result<(), #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                match result {
                                    Ok(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::Async,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Async(stage),
                                            stage_output: None,
                                        }) {
                                            unreachable!("Async response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::Async,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::Async(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("Async response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, None, Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, None, None) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageAsync)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageAsync
                        | {
                            response.map(|response| {
                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                    ty: crate::workflow::stage::StageType::Async,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::Async(stage),
                                    stage_output: None,
                                }) {
                                    unreachable!("Async response handler error: Completion event send error: {}", send_err);
                                }
                            });
                        })
                    })}
                }
            };

            quote! {
                crate::workflow::stage::Stage::Async(crate::workflow::stage::StageAsync {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    run_async: Box::new(self::stages::#stage_ident::core_functions::run_async) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    handle_async_response: #async_response_handler,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        } else {
            let async_response_handler_placeholder = quote! { Box::new(|
                _module_name: &'static str,
                _workflow_name: &'static str,
                _response: Option<Box<dyn std::any::Any + Send + Sync>>,
                _completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
            | -> Box<dyn FnOnce(crate::workflow::stage::StageAsync)> {
                Box::new(move |
                    _stage: crate::workflow::stage::StageAsync
                | {
                    unreachable!("Tried to call async response handler placeholder");
                })
            })};

            quote! {
                crate::workflow::stage::Stage::Async(crate::workflow::stage::StageAsync {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    run_async: Box::new(self::stages::#stage_ident::core_functions::run_async) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    handle_async_response: #async_response_handler_placeholder,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_in_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.input.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Input })
    }

    pub fn get_out_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
    }

    pub fn get_err_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.error.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error })
    }
}

impl TypedStage<EcsWhile> {
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_state_type_path: Option<&TokenStream>,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let signature = signature.generate();
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = stringify!(#stage_name);

                pub mod core_types {
                    use super::super::super::workflow_imports::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let stage_literal = if !is_last {
            let ecs_while_response_handler = match (
                this_stage_state_type_path,
                this_stage_out_type_path,
                this_stage_err_type_path,
                next_stage_in_type_path,
            ) {
                (Some(this_state_path), Some(this_out_path), Some(this_err_path), Some(next_in_path)) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<#this_state_path, #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                
                                match outcome_result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(state) => {
                                                let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_state: state,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            },
                                            crate::workflow::types::Outcome::Completed(output) => {
                                                let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                                let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);
        
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_output: output,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(this_state_path), Some(this_out_path), Some(this_err_path), None) => {
                    if is_last {
                        let stage_err_name = format!("{}Error", stage_name.as_str());
                        let stage_err_name =
                            Ident::new(stage_err_name.as_str(), stage_ident.span());

                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageEcsWhile,
                            | {
                                response.map(|response| {
                                    let outcome_result: Result<crate::workflow::types::Outcome<#this_state_path, #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                    match outcome_result {
                                        Ok(outcome) => {
                                            match outcome {
                                                crate::workflow::types::Outcome::Waiting(state) => {
                                                    let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                                    if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                        ty: crate::workflow::stage::StageType::EcsWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                        stage_state: state,
                                                    }) {
                                                        unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                                    }
                                                },
                                                crate::workflow::types::Outcome::Completed(output) => {
                                                    let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                                    if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                        ty: crate::workflow::stage::StageType::EcsWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                        stage_output: output,
                                                    }) {
                                                        unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                                    }
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            let error = #workflow_path::Error::#stage_err_name(error);
                                            let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                                ty: crate::workflow::stage::StageType::EcsWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                stage_error: error,
                                            }) {
                                                unreachable!("EcsWhile response handler error: Failure event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (Some(this_state_path), Some(this_out_path), None, Some(next_in_path)) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<#this_state_path, #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");

                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(state) => {
                                        let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_state: state,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    },
                                    crate::workflow::types::Outcome::Completed(output) => {
                                        let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                        let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_output: output,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(this_state_path), Some(this_out_path), None, None) => {
                    if is_last {
                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageEcsWhile,
                            | {
                                response.map(|response| {
                                    let outcome: crate::workflow::types::Outcome<#this_state_path, #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");

                                    match outcome {
                                        crate::workflow::types::Outcome::Waiting(state) => {
                                            let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                ty: crate::workflow::stage::StageType::EcsWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                stage_state: state,
                                            }) {
                                                unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                            }
                                        }
                                        crate::workflow::types::Outcome::Completed(output) => {
                                            let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                            if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                ty: crate::workflow::stage::StageType::EcsWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                stage_output: output,
                                            }) {
                                                unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (Some(_), None, Some(_), Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (Some(this_state_path), None, Some(this_err_path), None) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<#this_state_path, ()>, #this_err_path> = *response.downcast().expect("Failed to downcast response result outcome data");
                                
                                match result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(state) => {
                                                let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_state: state,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            }
                                            crate::workflow::types::Outcome::Completed(_) => {
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_output: None,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(_), None, None, Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (Some(this_state_path), None, None, None) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<#this_state_path, ()> = *response.downcast().expect("Failed to downcast response outcome data");
                                
                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(state) => {
                                        let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_state: state,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    }
                                    crate::workflow::types::Outcome::Completed(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_output: None,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, Some(this_out_path), Some(this_err_path), Some(next_in_path)) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<(), #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                
                                match outcome_result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(_) => {
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_state: None,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            },
                                            crate::workflow::types::Outcome::Completed(output) => {
                                                let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                                let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);
        
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_output: output,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, Some(this_out_path), Some(this_err_path), None) => {
                    if is_last {
                        let stage_err_name = format!("{}Error", stage_name.as_str());
                        let stage_err_name =
                            Ident::new(stage_err_name.as_str(), stage_ident.span());

                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageEcsWhile,
                            | {
                                response.map(|response| {
                                    let outcome_result: Result<crate::workflow::types::Outcome<(), #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                    match outcome_result {
                                        Ok(outcome) => {
                                            match outcome {
                                                crate::workflow::types::Outcome::Waiting(_) => {
                                                    if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                        ty: crate::workflow::stage::StageType::EcsWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                        stage_state: None,
                                                    }) {
                                                        unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                                    }
                                                },
                                                crate::workflow::types::Outcome::Completed(output) => {
                                                    let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                                    if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                        ty: crate::workflow::stage::StageType::EcsWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                        stage_output: output,
                                                    }) {
                                                        unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                                    }
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            let error = #workflow_path::Error::#stage_err_name(error);
                                            let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                                ty: crate::workflow::stage::StageType::EcsWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                stage_error: error,
                                            }) {
                                                unreachable!("EcsWhile response handler error: Failure event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (None, Some(this_out_path), None, Some(next_in_path)) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<(), #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");

                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(_) => {
                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_state: None,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    },
                                    crate::workflow::types::Outcome::Completed(_) => {
                                        let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                        let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                        let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_output: output,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    },
                                }
                            });
                        })
                    })}
                }
                (None, Some(this_out_path), None, None) => {
                    if is_last {
                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageEcsWhile,
                            | {
                                // TODO: IMPORTANT: Empty responses should be handled for non-while stages, and should outright panic for while stages
                                // Note: Currently, we simply ignore the response and do nothing if it is empty, always, in every case, lol 
                                response.map(|response| {
                                    let outcome: crate::workflow::types::Outcome<(), #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");
                                    
                                    match outcome {
                                        crate::workflow::types::Outcome::Waiting(_) => {
                                            if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                ty: crate::workflow::stage::StageType::EcsWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                stage_state: None,
                                            }) {
                                                unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                            }
                                        }
                                        crate::workflow::types::Outcome::Completed(output) => {
                                            let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                            if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                ty: crate::workflow::stage::StageType::EcsWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                stage_output: output,
                                            }) {
                                                unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (None, None, Some(_), Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, None, Some(this_err_path), None) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<(), ()>, #this_err_path> = *response.downcast().expect("Failed to downcast response result outcome data");
                                
                                match result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(_) => {
                                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_state: None,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            }
                                            crate::workflow::types::Outcome::Completed(_) => {
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                                    stage_output: None,
                                                }) {
                                                    unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, None, None, Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, None, None, None) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<(), ()> = *response.downcast().expect("Failed to downcast response outcome data");

                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(_) => {
                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_state: None,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    }
                                    crate::workflow::types::Outcome::Completed(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::EcsWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                            stage_output: None,
                                        }) {
                                            unreachable!("EcsWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
            };

            quote! {
                crate::workflow::stage::Stage::EcsWhile(crate::workflow::stage::StageEcsWhile {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    setup_ecs_while: Box::new(self::stages::#stage_ident::core_functions::setup_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_ecs_while: Box::new(self::stages::#stage_ident::core_functions::run_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    handle_ecs_while_response: #ecs_while_response_handler,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        } else {
            let ecs_while_response_handler_placeholder = quote! { Box::new(|
                _module_name: &'static str,
                _workflow_name: &'static str,
                _response: Option<Box<dyn std::any::Any + Send + Sync>>,
                _completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
            | -> Box<dyn FnOnce(crate::workflow::stage::StageEcsWhile)> {
                Box::new(move |
                    _stage: crate::workflow::stage::StageEcsWhile
                | {
                    unreachable!("Tried to call ecs while response handler placeholder");
                })
            })};

            quote! {
                crate::workflow::stage::Stage::EcsWhile(crate::workflow::stage::StageEcsWhile {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    setup_ecs_while: Box::new(self::stages::#stage_ident::core_functions::setup_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_ecs_while: Box::new(self::stages::#stage_ident::core_functions::run_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    handle_ecs_while_response: #ecs_while_response_handler_placeholder,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_in_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.input.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Input })
    }

    pub fn get_state_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.state.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::State })
    }

    pub fn get_out_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
    }

    pub fn get_err_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.error.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error })
    }
}

impl TypedStage<RenderWhile> {
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_state_type_path: Option<&TokenStream>,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let signature = signature.generate();
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = stringify!(#stage_name);

                pub mod core_types {
                    use super::super::super::workflow_imports::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let stage_literal = if !is_last {
            let render_while_response_handler = match (
                this_stage_state_type_path,
                this_stage_out_type_path,
                this_stage_err_type_path,
                next_stage_in_type_path,
            ) {
                (Some(this_state_path), Some(this_out_path), Some(this_err_path), Some(next_in_path)) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<#this_state_path, #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                
                                match outcome_result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(state) => {
                                                let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_state: state,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            },
                                            crate::workflow::types::Outcome::Completed(output) => {
                                                let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                                let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);
        
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_output: output,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(this_state_path), Some(this_out_path), Some(this_err_path), None) => {
                    if is_last {
                        let stage_err_name = format!("{}Error", stage_name.as_str());
                        let stage_err_name =
                            Ident::new(stage_err_name.as_str(), stage_ident.span());

                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageRenderWhile,
                            | {
                                response.map(|response| {
                                    let outcome_result: Result<crate::workflow::types::Outcome<#this_state_path, #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                    match outcome_result {
                                        Ok(outcome) => {
                                            match outcome {
                                                crate::workflow::types::Outcome::Waiting(state) => {
                                                    let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                                    if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                        ty: crate::workflow::stage::StageType::RenderWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                        stage_state: state,
                                                    }) {
                                                        unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                                    }
                                                },
                                                crate::workflow::types::Outcome::Completed(output) => {
                                                    let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                                    if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                        ty: crate::workflow::stage::StageType::RenderWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                        stage_output: output,
                                                    }) {
                                                        unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                                    }
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            let error = #workflow_path::Error::#stage_err_name(error);
                                            let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                                ty: crate::workflow::stage::StageType::RenderWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                stage_error: error,
                                            }) {
                                                unreachable!("RenderWhile response handler error: Failure event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (Some(this_state_path), Some(this_out_path), None, Some(next_in_path)) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<#this_state_path, #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");

                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(state) => {
                                        let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_state: state,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    },
                                    crate::workflow::types::Outcome::Completed(output) => {
                                        let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                        let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_output: output,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(this_state_path), Some(this_out_path), None, None) => {
                    if is_last {
                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageRenderWhile,
                            | {
                                response.map(|response| {
                                    let outcome: crate::workflow::types::Outcome<#this_state_path, #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");

                                    match outcome {
                                        crate::workflow::types::Outcome::Waiting(state) => {
                                            let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                ty: crate::workflow::stage::StageType::RenderWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                stage_state: state,
                                            }) {
                                                unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                            }
                                        }
                                        crate::workflow::types::Outcome::Completed(output) => {
                                            let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                            if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                ty: crate::workflow::stage::StageType::RenderWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                stage_output: output,
                                            }) {
                                                unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (Some(_), None, Some(_), Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (Some(this_state_path), None, Some(this_err_path), None) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<#this_state_path, ()>, #this_err_path> = *response.downcast().expect("Failed to downcast response result outcome data");
                                
                                match result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(state) => {
                                                let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_state: state,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            }
                                            crate::workflow::types::Outcome::Completed(_) => {
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_output: None,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (Some(_), None, None, Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (Some(this_state_path), None, None, None) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<#this_state_path, ()> = *response.downcast().expect("Failed to downcast response outcome data");
                                
                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(state) => {
                                        let state = Some(Box::new(state) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_state: state,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    }
                                    crate::workflow::types::Outcome::Completed(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_output: None,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, Some(this_out_path), Some(this_err_path), Some(next_in_path)) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<(), #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                
                                match outcome_result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(_) => {
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_state: None,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            },
                                            crate::workflow::types::Outcome::Completed(output) => {
                                                let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                                let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);
        
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_output: output,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, Some(this_out_path), Some(this_err_path), None) => {
                    if is_last {
                        let stage_err_name = format!("{}Error", stage_name.as_str());
                        let stage_err_name =
                            Ident::new(stage_err_name.as_str(), stage_ident.span());

                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageRenderWhile,
                            | {
                                response.map(|response| {
                                    let outcome_result: Result<crate::workflow::types::Outcome<(), #this_out_path>, #this_err_path> = *response.downcast().expect("Failed to downcast response result data");
                                    match outcome_result {
                                        Ok(outcome) => {
                                            match outcome {
                                                crate::workflow::types::Outcome::Waiting(_) => {
                                                    if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                        ty: crate::workflow::stage::StageType::RenderWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                        stage_state: None,
                                                    }) {
                                                        unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                                    }
                                                },
                                                crate::workflow::types::Outcome::Completed(output) => {
                                                    let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                                    if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                        ty: crate::workflow::stage::StageType::RenderWhile,
                                                        module_name,
                                                        workflow_name,
                                                        current_stage: #index_literal,
                                                        stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                        stage_output: output,
                                                    }) {
                                                        unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                                    }
                                                }
                                            }
                                        }
                                        Err(error) => {
                                            let error = #workflow_path::Error::#stage_err_name(error);
                                            let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                            if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                                ty: crate::workflow::stage::StageType::RenderWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                stage_error: error,
                                            }) {
                                                unreachable!("RenderWhile response handler error: Failure event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (None, Some(this_out_path), None, Some(next_in_path)) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<(), #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");

                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(_) => {
                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_state: None,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    },
                                    crate::workflow::types::Outcome::Completed(_) => {
                                        let output: #this_out_path = *response.downcast().expect("Failed to downcast response output data");
                                        let output: #next_in_path = unsafe { std::mem::transmute(output) };
                                        let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_output: output,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    },
                                }
                            });
                        })
                    })}
                }
                (None, Some(this_out_path), None, None) => {
                    if is_last {
                        quote! { Box::new(|
                            module_name: &'static str,
                            workflow_name: &'static str,
                            response: Option<Box<dyn std::any::Any + Send + Sync>>,
                            wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                            completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                            _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                        | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                            Box::new(move |
                                stage: crate::workflow::stage::StageRenderWhile,
                            | {
                                // TODO: IMPORTANT: Empty responses should be handled for non-while stages, and should outright panic for while stages
                                // Note: Currently, we simply ignore the response and do nothing if it is empty, always, in every case, lol 
                                response.map(|response| {
                                    let outcome: crate::workflow::types::Outcome<(), #this_out_path> = *response.downcast().expect("Failed to downcast response outcome data");
                                    
                                    match outcome {
                                        crate::workflow::types::Outcome::Waiting(_) => {
                                            if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                ty: crate::workflow::stage::StageType::RenderWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                stage_state: None,
                                            }) {
                                                unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                            }
                                        }
                                        crate::workflow::types::Outcome::Completed(output) => {
                                            let output = Some(Box::new(output) as Box<dyn std::any::Any + Send + Sync>)
        
                                            if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                ty: crate::workflow::stage::StageType::RenderWhile,
                                                module_name,
                                                workflow_name,
                                                current_stage: #index_literal,
                                                stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                stage_output: output,
                                            }) {
                                                unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                            }
                                        }
                                    }
                                });
                            })
                        })}
                    } else {
                        unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                    }
                }
                (None, None, Some(_), Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, None, Some(this_err_path), None) => {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome_result: Result<crate::workflow::types::Outcome<(), ()>, #this_err_path> = *response.downcast().expect("Failed to downcast response result outcome data");
                                
                                match result {
                                    Ok(outcome) => {
                                        match outcome {
                                            crate::workflow::types::Outcome::Waiting(_) => {
                                                if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_state: None,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                                }
                                            }
                                            crate::workflow::types::Outcome::Completed(_) => {
                                                if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                                    module_name,
                                                    workflow_name,
                                                    current_stage: #index_literal,
                                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                                    stage_output: None,
                                                }) {
                                                    unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                                }
                                            }
                                        }
                                    }
                                    Err(error) => {
                                        let error = #workflow_path::Error::#stage_err_name(error);
                                        let error = Some(Box::new(error) as Box<dyn std::any::Any + Send + Sync>);

                                        if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_error: error,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Failure event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
                (None, None, None, Some(_)) => {
                    unreachable!("This stage has no output, but the next stage has input!")
                }
                (None, None, None, None) => {
                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<Box<dyn std::any::Any + Send + Sync>>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile
                        | {
                            response.map(|response| {
                                let outcome: crate::workflow::types::Outcome<(), ()> = *response.downcast().expect("Failed to downcast response outcome data");

                                match outcome {
                                    crate::workflow::types::Outcome::Waiting(_) => {
                                        if let Err(send_err) = wait_sender.send(crate::workflow::events::StageWaitEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_state: None,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Wait event send error: {}", send_err);
                                        }
                                    }
                                    crate::workflow::types::Outcome::Completed(_) => {
                                        if let Err(send_err) = completion_sender.send(crate::workflow::events::StageCompletionEvent {
                                            ty: crate::workflow::stage::StageType::RenderWhile,
                                            module_name,
                                            workflow_name,
                                            current_stage: #index_literal,
                                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                            stage_output: None,
                                        }) {
                                            unreachable!("RenderWhile response handler error: Completion event send error: {}", send_err);
                                        }
                                    }
                                }
                            });
                        })
                    })}
                }
            };

            quote! {
                crate::workflow::stage::Stage::RenderWhile(crate::workflow::stage::StageRenderWhile {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    setup_render_while: Box::new(self::stages::#stage_ident::core_functions::setup_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_render_while: Box::new(self::stages::#stage_ident::core_functions::run_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    handle_render_while_response: #render_while_response_handler,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        } else {
            let render_while_response_handler_placeholder = quote! { Box::new(|
                _module_name: &'static str,
                _workflow_name: &'static str,
                _response: Option<Box<dyn std::any::Any + Send + Sync>>,
                _completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                _failure_sender: crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>
            | -> Box<dyn FnOnce(crate::workflow::stage::StageRenderWhile)> {
                Box::new(move |
                    _stage: crate::workflow::stage::StageRenderWhile
                | {
                    unreachable!("Tried to call render while response handler placeholder");
                })
            })};

            quote! {
                crate::workflow::stage::Stage::RenderWhile(crate::workflow::stage::StageRenderWhile {
                    index: #index_literal,
                    name: stringify!(#stage_name),
                    signature: #signature,
                    setup_render_while: Box::new(self::stages::#stage_ident::core_functions::setup_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_render_while: Box::new(self::stages::#stage_ident::core_functions::run_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    handle_render_while_response: #render_while_response_handler_placeholder,
                    completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                    failure_sender: crate::workflow::channels::get_stage_failure_sender().clone(),
                })
            }
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }

    pub fn get_index(&self) -> usize {
        self.index
    }

    pub fn get_in_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.input.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Input })
    }

    pub fn get_state_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.state.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::State })
    }

    pub fn get_out_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
    }

    pub fn get_err_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        let stage_ident = &self.name;
        let stage_ident = Ident::new(
            stage_ident.to_string().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = &self.core_types;

        core_types.error.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error })
    }
}
