#![allow(clippy::crate_in_macro_def)]

pub mod grid;
pub mod subgrid;
pub mod unit;

pub mod systems;

use crate::bevy::prelude::*;

pub(crate) struct PosPlugin;
impl Plugin for PosPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(grid::GridPlugin)
            .add_plugins(subgrid::SubgridPlugin)
            .add_plugins(unit::UnitPlugin)
            // NOTE: origin-shift authority is currently driven by USF transform fields in runtime systems
            // (player input + render pivot systems), so this legacy position manager is disabled.
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
            use crate::bevy::math::IVec3;
            use crate::usf::pos::grid::types::GridVec;

            let stack = vec![IVec3::from($first) $(, IVec3::from($rest))*];
            GridVec::try_from(stack).unwrap()
        }
    };
}

#[macro_export]
macro_rules! subgrid_extent {
    ([$first:expr $(, $rest:expr)*]: $sub:expr) => {
        {
            use crate::bevy::math::IVec3;
            use crate::usf::pos::subgrid::types::SubgridVec;

            let stack = vec![IVec3::from($first) $(, IVec3::from($rest))*];
            SubgridVec::try_from((stack, IVec3::from($sub))).unwrap()
        }
    };
}

#[macro_export]
macro_rules! unit_extent {
    ([$first:expr $(, $rest:expr)*]: $unit:expr) => {{
        {
            use crate::bevy::math::{IVec3, Vec3};
            use crate::usf::pos::unit::types::UnitVec;

            let stack = vec![IVec3::from($first) $(, IVec3::from($rest))*];
            UnitVec::try_from((stack, Vec3::from($unit))).unwrap()
        }
    }};
}
