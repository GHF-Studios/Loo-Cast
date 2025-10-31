pub mod components;
pub mod functions;
pub mod systems;

use bevy::prelude::*;
use components::{RenderProxyHandle, RenderProxy};
use systems::{update_render_proxies, despawn_orphaned_render_proxies};

use crate::core::run_conditions::run_after_startup_finished;
use crate::time::run_conditions::run_if_not_paused;

pub(crate) struct RenderPlugin;
impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, (
                update_render_proxies.run_if(run_after_startup_finished.and(run_if_not_paused)),
                despawn_orphaned_render_proxies.before(update_render_proxies).run_if(run_after_startup_finished)
            ))
            .register_type::<RenderProxyHandle>()
            .register_type::<RenderProxy>();
    }
}
