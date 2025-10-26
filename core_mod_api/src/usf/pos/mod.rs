#![allow(clippy::crate_in_macro_def)]

pub mod grid;
pub mod subgrid;
pub mod unit;

#[macro_export]
macro_rules! grid_extent {
    ([$first:expr $(, $rest:expr)*]) => {
        {
            use bevy::math::IVec2;
            use crate::usf::pos::grid::types::GridExtent;

            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            GridExtent::try_from(stack).unwrap()
        }
    };
}

#[macro_export]
macro_rules! subgrid_extent {
    ([$first:expr $(, $rest:expr)*]: $sub:expr) => {
        {
            use bevy::math::IVec2;
            use crate::usf::pos::subgrid::types::SubgridExtent;
            
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            SubgridExtent::try_from((stack, IVec2::from($sub))).unwrap()
        }
    };
}

#[macro_export]
macro_rules! unit_extent {
    ([$first:expr $(, $rest:expr)*]: $unit:expr) => {{
        {
            use bevy::math::{IVec2, Vec2};
            use crate::usf::pos::unit::types::UnitExtent;
            
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            UnitExtent::try_from((stack, Vec2::from($unit))).unwrap()
        }
    }};
}
