use super::stage::{Async, Ecs, EcsWhile, Render, RenderWhile, StageSignature};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use std::marker::PhantomData;
use syn::{
    parse::{Parse, ParseStream},
    spanned::Spanned,
    token::Pub,
    Fields, ItemEnum, ItemStruct, Lifetime, LifetimeParam, Result, Visibility,
};

fn align_core_struct(item: &mut ItemStruct) {
    let span = item.ident.span();

    item.vis = Visibility::Public(Pub(span));

    match &mut item.fields {
        Fields::Named(named) => {
            for field in &mut named.named {
                field.vis = Visibility::Public(Pub(span));
            }
        }
        Fields::Unnamed(unnamed) => {
            for field in &mut unnamed.unnamed {
                field.vis = Visibility::Public(Pub(span));
            }
        }
        Fields::Unit => {}
    }
}

fn align_core_enum(item: &mut ItemEnum) {
    let span = item.ident.span();

    item.vis = Visibility::Public(Pub(span));
}

pub struct Input;
pub struct State;
pub struct Output;
pub struct Error;
pub struct MainAccess;
pub struct RenderAccess;

pub enum CoreType<T> {
    Struct(ItemStruct, PhantomData<T>),
    Enum(ItemEnum, PhantomData<T>),
}

impl CoreType<Input> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => item.to_token_stream(),
            CoreType::Enum(item, _) => item.to_token_stream(),
        }
    }
}

impl CoreType<State> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => item.to_token_stream(),
            CoreType::Enum(item, _) => item.to_token_stream(),
        }
    }
}

impl CoreType<Output> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => item.to_token_stream(),
            CoreType::Enum(item, _) => item.to_token_stream(),
        }
    }
}

impl CoreType<Error> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(std::fmt::Debug, Error)]
                    #item
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
            }
            CoreType::Enum(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(std::fmt::Debug, Error)]
                    #item
                    impl std::fmt::Display for Error {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            write!(f, "{:?}", self)
                        }
                    }
                }
            }
        }
    }
}

impl CoreType<MainAccess> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(bevy::ecs::system::SystemParam)]
                    #item
                }
            }
            CoreType::Enum(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(bevy::ecs::system::SystemParam)]
                    #item
                }
            }
        }
    }
}

impl CoreType<RenderAccess> {
    pub fn generate(&self) -> TokenStream {
        match self {
            CoreType::Struct(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(bevy::ecs::system::SystemParam)]
                    #item
                }
            }
            CoreType::Enum(item, _) => {
                let item = item.to_token_stream();
                quote! {
                    #[derive(bevy::ecs::system::SystemParam)]
                    #item
                }
            }
        }
    }
}

pub struct CoreTypes<T> {
    pub _phantom_data: PhantomData<T>,
    pub input: Option<CoreType<Input>>,
    pub state: Option<CoreType<State>>,
    pub output: Option<CoreType<Output>>,
    pub error: Option<CoreType<Error>>,
    pub main_access: Option<CoreType<MainAccess>>,
    pub render_access: Option<CoreType<RenderAccess>>,
}

impl Parse for CoreTypes<Ecs> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;
        let mut main_access_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref s)
                    if s.ident == "State"
                        || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") =>
                {
                    return Err(input.error("State is not allowed in Ecs stages"));
                }
                syn::Item::Struct(ref mut s) if s.ident == "MainAccess" => {
                    align_core_struct(s);
                    if main_access_type.is_some() {
                        return Err(input.error("Duplicate MainAccess type"));
                    }
                    main_access_type = Some(CoreType::<MainAccess>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "MainAccess" => {
                    align_core_enum(e);
                    if main_access_type.is_some() {
                        return Err(input.error("Duplicate MainAccess type"));
                    }
                    main_access_type = Some(CoreType::<MainAccess>::Enum(e.clone(), PhantomData));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            _phantom_data: PhantomData,
            input: input_type,
            state: None,
            output: output_type,
            error: error_type,
            main_access: main_access_type,
            render_access: None,
        })
    }
}

impl Parse for CoreTypes<EcsWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut state_type = None;
        let mut output_type = None;
        let mut error_type = None;
        let mut main_access_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "State" => {
                    align_core_struct(s);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "State" => {
                    align_core_enum(e);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "MainAccess" => {
                    align_core_struct(s);
                    if main_access_type.is_some() {
                        return Err(input.error("Duplicate MainAccess type"));
                    }
                    main_access_type = Some(CoreType::<MainAccess>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "MainAccess" => {
                    align_core_enum(e);
                    if main_access_type.is_some() {
                        return Err(input.error("Duplicate MainAccess type"));
                    }
                    main_access_type = Some(CoreType::<MainAccess>::Enum(e.clone(), PhantomData));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            _phantom_data: PhantomData,
            input: input_type,
            state: state_type,
            output: output_type,
            error: error_type,
            main_access: main_access_type,
            render_access: None,
        })
    }
}

impl Parse for CoreTypes<Render> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;
        let mut render_access_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref s)
                    if s.ident == "State"
                        || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") =>
                {
                    return Err(input.error("State is not allowed in Render stages"));
                }
                syn::Item::Struct(ref mut s) if s.ident == "RenderAccess" => {
                    align_core_struct(s);
                    if render_access_type.is_some() {
                        return Err(input.error("Duplicate RenderAccess type"));
                    }
                    render_access_type =
                        Some(CoreType::<RenderAccess>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "RenderAccess" => {
                    align_core_enum(e);
                    if render_access_type.is_some() {
                        return Err(input.error("Duplicate RenderAccess type"));
                    }
                    render_access_type =
                        Some(CoreType::<RenderAccess>::Enum(e.clone(), PhantomData));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            _phantom_data: PhantomData,
            input: input_type,
            state: None,
            output: output_type,
            error: error_type,
            main_access: None,
            render_access: render_access_type,
        })
    }
}

impl Parse for CoreTypes<RenderWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut state_type = None;
        let mut output_type = None;
        let mut error_type = None;
        let mut render_access_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "State" => {
                    align_core_struct(s);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "State" => {
                    align_core_enum(e);
                    if state_type.is_some() {
                        return Err(input.error("Duplicate State type"));
                    }
                    state_type = Some(CoreType::<State>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "RenderAccess" => {
                    align_core_struct(s);
                    if render_access_type.is_some() {
                        return Err(input.error("Duplicate RenderAccess type"));
                    }
                    render_access_type =
                        Some(CoreType::<RenderAccess>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "RenderAccess" => {
                    align_core_enum(e);
                    if render_access_type.is_some() {
                        return Err(input.error("Duplicate RenderAccess type"));
                    }
                    render_access_type =
                        Some(CoreType::<RenderAccess>::Enum(e.clone(), PhantomData));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            _phantom_data: PhantomData,
            input: input_type,
            state: state_type,
            output: output_type,
            error: error_type,
            main_access: None,
            render_access: render_access_type,
        })
    }
}

impl Parse for CoreTypes<Async> {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut input_type = None;
        let mut output_type = None;
        let mut error_type = None;

        while !input.is_empty() {
            let mut item: syn::Item = input.parse()?;
            match item {
                syn::Item::Struct(ref mut s) if s.ident == "Input" => {
                    align_core_struct(s);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Input" => {
                    align_core_enum(e);
                    if input_type.is_some() {
                        return Err(input.error("Duplicate Input type"));
                    }
                    input_type = Some(CoreType::<Input>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Output" => {
                    align_core_struct(s);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Output" => {
                    align_core_enum(e);
                    if output_type.is_some() {
                        return Err(input.error("Duplicate Output type"));
                    }
                    output_type = Some(CoreType::<Output>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref mut s) if s.ident == "Error" => {
                    align_core_struct(s);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Struct(s.clone(), PhantomData));
                }
                syn::Item::Enum(ref mut e) if e.ident == "Error" => {
                    align_core_enum(e);
                    if error_type.is_some() {
                        return Err(input.error("Duplicate Error type"));
                    }
                    error_type = Some(CoreType::<Error>::Enum(e.clone(), PhantomData));
                }
                syn::Item::Struct(ref s)
                    if s.ident == "State"
                        || matches!(item, syn::Item::Enum(ref e) if e.ident == "State") =>
                {
                    return Err(input.error("State is not allowed in Async stages"));
                }
                _ => return Err(input.error("Invalid or misplaced core type declaration")),
            }
        }

        Ok(CoreTypes {
            _phantom_data: PhantomData,
            input: input_type,
            state: None,
            output: output_type,
            error: error_type,
            main_access: None,
            render_access: None,
        })
    }
}

impl CoreTypes<Ecs> {
    pub fn generate_stage_type_dependent_stuff(
        &self,
        module_name: &str,
        workflow_name: &str,
        stage_index: usize,
    ) -> TokenStream {
        quote! {
            static FILL_WORKFLOW_STAGE_BUFFER_SENDER: std::sync::OnceLock<FillWorkflowStageEcsBufferEventSender> = std::sync::OnceLock::new();

            pub fn initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageEcsBufferEventReceiver {
                let (fill_workflow_stage_buffer_sender, fill_workflow_stage_buffer_receiver) = crossbeam_channel::unbounded();
                let fill_workflow_stage_buffer_sender = FillWorkflowStageEcsBufferEventSender {
                    module_name: #module_name,
                    workflow_name: #workflow_name,
                    stage_index: #stage_index,
                    sender: fill_workflow_stage_buffer_sender
                };
                let fill_workflow_stage_buffer_receiver = FillWorkflowStageEcsBufferEventReceiver(fill_workflow_stage_buffer_receiver);

                FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .set(fill_workflow_stage_buffer_sender)
                    .expect("Fill workflow stage buffer sender already initialized!");

                fill_workflow_stage_buffer_receiver
            }

            pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageEcsBufferEventSender {
                let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .get()
                    .expect("Fill workflow stage buffer sender not initialized!");
                let sender: Box<dyn crate::DynFillWorkflowStageEcsBufferEventSender> = dyn_clone::clone_box(sender);

                if let Some(sender) = sender.as_any_ref().downcast_ref::<FillWorkflowStageEcsBufferEventSender>() {
                    sender.clone()
                } else {
                    panic!("Sender was not the expected concrete type!");
                }
            }

            pub struct FillWorkflowStageEcsBufferEvent {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                stage: crate::workflow::stage::StageEcs,
                stage_data: Option<Box<dyn std::any::Any + Send + Sync>>,
            }
            impl crate::FillWorkflowStageEcsBufferEventMarker for FillWorkflowStageEcsBufferEvent {
                fn as_any(self) -> Box<dyn std::any::Any> {
                    Box::new(self)
                }
            }

            #[derive(Resource)]
            pub struct FillWorkflowStageEcsBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageEcsBufferEvent>);

            #[derive(Clone, Debug)]
            pub struct FillWorkflowStageEcsBufferEventSender {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                sender: crossbeam_channel::Sender<FillWorkflowStageEcsBufferEvent>
            }
            impl crate::DynFillWorkflowStageEcsBufferEventSender for FillWorkflowStageEcsBufferEventSender {
                fn module_name(&self) -> &'static str {
                    self.module_name
                }

                fn workflow_name(&self) -> &'static str {
                    self.workflow_name
                }

                fn stage_index(&self) -> usize {
                    self.stage_index
                }

                fn send(&self, event: Box<dyn crate::FillWorkflowStageEcsBufferEventMarker>) {
                    let event = event.as_any().downcast::<FillWorkflowStageEcsBufferEvent>().expect("Downcast failed! Reason for type-mismatch unknown");
                    self.sender.send(*event);
                }

                fn as_any_ref(&self) -> &dyn std::any::Any {
                    self
                }
            }
        }
    }
}

impl CoreTypes<Render> {
    pub fn generate_stage_type_dependent_stuff(
        &self,
        module_name: &str,
        workflow_name: &str,
        stage_index: usize,
    ) -> TokenStream {
        quote! {
            static FILL_WORKFLOW_STAGE_BUFFER_SENDER: std::sync::OnceLock<FillWorkflowStageRenderBufferEventSender> = std::sync::OnceLock::new();

            pub fn initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageRenderBufferEventReceiver {
                let (fill_workflow_stage_buffer_sender, fill_workflow_stage_buffer_receiver) = crossbeam_channel::unbounded();
                let fill_workflow_stage_buffer_sender = FillWorkflowStageRenderBufferEventSender {
                    module_name: #module_name,
                    workflow_name: #workflow_name,
                    stage_index: #stage_index,
                    sender: fill_workflow_stage_buffer_sender
                };
                let fill_workflow_stage_buffer_receiver = FillWorkflowStageRenderBufferEventReceiver(fill_workflow_stage_buffer_receiver);

                FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .set(fill_workflow_stage_buffer_sender)
                    .expect("Fill workflow stage buffer sender already initialized!");

                fill_workflow_stage_buffer_receiver
            }

            pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageRenderBufferEventSender {
                let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .get()
                    .expect("Fill workflow stage buffer sender not initialized!");
                let sender: Box<dyn crate::DynFillWorkflowStageRenderBufferEventSender> = dyn_clone::clone_box(sender);

                if let Some(sender) = sender.as_any_ref().downcast_ref::<FillWorkflowStageRenderBufferEventSender>() {
                    sender.clone()
                } else {
                    panic!("Sender was not the expected concrete type!");
                }
            }

            pub struct FillWorkflowStageRenderBufferEvent {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                stage: crate::workflow::stage::StageRender,
                stage_data: Option<Box<dyn std::any::Any + Send + Sync>>,
            }
            impl crate::FillWorkflowStageRenderBufferEventMarker for FillWorkflowStageRenderBufferEvent {
                fn as_any(self) -> Box<dyn std::any::Any> {
                    Box::new(self)
                }
            }

            #[derive(Resource)]
            pub struct FillWorkflowStageRenderBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageRenderBufferEvent>);

            #[derive(Clone, Debug)]
            pub struct FillWorkflowStageRenderBufferEventSender {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                sender: crossbeam_channel::Sender<FillWorkflowStageRenderBufferEvent>
            }
            impl crate::DynFillWorkflowStageRenderBufferEventSender for FillWorkflowStageRenderBufferEventSender {
                fn module_name(&self) -> &'static str {
                    self.module_name
                }

                fn workflow_name(&self) -> &'static str {
                    self.workflow_name
                }

                fn stage_index(&self) -> usize {
                    self.stage_index
                }

                fn send(&self, event: Box<dyn crate::FillWorkflowStageRenderBufferEventMarker>) {
                    let event = event.as_any().downcast::<FillWorkflowStageRenderBufferEvent>().expect("Downcast failed! Reason for type-mismatch unknown");
                    self.sender.send(*event);
                }

                fn as_any_ref(&self) -> &dyn std::any::Any {
                    self
                }
            }
        }
    }
}

impl CoreTypes<Async> {
    pub fn generate_stage_type_dependent_stuff(
        &self,
        module_name: &str,
        workflow_name: &str,
        stage_index: usize,
    ) -> TokenStream {
        quote! {
            static FILL_WORKFLOW_STAGE_BUFFER_SENDER: std::sync::OnceLock<FillWorkflowStageAsyncBufferEventSender> = std::sync::OnceLock::new();

            pub fn initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageAsyncBufferEventReceiver {
                let (fill_workflow_stage_buffer_sender, fill_workflow_stage_buffer_receiver) = crossbeam_channel::unbounded();
                let fill_workflow_stage_buffer_sender = FillWorkflowStageAsyncBufferEventSender {
                    module_name: #module_name,
                    workflow_name: #workflow_name,
                    stage_index: #stage_index,
                    sender: fill_workflow_stage_buffer_sender
                };
                let fill_workflow_stage_buffer_receiver = FillWorkflowStageAsyncBufferEventReceiver(fill_workflow_stage_buffer_receiver);

                FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .set(fill_workflow_stage_buffer_sender)
                    .expect("Fill workflow stage buffer sender already initialized!");

                fill_workflow_stage_buffer_receiver
            }

            pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageAsyncBufferEventSender {
                let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .get()
                    .expect("Fill workflow stage buffer sender not initialized!");
                let sender: Box<dyn crate::DynFillWorkflowStageAsyncBufferEventSender> = dyn_clone::clone_box(sender);

                if let Some(sender) = sender.as_any_ref().downcast_ref::<FillWorkflowStageAsyncBufferEventSender>() {
                    sender.clone()
                } else {
                    panic!("Sender was not the expected concrete type!");
                }
            }

            pub struct FillWorkflowStageAsyncBufferEvent {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                stage: crate::workflow::stage::StageAsync,
                stage_data: Option<Box<dyn std::any::Any + Send + Sync>>,
            }
            impl crate::FillWorkflowStageAsyncBufferEventMarker for FillWorkflowStageAsyncBufferEvent {
                fn as_any(self) -> Box<dyn std::any::Any> {
                    Box::new(self)
                }
            }

            #[derive(Resource)]
            pub struct FillWorkflowStageAsyncBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageAsyncBufferEvent>);

            #[derive(Clone, Debug)]
            pub struct FillWorkflowStageAsyncBufferEventSender {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                sender: crossbeam_channel::Sender<FillWorkflowStageAsyncBufferEvent>
            }
            impl crate::DynFillWorkflowStageAsyncBufferEventSender for FillWorkflowStageAsyncBufferEventSender {
                fn module_name(&self) -> &'static str {
                    self.module_name
                }

                fn workflow_name(&self) -> &'static str {
                    self.workflow_name
                }

                fn stage_index(&self) -> usize {
                    self.stage_index
                }

                fn send(&self, event: Box<dyn crate::FillWorkflowStageAsyncBufferEventMarker>) {
                    let event = event.as_any().downcast::<FillWorkflowStageAsyncBufferEvent>().expect("Downcast failed! Reason for type-mismatch unknown");
                    self.sender.send(*event);
                }

                fn as_any_ref(&self) -> &dyn std::any::Any {
                    self
                }
            }
        }
    }
}

impl CoreTypes<EcsWhile> {
    pub fn generate_stage_type_dependent_stuff(
        &self,
        module_name: &str,
        workflow_name: &str,
        stage_index: usize,
    ) -> TokenStream {
        quote! {
            static FILL_WORKFLOW_STAGE_BUFFER_SENDER: std::sync::OnceLock<FillWorkflowStageEcsWhileBufferEventSender> = std::sync::OnceLock::new();

            pub fn initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageEcsWhileBufferEventReceiver {
                let (fill_workflow_stage_buffer_sender, fill_workflow_stage_buffer_receiver) = crossbeam_channel::unbounded();
                let fill_workflow_stage_buffer_sender = FillWorkflowStageEcsWhileBufferEventSender {
                    module_name: #module_name,
                    workflow_name: #workflow_name,
                    stage_index: #stage_index,
                    sender: fill_workflow_stage_buffer_sender
                };
                let fill_workflow_stage_buffer_receiver = FillWorkflowStageEcsWhileBufferEventReceiver(fill_workflow_stage_buffer_receiver);

                FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .set(fill_workflow_stage_buffer_sender)
                    .expect("Fill workflow stage buffer sender already initialized!");

                fill_workflow_stage_buffer_receiver
            }

            pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageEcsWhileBufferEventSender {
                let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .get()
                    .expect("Fill workflow stage buffer sender not initialized!");
                let sender: Box<dyn crate::DynFillWorkflowStageEcsWhileBufferEventSender> = dyn_clone::clone_box(sender);

                if let Some(sender) = sender.as_any_ref().downcast_ref::<FillWorkflowStageEcsWhileBufferEventSender>() {
                    sender.clone()
                } else {
                    panic!("Sender was not the expected concrete type!");
                }
            }

            pub struct FillWorkflowStageEcsWhileBufferEvent {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                stage: crate::workflow::stage::StageEcsWhile,
                stage_data: Option<Box<dyn std::any::Any + Send + Sync>>,
            }
            impl crate::FillWorkflowStageEcsWhileBufferEventMarker for FillWorkflowStageEcsWhileBufferEvent {
                fn as_any(self) -> Box<dyn std::any::Any> {
                    Box::new(self)
                }
            }

            #[derive(Resource)]
            pub struct FillWorkflowStageEcsWhileBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageEcsWhileBufferEvent>);

            #[derive(Clone, Debug)]
            pub struct FillWorkflowStageEcsWhileBufferEventSender {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                sender: crossbeam_channel::Sender<FillWorkflowStageEcsWhileBufferEvent>
            }
            impl crate::DynFillWorkflowStageEcsWhileBufferEventSender for FillWorkflowStageEcsWhileBufferEventSender {
                fn module_name(&self) -> &'static str {
                    self.module_name
                }

                fn workflow_name(&self) -> &'static str {
                    self.workflow_name
                }

                fn stage_index(&self) -> usize {
                    self.stage_index
                }

                fn send(&self, event: Box<dyn crate::FillWorkflowStageEcsWhileBufferEventMarker>) {
                    let event = event.as_any().downcast::<FillWorkflowStageEcsWhileBufferEvent>().expect("Downcast failed! Reason for type-mismatch unknown");
                    self.sender.send(*event);
                }

                fn as_any_ref(&self) -> &dyn std::any::Any {
                    self
                }
            }
        }
    }
}

impl CoreTypes<RenderWhile> {
    pub fn generate_stage_type_dependent_stuff(
        &self,
        module_name: &str,
        workflow_name: &str,
        stage_index: usize,
    ) -> TokenStream {
        quote! {
            static FILL_WORKFLOW_STAGE_BUFFER_SENDER: std::sync::OnceLock<FillWorkflowStageRenderWhileBufferEventSender> = std::sync::OnceLock::new();

            pub fn initialize_fill_workflow_stage_buffer_channel() -> FillWorkflowStageRenderWhileBufferEventReceiver {
                let (fill_workflow_stage_buffer_sender, fill_workflow_stage_buffer_receiver) = crossbeam_channel::unbounded();
                let fill_workflow_stage_buffer_sender = FillWorkflowStageRenderWhileBufferEventSender {
                    module_name: #module_name,
                    workflow_name: #workflow_name,
                    stage_index: #stage_index,
                    sender: fill_workflow_stage_buffer_sender
                };
                let fill_workflow_stage_buffer_receiver = FillWorkflowStageRenderWhileBufferEventReceiver(fill_workflow_stage_buffer_receiver);

                FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .set(fill_workflow_stage_buffer_sender)
                    .expect("Fill workflow stage buffer sender already initialized!");

                fill_workflow_stage_buffer_receiver
            }

            pub fn get_fill_workflow_stage_buffer_sender() -> FillWorkflowStageRenderWhileBufferEventSender {
                let sender = FILL_WORKFLOW_STAGE_BUFFER_SENDER
                    .get()
                    .expect("Fill workflow stage buffer sender not initialized!");
                let sender: Box<dyn crate::DynFillWorkflowStageRenderWhileBufferEventSender> = dyn_clone::clone_box(sender);

                if let Some(sender) = sender.as_any_ref().downcast_ref::<FillWorkflowStageRenderWhileBufferEventSender>() {
                    sender.clone()
                } else {
                    panic!("Sender was not the expected concrete type!");
                }
            }

            pub struct FillWorkflowStageRenderWhileBufferEvent {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                stage: crate::workflow::stage::StageRenderWhile,
                stage_data: Option<Box<dyn std::any::Any + Send + Sync>>,
            }
            impl crate::FillWorkflowStageRenderWhileBufferEventMarker for FillWorkflowStageRenderWhileBufferEvent {
                fn as_any(self) -> Box<dyn std::any::Any> {
                    Box::new(self)
                }
            }

            #[derive(Resource)]
            pub struct FillWorkflowStageRenderWhileBufferEventReceiver(pub crossbeam_channel::Receiver<FillWorkflowStageRenderWhileBufferEvent>);

            #[derive(Clone, Debug)]
            pub struct FillWorkflowStageRenderWhileBufferEventSender {
                module_name: &'static str,
                workflow_name: &'static str,
                stage_index: usize,
                sender: crossbeam_channel::Sender<FillWorkflowStageRenderWhileBufferEvent>
            }
            impl crate::DynFillWorkflowStageRenderWhileBufferEventSender for FillWorkflowStageRenderWhileBufferEventSender {
                fn module_name(&self) -> &'static str {
                    self.module_name
                }

                fn workflow_name(&self) -> &'static str {
                    self.workflow_name
                }

                fn stage_index(&self) -> usize {
                    self.stage_index
                }

                fn send(&self, event: Box<dyn crate::FillWorkflowStageRenderWhileBufferEventMarker>) {
                    let event = event.as_any().downcast::<FillWorkflowStageRenderWhileBufferEvent>().expect("Downcast failed! Reason for type-mismatch unknown");
                    self.sender.send(*event);
                }

                fn as_any_ref(&self) -> &dyn std::any::Any {
                    self
                }
            }
        }
    }
}

impl<T> CoreTypes<T> {
    pub fn generate(&self, stage_type_dependent_stuff: TokenStream) -> TokenStream {
        let input = self.input.as_ref().map(|t| t.generate());
        let state = self.state.as_ref().map(|t| t.generate());
        let output = self.output.as_ref().map(|t| t.generate());
        let error = self.error.as_ref().map(|t| t.generate());
        let main_access = self.main_access.as_ref().map(|t| t.generate());
        let render_access = self.render_access.as_ref().map(|t| t.generate());

        let imports = if self.error.is_some() {
            Some(quote! {
                use thiserror::Error;
            })
        } else {
            None
        };

        quote! {
            #imports

            #input
            #state
            #output
            #error
            #main_access
            #render_access

            #[derive(Resource, Default)]
            pub enum StageBuffer {
                #[default]
                None,
                Some {
                    module_name: &'static str,
                    workflow_name: &'static str,
                    stage_index: usize,
                    stage: crate::workflow::stage::Stage,
                    stage_data: Option<Box<dyn std::any::Any + Send + Sync>>,
                }
            }
            impl StageBuffer {
                pub fn fill(
                    &mut self,
                    module_name: &'static str,
                    workflow_name: &'static str,
                    stage_index: usize,
                    stage: crate::workflow::stage::Stage,
                    stage_data: Option<Box<dyn std::any::Any + Send + Sync>>,
                ) {
                    match std::mem::take(self) {
                        StageBuffer::None => {
                            *self = StageBuffer::Some {
                                module_name,
                                workflow_name,
                                stage_index,
                                stage,
                                stage_data,
                            }
                        },
                        StageBuffer::Some { .. } => unreachable!("Stage buffer is not empty")
                    }
                }

                pub fn empty(
                    &mut self,
                ) -> (
                    &'static str,
                    &'static str,
                    usize,
                    crate::workflow::stage::Stage,
                    Option<Box<dyn std::any::Any + Send + Sync>>,
                ) {
                    match std::mem::take(self) {
                        StageBuffer::None => {
                            unreachable!("Stage buffer is not filled");
                        }
                        StageBuffer::Some {
                            module_name,
                            workflow_name,
                            stage_index,
                            stage,
                            stage_data,
                        } => {
                            (
                                module_name,
                                workflow_name,
                                stage_index,
                                stage,
                                stage_data,
                            )
                        }
                    }
                }

                pub fn is_empty(&self) -> bool {
                    matches!(self, StageBuffer::None)
                }
            }

            #stage_type_dependent_stuff
        }
    }

    pub fn has_input(&self) -> bool {
        self.input.is_some()
    }

    pub fn has_output(&self) -> bool {
        self.output.is_some()
    }

    pub fn has_error(&self) -> bool {
        self.error.is_some()
    }

    pub fn get_signature(&self) -> StageSignature {
        let has_input = self.has_input();
        let has_output = self.has_output();
        let has_error = self.has_error();

        match (has_input, has_output, has_error) {
            (false, false, false) => StageSignature::None,
            (false, false, true) => StageSignature::E,
            (false, true, false) => StageSignature::O,
            (false, true, true) => StageSignature::OE,
            (true, false, false) => StageSignature::I,
            (true, false, true) => StageSignature::IE,
            (true, true, false) => StageSignature::IO,
            (true, true, true) => StageSignature::IOE,
        }
    }
}
