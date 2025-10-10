use bevy::math::Vec2;

use crate::usf::scale::{Scale, DynScale};
use crate::utils::types::I128Vec2;

// use super::types::{WorldCoord, GridCoord};
// 
// pub trait Vec2Ext {
//     fn to_world_coord(self, scale: Scale, effective_grid_xy: I128Vec2) -> WorldCoord;
//     fn to_grid_coord(self, scale: Scale, grid_origin_offset: I128Vec2) -> GridCoord;
// }
// 
// impl Vec2Ext for Vec2 {
//     fn to_world_coord(self, scale: Scale, effective_grid_xy: I128Vec2) -> WorldCoord {
//         WorldCoord { grid_coord: GridCoord::new(scale, effective_grid_xy.x, effective_grid_xy.y), local_offset: Vec2::new(self.x, self.y) }
//     }
// 
//     fn to_grid_coord(self, scale: Scale, grid_origin_offset: I128Vec2) -> GridCoord {
//         let chunk_size = 1000.0 * scale.scale_factor() as f32;
//         let chunk_x = ((self.x + chunk_size / 2.0) / chunk_size).floor() as i128 + grid_origin_offset.x;
//         let chunk_y = ((self.y + chunk_size / 2.0) / chunk_size).floor() as i128 + grid_origin_offset.y;
//         GridCoord { scale, xy: I128Vec2::new(chunk_x, chunk_y) }
//     }
// }
// 
// pub trait I128Vec2Ext {
//     fn to_world_coord(self, grid_origin_offset: I128Vec2) -> Vec2;
// }
// 
// impl I128Vec2Ext for I128Vec2 {
//     fn to_world_coord(self, grid_origin_offset: I128Vec2) -> Vec2 {
//         let chunk_size = 1000.0;
//         let chunk_diff = self - grid_origin_offset;
//         Vec2::new(
//             chunk_diff.x as f32 * chunk_size,
//             chunk_diff.y as f32 * chunk_size,
//         )
//     }
// }