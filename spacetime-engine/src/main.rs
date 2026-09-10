use bevy::prelude::*;
use spacetime_engine::ecs::component_conflict::ComponentConflictPlugin;
use spacetime_engine::{OriginalUsfEntity, ProxyMutableUsfEntity};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ComponentConflictPlugin)
        .run();
}
