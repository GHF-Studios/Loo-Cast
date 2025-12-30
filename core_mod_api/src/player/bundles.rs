use bevy::prelude::*;

use crate::follower::components::FollowerTarget;
use crate::usf::scale::Scale;
use crate::{
    chunk::components::{ChunkActor, ChunkLoader},
    config::statics::CONFIG,
};

use super::components::Player;

#[derive(Bundle, Reflect)]
pub struct PlayerBundle {
    pub chunk_actor: ChunkActor,
    pub chunk_loader: ChunkLoader,
    pub player: Player,
    pub sprite: Sprite,
    pub transform: Transform,
    pub name: Name,
    pub pickable: Pickable,
    pub follower_target: FollowerTarget,
}
impl Default for PlayerBundle {
    fn default() -> Self {
        let player_size = CONFIG().get::<f32>("player/base_size");
        let half_player_size = player_size / 2.0;

        PlayerBundle {
            chunk_actor: Default::default(),
            chunk_loader: Default::default(),
            player: Default::default(),
            sprite: Sprite {
                color: Color::srgb(0.0, 0.77, 0.33),
                rect: Some(Rect::new(-half_player_size, -half_player_size, half_player_size, half_player_size)),
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, Scale::MAX.compute_z()),
                ..Default::default()
            },
            name: Name::new("player"),
            pickable: Pickable::default(),
            follower_target: FollowerTarget { id: "main_camera".to_string() },
        }
    }
}
