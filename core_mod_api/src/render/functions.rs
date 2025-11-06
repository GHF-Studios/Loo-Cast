use bevy::prelude::*;

use crate::render::components::RenderProxy;

pub fn make_sprite_proxy_bundle(
    image: Handle<Image>,
    pos: Vec2,
    scale: f32,
    source_entity: Entity,
    chunk_z: f32,
) -> impl Bundle {
    (
        Transform {
            translation: pos.extend(chunk_z),
            scale: Vec3::splat(scale),
            ..Default::default()
        },
        Sprite {
            image,
            ..Default::default()
        },
        Pickable::default(),
        RenderProxy {
            source: source_entity,
        },
    )
}
