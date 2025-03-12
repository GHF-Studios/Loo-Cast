pub(in super) mod core_function;
pub(in super) mod core_type;
pub(in super) mod stage;
pub(in super) mod use_statement;
pub(in super) mod user_item;

pub(in super) mod kw {
    syn::custom_keyword!(Input);
    syn::custom_keyword!(State);
    syn::custom_keyword!(Output);
    syn::custom_keyword!(Error);
    syn::custom_keyword!(Result);
    syn::custom_keyword!(Outcome);
    syn::custom_keyword!(input);
    syn::custom_keyword!(state);
    syn::custom_keyword!(output);
    syn::custom_keyword!(error);
    syn::custom_keyword!(world);
    syn::custom_keyword!(name);
    syn::custom_keyword!(workflows);
    syn::custom_keyword!(user_imports);
    syn::custom_keyword!(user_items);
    syn::custom_keyword!(stages);
    syn::custom_keyword!(core_types);
    syn::custom_keyword!(core_functions);
}

use syn::{parse::Parse, parse_str, Ident, Token, braced, bracketed, Result, LitStr};
use quote::quote;
use proc_macro2::TokenStream;
use heck::ToSnakeCase;
use stage::Stages;
use use_statement::UseStatements;
use user_item::UserItems;

pub struct WorkflowModule {
    pub name: Ident,
    pub workflows: Vec<Workflow>,
}

impl Parse for WorkflowModule {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let _: kw::name = input.parse()?;
        input.parse::<Token![:]>()?;
        let name: LitStr = input.parse()?; 
        let name = Ident::new(&name.value(), name.span());

        input.parse::<Token![,]>()?;

        let _: kw::workflows = input.parse()?;
        input.parse::<Token![:]>()?;
        let content;
        bracketed!(content in input);

        let mut workflows = Vec::new();
        while !content.is_empty() {
            workflows.push(content.parse()?);
        }

        Ok(WorkflowModule { name, workflows })
    }
}

impl WorkflowModule {
    pub fn generate(self) -> TokenStream {
        let module_ident = &self.name;
        let module_name = module_ident.to_string();
        let module_ident = Ident::new(module_name.as_str().to_snake_case().as_str(), module_ident.span());
        let (workflow_modules, workflow_data): (Vec<_>, Vec<_>) = self.workflows.into_iter().map(|w| w.generate(module_ident.clone())).unzip();

        let workflow_literals = workflow_data.into_iter().map(|(signature, ident)| match (signature, ident) {
            (WorkflowSignature::None, ident) => quote! { #ident::Type::create_workflow() },
            (WorkflowSignature::E, ident) => quote! { #ident::TypeE::create_workflow() },
            (WorkflowSignature::O, ident) => quote! { #ident::TypeO::create_workflow() },
            (WorkflowSignature::OE, ident) => quote! { #ident::TypeOE::create_workflow() },
            (WorkflowSignature::I, ident) => quote! { #ident::TypeI::create_workflow() },
            (WorkflowSignature::IE, ident) => quote! { #ident::TypeIE::create_workflow() },
            (WorkflowSignature::IO, ident) => quote! { #ident::TypeIO::create_workflow() },
            (WorkflowSignature::IOE, ident) => quote! { #ident::TypeIOE::create_workflow() },
        }).collect::<Vec<_>>();

        quote! {
            pub mod #module_ident {
                pub const NAME: &str = stringify!(#module_name);

                pub fn register_workflow_type_module(workflow_type_module_registry: &mut crate::workflow::resources::WorkflowTypeModuleRegistry) {
                    workflow_type_module_registry.register(
                        crate::workflow::types::WorkflowTypeModule {
                            name: stringify!(#module_name),
                            workflow_types: vec![
                                #(#workflow_literals),*
                            ],
                        }
                    );
                }
                
                #(#workflow_modules)*
            }
        }
    }
}

pub enum WorkflowSignature {
    None,
    E,
    O,
    OE,
    I,
    IE,
    IO,
    IOE,
}
pub struct Workflow {
    pub name: Ident,
    pub signature: WorkflowSignature,              
    pub user_imports: UseStatements,  
    pub user_items: UserItems,      
    pub stages: Stages,
}

impl Parse for Workflow {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?; 
        
        let content;
        braced!(content in input);

        let _: kw::user_imports = content.parse()?;
        content.parse::<Token![:]>()?;
        let user_imports_content;
        braced!(user_imports_content in content);
        let user_imports: UseStatements = user_imports_content.parse()?;

        content.parse::<Token![,]>()?;

        let _: kw::user_items = content.parse()?;
        content.parse::<Token![:]>()?;
        let user_items_content;
        braced!(user_items_content in content);
        let user_items: UserItems = user_items_content.parse()?; 

        content.parse::<Token![,]>()?;

        let _: kw::stages = content.parse()?;
        content.parse::<Token![:]>()?;
        let stages_content;
        bracketed!(stages_content in content);
        let stages: Stages = stages_content.parse()?;
        
        let lookahead = content.lookahead1();
        if lookahead.peek(Token![,]) {
            let _ = content.parse::<Token![,]>()?;
        }

        let signature = {
            let stages = &stages.0;
            let (has_input, has_output, has_error) = if stages.len() == 1 {
                let only_stage = stages.first().unwrap();

                (only_stage.has_input(), only_stage.has_output(), only_stage.has_error())
            } else {
                let first_stage = stages.first().unwrap();
                let last_stage = stages.last().unwrap();

                (first_stage.has_input(), last_stage.has_output(), stages.iter().any(|s| s.has_error()))
            };

            match (has_input, has_output, has_error) {
                (false, false, false) => WorkflowSignature::None,
                (false, false, true) => WorkflowSignature::E,
                (false, true, false) => WorkflowSignature::O,
                (false, true, true) => WorkflowSignature::OE,
                (true, false, false) => WorkflowSignature::I,
                (true, false, true) => WorkflowSignature::IE,
                (true, true, false) => WorkflowSignature::IO,
                (true, true, true) => WorkflowSignature::IOE,
            }
        };

        Ok(Workflow { name, signature, user_imports, user_items, stages })
    }
}

trait IteratorExt: Iterator {
    fn unzip3<A, B, C, IA, IB, IC>(self) -> (IA, IB, IC)
    where
        Self: Sized + Iterator<Item = (A, B, C)>,
        IA: FromIterator<A>,
        IB: FromIterator<B>,
        IC: FromIterator<C>,
    {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();

        for (x, y, z) in self {
            a.push(x);
            b.push(y);
            c.push(z);
        }

        (a.into_iter().collect(), b.into_iter().collect(), c.into_iter().collect())
    }
}
impl<T: ?Sized> IteratorExt for T where T: Iterator {}

impl Workflow {
    pub fn generate(self, workflow_module_ident: Ident) -> (TokenStream, (WorkflowSignature, Ident)) {
        let workflow_ident = &self.name;
        let workflow_name = workflow_ident.to_string();
        let workflow_ident = Ident::new(workflow_name.as_str().to_snake_case().as_str(), workflow_ident.span());

        let workflow_module = match self.signature {
            WorkflowSignature::None => {
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();
                
                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run() {
                            crate::workflow::functions::run_workflow::<Type>().await
                        }
                        
                        pub struct Type;
                        impl crate::workflow::traits::WorkflowType for Type {
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl Type {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
            WorkflowSignature::E => {
                let error_enum = {
                    let workflow_errors = self.stages.0.iter().filter_map(|s| {
                        if !s.has_error() {
                            return None;
                        }
    
                        let stage_ident = s.name();
                        let stage_name_pascal_case = stage_ident.to_string();
                        let stage_name_snake_case = stage_name_pascal_case.to_snake_case();
                        let stage_error_name: TokenStream = parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(format!("self::stages::{}::core_types::Error", stage_name_snake_case).as_str()).unwrap();
                        
                        Some(quote! { 
                            #stage_error_name(#stage_error_path)
                        })
                    });

                    if self.stages.0.iter().any(|s| s.has_error()) {
                        quote! { 
                            #[derive(std::fmt::Debug, thiserror::Error)]
                            pub enum Error {
                                #(#workflow_errors),*
                            }
                            impl std::fmt::Display for Error {
                                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                    write!(f, "{:?}", self)
                                }
                            }
                        }
                    } else {
                        quote! {}
                    }
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run() -> Result<(), <TypeE as crate::workflow::traits::WorkflowTypeE>::Error> {
                            crate::workflow::functions::run_workflow_e::<Type>().await
                        }

                        #error_enum
                        
                        pub struct TypeE;
                        impl crate::workflow::traits::WorkflowTypeE for TypeE {
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl TypeE {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
            WorkflowSignature::O => {
                let last_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident = Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    first_stage_ident
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run() -> <TypeO as crate::workflow::traits::WorkflowTypeO>::Output {
                            crate::workflow::functions::run_workflow_o::<TypeO>().await
                        }
                        
                        pub struct TypeO;
                        impl crate::workflow::traits::WorkflowTypeO for TypeO {
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl TypeO {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
            WorkflowSignature::OE => {
                let last_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident = Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    first_stage_ident
                };
                let error_enum = {
                    let workflow_errors = self.stages.0.iter().filter_map(|s| {
                        if !s.has_error() {
                            return None;
                        }
    
                        let stage_ident = s.name();
                        let stage_name_pascal_case = stage_ident.to_string();
                        let stage_name_snake_case = stage_name_pascal_case.to_snake_case();
                        let stage_error_name: TokenStream = parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(format!("self::stages::{}::core_types::Error", stage_name_snake_case).as_str()).unwrap();
                        
                        Some(quote! { 
                            #stage_error_name(#stage_error_path)
                        })
                    });

                    if self.stages.0.iter().any(|s| s.has_error()) {
                        quote! { 
                            #[derive(std::fmt::Debug, thiserror::Error)]
                            pub enum Error {
                                #(#workflow_errors),*
                            }
                            impl std::fmt::Display for Error {
                                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                    write!(f, "{:?}", self)
                                }
                            }
                        }
                    } else {
                        quote! {}
                    }
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run() -> Result<<TypeOE as crate::workflow::traits::WorkflowTypeOE>::Output, <TypeOE as crate::workflow::traits::WorkflowTypeOE>::Error> {
                            crate::workflow::functions::run_workflow_oe::<TypeOE>().await
                        }
                        
                        #error_enum

                        pub struct TypeOE;
                        impl crate::workflow::traits::WorkflowTypeOE for TypeOE {
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl TypeOE {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
            WorkflowSignature::I => {
                let first_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident = Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    first_stage_ident
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run(input: <TypeI as crate::workflow::traits::WorkflowTypeI>::Input) -> () {
                            crate::workflow::functions::run_workflow_i::<TypeI>(input).await
                        }
                        
                        pub struct TypeI;
                        impl crate::workflow::traits::WorkflowTypeI for TypeI {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl TypeI {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
            WorkflowSignature::IE => {
                let first_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident = Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    first_stage_ident
                };
                let error_enum = {
                    let workflow_errors = self.stages.0.iter().filter_map(|s| {
                        if !s.has_error() {
                            return None;
                        }
    
                        let stage_ident = s.name();
                        let stage_name_pascal_case = stage_ident.to_string();
                        let stage_name_snake_case = stage_name_pascal_case.to_snake_case();
                        let stage_error_name: TokenStream = parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(format!("self::stages::{}::core_types::Error", stage_name_snake_case).as_str()).unwrap();
                        
                        Some(quote! { 
                            #stage_error_name(#stage_error_path)
                        })
                    });

                    if self.stages.0.iter().any(|s| s.has_error()) {
                        quote! { 
                            #[derive(std::fmt::Debug, thiserror::Error)]
                            pub enum Error {
                                #(#workflow_errors),*
                            }
                            impl std::fmt::Display for Error {
                                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                    write!(f, "{:?}", self)
                                }
                            }
                        }
                    } else {
                        quote! {}
                    }
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();
        
                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run(input: <TypeIE as crate::workflow::traits::WorkflowTypeIE>::Input) -> Result<(), <TypeIE as crate::workflow::traits::WorkflowTypeIE>::Error> {
                            crate::workflow::functions::run_workflow_ie::<TypeIE>(input).await
                        }
                        
                        #error_enum

                        pub struct TypeIE;
                        impl crate::workflow::traits::WorkflowTypeIE for TypeIE {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl TypeIE {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
            WorkflowSignature::IO => {
                let (first_stage_ident, last_stage_ident) = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let last_stage_ident = self.stages.0.last().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let last_stage_name = last_stage_ident.to_string().to_snake_case();
                    let first_stage_ident = Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    let last_stage_ident = Ident::new(last_stage_name.as_str(), last_stage_ident.span());
                    (first_stage_ident, last_stage_ident)
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run(input: <TypeIO as crate::workflow::traits::WorkflowTypeIO>::Input) -> <TypeIO as crate::workflow::traits::WorkflowTypeIO>::Output {
                            crate::workflow::functions::run_workflow_io::<TypeIO>(input).await
                        }
                        
                        pub struct TypeIO;
                        impl crate::workflow::traits::WorkflowTypeIO for TypeIO {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl TypeIO {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
            WorkflowSignature::IOE => {
                let (first_stage_ident, last_stage_ident) = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let last_stage_ident = self.stages.0.last().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let last_stage_name = last_stage_ident.to_string().to_snake_case();
                    let first_stage_ident = Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    let last_stage_ident = Ident::new(last_stage_name.as_str(), last_stage_ident.span());
                    (first_stage_ident, last_stage_ident)
                };
                let error_enum = {
                    let workflow_errors = self.stages.0.iter().filter_map(|s| {
                        if !s.has_error() {
                            return None;
                        }
    
                        let stage_ident = s.name();
                        let stage_name_pascal_case = stage_ident.to_string();
                        let stage_name_snake_case = stage_name_pascal_case.to_snake_case();
                        let stage_error_name: TokenStream = parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(format!("self::stages::{}::core_types::Error", stage_name_snake_case).as_str()).unwrap();
                        
                        Some(quote! { 
                            #stage_error_name(#stage_error_path)
                        })
                    });

                    if self.stages.0.iter().any(|s| s.has_error()) {
                        quote! { 
                            #[derive(std::fmt::Debug, thiserror::Error)]
                            pub enum Error {
                                #(#workflow_errors),*
                            }
                            impl std::fmt::Display for Error {
                                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                                    write!(f, "{:?}", self)
                                }
                            }
                        }
                    } else {
                        quote! {}
                    }
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_output_type_paths,
                    stage_input_type_paths 
                ): (Vec<_>, Vec<_>) = self.stages.0.iter().map(|stage| (
                    stage.get_output_type_path(workflow_module_ident.clone(), workflow_ident.clone()),
                    stage.get_input_type_path(workflow_module_ident.clone(), workflow_ident.clone())
                )).unzip();
                let (stage_modules, stage_literals, stage_data_type_transmuters): (Vec<_>, Vec<_>, Vec<_>) = self
                    .stages.0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_output_type_path = stage_output_type_paths[index].as_ref();
                        let (next_stage_input_type_path, is_last) = if index < stage_count - 1 {
                            (stage_input_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };
                        
                        stage.generate(
                            this_stage_output_type_path,
                            next_stage_input_type_path,
                            is_last,
                        )
                    })
                    .unzip3();
                let stage_data_type_transmuters: Vec<TokenStream> = stage_data_type_transmuters.into_iter().flatten().collect();
        
                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub async fn run(input: <TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Input) -> Result<<TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Output, <TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Error> {
                            crate::workflow::functions::run_workflow_ioe::<TypeIOE>(input).await
                        }
                        
                        #error_enum

                        pub struct TypeIOE;
                        impl crate::workflow::traits::WorkflowTypeIOE for TypeIOE {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        impl TypeIOE {
                            pub fn create_workflow() -> crate::workflow::types::WorkflowType {
                                crate::workflow::types::WorkflowType {
                                    name: self::NAME,
                                    stages: vec![
                                        #(#stage_literals),*
                                    ],
                                }
                            }

                            pub fn advance_workflow_data_type(data: Option<Box<dyn std::any::Any + Send + Sync>>, new_stage: usize) -> Option<Box<dyn std::any::Any + Send + Sync>> {
                                static STAGE_DATA_TYPE_TRANSMUTERS: once_cell::sync::Lazy<std::sync::Mutex<Vec<Box<dyn FnMut(Option<Box<dyn std::any::Any+Send+Sync>>)->Option<Box<dyn std::any::Any + Send + Sync>> + Send + Sync>>>> = once_cell::sync::Lazy::new(|| {std::sync::Mutex::new(vec![
                                    #(#stage_data_type_transmuters),*
                                ])});

                                STAGE_DATA_TYPE_TRANSMUTERS.lock().expect("Failed to lock mutex")[new_stage](data)
                            }
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stage_modules)*
                        }
                    }
                }
            },
        };

        (workflow_module, (self.signature, workflow_ident))
    }
}
