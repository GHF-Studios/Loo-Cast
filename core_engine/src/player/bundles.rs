use super::components::PlayerComponent;
use crate::{chunk_actor::components::ChunkActorComponent, chunk_loader::components::ChunkLoaderComponent};
use bevy::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    chunk_actor: ChunkActorComponent,
    chunk_loader: ChunkLoaderComponent,
    player: PlayerComponent,
    sprite_bundle: SpriteBundle,
}
impl PlayerBundle {
    pub fn chunk_loader(&self) -> &ChunkLoaderComponent {
        &self.chunk_loader
    }
}

impl Default for PlayerBundle {
    fn default() -> Self {
        PlayerBundle {
            chunk_actor: Default::default(),
            chunk_loader: ChunkLoaderComponent::new("player".to_string()),
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
