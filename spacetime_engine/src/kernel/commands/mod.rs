use spacetime_engine_macro::define_commands_module;

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
                    entity_id: u32,
                },
                Output {},
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
        ]
    }
}

#[derive(bevy::ecs::event::Event)]
struct HelloWorldCommandRequest {
    pub input: HelloWorldCommandInput,
}

#[derive(bevy::ecs::event::Event)]
struct HelloWorldCommandResponse {
    pub output: HelloWorldCommandOutput,
    pub error: HelloWorldCommandError,
}

// A command script should essentially be a closure, which somehow chains commands together
// and when needing to await their response, they can be paused so the command script manager can continue work on other command scripts.
// somehow the command script manager needs to be able to halt execution of the command script if it's currently waiting for a command's returned future to be ready.

fn dispatch_commands_task() {
    
}

async fn sex() {
    dispatch_commands_task(async || {
        match spawn_entity().await {
            Ok(entity_id) => {
                println!("Entity created with ID: {}", entity_id);
                match modify_entity(entity_id).await {
                    Ok(_) => println!("Entity modified successfully"),
                    Err(e) => println!("Failed to modify entity: {}", e),
                }
            },
            Err(e) => println!("Failed to spawn entity: {}", e),
        }
    });
}

fn handle_hello_world_command_request(request: HelloWorldCommandRequest);
fn handle_hello_world_command_response(response: HelloWorldCommandResponse);