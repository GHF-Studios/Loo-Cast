use syn::{parse::Parse, parse_str, Ident, Token, braced, bracketed, Result, LitStr};
use quote::quote;
use proc_macro2::TokenStream;
use heck::ToSnakeCase;
use super::stage::Stages;
use super::use_statement::UseStatements;
use super::user_item::UserItems;

pub struct WorkflowModule {
    pub name: Ident,
    pub workflows: Vec<Workflow>,
}

impl Parse for WorkflowModule {
    fn parse(input: syn::parse::ParseStream) -> Result<Self> {
        let _: super::kw::name = input.parse()?;
        input.parse::<Token![:]>()?;
        let name: LitStr = input.parse()?; 
        let name = Ident::new(&name.value(), name.span());

        input.parse::<Token![,]>()?;

        let _: super::kw::workflows = input.parse()?;
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
        let workflows = self.workflows.into_iter().map(|w| w.generate());

        quote! {
            pub mod #module_ident {
                pub const NAME: &str = stringify!(#module_name);
                #(#workflows)*
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

        let _: super::kw::user_imports = content.parse()?;
        content.parse::<Token![:]>()?;
        let user_imports_content;
        braced!(user_imports_content in content);
        let user_imports: UseStatements = user_imports_content.parse()?;

        content.parse::<Token![,]>()?;

        let _: super::kw::user_items = content.parse()?;
        content.parse::<Token![:]>()?;
        let user_items_content;
        braced!(user_items_content in content);
        let user_items: UserItems = user_items_content.parse()?; 

        content.parse::<Token![,]>()?;

        let _: super::kw::stages = content.parse()?;
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

impl Workflow {
    pub fn generate(self) -> TokenStream {
        let workflow_ident = &self.name;
        let workflow_name = workflow_ident.to_string();
        let workflow_ident = Ident::new(workflow_name.as_str().to_snake_case().as_str(), workflow_ident.span());

        match self.signature {
            WorkflowSignature::None => {
                let imports = self.user_imports.generate();
                let user_items = self.user_items.generate();
                let stages = self.stages.0.into_iter().map(|s| s.generate());

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub struct Type;
                        impl crate::workflow::traits::WorkflowType for Type {
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
                        }
                    }
                }
            },
            WorkflowSignature::E => {
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
                let stages = self.stages.0.into_iter().map(|s| s.generate());

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);

                        #error_enum
                        
                        pub struct Type;
                        impl crate::workflow::traits::WorkflowTypeE for Type {
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
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
                let stages = self.stages.0.into_iter().map(|s| s.generate());

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub struct Type;
                        impl crate::workflow::traits::WorkflowTypeO for Type {
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
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
                let stages = self.stages.0.into_iter().map(|s| s.generate());

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        #error_enum

                        pub struct Type;
                        impl crate::workflow::traits::WorkflowTypeOE for Type {
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
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
                let stages = self.stages.0.into_iter().map(|s| s.generate());

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub struct Type;
                        impl crate::workflow::traits::WorkflowTypeI for Type {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
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
                let stages = self.stages.0.into_iter().map(|s| s.generate());
        
                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        #error_enum

                        pub struct Type;
                        impl crate::workflow::traits::WorkflowTypeIE for Type {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
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
                let stages = self.stages.0.into_iter().map(|s| s.generate());

                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        pub struct Type;
                        impl crate::workflow::traits::WorkflowTypeIO for Type {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
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
                let stages = self.stages.0.into_iter().map(|s| s.generate());
        
                quote! {
                    pub mod #workflow_ident {
                        pub const NAME: &str = stringify!(#workflow_name);
                        
                        #error_enum

                        pub struct Type;
                        impl crate::workflow::traits::WorkflowTypeIOE for Type {
                            type Input = self::stages::#first_stage_ident::core_types::Input;
                            type Output = self::stages::#last_stage_ident::core_types::Output;
                            type Error = Error;
                        
                            const MODULE_NAME: &'static str = super::NAME;
                            const WORKFLOW_NAME: &'static str = self::NAME;
                        }
                        
                        pub mod workflow_imports {
                            #imports
                        }
        
                        pub mod user_items {
                            use super::workflow_imports::*;
        
                            #user_items
                        }
        
                        pub mod stages {
                            #(#stages)*
                        }
                    }
                }
            },
        }
    }
}
