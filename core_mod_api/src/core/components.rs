use bevy::prelude::*;
use std::marker::PhantomData;

/// A generic marker component to indicate that an entity's ownership of a certain component should be considered "meta" by certain systems which need to be split into "diegetic" and "meta" operational modes.
/// For example, this can be used to mark entities that own `Sprite` components but should not partake in regular sprite picking; such as sprite-based selection outlines, sprite-based gizmos, sprite-based HUD elements, etc.
#[derive(Component, Default, Reflect)]
#[reflect(Component)]
pub struct Meta<T: Component>(#[reflect(ignore)] PhantomData<T>);