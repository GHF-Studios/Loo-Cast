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
            Code |input| -> Result<Output, Error> {
                match input.value {
                    0 => {
                        *self = HelloWorldCommand::Executed {
                            result: Ok(HelloWorldCommandOutput {
                                value: 0,
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
                InvalidEndPoint,
            },
            Code |input| -> Result<Output, Error> {
                if input.start_point.0 == 0 && input.start_point.1 == 0 {
                    if input.end_point.0 == 0 && input.end_point.1 == 0 {
                        println!("Pretending to draw Gizmo Line!");

                        return Ok(DrawGizmoLineCommandOutput {
                            line_id: 0,
                        })
                    } else {
                        return Err(DrawGizmoLineCommandError::InvalidEndPoint)
                    }
                } else {
                    return Err(DrawGizmoLineCommandError::InvalidStartPoint)
                }
            }
        }
    }
}

// This does not belong to the Test Command Module, but is a general Commands-related concept. The TestCommand Module's presence is to demonstrate the concept of a Command Module.
pub trait CommandsModule {
    fn module_name() -> &'static str;
}
