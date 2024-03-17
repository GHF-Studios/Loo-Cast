use std::fmt::{Error, Display};
use spacetime_engine_derive::define_commands_module;

define_commands_module! {
    Test {
        module_path: crate::kernel::commands,
        commands: [
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
                            return Ok(HelloWorldCommandOutput {
                                value: 0,
                            });
                        },
                        _ => {
                            return Err(HelloWorldCommandError::InvalidInput);
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
        ]
    }
}

pub fn test() {
    let test_commands = TestCommands {};
    test_commands.hello_world(0);
}