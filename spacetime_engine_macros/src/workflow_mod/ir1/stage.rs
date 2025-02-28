use syn::{parse::Parse, Ident, Result, Token, braced, bracketed, parse::ParseStream};
use quote::quote;
use proc_macro2::TokenStream;
use super::core_type::CoreTypes;
use super::core_function::CoreFunctions;

pub struct Ecs;
pub struct EcsWhile;
pub struct Render;
pub struct RenderWhile;
pub struct Async;

pub struct Stages {
    pub stages: Vec<Stage>,
}

pub enum Stage {
    Ecs(TypedStage<Ecs>),
    EcsWhile(TypedStage<EcsWhile>),
    Render(TypedStage<Render>),
    RenderWhile(TypedStage<RenderWhile>),
    Async(TypedStage<Async>),
}

pub struct TypedStage<T> {
    pub name: Ident,
    pub core_types: CoreTypes<T>,
    pub core_functions: CoreFunctions<T>,
}

impl Parse for Stage {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.fork();
        let stage_name: Ident = lookahead.parse()?;

        match stage_name.to_string().as_str() {
            "Ecs" => input.parse().map(Stage::Ecs),
            "EcsWhile" => input.parse().map(Stage::EcsWhile),
            "Render" => input.parse().map(Stage::Render),
            "RenderWhile" => input.parse().map(Stage::RenderWhile),
            "Async" => input.parse().map(Stage::Async),
            _ => Err(input.error("Invalid stage type")),
        }
    }
}

impl Stage {
    pub fn generate(self) -> TokenStream {
        match self {
            Stage::Ecs(stage) => stage.generate(),
            Stage::EcsWhile(stage) => stage.generate(),
            Stage::Render(stage) => stage.generate(),
            Stage::RenderWhile(stage) => stage.generate(),
            Stage::Async(stage) => stage.generate(),
        }
    }
}


impl Parse for TypedStage<Ecs> {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let core_types: CoreTypes<Ecs> = content.parse()?;
        let core_functions: CoreFunctions<Ecs> = content.parse()?;

        Ok(TypedStage { name, core_types, core_functions })
    }
}

impl Parse for TypedStage<EcsWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let core_types: CoreTypes<EcsWhile> = content.parse()?;
        let core_functions: CoreFunctions<EcsWhile> = content.parse()?;

        Ok(TypedStage { name, core_types, core_functions })
    }
}

impl Parse for TypedStage<Render> {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let core_types: CoreTypes<Render> = content.parse()?;
        let core_functions: CoreFunctions<Render> = content.parse()?;

        Ok(TypedStage { name, core_types, core_functions })
    }
}

impl Parse for TypedStage<RenderWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let core_types: CoreTypes<RenderWhile> = content.parse()?;
        let core_functions: CoreFunctions<RenderWhile> = content.parse()?;

        Ok(TypedStage { name, core_types, core_functions })
    }
}

impl Parse for TypedStage<Async> {
    fn parse(input: ParseStream) -> Result<Self> {
        let name: Ident = input.parse()?;
        let content;
        braced!(content in input);

        let core_types: CoreTypes<Async> = content.parse()?;
        
        let core_functions: CoreFunctions<Async> = content.parse()?;

        Ok(TypedStage { name, core_types, core_functions })
    }
}

impl Parse for Stages {
    fn parse(input: ParseStream) -> Result<Self> {
        let content;
        bracketed!(content in input);

        let mut stages = Vec::new();
        while !content.is_empty() {
            stages.push(content.parse()?);

            let lookahead = content.lookahead1();
            if lookahead.peek(Token![,]) {
                let _ = content.parse::<Token![,]>()?;
            }
        }

        Ok(Stages { stages })
    }
}

impl TypedStage<Ecs> {
    pub fn generate(self) -> TokenStream {
        let stage_name = &self.name;
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        quote! {
            pub mod #stage_name {
                pub mod core_types {
                    #core_types
                }

                pub mod core_functions {
                    #core_functions
                }
            }
        }
    }
}

impl TypedStage<EcsWhile> {
    pub fn generate(self) -> TokenStream {
        let stage_name = &self.name;
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        quote! {
            pub mod #stage_name {
                pub mod core_types {
                    #core_types
                }

                pub mod core_functions {
                    #core_functions
                }
            }
        }
    }
}

impl TypedStage<Render> {
    pub fn generate(self) -> TokenStream {
        let stage_name = &self.name;
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        quote! {
            pub mod #stage_name {
                pub mod core_types {
                    #core_types
                }

                pub mod core_functions {
                    #core_functions
                }
            }
        }
    }
}

impl TypedStage<RenderWhile> {
    pub fn generate(self) -> TokenStream {
        let stage_name = &self.name;
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        quote! {
            pub mod #stage_name {
                pub mod core_types {
                    #core_types
                }

                pub mod core_functions {
                    #core_functions
                }
            }
        }
    }
}

impl TypedStage<Async> {
    pub fn generate(self) -> TokenStream {
        let stage_name = &self.name;
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

        quote! {
            pub mod #stage_name {
                pub mod core_types {
                    #core_types
                }

                pub mod core_functions {
                    #core_functions
                }
            }
        }
    }
}