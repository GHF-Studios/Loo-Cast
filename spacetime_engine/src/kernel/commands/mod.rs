use spacetime_engine_derive::define_commands_module;

pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

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
                        0..=9 => {
                            Ok(HelloWorldCommandOutput {
                                value: 0,
                            })
                        },
                        _ => {
                            Err(HelloWorldCommandError::InvalidInput)
                        },
                    }
                }
            },
            DrawGizmoLine {
                Input {
                    start_point: Point,
                    end_point: Point,
                },
                Output {
                    line_id: u32,
                },
                Error {
                    InvalidStartPoint,
                    InvalidEndPoint,
                },
                Code |input| -> Result<Output, Error> {
                    if input.start_point.x == 0 && input.start_point.y == 0 {
                        if input.end_point.x == 0 && input.end_point.y == 0 {
                            Ok(DrawGizmoLineCommandOutput {
                                line_id: 0,
                            })
                        } else {
                            Err(DrawGizmoLineCommandError::InvalidEndPoint)
                        }
                    } else {
                        Err(DrawGizmoLineCommandError::InvalidStartPoint)
                    }
                }
            },
            DrawGizmoCircle {
                Input {
                    center: Point,
                    radius: i32,
                },
                Output {
                    circle_id: u32,
                },
                Error {
                    InvalidCenter,
                    InvalidRadius,
                },
                Code |input| -> Result<Output, Error> {
                    if input.center.x == 0 && input.center.y == 0 {
                        if input.radius > 0 {
                            Ok(DrawGizmoCircleCommandOutput {
                                circle_id: 0,
                            })
                        } else {
                            Err(DrawGizmoCircleCommandError::InvalidRadius)
                        }
                    } else {
                        Err(DrawGizmoCircleCommandError::InvalidCenter)
                    }
                }
            },
            SpawnEntity {
                Input {
                    position: Point,
                },
                Output {
                    entity_id: u32,
                },
                Error {
                    InvalidPosition,
                },
                Code |input| -> Result<Output, Error> {
                    if input.position.x == 0 && input.position.y == 0 {
                        Ok(SpawnEntityCommandOutput {
                            entity_id: 0,
                        })
                    } else {
                        Err(SpawnEntityCommandError::InvalidPosition)
                    }
                }
            },
            DespawnEntity {
                Input {
                    entity_id: u32,
                },
                Output {
                    success: bool,
                },
                Error {
                    InvalidEntityId,
                },
                Code |input| -> Result<Output, Error> {
                    if input.entity_id == 0 {
                        Ok(DespawnEntityCommandOutput {
                            success: true,
                        })
                    } else {
                        Err(DespawnEntityCommandError::InvalidEntityId)
                    }
                }
            },
        ]
    }
}
