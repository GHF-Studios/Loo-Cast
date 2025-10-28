#![allow(clippy::crate_in_macro_def)]

pub mod grid;
pub mod subgrid;
pub mod unit;

pub mod resources;

use bevy::prelude::*;

use resources::OriginOffset;

pub(crate) struct PosPlugin;
impl Plugin for PosPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(grid::GridPlugin)
            .add_plugins(subgrid::SubgridPlugin)
            .add_plugins(unit::UnitPlugin)

            .insert_resource(OriginOffset::default())

            .register_type::<OriginOffset>();
    }
}

#[macro_export]
macro_rules! grid_extent {
    ([$first:expr $(, $rest:expr)*]) => {
        {
            use bevy::math::IVec2;
            use crate::usf::pos::grid::types::GridVec;

            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            GridVec::try_from(stack).unwrap()
        }
    };
}

#[macro_export]
macro_rules! subgrid_extent {
    ([$first:expr $(, $rest:expr)*]: $sub:expr) => {
        {
            use bevy::math::IVec2;
            use crate::usf::pos::subgrid::types::SubgridVec;
            
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            SubgridVec::try_from((stack, IVec2::from($sub))).unwrap()
        }
    };
}

#[macro_export]
macro_rules! unit_extent {
    ([$first:expr $(, $rest:expr)*]: $unit:expr) => {{
        {
            use bevy::math::{IVec2, Vec2};
            use crate::usf::pos::unit::types::UnitVec;
            
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            UnitVec::try_from((stack, Vec2::from($unit))).unwrap()
        }
    }};
}
