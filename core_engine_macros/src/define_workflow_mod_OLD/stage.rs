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

#[allow(clippy::upper_case_acronyms)]
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

    pub fn has_input(&self) -> bool {
        matches!(
            self,
            StageSignature::I | StageSignature::IE | StageSignature::IO | StageSignature::IOE
        )
    }

    pub fn has_output(&self) -> bool {
        matches!(
            self,
            StageSignature::O | StageSignature::OE | StageSignature::IO | StageSignature::IOE
        )
    }

    pub fn has_error(&self) -> bool {
        matches!(
            self,
            StageSignature::E | StageSignature::OE | StageSignature::IE | StageSignature::IOE
        )
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

    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_state_type_path: Option<&TokenStream>,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        module_name: &str,
        workflow_name: &str,
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
                    module_name,
                    workflow_name,
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
                    module_name,
                    workflow_name,
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
                    module_name,
                    workflow_name,
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
                    module_name,
                    workflow_name,
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
                    module_name,
                    workflow_name,
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
            }
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
    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
        module_name: &str,
        workflow_name: &str,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let core_types =
            self.core_types
                .generate(self.core_types.generate_stage_type_dependent_stuff(
                    module_name,
                    workflow_name,
                    self.index,
                ));
        let core_functions = {
            let output_type_name: String = this_stage_out_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let error_type_name: String = this_stage_err_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            self.core_functions
                .generate(signature, output_type_name, error_type_name)
        };
        let signature = signature.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = #stage_name;

                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use bevy::prelude::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let ecs_run_response_handler = match (
            this_stage_out_type_path,
            this_stage_err_type_path,
            next_stage_in_type_path,
        ) {
            (Some(this_stage_out_type_path), Some(this_stage_err_type_path), Some(next_stage_in_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcs
                    | {
                        let response = response.expect("Ecs stages with output and error must have a response");
                        let result: Result<#this_stage_out_type_path, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(output) => {
                                #stage_output_transmutation
                                let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                let error = crate::workflow::response::TypedWorkflowResponseOE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(this_stage_out_type_path), Some(this_stage_err_type_path), None) => {
                if is_last {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let error_type_name: String = this_stage_err_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcs
                        | {
                            let response = response.expect("Ecs stages with output and error (last stage) must have a response");
                            let result: Result<#this_stage_out_type_path, #this_stage_err_type_path> = response.into_inner();

                            match result {
                                Ok(output) => {
                                    let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                    let error = crate::workflow::response::TypedWorkflowResponseOE {
                                        module_name,
                                        workflow_name,
                                        result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                    };
                                    let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                    let failure_sender = match failure_sender {
                                        Some(failure_sender) => failure_sender,
                                        None => {
                                            unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                        }
                                    };

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (Some(this_stage_out_type_path), None, Some(next_stage_in_type_path)) => {
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcs
                    | {
                        let response = response.expect("Ecs stages with output must have a response");
                        let output: #this_stage_out_type_path = response.into_inner();
                        #stage_output_transmutation
                        let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                    })
                })}
            }
            (Some(this_stage_out_type_path), None, None) => {
                if is_last {
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcs,
                        | {
                            let response = response.expect("Ecs stages with output and error must have a response");
                            let output: #this_stage_out_type_path = response.into_inner();
                            let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (None, Some(_), Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (None, Some(this_stage_err_type_path), None) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcs
                    | {
                        let response = response.expect("Ecs stages with error must have a response");
                        let result: Result<(), #this_stage_err_type_path> = response.into_inner();

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
                                let error = crate::workflow::response::TypedWorkflowResponseE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    _response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcs
                    | {
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
                    })
                })}
            }
        };

        let failure_sender = if self.core_types.error.is_some() {
            quote! { Some(crate::workflow::channels::get_stage_failure_sender().clone()) }
        } else {
            quote! { None }
        };

        let stage_literal = quote! {
            crate::workflow::stage::Stage::Ecs(crate::workflow::stage::StageEcs {
                index: #index_literal,
                name: #stage_name,
                signature: #signature,
                handle_ecs_run_response: #ecs_run_response_handler,
                completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                failure_sender: #failure_sender,
            })
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
    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
        module_name: &str,
        workflow_name: &str,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let core_types =
            self.core_types
                .generate(self.core_types.generate_stage_type_dependent_stuff(
                    module_name,
                    workflow_name,
                    self.index,
                ));
        let core_functions = {
            let output_type_name: String = this_stage_out_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let error_type_name: String = this_stage_err_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            self.core_functions
                .generate(signature, output_type_name, error_type_name)
        };
        let signature = signature.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = #stage_name;

                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use bevy::prelude::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let render_run_response_handler = match (
            this_stage_out_type_path,
            this_stage_err_type_path,
            next_stage_in_type_path,
        ) {
            (Some(this_stage_out_type_path), Some(this_stage_err_type_path), Some(next_stage_in_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRender
                    | {
                        let response = response.expect("Render stages with output and error must have a response");
                        let result: Result<#this_stage_out_type_path, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(output) => {
                                #stage_output_transmutation
                                let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                let error = crate::workflow::response::TypedWorkflowResponseOE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(this_stage_out_type_path), Some(this_stage_err_type_path), None) => {
                if is_last {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let error_type_name: String = this_stage_err_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRender
                        | {
                            let response = response.expect("Render stages with output and error (last stage) must have a response");
                            let result: Result<#this_stage_out_type_path, #this_stage_err_type_path> = response.into_inner();

                            match result {
                                Ok(output) => {
                                    let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                    let error = crate::workflow::response::TypedWorkflowResponseOE {
                                        module_name,
                                        workflow_name,
                                        result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                    };
                                    let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                    let failure_sender = match failure_sender {
                                        Some(failure_sender) => failure_sender,
                                        None => {
                                            unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                        }
                                    };

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (Some(this_stage_out_type_path), None, Some(next_stage_in_type_path)) => {
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRender
                    | {
                        let response = response.expect("Render stages with output (last stage) must have a response");
                        let output: #this_stage_out_type_path = response.into_inner();
                        #stage_output_transmutation
                        let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                    })
                })}
            }
            (Some(this_stage_out_type_path), None, None) => {
                if is_last {
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRender,
                        | {
                            let response = response.expect("Render stages with output (last stage) must have a response");
                            let output: #this_stage_out_type_path = response.into_inner();
                            let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (None, Some(_), Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (None, Some(this_stage_err_type_path), None) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRender
                    | {
                        let response = response.expect("Render stages with error (last stage) must have a response");
                        let result: Result<(), #this_stage_err_type_path> = response.into_inner();

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
                                let error = crate::workflow::response::TypedWorkflowResponseE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    _response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRender
                    | {
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
                    })
                })}
            }
        };

        let failure_sender = if self.core_types.error.is_some() {
            quote! { Some(crate::workflow::channels::get_stage_failure_sender().clone()) }
        } else {
            quote! { None }
        };

        let stage_literal = quote! {
            crate::workflow::stage::Stage::Render(crate::workflow::stage::StageRender {
                index: #index_literal,
                name: #stage_name,
                signature: #signature,
                handle_render_run_response: #render_run_response_handler,
                completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                failure_sender: #failure_sender,
            })
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
    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
        module_name: &str,
        workflow_name: &str,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let core_types =
            self.core_types
                .generate(self.core_types.generate_stage_type_dependent_stuff(
                    module_name,
                    workflow_name,
                    self.index,
                ));
        let core_functions = {
            let output_type_name: String = this_stage_out_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let error_type_name: String = this_stage_err_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            self.core_functions
                .generate(signature, output_type_name, error_type_name)
        };
        let signature = signature.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = #stage_name;

                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use bevy::prelude::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let async_run_response_handler = match (
            this_stage_out_type_path,
            this_stage_err_type_path,
            next_stage_in_type_path,
        ) {
            (Some(this_stage_out_type_path), Some(this_stage_err_type_path), Some(next_stage_in_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageAsync
                    | {
                        let response = response.expect("Async stages with output and error must have a response");
                        let result: Result<#this_stage_out_type_path, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(output) => {
                                #stage_output_transmutation
                                let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                let error = crate::workflow::response::TypedWorkflowResponseOE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(this_stage_out_type_path), Some(this_stage_err_type_path), None) => {
                if is_last {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let error_type_name: String = this_stage_err_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageAsync
                        | {
                            let response = response.expect("Async stages with output and error (last stage) must have a response");
                            let result: Result<#this_stage_out_type_path, #this_stage_err_type_path> = response.into_inner();

                            match result {
                                Ok(output) => {
                                    let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                    let error = crate::workflow::response::TypedWorkflowResponseOE {
                                        module_name,
                                        workflow_name,
                                        result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                    };
                                    let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                    let failure_sender = match failure_sender {
                                        Some(failure_sender) => failure_sender,
                                        None => {
                                            unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                        }
                                    };

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (Some(this_stage_out_type_path), None, Some(next_stage_in_type_path)) => {
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageAsync
                    | {
                        let response = response.expect("Async stages with output must have a response");
                        let output: #this_stage_out_type_path = response.into_inner();
                        #stage_output_transmutation
                        let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                    })
                })}
            }
            (Some(this_stage_out_type_path), None, None) => {
                if is_last {
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageAsync,
                        | {
                            let response = response.expect("Async stages with output (last stage) must have a response");
                            let output: #this_stage_out_type_path = response.into_inner();
                            let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (None, Some(_), Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (None, Some(this_stage_err_type_path), None) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageAsync
                    | {
                        let response = response.expect("Async stages with error must have a response");
                        let result: Result<(), #this_stage_err_type_path> = response.into_inner();

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
                                let error = crate::workflow::response::TypedWorkflowResponseE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    _response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageAsync
                    | {
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
                    })
                })}
            }
        };

        let failure_sender = if self.core_types.error.is_some() {
            quote! { Some(crate::workflow::channels::get_stage_failure_sender().clone()) }
        } else {
            quote! { None }
        };

        let stage_literal = quote! {
            crate::workflow::stage::Stage::Async(crate::workflow::stage::StageAsync {
                index: #index_literal,
                name: #stage_name,
                signature: #signature,
                handle_async_run_response: #async_run_response_handler,
                completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                failure_sender: #failure_sender,
            })
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
    #[allow(clippy::too_many_arguments)]
    pub fn generate(
        self,
        workflow_path: &TokenStream,
        this_stage_state_type_path: Option<&TokenStream>,
        this_stage_out_type_path: Option<&TokenStream>,
        this_stage_err_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
        signature: StageSignature,
        module_name: &str,
        workflow_name: &str,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let core_types =
            self.core_types
                .generate(self.core_types.generate_stage_type_dependent_stuff(
                    module_name,
                    workflow_name,
                    self.index,
                ));
        let core_functions = {
            let state_type_name: String = this_stage_state_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let output_type_name: String = this_stage_out_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let error_type_name: String = this_stage_err_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            self.core_functions.generate(
                signature,
                state_type_name,
                output_type_name,
                error_type_name,
            )
        };
        let signature = signature.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = #stage_name;

                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use bevy::prelude::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let ecs_while_setup_response_handler = match (
            this_stage_state_type_path,
            this_stage_err_type_path,
        ) {
            (Some(this_stage_state_type_path), Some(this_stage_err_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>,
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with state and error must have a response");
                        let result: Result<#this_stage_state_type_path, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(state) => {
                                let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

                                if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                                    stage_state: state,
                                }) {
                                    unreachable!("EcsWhile response handler error: Setup event send error: {}", send_err);
                                }
                            }
                            Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("EcsWhile response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(this_stage_state_type_path), None) => {
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with state must have a response");
                        let state: #this_stage_state_type_path = response.into_inner();
                        let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                            ty: crate::workflow::stage::StageType::EcsWhile,
                            module_name,
                            workflow_name,
                            current_stage: #index_literal,
                            stage_return: crate::workflow::stage::Stage::EcsWhile(stage),
                            stage_state: state,
                        }) {
                            unreachable!("EcsWhile response handler error: Setup event send error: {}", send_err);
                        }
                    })
                })}
            }
            (None, Some(this_stage_err_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>,
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with error must have a response");
                        let result: Result<(), #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(_) => {
                                if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::EcsWhile(None),
                                    stage_state: None,
                                }) {
                                    unreachable!("EcsWhile response handler error: Setup event send error: {}", send_err);
                                }
                            }
                            Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("EcsWhile response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

                                if let Err(send_err) = failure_sender.send(crate::workflow::events::StageFailureEvent {
                                    ty: crate::workflow::stage::StageType::EcsWhile,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::EcsWhile(None),
                                    stage_error: error,
                                }) {
                                    unreachable!("EcsWhile response handler error: Failure event send error: {}", send_err);
                                }
                            }
                        }
                    })
                })}
            }
            (None, None) => {
                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    _response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                            ty: crate::workflow::stage::StageType::EcsWhile,
                            module_name,
                            workflow_name,
                            current_stage: #index_literal,
                            stage_return: crate::workflow::stage::Stage::EcsWhile(None),
                            stage_state: None,
                        }) {
                            unreachable!("EcsWhile response handler error: Setup event send error: {}", send_err);
                        }
                    })
                })}
            }
        };

        let ecs_while_run_response_handler = match (
            this_stage_state_type_path,
            this_stage_out_type_path,
            this_stage_err_type_path,
            next_stage_in_type_path,
        ) {
            (
                Some(this_stage_state_type_path),
                Some(this_stage_out_type_path),
                Some(this_stage_err_type_path),
                Some(next_stage_in_type_path),
            ) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with output and error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                        match outcome_result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(state) => {
                                        let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                    crate::workflow::types::Outcome::Done(output) => {
                                        #stage_output_transmutation
                                        let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                let error = crate::workflow::response::TypedWorkflowResponseOE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(this_stage_state_type_path), Some(this_stage_out_type_path), Some(this_stage_err_type_path), None) => {
                if is_last {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                    let state_type_name: String = this_stage_state_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let error_type_name: String = this_stage_err_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile,
                        | {
                            let response = response.expect("EcsWhile stages with output and error (last stage) must have a response");
                            let outcome_result: Result<crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                            match outcome_result {
                                Ok(outcome) => {
                                    match outcome {
                                        crate::workflow::types::Outcome::Wait(state) => {
                                            let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                        crate::workflow::types::Outcome::Done(output) => {
                                            let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                    let error = crate::workflow::response::TypedWorkflowResponseOE {
                                        module_name,
                                        workflow_name,
                                        result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                    };
                                    let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                    let failure_sender = match failure_sender {
                                        Some(failure_sender) => failure_sender,
                                        None => {
                                            unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                        }
                                    };

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (Some(this_stage_state_type_path), Some(this_stage_out_type_path), None, Some(next_stage_in_type_path)) => {
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with output must have a response");
                        let outcome: crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(state) => {
                                let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                            crate::workflow::types::Outcome::Done(output) => {
                                #stage_output_transmutation
                                let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                    })
                })}
            }
            (Some(this_stage_state_type_path), Some(this_stage_out_type_path), None, None) => {
                if is_last {
                    let state_type_name: String = this_stage_state_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile,
                        | {
                            let response = response.expect("EcsWhile stages with output (last stage) must have a response");
                            let outcome: crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path> = response.into_inner();

                            match outcome {
                                crate::workflow::types::Outcome::Wait(state) => {
                                    let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                crate::workflow::types::Outcome::Done(output) => {
                                    let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (Some(_), None, Some(_), Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (Some(this_stage_state_type_path), None, Some(this_stage_err_type_path), None) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<#this_stage_state_type_path, ()>, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(state) => {
                                        let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                    crate::workflow::types::Outcome::Done(_) => {
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
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(_), None, None, Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (Some(this_stage_state_type_path), None, None, None) => {
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages must have a response");
                        let outcome: crate::workflow::types::Outcome<#this_stage_state_type_path, ()> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(state) => {
                                let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                            crate::workflow::types::Outcome::Done(_) => {
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
                    })
                })}
            }
            (None, Some(this_stage_out_type_path), Some(this_stage_err_type_path), Some(next_stage_in_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with output and error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<(), #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                        match outcome_result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(_) => {
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
                                    crate::workflow::types::Outcome::Done(output) => {
                                        #stage_output_transmutation
                                        let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                let error = crate::workflow::response::TypedWorkflowResponseOE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (None, Some(this_stage_out_type_path), Some(this_stage_err_type_path), None) => {
                if is_last {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let error_type_name: String = this_stage_err_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile,
                        | {
                            let response = response.expect("EcsWhile stages with output and error (last stage) must have a response");
                            let outcome_result: Result<crate::workflow::types::Outcome<(), #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                            match outcome_result {
                                Ok(outcome) => {
                                    match outcome {
                                        crate::workflow::types::Outcome::Wait(_) => {
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
                                        crate::workflow::types::Outcome::Done(output) => {
                                            let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                    let error = crate::workflow::response::TypedWorkflowResponseOE {
                                        module_name,
                                        workflow_name,
                                        result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                    };
                                    let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                    let failure_sender = match failure_sender {
                                        Some(failure_sender) => failure_sender,
                                        None => {
                                            unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                        }
                                    };

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (None, Some(this_stage_out_type_path), None, Some(next_stage_in_type_path)) => {
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with output must have a response");
                        let outcome: crate::workflow::types::Outcome<(), #this_stage_out_type_path> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(_) => {
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
                            crate::workflow::types::Outcome::Done(_) => {
                                let output: #this_stage_out_type_path = response.into_inner();
                                #stage_output_transmutation
                                let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                    })
                })}
            }
            (None, Some(this_stage_out_type_path), None, None) => {
                if is_last {
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageEcsWhile,
                        | {
                            let response = response.expect("EcsWhile stages with output (last stage) must have a response");
                            let outcome: crate::workflow::types::Outcome<(), #this_stage_out_type_path> = response.into_inner();

                            match outcome {
                                crate::workflow::types::Outcome::Wait(_) => {
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
                                crate::workflow::types::Outcome::Done(output) => {
                                    let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (None, None, Some(_), Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (None, None, Some(this_stage_err_type_path), None) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages with error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<(), ()>, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(_) => {
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
                                    crate::workflow::types::Outcome::Done(_) => {
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
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageEcsWhile
                    | {
                        let response = response.expect("EcsWhile stages must have a response");
                        let outcome: crate::workflow::types::Outcome<(), ()> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(_) => {
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
                            crate::workflow::types::Outcome::Done(_) => {
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
                    })
                })}
            }
        };

        let failure_sender = if self.core_types.error.is_some() {
            quote! { Some(crate::workflow::channels::get_stage_failure_sender().clone()) }
        } else {
            quote! { None }
        };

        let stage_literal = quote! {
            crate::workflow::stage::Stage::EcsWhile(crate::workflow::stage::StageEcsWhile {
                index: #index_literal,
                name: #stage_name,
                signature: #signature,
                handle_ecs_while_setup_response: #ecs_while_setup_response_handler,
                handle_ecs_while_run_response: #ecs_while_run_response_handler,
                setup_sender: crate::workflow::channels::get_stage_setup_sender().clone(),
                wait_sender: crate::workflow::channels::get_stage_wait_sender().clone(),
                completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                failure_sender: #failure_sender,
            })
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

#[allow(clippy::too_many_arguments)]
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
        module_name: &str,
        workflow_name: &str,
    ) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());
        let core_types =
            self.core_types
                .generate(self.core_types.generate_stage_type_dependent_stuff(
                    module_name,
                    workflow_name,
                    self.index,
                ));
        let core_functions = {
            let state_type_name: String = this_stage_state_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let output_type_name: String = this_stage_out_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();
            let error_type_name: String = this_stage_err_type_path
                .cloned()
                .unwrap_or_default()
                .to_string()
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect();

            self.core_functions.generate(
                signature,
                state_type_name,
                output_type_name,
                error_type_name,
            )
        };
        let signature = signature.generate();

        let stage_module = quote! {
            pub mod #stage_ident {
                pub const NAME: &str = #stage_name;

                pub mod core_types {
                    use super::super::super::workflow_imports::*;
                    use bevy::prelude::*;

                    #core_types
                }

                pub mod core_functions {
                    use super::super::super::workflow_imports::*;
                    use super::core_types::*;

                    #core_functions
                }
            }
        };
        let render_while_setup_response_handler = match (
            this_stage_state_type_path,
            this_stage_err_type_path,
        ) {
            (Some(this_stage_state_type_path), Some(this_stage_err_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>,
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with state and error must have a response");
                        let result: Result<#this_stage_state_type_path, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(state) => {
                                let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

                                if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                    stage_state: state,
                                }) {
                                    unreachable!("RenderWhile response handler error: Setup event send error: {}", send_err);
                                }
                            }
                            Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("RenderWhile response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(this_stage_state_type_path), None) => {
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with state must have a response");
                        // TODO: MINOR: Error message below should be "Failed to downcast setup reponse state data", and like `setup response` instead of `response` in general for all setup response handlers
                        let state: #this_stage_state_type_path = response.into_inner();

                        let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                            ty: crate::workflow::stage::StageType::RenderWhile,
                            module_name,
                            workflow_name,
                            current_stage: #index_literal,
                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                            stage_state: state,
                        }) {
                            unreachable!("RenderWhile response handler error: Setup event send error: {}", send_err);
                        }
                    })
                })}
            }
            (None, Some(this_stage_err_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with error must have a response");
                        let result: Result<(), #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(_) => {
                                if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                                    ty: crate::workflow::stage::StageType::RenderWhile,
                                    module_name,
                                    workflow_name,
                                    current_stage: #index_literal,
                                    stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                                    stage_state: None,
                                }) {
                                    unreachable!("RenderWhile response handler error: Setup event send error: {}", send_err);
                                }
                            }
                            Err(error) => {
                                let error = crate::workflow::response::TypedWorkflowResponseE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("RenderWhile response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (None, None) => {
                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    setup_sender: crossbeam_channel::Sender<crate::workflow::events::StageSetupEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages must have a response");
                        let state = Some(response);

                        if let Err(send_err) = setup_sender.send(crate::workflow::events::StageSetupEvent {
                            ty: crate::workflow::stage::StageType::RenderWhile,
                            module_name,
                            workflow_name,
                            current_stage: #index_literal,
                            stage_return: crate::workflow::stage::Stage::RenderWhile(stage),
                            stage_state: state,
                        }) {
                            unreachable!("RenderWhile response handler error: Setup event send error: {}", send_err);
                        }
                    })
                })}
            }
        };

        let render_while_run_response_handler = match (
            this_stage_state_type_path,
            this_stage_out_type_path,
            this_stage_err_type_path,
            next_stage_in_type_path,
        ) {
            (
                Some(this_stage_state_type_path),
                Some(this_stage_out_type_path),
                Some(this_stage_err_type_path),
                Some(next_stage_in_type_path),
            ) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with output and error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                        match outcome_result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(state) => {
                                        let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                    crate::workflow::types::Outcome::Done(output) => {
                                        #stage_output_transmutation
                                        let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                let error = crate::workflow::response::TypedWorkflowResponseOE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(this_stage_state_type_path), Some(this_stage_out_type_path), Some(this_stage_err_type_path), None) => {
                if is_last {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                    let state_type_name: String = this_stage_state_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let error_type_name: String = this_stage_err_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile,
                        | {
                            let response = response.expect("RenderWhile stages with output and error (last stage) must have a response");
                            let outcome_result: Result<crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                            match outcome_result {
                                Ok(outcome) => {
                                    match outcome {
                                        crate::workflow::types::Outcome::Wait(state) => {
                                            let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                        crate::workflow::types::Outcome::Done(output) => {
                                            let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                    let error = crate::workflow::response::TypedWorkflowResponseOE {
                                        module_name,
                                        workflow_name,
                                        result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                    };
                                    let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                    let failure_sender = match failure_sender {
                                        Some(failure_sender) => failure_sender,
                                        None => {
                                            unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                        }
                                    };

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (Some(this_stage_state_type_path), Some(this_stage_out_type_path), None, Some(next_stage_in_type_path)) => {
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with output must have a response");
                        let outcome: crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(state) => {
                                let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                            crate::workflow::types::Outcome::Done(output) => {
                                #stage_output_transmutation
                                let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                    })
                })}
            }
            (Some(this_stage_state_type_path), Some(this_stage_out_type_path), None, None) => {
                if is_last {
                    let state_type_name: String = this_stage_state_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile,
                        | {
                            let response = response.expect("RenderWhile stages with output (last stage) must have a response");
                            let outcome: crate::workflow::types::Outcome<#this_stage_state_type_path, #this_stage_out_type_path> = response.into_inner();

                            match outcome {
                                crate::workflow::types::Outcome::Wait(state) => {
                                    let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                crate::workflow::types::Outcome::Done(output) => {
                                    let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (Some(_), None, Some(_), Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (Some(this_stage_state_type_path), None, Some(this_stage_err_type_path), None) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<#this_stage_state_type_path, ()>, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(state) => {
                                        let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                                    crate::workflow::types::Outcome::Done(_) => {
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
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (Some(_), None, None, Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (Some(this_stage_state_type_path), None, None, None) => {
                let state_type_name: String = this_stage_state_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages must have a response");
                        let outcome: crate::workflow::types::Outcome<#this_stage_state_type_path, ()> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(state) => {
                                let state = Some(crate::debug::types::AnySendSyncNamedBox::new(state, #state_type_name.to_string()));

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
                            crate::workflow::types::Outcome::Done(_) => {
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
                    })
                })}
            }
            (None, Some(this_stage_out_type_path), Some(this_stage_err_type_path), Some(next_stage_in_type_path)) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with output and error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<(), #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                        match outcome_result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(_) => {
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
                                    crate::workflow::types::Outcome::Done(output) => {
                                        #stage_output_transmutation
                                        let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                let error = crate::workflow::response::TypedWorkflowResponseOE {
                                    module_name,
                                    workflow_name,
                                    result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                };
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    })
                })}
            }
            (None, Some(this_stage_out_type_path), Some(this_stage_err_type_path), None) => {
                if is_last {
                    let stage_err_name = format!("{}Error", stage_name.as_str());
                    let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();
                    let error_type_name: String = this_stage_err_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile,
                        | {
                            let response = response.expect("RenderWhile stages with output and error (last stage) must have a response");
                            let outcome_result: Result<crate::workflow::types::Outcome<(), #this_stage_out_type_path>, #this_stage_err_type_path> = response.into_inner();

                            match outcome_result {
                                Ok(outcome) => {
                                    match outcome {
                                        crate::workflow::types::Outcome::Wait(_) => {
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
                                        crate::workflow::types::Outcome::Done(output) => {
                                            let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                                    let error = crate::workflow::response::TypedWorkflowResponseOE {
                                        module_name,
                                        workflow_name,
                                        result: Err(crate::debug::types::AnySendSyncNamedBox::new(#workflow_path::Error::#stage_err_name(error), #error_type_name.to_string()))
                                    };
                                    let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                    let failure_sender = match failure_sender {
                                        Some(failure_sender) => failure_sender,
                                        None => {
                                            unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                        }
                                    };

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (None, Some(this_stage_out_type_path), None, Some(next_stage_in_type_path)) => {
                let stage_output_transmutation = if is_last {
                    quote! {}
                } else {
                    quote! { let output: #next_stage_in_type_path = unsafe { std::mem::transmute(output) }; }
                };
                let output_type_name: String = this_stage_out_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with output must have a response");
                        let outcome: crate::workflow::types::Outcome<(), #this_stage_out_type_path> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(_) => {
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
                            crate::workflow::types::Outcome::Done(_) => {
                                let output: #this_stage_out_type_path = response.into_inner();
                                #stage_output_transmutation
                                let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                    })
                })}
            }
            (None, Some(this_stage_out_type_path), None, None) => {
                if is_last {
                    let output_type_name: String = this_stage_out_type_path
                        .to_string()
                        .chars()
                        .filter(|c| !c.is_whitespace())
                        .collect();

                    quote! { Box::new(|
                        module_name: &'static str,
                        workflow_name: &'static str,
                        response: Option<crate::debug::types::AnySendSyncNamedBox>,
                        wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                        completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                        _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                    | {
                        Box::new(move |
                            stage: crate::workflow::stage::StageRenderWhile,
                        | {
                            let response = response.expect("RenderWhile stages with output must have a response");
                            let outcome: crate::workflow::types::Outcome<(), #this_stage_out_type_path> = response.into_inner();

                            match outcome {
                                crate::workflow::types::Outcome::Wait(_) => {
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
                                crate::workflow::types::Outcome::Done(output) => {
                                    let output = Some(crate::debug::types::AnySendSyncNamedBox::new(output, #output_type_name.to_string()));

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
                        })
                    })}
                } else {
                    unreachable!("This stage has output, but the next stage has no input, so this stage must be the last stage, but it is not flagged as such!")
                }
            }
            (None, None, Some(_), Some(_)) => {
                unreachable!("This stage has no output, but the next stage has input!")
            }
            (None, None, Some(this_stage_err_type_path), None) => {
                let stage_err_name = format!("{}Error", stage_name.as_str());
                let stage_err_name = Ident::new(stage_err_name.as_str(), stage_ident.span());
                let error_type_name: String = this_stage_err_type_path
                    .to_string()
                    .chars()
                    .filter(|c| !c.is_whitespace())
                    .collect();

                quote! { Box::new(|
                    module_name: &'static str,
                    workflow_name: &'static str,
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages with error must have a response");
                        let outcome_result: Result<crate::workflow::types::Outcome<(), ()>, #this_stage_err_type_path> = response.into_inner();

                        match result {
                            Ok(outcome) => {
                                match outcome {
                                    crate::workflow::types::Outcome::Wait(_) => {
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
                                    crate::workflow::types::Outcome::Done(_) => {
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
                                let error = Some(crate::debug::types::AnySendSyncNamedBox::new(error, #error_type_name.to_string()));

                                let failure_sender = match failure_sender {
                                    Some(failure_sender) => failure_sender,
                                    None => {
                                        unreachable!("Ecs response handler error: Failure event send error: No failure sender provided");
                                    }
                                };

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
                    response: Option<crate::debug::types::AnySendSyncNamedBox>,
                    wait_sender: crossbeam_channel::Sender<crate::workflow::events::StageWaitEvent>,
                    completion_sender: crossbeam_channel::Sender<crate::workflow::events::StageCompletionEvent>,
                    _failure_sender: Option<crossbeam_channel::Sender<crate::workflow::events::StageFailureEvent>>
                | {
                    Box::new(move |
                        stage: crate::workflow::stage::StageRenderWhile
                    | {
                        let response = response.expect("RenderWhile stages must have a response");
                        let outcome: crate::workflow::types::Outcome<(), ()> = response.into_inner();

                        match outcome {
                            crate::workflow::types::Outcome::Wait(_) => {
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
                            crate::workflow::types::Outcome::Done(_) => {
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
                    })
                })}
            }
        };

        let failure_sender = if self.core_types.error.is_some() {
            quote! { Some(crate::workflow::channels::get_stage_failure_sender().clone()) }
        } else {
            quote! { None }
        };

        let stage_literal = quote! {
            crate::workflow::stage::Stage::RenderWhile(crate::workflow::stage::StageRenderWhile {
                index: #index_literal,
                name: #stage_name,
                signature: #signature,
                handle_render_while_setup_response: #render_while_setup_response_handler,
                handle_render_while_run_response: #render_while_run_response_handler,
                setup_sender: crate::workflow::channels::get_stage_setup_sender().clone(),
                wait_sender: crate::workflow::channels::get_stage_wait_sender().clone(),
                completion_sender: crate::workflow::channels::get_stage_completion_sender().clone(),
                failure_sender: #failure_sender,
            })
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
