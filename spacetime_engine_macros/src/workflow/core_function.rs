/// Represents a single function inside a stage.
pub struct CoreFunction {
    function_type: FunctionType, // Function type (RunEcs, SetupEcsWhile, etc.)
    signature: FunctionSignature, // Parameters & return type
    body: String,                 // Raw Rust function body
}

/// Enum for function types within a stage.
pub enum FunctionType {
    RunEcs,
    RunRender,
    RunAsync,
    SetupEcsWhile,
    RunEcsWhile,
    SetupRenderWhile,
    RunRenderWhile,
}

/// Represents a parsed function signature.
pub struct FunctionSignature {
    name: String,
    params: Vec<FunctionParam>,
    return_type: Option<String>, // Example: "Result<Output, Error>"
}

/// Represents a function parameter.
pub struct FunctionParam {
    name: String,
    ty: String, // Example: "&mut World"
}