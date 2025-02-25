mod core_function;
mod core_type;
mod stage;
mod use_statement;
mod user_function;
mod user_type;

use core_function::*;
use core_type::*;
use stage::*;
use use_statement::*;
use user_function::*;
use user_type::*;

/// Represents the entire `vorkflow_mod! { ... }` macro input.
struct WorkflowModule {
    name: String,                      // Name of the module (e.g., "Gpu", "Chunk")
    workflows: Vec<Workflow>,
}

/// Represents an individual workflow inside the module.
struct Workflow {
    name: String,
    user_imports: Vec<UseStatement>,  // <-- Parsed Rust imports, not raw strings
    user_types: Vec<UserType>,        // <-- Parsed Rust types (structs, enums, type aliases)
    user_functions: Vec<UserFunction>, // <-- Parsed Rust functions
    stages: Vec<Stage>,
}
