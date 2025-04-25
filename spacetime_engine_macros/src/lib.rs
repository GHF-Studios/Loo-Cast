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

    let plugin_addition_literals = idents.iter().map(|ident| {
        let mod_name = ident.to_string().to_lowercase();
        let mod_ident = Ident::new(mod_name.as_str(), ident.span());
        let plugin_name = syn::Ident::new(&format!("{}WorkflowsPlugin", ident), ident.span());
        quote! {
            .add(crate::#mod_ident::workflows::#mod_ident::#plugin_name)
        }
    });

    let main_access_enum_variants = vec![quote!{ MainAccess::Placeholder }];
    let render_access_enum_variants = vec![quote!{ RenderAccess::Placeholder }];
    let stage_buffer_enum_variants = vec![quote!{ Placeholder }];

    let expanded = quote! {
        pub(super) fn push_ecs_stages_to_ecs_buffers_system(buffer: ResMut<EcsStageBuffer>) {
        }

        pub struct WorkflowModule {
            name: &'static str,
            workflows: &'static [Workflow],
        }
        
        pub struct Workflow {
            name: &'static str,
            stages: &'static [WorkflowStage],
        }
        
        pub enum WorkflowStage {
            Ecs(WorkflowStageEcs),
            Render(WorkflowStageRender),
            Async(WorkflowStageAsync),
            EcsWhile(WorkflowStageEcsWhile),
            RenderWhile(WorkflowStageRenderWhile),
        }
        
        pub struct WorkflowStageEcs {
            name: &'static str,
            polling_function: fn(stage_buffer: bevy::prelude::ResMut<TypedStageBuffer>, main_access: MainAccess),
        }
        
        pub struct WorkflowStageRender {
            name: &'static str,
            polling_function: fn(stage_buffer: bevy::prelude::ResMut<TypedStageBuffer>, render_access: RenderAccess),
        }
        
        pub struct WorkflowStageAsync {
            name: &'static str,
            polling_function: fn(stage_buffer: bevy::prelude::ResMut<TypedStageBuffer>),
        }
        
        pub struct WorkflowStageEcsWhile {
            name: &'static str,
            polling_function: fn(stage_buffer: bevy::prelude::ResMut<TypedStageBuffer>, workflow_map: ResMut<crate::workflow::resources::WorkflowMap>, main_access: MainAccess),
        }
        
        pub struct WorkflowStageRenderWhile {
            name: &'static str,
            polling_function: fn(stage_buffer: bevy::prelude::ResMut<TypedStageBuffer>, render_workflow_state_extract: ResMut<crate::workflow::resources::RenderWhileWorkflowStateExtract>, render_access: RenderAccess),
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
