pub(super) mod core_function;
pub(super) mod core_type;
pub(super) mod stage;
pub(super) mod use_statement;
pub(super) mod user_item;

pub(super) mod kw {
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

use heck::ToSnakeCase;
use proc_macro2::TokenStream;
use quote::quote;
use stage::Stages;
use syn::{braced, bracketed, parse::Parse, parse_str, Ident, LitStr, Path, Result, Token};
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
        let module_ident = Ident::new(
            module_name.as_str().to_snake_case().as_str(),
            module_ident.span(),
        );
        let plugin_name = format!("{}WorkflowsPlugin", module_name);
        let plugin_ident = Ident::new(plugin_name.as_str(), module_ident.span());

        let (workflow_modules, workflow_data, workflow_plugin_addition_literals): (
            Vec<_>,
            Vec<_>,
            Vec<_>,
        ) = self
            .workflows
            .into_iter()
            .map(|w| w.generate(module_ident.clone()))
            .unzip3();

        let workflow_literals = workflow_data
            .into_iter()
            .map(|(signature, ident)| match (signature, ident) {
                (WorkflowSignature::None, ident) => quote! { #ident::Type::create_workflow() },
                (WorkflowSignature::E, ident) => quote! { #ident::TypeE::create_workflow() },
                (WorkflowSignature::O, ident) => quote! { #ident::TypeO::create_workflow() },
                (WorkflowSignature::OE, ident) => quote! { #ident::TypeOE::create_workflow() },
                (WorkflowSignature::I, ident) => quote! { #ident::TypeI::create_workflow() },
                (WorkflowSignature::IE, ident) => quote! { #ident::TypeIE::create_workflow() },
                (WorkflowSignature::IO, ident) => quote! { #ident::TypeIO::create_workflow() },
                (WorkflowSignature::IOE, ident) => quote! { #ident::TypeIOE::create_workflow() },
            })
            .collect::<Vec<_>>();

        let workflow_module_declaration = quote! {
            pub mod #module_ident {
                use bevy::prelude::*;

                pub const NAME: &str = stringify!(#module_name);

                pub struct #plugin_ident;
                impl Plugin for #plugin_ident {
                    fn build(&self, app: &mut App) {
                        app
                            .add_systems(PreStartup, register_workflow_type_module)
                            #(#workflow_plugin_addition_literals)*;
                    }
                }

                fn register_workflow_type_module(mut workflow_type_module_registry: ResMut<crate::workflow::resources::WorkflowTypeModuleRegistry>) {
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
        };

        workflow_module_declaration
    }
}

#[allow(clippy::upper_case_acronyms)]
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

                (
                    only_stage.has_input(),
                    only_stage.has_output(),
                    only_stage.has_error(),
                )
            } else {
                let first_stage = stages.first().unwrap();
                let last_stage = stages.last().unwrap();

                (
                    first_stage.has_input(),
                    last_stage.has_output(),
                    stages.iter().any(|s| s.has_error()),
                )
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

        Ok(Workflow {
            name,
            signature,
            user_imports,
            user_items,
            stages,
        })
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

        (
            a.into_iter().collect(),
            b.into_iter().collect(),
            c.into_iter().collect(),
        )
    }

    fn unzip4<A, B, C, D, IA, IB, IC, ID>(self) -> (IA, IB, IC, ID)
    where
        Self: Sized + Iterator<Item = (A, B, C, D)>,
        IA: FromIterator<A>,
        IB: FromIterator<B>,
        IC: FromIterator<C>,
        ID: FromIterator<D>,
    {
        let mut a = Vec::new();
        let mut b = Vec::new();
        let mut c = Vec::new();
        let mut d = Vec::new();

        for (x, y, z, w) in self {
            a.push(x);
            b.push(y);
            c.push(z);
            d.push(w);
        }

        (
            a.into_iter().collect(),
            b.into_iter().collect(),
            c.into_iter().collect(),
            d.into_iter().collect(),
        )
    }
}
impl<T: ?Sized> IteratorExt for T where T: Iterator {}

impl Workflow {
    pub fn generate(
        self,
        workflow_module_ident: Ident,
    ) -> (TokenStream, (WorkflowSignature, Ident), TokenStream) {
        let workflow_ident = &self.name;
        let workflow_name = workflow_ident.to_string();
        let workflow_ident = Ident::new(
            workflow_name.as_str().to_snake_case().as_str(),
            workflow_ident.span(),
        );
        let workflow_path = format!(
            "crate::{}::workflows::{}::{}",
            workflow_module_ident, workflow_module_ident, workflow_ident
        );
        let workflow_path = Path {
            leading_colon: None,
            segments: workflow_path
                .split("::")
                .map(|s| Ident::new(s, workflow_ident.span()))
                .map(|s| syn::PathSegment {
                    ident: s,
                    arguments: syn::PathArguments::None,
                })
                .collect(),
        };
        let workflow_path = quote! { #workflow_path };
        let workflow_plugin_name = format!("{}WorkflowPlugin", workflow_name);
        let workflow_plugin_ident =
            Ident::new(workflow_plugin_name.as_str(), workflow_ident.span());

        let workflow_stage_systems_registration_literals = {
            let mut workflow_stage_systems_registration_literals = vec![];

            for stage in self.stages.0.iter() {
                let stage_ident = stage.name();
                let workflow_stage_system_name =
                    format!("{}", stage_ident).as_str().to_snake_case();
                let workflow_stage_module_ident =
                    Ident::new(workflow_stage_system_name.as_str(), stage_ident.span());

                workflow_stage_systems_registration_literals.push(quote! {
                    .add_systems(bevy::prelude::Update, stages::#workflow_stage_module_ident::core_functions::poll_ecs_system)
                });
            }

            workflow_stage_systems_registration_literals
        };

        let workflow_plugin_declaration = {
            if workflow_stage_systems_registration_literals.is_empty() {
                quote! {
                    pub(crate) struct #workflow_plugin_ident;
                    impl bevy::prelude::Plugin for #workflow_plugin_ident {
                        fn build(&self, app: &mut bevy::prelude::App) {}
                    }
                }
            } else {
                quote! {
                    pub(crate) struct #workflow_plugin_ident;
                    impl bevy::prelude::Plugin for #workflow_plugin_ident {
                        fn build(&self, app: &mut bevy::prelude::App) {
                            app
                                #(#workflow_stage_systems_registration_literals)*;
                        }
                    }
                }
            }
        };

        let workflow_plugin_addition_literal = {
            quote! {
                .add_plugins(crate::#workflow_module_ident::workflows::#workflow_module_ident::#workflow_ident::#workflow_plugin_ident)
            }
        };

        let workflow_module = match self.signature {
            WorkflowSignature::None => {
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run() {
                            crate::workflow::functions::run_workflow::<Type>().await
                        }

                        #workflow_plugin_declaration

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
            }
            WorkflowSignature::E => {
                let error_enum = {
                    let workflow_errors = self.stages.0.iter().filter_map(|s| {
                        if !s.has_error() {
                            return None;
                        }

                        let stage_ident = s.name();
                        let stage_name_pascal_case = stage_ident.to_string();
                        let stage_name_snake_case = stage_name_pascal_case.to_snake_case();
                        let stage_error_name: TokenStream =
                            parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(
                            format!("self::stages::{}::core_types::Error", stage_name_snake_case)
                                .as_str(),
                        )
                        .unwrap();

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
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run() -> Result<(), <TypeE as crate::workflow::traits::WorkflowTypeE>::Error> {
                            crate::workflow::functions::run_workflow_e::<TypeE>().await
                        }

                        #workflow_plugin_declaration

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
            }
            WorkflowSignature::O => {
                let last_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident =
                        Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    first_stage_ident
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run() -> <TypeO as crate::workflow::traits::WorkflowTypeO>::Output {
                            crate::workflow::functions::run_workflow_o::<TypeO>().await
                        }

                        #workflow_plugin_declaration

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
            }
            WorkflowSignature::OE => {
                let last_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident =
                        Ident::new(first_stage_name.as_str(), first_stage_ident.span());
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
                        let stage_error_name: TokenStream =
                            parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(
                            format!("self::stages::{}::core_types::Error", stage_name_snake_case)
                                .as_str(),
                        )
                        .unwrap();

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
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run() -> Result<<TypeOE as crate::workflow::traits::WorkflowTypeOE>::Output, <TypeOE as crate::workflow::traits::WorkflowTypeOE>::Error> {
                            crate::workflow::functions::run_workflow_oe::<TypeOE>().await
                        }

                        #workflow_plugin_declaration

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
            }
            WorkflowSignature::I => {
                let first_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident =
                        Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    first_stage_ident
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run(input: <TypeI as crate::workflow::traits::WorkflowTypeI>::Input) -> () {
                            crate::workflow::functions::run_workflow_i::<TypeI>(input).await
                        }

                        #workflow_plugin_declaration

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
            }
            WorkflowSignature::IE => {
                let first_stage_ident = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let first_stage_ident =
                        Ident::new(first_stage_name.as_str(), first_stage_ident.span());
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
                        let stage_error_name: TokenStream =
                            parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(
                            format!("self::stages::{}::core_types::Error", stage_name_snake_case)
                                .as_str(),
                        )
                        .unwrap();

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
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run(input: <TypeIE as crate::workflow::traits::WorkflowTypeIE>::Input) -> Result<(), <TypeIE as crate::workflow::traits::WorkflowTypeIE>::Error> {
                            crate::workflow::functions::run_workflow_ie::<TypeIE>(input).await
                        }

                        #workflow_plugin_declaration

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
            }
            WorkflowSignature::IO => {
                let (first_stage_ident, last_stage_ident) = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let last_stage_ident = self.stages.0.last().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let last_stage_name = last_stage_ident.to_string().to_snake_case();
                    let first_stage_ident =
                        Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    let last_stage_ident =
                        Ident::new(last_stage_name.as_str(), last_stage_ident.span());
                    (first_stage_ident, last_stage_ident)
                };
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stage_count = self.stages.0.len();
                let (
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run(input: <TypeIO as crate::workflow::traits::WorkflowTypeIO>::Input) -> <TypeIO as crate::workflow::traits::WorkflowTypeIO>::Output {
                            crate::workflow::functions::run_workflow_io::<TypeIO>(input).await
                        }

                        #workflow_plugin_declaration

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
            }
            WorkflowSignature::IOE => {
                let (first_stage_ident, last_stage_ident) = {
                    let first_stage_ident = self.stages.0.first().unwrap().name();
                    let last_stage_ident = self.stages.0.last().unwrap().name();
                    let first_stage_name = first_stage_ident.to_string().to_snake_case();
                    let last_stage_name = last_stage_ident.to_string().to_snake_case();
                    let first_stage_ident =
                        Ident::new(first_stage_name.as_str(), first_stage_ident.span());
                    let last_stage_ident =
                        Ident::new(last_stage_name.as_str(), last_stage_ident.span());
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
                        let stage_error_name: TokenStream =
                            parse_str(format!("{}Error", stage_name_pascal_case).as_str()).unwrap();
                        let stage_error_path: TokenStream = parse_str(
                            format!("self::stages::{}::core_types::Error", stage_name_snake_case)
                                .as_str(),
                        )
                        .unwrap();

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
                    stage_state_type_paths,
                    stage_out_type_paths,
                    stage_err_type_paths,
                    stage_in_type_paths,
                ): (Vec<_>, Vec<_>, Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .iter()
                    .map(|stage| {
                        (
                            stage.get_state_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_out_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_err_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                            stage.get_in_type_path(
                                workflow_module_ident.clone(),
                                workflow_ident.clone(),
                            ),
                        )
                    })
                    .unzip4();
                let (stage_modules, stage_literals): (Vec<_>, Vec<_>) = self
                    .stages
                    .0
                    .into_iter()
                    .map(|stage| {
                        let index = stage.get_index();
                        let this_stage_state_type_path = stage_state_type_paths[index].as_ref();
                        let this_stage_out_type_path = stage_out_type_paths[index].as_ref();
                        let this_err_type_path = stage_err_type_paths[index].as_ref();
                        let (next_stage_in_type_path, is_last) = if index < stage_count - 1 {
                            (stage_in_type_paths[index + 1].as_ref(), false)
                        } else {
                            (None, true)
                        };

                        stage.generate(
                            &workflow_path,
                            this_stage_state_type_path,
                            this_stage_out_type_path,
                            this_err_type_path,
                            next_stage_in_type_path,
                            is_last,
                        )
                    })
                    .unzip();

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        pub async fn run(input: <TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Input) -> Result<<TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Output, <TypeIOE as crate::workflow::traits::WorkflowTypeIOE>::Error> {
                            crate::workflow::functions::run_workflow_ioe::<TypeIOE>(input).await
                        }

                        #workflow_plugin_declaration

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
            }
        };

        (
            workflow_module,
            (self.signature, workflow_ident),
            workflow_plugin_addition_literal,
        )
    }
}
