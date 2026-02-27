pub mod clone_closure;
pub mod clone_lazy;
pub mod premium_box;
pub mod progress;
pub mod string;

use crate::bevy::prelude::*;

pub(crate) struct UtilsPlugin;
impl Plugin for UtilsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<premium_box::AnySendSyncPremiumBox>();
    }
}
