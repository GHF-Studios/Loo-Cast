use bevy::prelude::*;
use crate::{chunk_actor::components::ChunkActorComponent, chunk_loader::components::ChunkLoaderComponent};
use super::components::Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    chunk_actor: ChunkActorComponent,
    chunk_loader: ChunkLoaderComponent,
    player: Player,
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
                    rect: Some(Rect::new(-16.0, -16.0, 16.0, 16.0)),
                    ..Default::default()
                },
                ..Default::default()
            },
        }
    }
}