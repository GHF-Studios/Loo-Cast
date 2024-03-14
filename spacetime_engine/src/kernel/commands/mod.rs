// Type: Struct
// Naming Convention: "[MODULE_NAME]Commands"
pub trait CommandModule {
    fn module_name() -> &'static str;
}

// Type: Struct
// Naming Convention: "[COMMAND_NAME]Command"
pub trait Command {
    type Module: CommandModule;

    fn execute(&self);
}

// Type: Enum
// Naming Convention: [COMMAND_NAME]CommandError
pub trait CommandError: std::error::Error {
    type Command: Command;
}

// Type: Enum
// Naming Convention: [COMMAND_NAME]CommandSuccess
pub trait CommandSuccess: std::fmt::Debug {
    type Command: Command;
}

// Type: Struct
// Naming Convention: "[MODULE_NAME]CommandQueueGroup"
pub trait CommandQueueGroup {
    type CommandModule: Command;
}