use base_mod_api::*;
use base_mod_macros::*;
use core_mod_api::*;
use core_mod_macros::api_initializer;

api_initializer!("base_mod");






// WIP/Sketches

use core_mod_api::usf;
use usf::pos::unit::types::*;

struct ImmoveableSquareModel {
    origin: UnitExtent,
    extent: [UnitExtent; 2],
}

impl DiscretePhenomenaModel for ImmoveableSquareModel {
    fn is_within_bounds(&self, chunk: &GridPos) -> bool {
        // simple spatial AABB test
    }

    fn spawn_in(&self, chunk: &GridPos) -> Option<Phenomenon> {
        Some(Phenomenon::new(self, chunk))
    }
}

struct Phenomenon {
    model_id: PhenomenaModelId,
    chunk: GridPos,
    spatial_bounds: AABB<FixedVec2>,
    scale: Scale,
    // Maybe links to parent phenomena, maybe not
}
