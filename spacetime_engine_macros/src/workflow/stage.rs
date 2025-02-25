use super::core_type::CoreTypes;
use super::core_function::CoreFunction;

/// Represents a single stage inside a workflow.
pub struct Stage {
    name: String,              // Name of the stage (e.g., "ValidateAndSpawn")
    stage_type: StageType,     // Type of stage (Ecs, EcsWhile, etc.)
    core_types: CoreTypes,     // Input, Output, Error, State
    core_functions: Vec<CoreFunction>, // Function implementations
}

/// Enum for the five possible stage types.
pub enum StageType {
    Ecs,
    EcsWhile,
    Render,
    RenderWhile,
    Async,
}
