use bevy::prelude::*;
use std::sync::Arc;

use super::pos::GridPos;
use super::scale::Scale;

#[derive(Resource)]
pub struct ScaleOrigins {
    inner: [GridPos; 71]
}
impl Default for ScaleOrigins {
    fn default() -> Self {
        let mut inner: [Option<GridPos>; 71] = [const { None }; 71];

        let mut current_origin = GridPos::new_root(IVec2::ZERO);
        inner[0] = Some(current_origin.clone());

        for scale_index in 1..=70 {
            let scale = Scale::from_index_from_top(scale_index).unwrap();

            current_origin = GridPos::new(current_origin, IVec2::ZERO);
            inner[scale_index as usize] = Some(current_origin.clone());
        }

        let inner = inner.map(|o| o.unwrap());
        
        Self { inner }
    }
}