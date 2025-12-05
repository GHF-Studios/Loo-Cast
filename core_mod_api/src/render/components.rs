use std::marker::PhantomData;

use bevy::prelude::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct UiCamera;

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
