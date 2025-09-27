use bevy::prelude::*;

use crate::{chunk_actor::components::ChunkActor, chunk_loader::components::ChunkLoader, config::statics::CONFIG, utils::components::InitHook};
use crate::usf::scale::ScaleMeter1;

use super::components::Player;

#[derive(Bundle, Reflect)]
pub struct PlayerBundle {
    chunk_actor: ChunkActor<ScaleMeter1>,
    chunk_loader: ChunkLoader<ScaleMeter1>,
    chunk_loader_init_hook: InitHook<ChunkLoader<ScaleMeter1>>,
    player: Player,
    sprite: Sprite,
    transform: Transform,
    name: Name,
}
impl PlayerBundle {
    pub fn chunk_loader(&self) -> &ChunkLoader<ScaleMeter1> {
        &self.chunk_loader
    }
}
impl Default for PlayerBundle {
    fn default() -> Self {
        let player_size = CONFIG().get::<f32>("player/base_size");
        let half_player_size = player_size / 2.0;

        PlayerBundle {
            chunk_actor: Default::default(),
            chunk_loader: ChunkLoader::<ScaleMeter1>::new("player_entity_chunk_loader".to_string()),
            chunk_loader_init_hook: InitHook::<ChunkLoader<ScaleMeter1>>::default(),
            player: Default::default(),
            sprite: Sprite {
                color: Color::srgb(0.0, 1.0, 0.0),
                rect: Some(Rect::new(-half_player_size, -half_player_size, half_player_size, half_player_size)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
            name: Name::new("player_entity"),
        }
    }
}
