use bevy::prelude::*;

#[derive(Component, Default)]
pub struct TestObjectComponent {
    pub movement: TestObjectMovement,
}

#[derive(Default)]
pub enum TestObjectMovement {
    #[default]
    Static,
    Circle { radius: f32, speed: f32 },
    Line { distance: f32, speed: f32 },
}