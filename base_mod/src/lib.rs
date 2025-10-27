use base_mod_api::*;
use base_mod_macros::*;
use core_mod_api::*;
use core_mod_macros::api_initializer;

api_initializer!("base_mod");






// WIP/Sketches

use core_mod_api::usf;
use usf::pos::unit::types::*;

// A homogeneous/inert immovable/infinite-mass square rigidbody
struct TestSquareRigidbody {
    origin: UnitExtent,
    extent: [UnitExtent; 2],
}

impl DiscretePhenomenaModel for TestSquareRigidbody {
    fn is_within_bounds(&self, unit_pos: &UnitExtent) -> bool {
        // simple spatial AABB test
    }

    fn spawn_in(&self, chunk: &GridPos) -> Option<Phenomenon> {
        Some(Phenomenon::new(self, chunk))
    }
}

