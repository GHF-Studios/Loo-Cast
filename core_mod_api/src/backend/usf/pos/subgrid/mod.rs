pub mod tests;
pub mod types;

use crate::bevy::prelude::*;

use types::SubgridVec;

pub(crate) struct SubgridPlugin;
impl Plugin for SubgridPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SubgridVec>();
    }
}
