use bevy::prelude::*;
use crate::{chunk_actor::components::ChunkActorComponent, chunk_loader::components::ChunkLoaderComponent};
use super::components::PlayerComponent;

#[derive(Bundle)]
pub struct PlayerBundle {
    chunk_actor: ChunkActorComponent,
    chunk_loader: ChunkLoaderComponent,
    player: PlayerComponent,
    sprite_bundle: SpriteBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            chunk_actor: Default::default(),
            chunk_loader: Default::default(),
            player: Default::default(),
            sprite_bundle: SpriteBundle {
                sprite: Sprite {
                    color: Color::srgb(0.0, 1.0, 0.0),
                    rect: Some(Rect::new(-16.0, -16.0, 16.0, 16.0)),
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(0.0, 0.0, 0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}