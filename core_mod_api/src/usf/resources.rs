use bevy::prelude::*;

use crate::utils::types::I128Vec2;

use super::pos::GridOffset;
use super::scale::Scale;

#[derive(Resource)]
pub struct ScaleOrigins {
    inner: [GridOffset; 71]
}
impl Default for ScaleOrigins {
    fn default() -> Self {
        let mut inner: [Option<GridOffset>; 71] = [const { None }; 71];

        let mut current_origin = GridOffset::new_origin(None, Scale::MAX);
        inner[0] = Some(current_origin.clone());

        for scale_index in 1..=70 {
            let scale = Scale::from_index_from_top(scale_index).unwrap();

            current_origin = GridOffset::new(Some(Box::new(current_origin)), scale, I128Vec2::ZERO);
            inner[scale_index as usize] = Some(current_origin.clone());
        }

        let inner = inner.map(|o| o.unwrap());
        
        Self { inner }
    }
}