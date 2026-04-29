use crate::bevy::prelude::*;

pub fn compute_chunk_transform(
    chunk_pos: IVec3,
    chunk_scale: i32,
    view_scale: i32,
    chunk_size: f32, // Usually 1000.0
) -> Transform {
    let scale_diff = chunk_scale - view_scale;

    let visual_scale = 10f32.powi(-scale_diff);
    let world_pos = chunk_pos.as_vec3() * chunk_size;

    Transform {
        translation: Vec3::new(world_pos.x, world_pos.y, world_pos.z + scale_diff as f32),
        scale: Vec3::splat(visual_scale),
        ..default()
    }
}
