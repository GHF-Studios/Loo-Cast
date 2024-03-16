use std::fmt::{Error, Display};
use spacetime_engine_derive::define_commands_module;

define_commands_module! {
    Test { 
        HelloWorld {
            Input {
                value: i32,
            },
            Output {
                value: i32,
            },
            Error {
                InvalidInput,
            },
            Code (input) => {
                if input.value == 0 {
                    println!("Hello World!");

                    return Ok(HelloWorldCommandOutput {
                        value: input.value,
                    })
                } else {
                    return Err(HelloWorldCommandError::InvalidInput)
                }
            }
        },
        DrawGizmoLine {
            Input {
                start_point: (i32, i32),
                end_point: (i32, i32),
            },
            Output {
                line_id: u32,
            },
            Error {
                InvalidStartPoint,
                InvalidEndPoint;
            },
            Code (input) => {
                if start_point.0 == 0 && start_point.1 == 0 {
                    if end_point.0 == 0 && end_point.1 == 0 {
                        println!("Drawing Gizmo Line!");

                        return Ok(DrawGizmoLineCommandOutput {
                            value: input.value,
                        })
                    } else {
                        return Err(DrawGizmoLineCommandError::InvalidEndPoint)
                    }
                } else {
                    return Err(DrawGizmoLineCommandError::InvalidPoints)
                }
            }
        }
    }
}



// This does not belong to the Test Command Module, but is a general Commands-related concept. The TestCommand Module's presence is to demonstrate the concept of a Command Module.
pub trait CommandsModule {
    fn module_name() -> &'static str;
}

pub(in crate::kernel::commands) trait TestCommand {
    type Module: CommandsModule;
    type Input: TestCommandInput<Command = Self>;
    type Output: TestCommandOutput<Command = Self>;
    type Error: TestCommandError<Command = Self>;

    fn initialize(input: Self::Input) -> Self;
    fn execute(&mut self);
    fn finalize(self) -> Option<Result<Self::Output, Self::Error>>;
}

pub(in crate::kernel::commands) trait TestCommandInput: Display {
    type Command: TestCommand;
}
pub(in crate::kernel::commands) trait TestCommandOutput: Display {
    type Command: TestCommand;
}
pub(in crate::kernel::commands) trait TestCommandError: Display {
    type Command: TestCommand;
}
pub(in crate::kernel::commands) trait TestCommandCode: Display {
}
pub struct TestCommands {
}

impl CommandsModule for TestCommands {
    fn module_name() -> &'static str {
        "TestCommands"
    }
}

impl TestCommands {
    pub fn hello_world(input_value: i32) -> Result<HelloWorldCommandOutput, HelloWorldCommandError> {
        let mut hello_world_command = HelloWorldCommand::initialize(HelloWorldCommandInput {
            value: input_value,
        });

        hello_world_command.execute();

        if let Some(hello_world_command_result) = hello_world_command.finalize() {
            hello_world_command_result
        } else {
            panic!("Command did not execute properly!");
        }
    }
}

pub(in crate::kernel::commands) enum HelloWorldCommand {
    Initialized {
        input: HelloWorldCommandInput,
    },
    Executed {
        result: Result<HelloWorldCommandOutput, HelloWorldCommandError>,
    },
}

impl TestCommand for HelloWorldCommand {
    type Module = TestCommands;
    type Input = HelloWorldCommandInput;
    type Output = HelloWorldCommandOutput;
    type Error = HelloWorldCommandError;

    fn initialize(input: Self::Input) -> Self {
        HelloWorldCommand::Initialized {
            input,
        }
    }

    fn execute(&mut self) {
        if let HelloWorldCommand::Initialized { input } = self {
            match input.value {
                0 => {
                    *self = HelloWorldCommand::Executed {
                        result: Ok(HelloWorldCommandOutput {
                            value: input.value,
                        }),
                    };
                },
                _ => {
                    *self = HelloWorldCommand::Executed {
                        result: Err(HelloWorldCommandError::InvalidInput),
                    };
                },
            }
        }
    }

    fn finalize(self) -> Option<Result<Self::Output, Self::Error>> {
        if let HelloWorldCommand::Executed { result } = self {
            Some(result)
        } else {
            None
        }
    }
}

pub struct HelloWorldCommandInput {
    pub value: i32,
}

impl TestCommandInput for HelloWorldCommandInput {
    type Command = HelloWorldCommand;
}

impl Display for HelloWorldCommandInput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
        write!(f, "HelloWorldCommandInput {{ value: {} }}", self.value)
    }
}

pub struct HelloWorldCommandOutput {
    pub value: i32,
}

impl TestCommandOutput for HelloWorldCommandOutput {
    type Command = HelloWorldCommand;
}

impl Display for HelloWorldCommandOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
        write!(f, "HelloWorldCommandOutput {{ value: {} }}", self.value)
    }
}

pub enum HelloWorldCommandError {
    InvalidInput,
}

impl TestCommandError for HelloWorldCommandError {
    type Command = HelloWorldCommand;
}

impl Display for HelloWorldCommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), Error> {
        write!(f, "HelloWorldCommandError")
    }
}