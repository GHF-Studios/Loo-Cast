pub mod stages;
pub mod imports {
    crate::workflow_imports_util!();

    use bevy::prelude::*;

    use crate::{
        chunk_actor::components::ChunkActorComponent, chunk_loader::components::ChunkLoaderComponent,
    };
    use crate::debug::components::{TestObjectComponent, TestObjectMovement};
}
pub mod user_items {
    crate::workflow_user_items_util!();

    pub fn spawn_test_object(
        commands: &mut Commands,
        position: Vec2,
        rotation: f32,
        scale: Vec2,
        movement: TestObjectMovement,
    ) {
        commands.spawn((
            ChunkActorComponent,
            ChunkLoaderComponent::default(),
            TestObjectComponent { movement },
            SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 0.0, 1.0),
                    rect: Some(Rect::new(-16.0, -16.0, 16.0, 16.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: position.extend(0.0),
                    rotation: Quat::from_rotation_z(rotation),
                    scale: scale.extend(1.0),
                },
                ..Default::default()
            },
        ));
    }
}

use spacetime_engine_macros::define_workflow;

define_workflow!("SpawnDebugObjects", [
    stage!(Ecs, "ValidateAndSpawn")
]);