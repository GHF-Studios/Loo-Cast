use bevy::prelude::*;

use crate::usf::scale::Scale;
use crate::{chunk_actor::components::ChunkActor, chunk_loader::components::ChunkLoader, config::statics::CONFIG, utils::lifecycle_hook::InitHook};

use super::components::Player;

#[derive(Bundle, Reflect)]
pub struct PlayerBundle {
    pub chunk_actor: ChunkActor,
    pub chunk_loader: ChunkLoader,
    pub chunk_loader_init_hook: InitHook<ChunkLoader>,
    pub player: Player,
    pub sprite: Sprite,
    pub transform: Transform,
    pub name: Name,
    pub pickable: Pickable,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        let player_size = CONFIG().get::<f32>("player/base_size");
        let half_player_size = player_size / 2.0;

        PlayerBundle {
            chunk_actor: Default::default(),
            chunk_loader: ChunkLoader::new("player_entity_chunk_loader".to_string(), Scale::default()),
            chunk_loader_init_hook: InitHook::<ChunkLoader>::default(),
            player: Default::default(),
            sprite: Sprite {
                color: Color::srgb(0.0, 0.77, 0.33),
                rect: Some(Rect::new(-half_player_size, -half_player_size, half_player_size, half_player_size)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 10.0),
                ..Default::default()
            },
            name: Name::new("player"),
            pickable: Pickable::default(),
        }
    }
}
