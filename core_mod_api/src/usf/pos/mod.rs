#![allow(clippy::crate_in_macro_def)]

pub mod grid;
pub mod subgrid;
pub mod unit;

#[macro_export]
macro_rules! grid_pos {
    ([$first:expr $(, $rest:expr)*]) => {
        {
            use bevy::math::IVec2;
            use crate::usf::pos::grid::types::GridPos;

            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            GridPos::try_from(stack).unwrap()
        }
    };
}

#[macro_export]
macro_rules! subgrid_pos {
    ([$first:expr $(, $rest:expr)*]: $sub:expr) => {
        {
            use bevy::math::IVec2;
            use crate::usf::pos::subgrid::types::SubgridPos;
            
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            SubgridPos::try_from((stack, IVec2::from($sub))).unwrap()
        }
    };
}

#[macro_export]
macro_rules! unit_pos {
    ([$first:expr $(, $rest:expr)*]: $unit:expr) => {{
        {
            use bevy::math::IVec2;
            use crate::usf::pos::unit::types::UnitPos;
            
            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            UnitPos::try_from((stack, Vec2::from($unit))).unwrap()
        }
    }};
}
