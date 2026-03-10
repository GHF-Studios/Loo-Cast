pub mod aspects;
pub mod phenomenon;
pub mod pos;
pub mod scale;
pub mod transform;

use crate::bevy::prelude::*;
use phenomenon::{PhenomenonId, PhenomenonKind, PhenomenonMeshWindow, PhenomenonNodeKey, PhenomenonNodeSeed, PhenomenonStateSnapshot};

pub(crate) struct UsfPlugin;
impl Plugin for UsfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(pos::PosPlugin)
            .add_plugins(transform::TransformPlugin)
            .add_plugins(phenomenon::PhenomenonPlugin)
            .register_type::<PhenomenonKind>()
            .register_type::<PhenomenonId>()
            .register_type::<PhenomenonNodeSeed>()
            .register_type::<PhenomenonNodeKey>()
            .register_type::<PhenomenonStateSnapshot>()
            .register_type::<PhenomenonMeshWindow>();
    }
}
