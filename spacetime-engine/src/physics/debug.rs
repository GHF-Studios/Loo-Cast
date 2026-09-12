//! Debug adapter for the underlying Avian physics backend.

use avian3d::debug_render::{PhysicsDebugPlugin, PhysicsGizmos};
use bevy::prelude::*;

use crate::debug::{AppDebugExt, DebugView, DebugViews};

struct PhysicsBackendDebugView;

impl DebugView for PhysicsBackendDebugView {
    const NAME: &'static str = "Physics / Avian";
}

pub(crate) fn configure(app: &mut App) {
    app.register_debug_view::<PhysicsBackendDebugView>()
        .add_plugins(PhysicsDebugPlugin)
        .add_systems(PreUpdate, sync_avian_debug_visibility);
}

fn sync_avian_debug_visibility(
    views: Res<DebugViews>,
    mut store: ResMut<GizmoConfigStore>,
) {
    let (config, _) = store.config_mut::<PhysicsGizmos>();
    config.enabled = views.enabled::<PhysicsBackendDebugView>();
}
