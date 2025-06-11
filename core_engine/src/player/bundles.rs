use super::components::Player;
use crate::{chunk_actor::components::ChunkActor, chunk_loader::components::ChunkLoader, utils::InitHook};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    chunk_actor: ChunkActor,
    chunk_loader: ChunkLoader,
    chunk_loader_init_hook: InitHook<ChunkLoader>,
    player: Player,
    sprite_bundle: SpriteBundle,
}
impl PlayerBundle {
    pub fn chunk_loader(&self) -> &ChunkLoader {
        &self.chunk_loader
    }
}
impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            chunk_actor: Default::default(),
            chunk_loader: ChunkLoader::new("player".to_string()),
            chunk_loader_init_hook: InitHook::<ChunkLoader>::default(),
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
