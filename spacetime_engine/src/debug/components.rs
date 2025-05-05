use bevy::prelude::*;

#[derive(Component, Default)]
pub struct DebugObjectComponent {
    pub movement: DebugObjectMovement,
}

#[derive(Default)]
pub enum DebugObjectMovement {
    #[default]
    Static,
    Circle {
        radius: f32,
        speed: f32,
    },
    Line {
        distance: f32,
        speed: f32,
    },
}
