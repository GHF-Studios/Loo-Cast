use syn::{parse::Parse, Ident, Result, Token, braced, bracketed, parse::ParseStream};
use quote::quote;
use proc_macro2::TokenStream;
use heck::ToSnakeCase;
use super::core_type::CoreTypes;
use super::core_function::CoreFunctions;

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
    pub core_types: CoreTypes<T>,
    pub core_functions: CoreFunctions<T>,
}

impl Parse for Stage {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.fork();

        let _stage_name: Ident = lookahead.parse()?;
        let _: Token![:] = lookahead.parse()?;
        let stage_type: Ident = lookahead.parse()?;

        match stage_type.to_string().as_str() {
            "Ecs" => {
                input.parse::<TypedStage::<Ecs>>().map(Stage::Ecs)
            },
            "EcsWhile" => {
                input.parse::<TypedStage::<EcsWhile>>().map(Stage::EcsWhile)
            },
            "Render" => {
                input.parse::<TypedStage::<Render>>().map(Stage::Render)
            },
            "RenderWhile" => {
                input.parse::<TypedStage::<RenderWhile>>().map(Stage::RenderWhile)
            },
            "Async" => {
                input.parse::<TypedStage::<Async>>().map(Stage::Async)
            },
            _ => Err(input.error("Invalid stage type")),
        }
    }
}

impl Stage {
    pub fn generate(self) -> (TokenStream, TokenStream) {
        match self {
            Stage::Ecs(stage) => stage.generate(),
            Stage::EcsWhile(stage) => stage.generate(),
            Stage::Render(stage) => stage.generate(),
            Stage::RenderWhile(stage) => stage.generate(),
            Stage::Async(stage) => stage.generate(),
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

    pub fn get_type(&self) -> StageType {
        match self {
            Stage::Ecs(_) => StageType::Ecs,
            Stage::EcsWhile(_) => StageType::EcsWhile,
            Stage::Render(_) => StageType::Render,
            Stage::RenderWhile(_) => StageType::RenderWhile,
            Stage::Async(_) => StageType::Async,
        }
    }
}

impl Parse for TypedStage<Ecs> {
    fn parse(input: ParseStream) -> Result<Self> {
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

        Ok(TypedStage { name: stage_name, core_types, core_functions })
    }
}

impl Parse for TypedStage<Render> {
    fn parse(input: ParseStream) -> Result<Self> {
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

        Ok(TypedStage { name: stage_name, core_types, core_functions })
    }
}

impl Parse for TypedStage<Async> {
    fn parse(input: ParseStream) -> Result<Self> {
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

        Ok(TypedStage { name: stage_name, core_types, core_functions })
    }
}

impl Parse for TypedStage<EcsWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
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

        Ok(TypedStage { name: stage_name, core_types, core_functions })
    }
}

impl Parse for TypedStage<RenderWhile> {
    fn parse(input: ParseStream) -> Result<Self> {
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

        Ok(TypedStage { name: stage_name, core_types, core_functions })
    }
}

impl Parse for Stages {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut stages = Vec::new();
        while !input.is_empty() {
            stages.push(input.parse()?);

            let lookahead = input.lookahead1();
            if lookahead.peek(Token![,]) {
                let _ = input.parse::<Token![,]>()?;
            }
        }

        if stages.is_empty() {
            return Err(input.error("No stages found"));
        }

        Ok(Stages(stages))
    }
}

impl TypedStage<Ecs> {
    pub fn generate(self) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(stage_name.as_str().to_snake_case().as_str(), stage_ident.span());
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

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
        let stage_literal = quote! {
            crate::workflow::stage::WorkflowStage::Ecs(crate::workflow::stage::WorkflowStageEcs {
                name: super::NAME,
                run: self::stages::#stage_name::core_functions::run
            })
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }
}

impl TypedStage<Render> {
    pub fn generate(self) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(stage_name.as_str().to_snake_case().as_str(), stage_ident.span());
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

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
        let stage_literal = quote! {
            crate::workflow::stage::WorkflowStage::Render(crate::workflow::stage::WorkflowStageRender {
                name: super::NAME,
                run: self::stages::#stage_name::core_functions::run
            })
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }
}

impl TypedStage<Async> {
    pub fn generate(self) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(stage_name.as_str().to_snake_case().as_str(), stage_ident.span());
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

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
        let stage_literal = quote! {
            crate::workflow::stage::WorkflowStage::Async(crate::workflow::stage::WorkflowStageAsync {
                name: super::NAME,
                run: self::stages::#stage_name::core_functions::run
            })
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }
}

impl TypedStage<EcsWhile> {
    pub fn generate(self) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(stage_name.as_str().to_snake_case().as_str(), stage_ident.span());
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

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
        let stage_literal = quote! {
            crate::workflow::stage::WorkflowStage::EcsWhile(crate::workflow::stage::WorkflowStageEcsWhile {
                name: super::NAME,
                setup: self::stages::#stage_name::core_functions::setup,
                run: self::stages::#stage_name::core_functions::run
            })
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }
}

impl TypedStage<RenderWhile> {
    pub fn generate(self) -> (TokenStream, TokenStream) {
        let stage_ident = &self.name;
        let stage_name = stage_ident.to_string();
        let stage_ident = Ident::new(stage_name.as_str().to_snake_case().as_str(), stage_ident.span());
        let core_types = self.core_types.generate();
        let core_functions = self.core_functions.generate();

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
        let stage_literal = quote! {
            crate::workflow::stage::WorkflowStage::RenderWhile(crate::workflow::stage::WorkflowStageRenderWhile {
                name: super::NAME,
                setup: self::stages::#stage_name::core_functions::setup,
                run: self::stages::#stage_name::core_functions::run
            })
        };

        (stage_module, stage_literal)
    }

    pub fn name(&self) -> &Ident {
        &self.name
    }
}
