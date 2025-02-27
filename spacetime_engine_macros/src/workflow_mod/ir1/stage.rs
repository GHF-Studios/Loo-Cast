use syn::{parse::Parse, Ident, Token, Result, braced, parse::ParseStream};
use super::core_type::CoreTypes;
use super::core_function::CoreFunctions;

/// Concrete Stage Types
pub struct Ecs;
pub struct EcsWhile;
pub struct Render;
pub struct RenderWhile;
pub struct Async;

/// Enum to store different stage types with type-level enforcement
pub enum Stage {
    Ecs(TypedStage<Ecs>),
    EcsWhile(TypedStage<EcsWhile>),
    Render(TypedStage<Render>),
    RenderWhile(TypedStage<RenderWhile>),
    Async(TypedStage<Async>),
}

/// Represents a stage inside a workflow
pub struct TypedStage<T> {
    pub name: Ident,
    pub core_types: CoreTypes<T>,
    pub core_functions: CoreFunctions<T>,
}

/// Parses the `StageIR` enum and dispatches to the correct type without consuming the Ident.
impl Parse for Stage {
    fn parse(input: ParseStream) -> Result<Self> {
        let lookahead = input.fork(); // Peek ahead without consuming
        let stage_name: Ident = lookahead.parse()?; // Read the Ident without consuming input

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

/// Explicitly implement parsing for each stage type to enforce constraints.

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
