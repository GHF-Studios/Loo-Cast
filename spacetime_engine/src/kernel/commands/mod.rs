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
                Input {},
                Output {},
                Error {},
                Code || -> () {
                    println!("Hello World!");
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
                    InvalidRadius,
                },
                Code |input| -> Result<Output, Error> {
                    if input.radius > 0 {
                        Ok(DrawGizmoCircleCommandOutput {
                            circle_id: 0,
                        })
                    } else {
                        Err(DrawGizmoCircleCommandError::InvalidRadius)
                    }
                }
            },
            SpawnEntity {
                Input {},
                Output {
                    entity_id: u32,
                },
                Error {},
                Code || -> Output {
                    SpawnEntityCommandOutput {
                        entity_id: 0,
                    }
                }
            },
            DespawnEntity {
                Input {
                    BevyCommands,
                    Primitive {
                        entity_id: u32,
                    },
                    Component {
                        health: Health,
                    },
                    Resource {
                        EntityManager
                    }
                },
                Output {
                    Primitive {
                        entity_id: u32,
                    },
                    Component {
                        health: Health,
                    },
                    Resource {
                        EntityManager
                    }
                },
                Error {
                    InvalidEntityId,
                },
                Code |input| -> Result<(), Error> {
                    if input.entity_id == 0 {
                        Ok(())
                    } else {
                        Err(DespawnEntityCommandError::InvalidEntityId)
                    }
                }
            },
            DepsawnAllEntities {
                Input {

                }
            }
        ]
    }
}