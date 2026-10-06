//! Enter/exit control transactions and embarked presentation state.

use super::*;

mod entry;
mod exit;
mod pose;
mod visibility;

pub(super) use entry::handle_ship_entry;
pub(super) use exit::handle_ship_exit;
use pose::resolve_disembark_pose;
pub(super) use visibility::enforce_embarked_player_hidden;
