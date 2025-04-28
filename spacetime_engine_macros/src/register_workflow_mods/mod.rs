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
        let modules = Punctuated::<WorkflowMod, Token![,]>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(Self { modules })
    }
}

impl Parse for WorkflowMod {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let workflows = Punctuated::<Workflow, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

        Ok(Self { name, workflows })
    }
}

impl Parse for Workflow {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let stages = Punctuated::<Stage, Token![,]>::parse_terminated(&content)?
            .into_iter()
            .collect();

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
            _ => Err(syn::Error::new(input.span(), "Invalid stage type! Expected one of: `Ecs`, `Render`, `Async`, `EcsWhile`, `RenderWhile`"))
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
                let mod_name = ident.to_string().to_lowercase();
                let mod_ident = Ident::new(mod_name.as_str(), ident.span());
                let plugin_name =
                    syn::Ident::new(&format!("{}WorkflowsPlugin", ident), ident.span());

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

            pub trait DynFillWorkflowStageEcsBufferEventSender: Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, event: Box<dyn FillWorkflowStageEcsBufferEventMarker>);
            }
            pub trait DynFillWorkflowStageRenderBufferEventSender: Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, event: Box<dyn FillWorkflowStageRenderBufferEventMarker>);
            }
            pub trait DynFillWorkflowStageAsyncBufferEventSender: Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, event: Box<dyn FillWorkflowStageAsyncBufferEventMarker>);
            }
            pub trait DynFillWorkflowStageEcsWhileBufferEventSender: Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, event: Box<dyn FillWorkflowStageEcsWhileBufferEventMarker>);
            }
            pub trait DynFillWorkflowStageRenderWhileBufferEventSender: Send + Sync {
                fn module_name(&self) -> &'static str;
                fn workflow_name(&self) -> &'static str;
                fn stage_index(&self) -> usize;
                fn send(&self, event: Box<dyn FillWorkflowStageRenderWhileBufferEventMarker>);
            }

            pub struct WorkflowModuleMetadata {
                name: &'static str,
                workflows: &'static [WorkflowMetadata],
            }

            pub struct WorkflowMetadata {
                name: &'static str,
                stages: &'static [WorkflowStageMetadata],
            }

            pub enum WorkflowStageMetadata {
                Ecs(Box<dyn DynFillWorkflowStageEcsBufferEventSender>),
                Render(Box<dyn DynFillWorkflowStageRenderBufferEventSender>),
                Async(Box<dyn DynFillWorkflowStageAsyncBufferEventSender>),
                EcsWhile(Box<dyn DynFillWorkflowStageEcsWhileBufferEventSender>),
                RenderWhile(Box<dyn DynFillWorkflowStageRenderWhileBufferEventSender>),
            }
        };

        let workflow_modules_metadata: Vec<_> = self
            .modules
            .into_iter()
            .map(|workflow_module| workflow_module.generate())
            .collect();
        let workflow_modules_metadata = quote! {
            pub static WORKFLOW_MODULES_METADATA: &[WorkflowModuleMetadata] = &[
                #(#workflow_modules_metadata),*
            ];
        };

        quote! {
            #workflow_modules_items

            #workflow_modules_metadata

            pub struct SpacetimeEngineWorkflowPlugins;
            impl bevy::prelude::PluginGroup for SpacetimeEngineWorkflowPlugins {
                fn build(self) -> bevy::app::PluginGroupBuilder {
                    bevy::app::PluginGroupBuilder::start::<Self>()
                        #(#plugin_addition_literals)*
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
            .map(|workflow| {
                workflow.generate(Ident::new(
                    workflow_module_name_snake_case.as_str(),
                    self.name.span(),
                ))
            })
            .collect();

        quote! {
            WorkflowModuleMetadata {
                name: #workflow_module_name,
                workflows: &[
                    #(#workflow_module_metadata),*
                ],
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
            .map(|workflow| {
                workflow.generate(
                    module_name.clone(),
                    Ident::new(workflow_name_snake_case.as_str(), self.name.span()),
                )
            })
            .collect();

        quote! {
            WorkflowMetadata {
                name: #workflow_name,
                stages: &[
                    #(#workflow_metadata),*
                ],
            }
        }
    }
}

impl Stage {
    pub fn generate(self, module_name: Ident, workflow_name: Ident) -> TokenStream {
        let stage_name = self.name.to_string();
        let stage_name_snake_case = stage_name.as_str().to_snake_case();
        let stage_name = Ident::new(stage_name_snake_case.as_str(), self.name.span());

        match self.ty {
            StageType::Ecs => {
                quote! {
                    WorkflowStageMetadata::Ecs(Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_name::core_types::initialize_fill_workflow_stage_buffer_channel()))
                }
            }
            StageType::Render => {
                quote! {
                    WorkflowStageMetadata::Render(Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_name::core_types::initialize_fill_workflow_stage_buffer_channel()))
                }
            }
            StageType::Async => {
                quote! {
                    WorkflowStageMetadata::Async(Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_name::core_types::initialize_fill_workflow_stage_buffer_channel()))
                }
            }
            StageType::EcsWhile => {
                quote! {
                    WorkflowStageMetadata::EcsWhile(Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_name::core_types::initialize_fill_workflow_stage_buffer_channel()))
                }
            }
            StageType::RenderWhile => {
                quote! {
                    WorkflowStageMetadata::RenderWhile(Box::new(crate::#module_name::workflows::#module_name::#workflow_name::stages::#stage_name::core_types::initialize_fill_workflow_stage_buffer_channel()))
                }
            }
        }
    }
}
