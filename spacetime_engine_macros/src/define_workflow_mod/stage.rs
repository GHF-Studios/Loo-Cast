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

pub enum StageType {
    Ecs,
    Render,
    Async,
    EcsWhile,
    RenderWhile,
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
            "EcsWhile" => TypedStage::<EcsWhile>::parse(input, index).map(Stage::EcsWhile),
            "Render" => TypedStage::<Render>::parse(input, index).map(Stage::Render),
            "RenderWhile" => TypedStage::<RenderWhile>::parse(input, index).map(Stage::RenderWhile),
            "Async" => TypedStage::<Async>::parse(input, index).map(Stage::Async),
            _ => Err(input.error("Invalid stage type")),
        }
    }

    pub fn generate(
        self,
        this_stage_out_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
    ) -> (TokenStream, TokenStream, Option<TokenStream>) {
        match self {
            Stage::Ecs(stage) => {
                stage.generate(this_stage_out_type_path, next_stage_in_type_path, is_last)
            }
            Stage::EcsWhile(stage) => {
                stage.generate(this_stage_out_type_path, next_stage_in_type_path, is_last)
            }
            Stage::Render(stage) => {
                stage.generate(this_stage_out_type_path, next_stage_in_type_path, is_last)
            }
            Stage::RenderWhile(stage) => {
                stage.generate(this_stage_out_type_path, next_stage_in_type_path, is_last)
            }
            Stage::Async(stage) => {
                stage.generate(this_stage_out_type_path, next_stage_in_type_path, is_last)
            }
        }
    }

    pub fn name(&self) -> &Ident {
        match self {
            Stage::Ecs(stage) => stage.name(),
            Stage::EcsWhile(stage) => stage.name(),
            Stage::Render(stage) => stage.name(),
            Stage::RenderWhile(stage) => stage.name(),
            Stage::Async(stage) => stage.name(),
        }
    }

    pub fn has_input(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_input(),
            Stage::EcsWhile(stage) => stage.core_types.has_input(),
            Stage::Render(stage) => stage.core_types.has_input(),
            Stage::RenderWhile(stage) => stage.core_types.has_input(),
            Stage::Async(stage) => stage.core_types.has_input(),
        }
    }

    pub fn has_state(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_state(),
            Stage::EcsWhile(stage) => stage.core_types.has_state(),
            Stage::Render(stage) => stage.core_types.has_state(),
            Stage::RenderWhile(stage) => stage.core_types.has_state(),
            Stage::Async(stage) => stage.core_types.has_state(),
        }
    }

    pub fn has_output(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_output(),
            Stage::EcsWhile(stage) => stage.core_types.has_output(),
            Stage::Render(stage) => stage.core_types.has_output(),
            Stage::RenderWhile(stage) => stage.core_types.has_output(),
            Stage::Async(stage) => stage.core_types.has_output(),
        }
    }

    pub fn has_error(&self) -> bool {
        match self {
            Stage::Ecs(stage) => stage.core_types.has_error(),
            Stage::EcsWhile(stage) => stage.core_types.has_error(),
            Stage::Render(stage) => stage.core_types.has_error(),
            Stage::RenderWhile(stage) => stage.core_types.has_error(),
            Stage::Async(stage) => stage.core_types.has_error(),
        }
    }

    pub fn get_index(&self) -> usize {
        match self {
            Stage::Ecs(stage) => stage.get_index(),
            Stage::EcsWhile(stage) => stage.get_index(),
            Stage::Render(stage) => stage.get_index(),
            Stage::RenderWhile(stage) => stage.get_index(),
            Stage::Async(stage) => stage.get_index(),
        }
    }

    pub fn get_type(&self) -> StageType {
        match self {
            Stage::Ecs(_) => StageType::Ecs,
            Stage::EcsWhile(_) => StageType::EcsWhile,
            Stage::Render(_) => StageType::Render,
            Stage::RenderWhile(_) => StageType::RenderWhile,
            Stage::Async(_) => StageType::Async,
        }
    }

    pub fn get_in_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        match self {
            Stage::Ecs(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
            Stage::EcsWhile(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
            Stage::Render(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
            Stage::RenderWhile(stage) => {
                stage.get_in_type_path(workflow_module_ident, workflow_ident)
            }
            Stage::Async(stage) => stage.get_in_type_path(workflow_module_ident, workflow_ident),
        }
    }

    pub fn get_out_type_path(
        &self,
        workflow_module_ident: Ident,
        workflow_ident: Ident,
    ) -> Option<TokenStream> {
        match self {
            Stage::Ecs(stage) => stage.get_out_type_path(workflow_module_ident, workflow_ident),
            Stage::EcsWhile(stage) => {
                stage.get_out_type_path(workflow_module_ident, workflow_ident)
            }
            Stage::Render(stage) => stage.get_out_type_path(workflow_module_ident, workflow_ident),
            Stage::RenderWhile(stage) => {
                stage.get_out_type_path(workflow_module_ident, workflow_ident)
            }
            Stage::Async(stage) => stage.get_out_type_path(workflow_module_ident, workflow_ident),
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
        this_stage_out_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
    ) -> (TokenStream, TokenStream, Option<TokenStream>) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());

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
            quote! {
                crate::workflow::stage::WorkflowStage::Ecs(crate::workflow::stage::WorkflowStageEcs {
                    name: stringify!(#stage_name),
                    run_ecs: Box::new(self::stages::#stage_ident::core_functions::run_ecs) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        Self::advance_workflow_data_type(data, #index_literal)
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        } else {
            quote! {
                crate::workflow::stage::WorkflowStage::Ecs(crate::workflow::stage::WorkflowStageEcs {
                    name: stringify!(#stage_name),
                    run_ecs: Box::new(self::stages::#stage_ident::core_functions::run_ecs) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    data_type_transmuter: Box::new(|_| {
                        unreachable!("Tried to call placeholder data type transmuter");
                        None
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        };
        let stage_data_type_transmuter = match (this_stage_out_type_path, next_stage_in_type_path) {
            (Some(this_out_path), Some(next_in_path)) => {
                Some(quote! { Box::new(
                    |data: Option<Box<dyn std::any::Any + Send + Sync>>| -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        match data {
                            Some(data) => {
                                // TODO: FIX: Properly handle the potential error case and properly divert the control flow and propagate the error in that case.
                                // A.K.A.: We need to implement a way to abort workflows after failing a stage
                                bevy::prelude::debug!("Trying to downcast type `{:?}` to type `{:?}`", data.type_id(), std::any::TypeId::of::<#this_out_path>());
                                let data: #this_out_path = *data.downcast().expect("Failed to downcast data");
                                let data: #next_in_path = unsafe { std::mem::transmute(data) };
                                Some(Box::new(data) as Box<dyn std::any::Any + Send + Sync>)
                            },
                            None => { None }
                        }
                    }
                )})
            }
            (Some(_), None) => {
                if is_last {
                    None
                } else {
                    unreachable!("This stage has input, but the next stage has no output, or this stage is the last stage!")
                }
            }
            (None, Some(_)) => {
                unreachable!("This stage has no input, but the next stage has output!")
            }
            (None, None) => None,
        };

        (stage_module, stage_literal, stage_data_type_transmuter)
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

        if core_types.has_error() {
            if core_types.has_output() {
                Some(quote! { Result<
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output,
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            } else {
                Some(quote! { Result<
                    (),
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            }
        } else {
            core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
        }
    }
}

impl TypedStage<Render> {
    pub fn generate(
        self,
        this_stage_out_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
    ) -> (TokenStream, TokenStream, Option<TokenStream>) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());

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
            quote! {
                crate::workflow::stage::WorkflowStage::Render(crate::workflow::stage::WorkflowStageRender {
                    name: stringify!(#stage_name),
                    run_render: Box::new(self::stages::#stage_ident::core_functions::run_render) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        Self::advance_workflow_data_type(data, #index_literal)
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        } else {
            quote! {
                crate::workflow::stage::WorkflowStage::Render(crate::workflow::stage::WorkflowStageRender {
                    name: stringify!(#stage_name),
                    run_render: Box::new(self::stages::#stage_ident::core_functions::run_render) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    data_type_transmuter: Box::new(|_| {
                        unreachable!("Tried to call placeholder data type transmuter");
                        None
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        };
        let stage_data_type_transmuter = match (this_stage_out_type_path, next_stage_in_type_path) {
            (Some(this_out_path), Some(next_in_path)) => {
                Some(quote! { Box::new(
                    |data: Option<Box<dyn std::any::Any + Send + Sync>>| -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        match data {
                            Some(data) => {
                                // TODO: FIX: Properly handle the potential error case and properly divert the control flow and propagate the error in that case.
                                // A.K.A.: We need to implement a way to abort workflows after failing a stage
                                bevy::prelude::debug!("Trying to downcast type `{:?}` to type `{:?}`", data.type_id(), std::any::TypeId::of::<#this_out_path>());
                                let data: #this_out_path = *data.downcast().expect("Failed to downcast data");
                                let data: #next_in_path = unsafe { std::mem::transmute(data) };
                                Some(Box::new(data) as Box<dyn std::any::Any + Send + Sync>)
                            },
                            None => { None }
                        }
                    }
                )})
            }
            (Some(_), None) => {
                if is_last {
                    None
                } else {
                    unreachable!("This stage has input, but the next stage has no output, or this stage is the last stage!")
                }
            }
            (None, Some(_)) => {
                unreachable!("This stage has no input, but the next stage has output!")
            }
            (None, None) => None,
        };

        (stage_module, stage_literal, stage_data_type_transmuter)
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

        if core_types.has_error() {
            if core_types.has_output() {
                Some(quote! { Result<
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output,
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            } else {
                Some(quote! { Result<
                    (),
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            }
        } else {
            core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
        }
    }
}

impl TypedStage<Async> {
    pub fn generate(
        self,
        this_stage_out_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
    ) -> (TokenStream, TokenStream, Option<TokenStream>) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());

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
            quote! {
                crate::workflow::stage::WorkflowStage::Async(crate::workflow::stage::WorkflowStageAsync {
                    name: stringify!(#stage_name),
                    run_async: Box::new(self::stages::#stage_ident::core_functions::run_async) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        Self::advance_workflow_data_type(data, #index_literal)
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        } else {
            quote! {
                crate::workflow::stage::WorkflowStage::Async(crate::workflow::stage::WorkflowStageAsync {
                    name: stringify!(#stage_name),
                    run_async: Box::new(self::stages::#stage_ident::core_functions::run_async) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        unreachable!("Tried to call placeholder data type transmuter");
                        None
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        };
        let stage_data_type_transmuter = match (this_stage_out_type_path, next_stage_in_type_path) {
            (Some(this_out_path), Some(next_in_path)) => {
                Some(quote! { Box::new(
                    |data: Option<Box<dyn std::any::Any + Send + Sync>>| -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        match data {
                            Some(data) => {
                                // TODO: FIX: Properly handle the potential error case and properly divert the control flow and propagate the error in that case.
                                // A.K.A.: We need to implement a way to abort workflows after failing a stage
                                bevy::prelude::debug!("Trying to downcast type `{:?}` to type `{:?}`", data.type_id(), std::any::TypeId::of::<#this_out_path>());
                                let data: #this_out_path = *data.downcast().expect("Failed to downcast data");
                                let data: #next_in_path = unsafe { std::mem::transmute(data) };
                                Some(Box::new(data) as Box<dyn std::any::Any + Send + Sync>)
                            },
                            None => { None }
                        }
                    }
                )})
            }
            (Some(_), None) => {
                if is_last {
                    None
                } else {
                    unreachable!("This stage has input, but the next stage has no output, or this stage is the last stage!")
                }
            }
            (None, Some(_)) => {
                unreachable!("This stage has no input, but the next stage has output!")
            }
            (None, None) => None,
        };

        (stage_module, stage_literal, stage_data_type_transmuter)
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

        if core_types.has_error() {
            if core_types.has_output() {
                Some(quote! { Result<
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output,
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            } else {
                Some(quote! { Result<
                    (),
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            }
        } else {
            core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
        }
    }
}

impl TypedStage<EcsWhile> {
    pub fn generate(
        self,
        this_stage_out_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
    ) -> (TokenStream, TokenStream, Option<TokenStream>) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());

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
            quote! {
                crate::workflow::stage::WorkflowStage::EcsWhile(crate::workflow::stage::WorkflowStageEcsWhile {
                    name: stringify!(#stage_name),
                    setup_ecs_while: Box::new(self::stages::#stage_ident::core_functions::setup_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_ecs_while: Box::new(self::stages::#stage_ident::core_functions::run_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        Self::advance_workflow_data_type(data, #index_literal)
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        } else {
            quote! {
                crate::workflow::stage::WorkflowStage::EcsWhile(crate::workflow::stage::WorkflowStageEcsWhile {
                    name: stringify!(#stage_name),
                    setup_ecs_while: Box::new(self::stages::#stage_ident::core_functions::setup_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_ecs_while: Box::new(self::stages::#stage_ident::core_functions::run_ecs_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        unreachable!("Tried to call placeholder data type transmuter");
                        None
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        };
        let stage_data_type_transmuter = match (this_stage_out_type_path, next_stage_in_type_path) {
            (Some(this_out_path), Some(next_in_path)) => {
                Some(quote! { Box::new(
                    |data: Option<Box<dyn std::any::Any + Send + Sync>>| -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        match data {
                            Some(data) => {
                                // TODO: FIX: Properly handle the potential error case and properly divert the control flow and propagate the error in that case.
                                // A.K.A.: We need to implement a way to abort workflows after failing a stage
                                bevy::prelude::debug!("Trying to downcast type `{:?}` to type `{:?}`", data.type_id(), std::any::TypeId::of::<#this_out_path>());
                                let data: #this_out_path = *data.downcast().expect("Failed to downcast data");
                                let data: #next_in_path = unsafe { std::mem::transmute(data) };
                                Some(Box::new(data) as Box<dyn std::any::Any + Send + Sync>)
                            },
                            None => { None }
                        }
                    }
                )})
            }
            (Some(_), None) => {
                if is_last {
                    None
                } else {
                    unreachable!("This stage has input, but the next stage has no output, or this stage is the last stage!")
                }
            }
            (None, Some(_)) => {
                unreachable!("This stage has no input, but the next stage has output!")
            }
            (None, None) => None,
        };

        (stage_module, stage_literal, stage_data_type_transmuter)
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

        if core_types.has_error() {
            if core_types.has_output() {
                Some(quote! { Result<
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output,
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            } else {
                Some(quote! { Result<
                    (),
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            }
        } else {
            core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
        }
    }
}

impl TypedStage<RenderWhile> {
    pub fn generate(
        self,
        this_stage_out_type_path: Option<&TokenStream>,
        next_stage_in_type_path: Option<&TokenStream>,
        is_last: bool,
    ) -> (TokenStream, TokenStream, Option<TokenStream>) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(
            stage_name.as_str().to_snake_case().as_str(),
            stage_ident.span(),
        );
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();
        let index_literal = LitInt::new(&(self.index).to_string(), stage_ident.span());

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
            quote! {
                crate::workflow::stage::WorkflowStage::RenderWhile(crate::workflow::stage::WorkflowStageRenderWhile {
                    name: stringify!(#stage_name),
                    setup_render_while: Box::new(self::stages::#stage_ident::core_functions::setup_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_render_while: Box::new(self::stages::#stage_ident::core_functions::run_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        Self::advance_workflow_data_type(data, #index_literal)
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        } else {
            quote! {
                crate::workflow::stage::WorkflowStage::RenderWhile(crate::workflow::stage::WorkflowStageRenderWhile {
                    name: stringify!(#stage_name),
                    setup_render_while: Box::new(self::stages::#stage_ident::core_functions::setup_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>,
                    run_render_while: Box::new(self::stages::#stage_ident::core_functions::run_render_while) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>, &mut bevy::prelude::World) -> Box<dyn std::any::Any + Send + Sync> + Send + Sync>,
                    data_type_transmuter: Box::new(|data| {
                        unreachable!("Tried to call placeholder data type transmuter");
                        None
                    }) as Box<dyn FnMut(Option<Box<dyn std::any::Any + Send + Sync>>) -> Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>
                })
            }
        };
        let stage_data_type_transmuter = match (this_stage_out_type_path, next_stage_in_type_path) {
            (Some(this_out_path), Some(next_in_path)) => {
                Some(quote! { Box::new(
                    |data: Option<Box<dyn std::any::Any + Send + Sync>>| -> Option<Box<dyn std::any::Any + Send + Sync>> {
                        match data {
                            Some(data) => {
                                // TODO: FIX: Properly handle the potential error case and properly divert the control flow and propagate the error in that case.
                                // A.K.A.: We need to implement a way to abort workflows after failing a stage
                                bevy::prelude::debug!("Trying to downcast type `{:?}` to type `{:?}`", data.type_id(), std::any::TypeId::of::<#this_out_path>());
                                let data: #this_out_path = *data.downcast().expect("Failed to downcast data");
                                let data: #next_in_path = unsafe { std::mem::transmute(data) };
                                Some(Box::new(data) as Box<dyn std::any::Any + Send + Sync>)
                            },
                            None => { None }
                        }
                    }
                )})
            }
            (Some(_), None) => {
                if is_last {
                    None
                } else {
                    unreachable!("This stage has input, but the next stage has no output, or this stage is the last stage!")
                }
            }
            (None, Some(_)) => {
                unreachable!("This stage has no input, but the next stage has output!")
            }
            (None, None) => None,
        };

        (stage_module, stage_literal, stage_data_type_transmuter)
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

        if core_types.has_error() {
            if core_types.has_output() {
                Some(quote! { Result<
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output,
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            } else {
                Some(quote! { Result<
                    (),
                    crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Error
                    >
                })
            }
        } else {
            core_types.output.as_ref().map(|_| quote! { crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::stages::#stage_ident::core_types::Output })
        }
    }
}
