use bevy::prelude::*;
use super::components::{TestObjectComponent, TestObjectMovement};

pub(in crate) fn test_object_movement_system(
    time: Res<Time>,
    mut query: Query<(&mut Transform, &TestObjectComponent)>,
) {
    for (mut transform, test_object) in query.iter_mut() {
        match &test_object.movement {
            TestObjectMovement::Static => {}
            TestObjectMovement::Circle { radius, speed } => {
                let time_factor = time.elapsed_seconds() * speed;
                transform.translation.x = radius * time_factor.cos();
                transform.translation.y = radius * time_factor.sin();
            }
            TestObjectMovement::Line { distance, speed } => {
                let time_factor = time.elapsed_seconds() * speed;
                let offset = time_factor.sin() * distance;
                transform.translation.x = offset;
            }
        }
    }
}