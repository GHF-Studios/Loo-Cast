#![allow(clippy::crate_in_macro_def)]

pub mod grid;
pub mod subgrid;
pub mod unit;

pub mod systems;

use crate::bevy::prelude::*;
use systems::{apply_new_origin_offset_system, realign_origin_offset_system, sync_logical_from_transform_system, update_managed_positions};

use crate::core::run_conditions::run_after_startup_finished;
use crate::time::run_conditions::run_if_not_paused;
pub(crate) struct PosPlugin;
impl Plugin for PosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(grid::GridPlugin)
            .add_plugins(subgrid::SubgridPlugin)
            .add_plugins(unit::UnitPlugin)
            .add_systems(PreUpdate, update_managed_positions.run_if(run_after_startup_finished.and(run_if_not_paused)))
            // .add_systems(
            //     Update,
            //     realign_origin_offset_system.run_if(run_after_startup_finished.and(run_if_not_paused)),
            // )
            // .add_systems(
            //     Last,
            //     (
            //         apply_new_origin_offset_system.run_if(run_after_startup_finished.and(run_if_not_paused)),
            //         sync_logical_from_transform_system
            //             .after(apply_new_origin_offset_system)
            //             .run_if(run_after_startup_finished.and(run_if_not_paused)),
            //     ),
            // )
            ;
    }
}

#[macro_export]
macro_rules! grid_extent {
    ([$first:expr $(, $rest:expr)*]) => {
        {
            use crate::bevy::math::IVec2;
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
            use crate::bevy::math::IVec2;
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
            use crate::bevy::math::{IVec2, Vec2};
            use crate::usf::pos::unit::types::UnitVec;

            let stack = vec![IVec2::from($first) $(, IVec2::from($rest))*];
            UnitVec::try_from((stack, Vec2::from($unit))).unwrap()
        }
    }};
}
