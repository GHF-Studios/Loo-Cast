use bevy::prelude::*;

use super::pos::GridPos;

#[derive(Resource)]
pub struct ScaleOrigins {
    inner: [GridPos; 71]
}
impl Default for ScaleOrigins {
    fn default() -> Self {
        let mut inner: [Option<GridPos>; 71] = [const { None }; 71];

        let mut current_origin = GridPos::new_root(IVec2::ZERO);
        inner[0] = Some(current_origin.clone());

        for inner in inner.iter_mut().skip(1) {
            current_origin = GridPos::new(current_origin, IVec2::ZERO);
            *inner = Some(current_origin.clone());
        }

        let inner = inner.map(|o| o.unwrap());
        
        Self { inner }
    }
}