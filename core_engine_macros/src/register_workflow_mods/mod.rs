use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use syn::braced;
use syn::parse::{Parse, ParseStream, Result};
use syn::punctuated::Punctuated;
use syn::{Ident, Token};

pub struct WorkflowMods {
    pub modules: Vec<WorkflowMod>,
}

pub struct WorkflowMod {
    pub name: Ident,
    pub workflows: Vec<Workflow>,
}

pub struct Workflow {
    pub name: Ident,
    pub stages: Vec<Stage>,
}

pub struct Stage {
    pub name: Ident,
    pub ty: StageType,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StageType {
    Ecs,
    Render,
    Async,
    EcsWhile,
    RenderWhile,
}

impl Parse for WorkflowMods {
    fn parse(input: ParseStream) -> Result<Self> {
        let modules = Punctuated::<WorkflowMod, Token![,]>::parse_terminated(input)?.into_iter().collect();
        Ok(Self { modules })
    }
}

impl Parse for WorkflowMod {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let workflows = Punctuated::<Workflow, Token![,]>::parse_terminated(&content)?.into_iter().collect();

        Ok(Self { name, workflows })
    }
}

impl Parse for Workflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let stages = Punctuated::<Stage, Token![,]>::parse_terminated(&content)?.into_iter().collect();

        Ok(Self { name, stages })
    }
}

impl Parse for Stage {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        input.parse::<Token![:]>()?;
        let ty: StageType = input.parse()?;
        Ok(Self { name, ty })
    }
}

impl Parse for StageType {
    fn parse(input: ParseStream) -> Result<Self> {
        let ty: Ident = input.parse()?;

        match ty.to_string().as_str() {
            "Ecs" => Ok(Self::Ecs),
            "Render" => Ok(Self::Render),
            "Async" => Ok(Self::Async),
            "EcsWhile" => Ok(Self::EcsWhile),
            "RenderWhile" => Ok(Self::RenderWhile),
            _ => Err(syn::Error::new(
                input.span(),
                "Invalid stage type! Expected one of: `Ecs`, `Render`, `Async`, `EcsWhile`, `RenderWhile`",
            )),
        }
    }
}

impl WorkflowMods {
    pub fn generate(self) -> TokenStream {
        let plugin_addition_literals: Vec<_> = self
            .modules
            .iter()
            .map(|workflow_module| {
                let ident = &workflow_module.name;
                let mod_name = ident.to_string().to_snake_case();
                let mod_ident = Ident::new(mod_name.as_str(), ident.span());
                let plugin_name = syn::Ident::new(&format!("{}WorkflowsPlugin", ident), ident.span());

                quote! {
                    .add(crate::#mod_ident::workflows::#mod_ident::#plugin_name)
                }
            })
            .collect();

        let workflow_modules_items = quote! {
            pub trait FillWorkflowStageEcsBufferEventMarker: std::any::Any + Send {}
            pub trait FillWorkflowStageRenderBufferEventMarker: std::any::Any + Send {}
            pub trait FillWorkflowStageAsyncBufferEventMarker: std::any::Any + Send {}
            pub trait FillWorkflowStageEcsWhileBufferEventMarker: std::any::Any + Send {}
            pub trait FillWorkflowStageRenderWhileBufferEventMarker: std::any::Any + Send {}

            pub trait AnyFillWorkflowStageEcsBufferEventMarker: std::any::Any + Send {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any>;
            }
            pub trait AnyFillWorkflowStageRenderBufferEventMarker: std::any::Any + Send {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any>;
            }
            pub trait AnyFillWorkflowStageAsyncBufferEventMarker: std::any::Any + Send {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any>;
            }
            pub trait AnyFillWorkflowStageEcsWhileBufferEventMarker: std::any::Any + Send {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any>;
            }
            pub trait AnyFillWorkflowStageRenderWhileBufferEventMarker: std::any::Any + Send {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any>;
            }

            impl<T: FillWorkflowStageEcsBufferEventMarker> AnyFillWorkflowStageEcsBufferEventMarker for T {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                    self
                }
            }
            impl<T: FillWorkflowStageRenderBufferEventMarker> AnyFillWorkflowStageRenderBufferEventMarker for T {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                    self
                }
            }
            impl<T: FillWorkflowStageAsyncBufferEventMarker> AnyFillWorkflowStageAsyncBufferEventMarker for T {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                    self
                }
            }
            impl<T: FillWorkflowStageEcsWhileBufferEventMarker> AnyFillWorkflowStageEcsWhileBufferEventMarker for T {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                    self
                }
            }
            impl<T: FillWorkflowStageRenderWhileBufferEventMarker> AnyFillWorkflowStageRenderWhileBufferEventMarker for T {
                fn as_any(self: Box<Self>) -> Box<dyn std::any::Any> {
                    self
                }
            }

            pub trait DynFillWorkflowStageEcsBufferEventSender: dyn_clone::DynClone + Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, module_name: &'static str, workflow_name: &'static str, stage_index: usize, stage: crate::workflow::stage::StageEcs, stage_buffer: Option<crate::debug::types::AnySendSyncPremiumBox>);
                fn as_any_ref(&self) -> &dyn std::any::Any;
            }
            pub trait DynFillWorkflowStageRenderBufferEventSender: dyn_clone::DynClone + Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, module_name: &'static str, workflow_name: &'static str, stage_index: usize, stage: crate::workflow::stage::StageRender, stage_buffer: Option<crate::debug::types::AnySendSyncPremiumBox>);
                fn as_any_ref(&self) -> &dyn std::any::Any;
            }
            pub trait DynFillWorkflowStageAsyncBufferEventSender: dyn_clone::DynClone + Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, module_name: &'static str, workflow_name: &'static str, stage_index: usize, stage: crate::workflow::stage::StageAsync, stage_buffer: Option<crate::debug::types::AnySendSyncPremiumBox>);
                fn as_any_ref(&self) -> &dyn std::any::Any;
            }
            pub trait DynFillWorkflowStageEcsWhileBufferEventSender: dyn_clone::DynClone + Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, module_name: &'static str, workflow_name: &'static str, stage_index: usize, stage: crate::workflow::stage::StageEcsWhile, stage_buffer: Option<crate::debug::types::AnySendSyncPremiumBox>);
                fn as_any_ref(&self) -> &dyn std::any::Any;
            }
            pub trait DynFillWorkflowStageRenderWhileBufferEventSender: dyn_clone::DynClone + Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, module_name: &'static str, workflow_name: &'static str, stage_index: usize, stage: crate::workflow::stage::StageRenderWhile, stage_buffer: Option<crate::debug::types::AnySendSyncPremiumBox>);
                fn as_any_ref(&self) -> &dyn std::any::Any;
            }
            dyn_clone::clone_trait_object!(DynFillWorkflowStageEcsBufferEventSender);
            dyn_clone::clone_trait_object!(DynFillWorkflowStageRenderBufferEventSender);
            dyn_clone::clone_trait_object!(DynFillWorkflowStageAsyncBufferEventSender);
            dyn_clone::clone_trait_object!(DynFillWorkflowStageEcsWhileBufferEventSender);
            dyn_clone::clone_trait_object!(DynFillWorkflowStageRenderWhileBufferEventSender);

            #[derive(Clone, Debug)]
            pub struct WorkflowModuleMetadata {
                name: &'static str,
                workflows: Box<[WorkflowMetadata]>,
            }

            #[derive(Clone, Debug)]
            pub struct WorkflowMetadata {
                name: &'static str,
                stages: Box<[WorkflowStageMetadata]>,
            }

            #[derive(Clone)]
            pub enum WorkflowStageMetadata {
                Ecs {
                    name: &'static str,
                    sender: Box<dyn DynFillWorkflowStageEcsBufferEventSender>
                },
                Render {
                    name: &'static str,
                    sender: Box<dyn DynFillWorkflowStageRenderBufferEventSender>
                },
                Async {
                    name: &'static str,
                    sender: Box<dyn DynFillWorkflowStageAsyncBufferEventSender>
                },
                EcsWhile {
                    name: &'static str,
                    sender: Box<dyn DynFillWorkflowStageEcsWhileBufferEventSender>
                },
                RenderWhile {
                    name: &'static str,
                    sender: Box<dyn DynFillWorkflowStageRenderWhileBufferEventSender>
                },
            }
            impl std::fmt::Debug for WorkflowStageMetadata {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        WorkflowStageMetadata::Ecs { .. } => write!(f, "Ecs"),
                        WorkflowStageMetadata::Render { .. } => write!(f, "Render"),
                        WorkflowStageMetadata::Async { .. } => write!(f, "Async"),
                        WorkflowStageMetadata::EcsWhile { .. } => write!(f, "EcsWhile"),
                        WorkflowStageMetadata::RenderWhile { .. } => write!(f, "RenderWhile"),
                    }
                }
            }
        };

        let workflow_modules_metadata: Vec<_> = self.modules.into_iter().map(|workflow_module| workflow_module.generate()).collect();
        let workflow_modules_metadata = quote! {
            static WORKFLOW_MODULES_METADATA: std::sync::OnceLock<Box<[WorkflowModuleMetadata]>> = std::sync::OnceLock::new();

            pub fn initialize_workflow_modules_metadata() {
                WORKFLOW_MODULES_METADATA
                    .set(Box::new([
                        #(#workflow_modules_metadata),*
                    ]))
                    .expect("Workflow modules metadata already initialized!");
            }

            pub fn get_workflow_modules_metadata() -> Box<[WorkflowModuleMetadata]> {
                WORKFLOW_MODULES_METADATA
                    .get()
                    .expect("Workflow modules metadata not initialized!")
                    .clone()
            }
        };

        quote! {
            #workflow_modules_items

            #workflow_modules_metadata

            pub struct SpacetimeEngineWorkflowPlugins;
            impl bevy::prelude::PluginGroup for SpacetimeEngineWorkflowPlugins {
                fn build(self) -> bevy::app::PluginGroupBuilder {
                    let plugin_group_builder = bevy::app::PluginGroupBuilder::start::<Self>()
                        #(#plugin_addition_literals)*;

                    initialize_workflow_modules_metadata();

                    plugin_group_builder
                }
            }
        }
    }
}

impl WorkflowMod {
    pub fn generate(self) -> TokenStream {
        let workflow_module_name = self.name.to_string();
        let workflow_module_name_snake_case = workflow_module_name.as_str().to_snake_case();
        let workflow_module_metadata: Vec<_> = self
            .workflows
            .into_iter()
            .map(|workflow| workflow.generate(Ident::new(workflow_module_name_snake_case.as_str(), self.name.span())))
            .collect();

        quote! {
            WorkflowModuleMetadata {
                name: #workflow_module_name,
                workflows: Box::new([
                    #(#workflow_module_metadata),*
                ]),
            }
        }
    }
}

impl Workflow {
    pub fn generate(self, module_name: Ident) -> TokenStream {
        let workflow_name = self.name.to_string();
        let workflow_name_snake_case = workflow_name.as_str().to_snake_case();
        let workflow_metadata: Vec<_> = self
            .stages
            .into_iter()
            .map(|workflow| workflow.generate(module_name.clone(), Ident::new(workflow_name_snake_case.as_str(), self.name.span())))
            .collect();

        quote! {
            WorkflowMetadata {
                name: #workflow_name,
                stages: Box::new([
                    #(#workflow_metadata),*
                ]),
            }
        }
    }
}

impl Stage {
    pub fn generate(self, module_name: Ident, workflow_name: Ident) -> TokenStream {
        let stage_name = self.name.to_string();
        let stage_name_snake_case = stage_name.as_str().to_snake_case();
        let stage_ident = Ident::new(stage_name_snake_case.as_str(), self.name.span());

        match self.ty {
            StageType::Ecs => {
                quote! {
                    WorkflowStageMetadata::Ecs {
                        name: #stage_name,
                        sender: Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_ident::core_types::pre_initialize_fill_workflow_stage_buffer_channel())
                    }
                }
            }
            StageType::Render => {
                quote! {
                    WorkflowStageMetadata::Render {
                        name: #stage_name,
                        sender: Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_ident::core_types::pre_initialize_fill_workflow_stage_buffer_channel())
                    }
                }
            }
            StageType::Async => {
                quote! {
                    WorkflowStageMetadata::Async {
                        name: #stage_name,
                        sender: Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_ident::core_types::pre_initialize_fill_workflow_stage_buffer_channel())
                    }
                }
            }
            StageType::EcsWhile => {
                quote! {
                    WorkflowStageMetadata::EcsWhile {
                        name: #stage_name,
                        sender: Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_ident::core_types::pre_initialize_fill_workflow_stage_buffer_channel())
                    }
                }
            }
            StageType::RenderWhile => {
                quote! {
                    WorkflowStageMetadata::RenderWhile {
                        name: #stage_name,
                        sender: Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_ident::core_types::pre_initialize_fill_workflow_stage_buffer_channel())
                    }
                }
            }
        }
    }
}
