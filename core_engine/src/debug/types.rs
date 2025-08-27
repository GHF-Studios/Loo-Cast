use bevy::prelude::Reflect;

#[derive(Default, Reflect)]
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