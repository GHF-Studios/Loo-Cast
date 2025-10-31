use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RenderProxyHandle {
    pub proxy_entity: Entity,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct RenderProxy {
    pub source: Entity,
}

