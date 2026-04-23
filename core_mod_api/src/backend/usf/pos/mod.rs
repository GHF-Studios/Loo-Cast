#![allow(clippy::crate_in_macro_def)]

pub mod grid;
pub mod subgrid;
pub mod types;
pub mod unit;

use crate::bevy::prelude::*;

pub(crate) struct PosPlugin;
impl Plugin for PosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(grid::GridPlugin)
            .add_plugins(subgrid::SubgridPlugin)
            .add_plugins(unit::UnitPlugin);
    }
}

#[macro_export]
macro_rules! grid_extent {
    ([$first:expr $(, $rest:expr)*]) => {
        {
            use crate::usf::pos::grid::types::GridVec;
            use crate::usf::pos::types::GridXyz;

            let stack = vec![GridXyz::from($first) $(, GridXyz::from($rest))*];
            GridVec::try_from(stack).unwrap()
        }
    };
}

#[macro_export]
macro_rules! subgrid_extent {
    ([$first:expr $(, $rest:expr)*]: $sub:expr) => {
        {
            use crate::usf::pos::subgrid::types::SubgridVec;
            use crate::usf::pos::types::{GridXyz, SubgridXyz};

            let stack = vec![GridXyz::from($first) $(, GridXyz::from($rest))*];
            SubgridVec::try_from((stack, SubgridXyz::from($sub))).unwrap()
        }
    };
}

#[macro_export]
macro_rules! unit_extent {
    ([$first:expr $(, $rest:expr)*]: $unit:expr) => {{
        {
            use crate::bevy::math::Vec3;
            use crate::usf::pos::unit::types::UnitVec;
            use crate::usf::pos::types::GridXyz;

            let stack = vec![GridXyz::from($first) $(, GridXyz::from($rest))*];
            UnitVec::try_from((stack, Vec3::from($unit))).unwrap()
        }
    }};
}
