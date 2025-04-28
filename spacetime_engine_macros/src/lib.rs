mod define_composite_workflow;
mod define_workflow_mod;
mod define_workflow_mod_OLD;

use define_composite_workflow::CompositeWorkflow;
use define_workflow_mod_OLD::WorkflowModule;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, punctuated::Punctuated, Ident, Token};

#[proc_macro]
pub fn define_composite_workflow(input: TokenStream) -> TokenStream {
    let composite_workflow = parse_macro_input!(input as CompositeWorkflow);
    composite_workflow.generate().into()
}

#[proc_macro]
pub fn define_workflow_mod_OLD(input: TokenStream) -> TokenStream {
    let workflow_module = parse_macro_input!(input as WorkflowModule);
    workflow_module.generate().into()
}

#[proc_macro]
pub fn define_workflow_mods(input: TokenStream) -> TokenStream {
    let idents = parse_macro_input!(input with Punctuated::<Ident, Token![,]>::parse_terminated);

    let workflow_modules_metadata = todo!();

    let plugin_addition_literals = idents.iter().map(|ident| {
        let mod_name = ident.to_string().to_lowercase();
        let mod_ident = Ident::new(mod_name.as_str(), ident.span());
        let plugin_name = syn::Ident::new(&format!("{}WorkflowsPlugin", ident), ident.span());
        quote! {
            .add(crate::#mod_ident::workflows::#mod_ident::#plugin_name)
        }
    });

    let expanded = quote! {
        pub static WORKFLOW_MODULES_METADATA: &[WorkflowModuleMetadata] = &[
            WorkflowModuleMetadata {
                name: "Camera",
                workflows: &[WorkflowMetadata] = &[
                    WorkflowMetadata {
                        name: "SpawnMainCamera",
                        stages: &[WorkflowStageMetadata] = &[
                            WorkflowStageMetadata::Ecs(WorkflowStageEcsMetadata {
                                name: "Spawn",
                                poll_fn: crate::camera::workflows::camera::spawn_main_camera::stages::spawn::core_functions::poll_fn,
                            }),
                        ],
                    },
                ],
            },
        ];

        pub trait FillWorkflowStageEcsBufferEventMarker: Send;
        pub trait FillWorkflowStageRenderBufferEventMarker: Send;
        pub trait FillWorkflowStageAsyncBufferEventMarker: Send;
        pub trait FillWorkflowStageEcsWhileBufferEventMarker: Send;
        pub trait FillWorkflowStageRenderWhileBufferEventMarker: Send;

        pub trait DynFillWorkflowStageEcsBufferEventSender<T: FillWorkflowStageEcsBufferEventMarker>: Send + Sync {
            fn module_name(&self) -> &'static str;
            fn workflow_name(&self) -> &'static str;
            fn stage_index(&self) -> usize;
            fn send(&self, event: T);
        }
        pub trait DynFillWorkflowStageRenderBufferEventSender<T: FillWorkflowStageRenderBufferEventMarker>: Send + Sync {
            fn module_name(&self) -> &'static str;
            fn workflow_name(&self) -> &'static str;
            fn stage_index(&self) -> usize;
            fn send(&self, event: T);
        }
        pub trait DynFillWorkflowStageAsyncBufferEventSender<T: FillWorkflowStageAsyncBufferEventMarker>: Send + Sync {
            fn module_name(&self) -> &'static str;
            fn workflow_name(&self) -> &'static str;
            fn stage_index(&self) -> usize;
            fn send(&self, event: T);
        }
        pub trait DynFillWorkflowStageEcsWhileBufferEventSender<T: FillWorkflowStageEcsWhileBufferEventMarker>: Send + Sync {
            fn module_name(&self) -> &'static str;
            fn workflow_name(&self) -> &'static str;
            fn stage_index(&self) -> usize;
            fn send(&self, event: T);
        }
        pub trait DynFillWorkflowStageRenderWhileBufferEventSender<T: FillWorkflowStageRenderWhileBufferEventMarker>: Send + Sync {
            fn module_name(&self) -> &'static str;
            fn workflow_name(&self) -> &'static str;
            fn stage_index(&self) -> usize;
            fn send(&self, event: T);
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

        pub struct SpacetimeEngineWorkflowPlugins;
        impl bevy::prelude::PluginGroup for SpacetimeEngineWorkflowPlugins {
            fn build(self) -> bevy::app::PluginGroupBuilder {
                bevy::app::PluginGroupBuilder::start::<Self>()
                    #(#plugin_addition_literals)*
            }
        }
    };

    expanded.into()
}

#[proc_macro]
pub fn define_workflow_mod(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro]
pub fn define_workflow(input: TokenStream) -> TokenStream {
    quote! {}.into()
}

#[proc_macro]
pub fn define_worfklow_stages(input: TokenStream) -> TokenStream {
    quote! {}.into()
}
