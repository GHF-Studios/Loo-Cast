define_data_module! {
    Test {
        module_path: crate::kernel::data,
        data: [
            Position {
                Type {
                    PrimitiveStruct
                },
                Fields {
                    pub x: i32,
                    pub y: i32,
                    pub z: i32
                },
                Impl {
                    pub fn new(x: i32, y: i32, z: i32) -> Self {
                        Position {
                            x,
                            y,
                            z
                        }
                    },
                },
                Attributes {
                    Debug,
                    Clone
                }
            },
            Rotation {
                Type {
                    PrimitiveStruct
                },
                Fields {
                    pub pitch: f32,
                    pub yaw: f32,
                    pub roll: f32
                },
                Impl {
                    pub fn new(pitch: f32, yaw: f32, roll: f32) -> Self {
                        Rotation {
                            pitch,
                            yaw,
                            roll
                        }
                    }
                },
                Attributes {
                    Debug,
                    Clone,
                }
            },
            Scale {
                Type {
                    PrimitiveStruct
                },
                Fields {
                    pub x: f32,
                    pub y: f32,
                    pub z: f32
                },
                Impl {
                    pub fn new(x: f32, y: f32, z: f32) -> Self {
                        Scale {
                            x,
                            y,
                            z
                        }
                    }
                },
                Attributes {
                    Debug,
                    Clone,
                }
            },
            TransformComponent {
                Type {
                    Component
                },
                Fields {
                    position: Position,
                    rotation: Rotation,
                    scale: Scale
                },
                Impl {
                    pub fn new(position: Position, rotation: Rotation, scale: Scale) -> Self {
                        Transform {
                            position,
                            rotation,
                            scale
                        }
                    },
                    pub fn get_position(&self) -> &Position {
                        &self.position
                    },
                    pub fn get_position_mut(&mut self) -> &mut Position {
                        &mut self.position
                    },
                    pub fn get_rotation(&self) -> &Rotation {
                        &self.rotation
                    },
                    pub fn get_rotation_mut(&mut self) -> &mut Rotation {
                        &mut self.rotation
                    },
                    pub fn get_scale(&self) -> &Scale {
                        &self.scale
                    },
                    pub fn get_scale_mut(&mut self) -> &mut Scale {
                        &mut self.scale
                    },
                },
                Attributes {
                    Default,
                    Debug,
                    Clone,
                }
            },
            GameState {
                Type {
                    State
                },
                Variants {
                    MainMenu,
                    InGame,
                    GameOver {
                        score: Score
                    },
                },
                Impl {
                    pub fn get_score(&self) -> Option<&Score> {
                        match self {
                            GameState::GameOver { score } => Some(score),
                            _ => None,
                        }
                    }
                },
                Attributes {
                    Debug
                }
            },
            HealthComponent {
                Type {
                    Component
                },
                Fields {
                    pub value: i32
                },
                Impl {
                    pub fn new(value: i32) -> Self {
                        Health {
                            value
                        }
                    }
                },
                Attributes {
                    Debug,
                    Clone,
                }
            },
            PhysicalConstantsResource {
                Type {
                    Resource
                },
                Fields {
                    gravity: f32,
                    atmospheric_density: f32,
                },
                Impl {
                    pub fn new(gravity: f32, atmospheric_density: f32) -> Self {
                        PhysicalConstants {
                            gravity,
                            atmospheric_density
                        }
                    },
                    pub fn get_gravity(&self) -> f32 {
                        self.gravity
                    },
                    pub fn get_atmospheric_density(&self) -> f32 {
                        self.atmospheric_density
                    },
                },
                Attributes {
                    Debug,
                    Clone,
                }
            }
        ]
    }
}