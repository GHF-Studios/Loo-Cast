//! Enter/exit control transactions and embarked presentation state.
//!
//! ## Module map
//!
//! - `entry`: Boarding selection and semantic control-transfer request.
//! - `exit`: Disembark transaction after a walkable pose is proven.
//! - `pose`: Physical standing-pose proof for disembark.
//! - `visibility`: Embarked player presentation follows semantic constituency.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

use super::*;

mod entry;
mod exit;
mod pose;
mod visibility;

pub(super) use entry::handle_ship_entry;
pub(super) use exit::handle_ship_exit;
use pose::resolve_disembark_pose;
pub(super) use visibility::sync_embarked_player_visibility;
