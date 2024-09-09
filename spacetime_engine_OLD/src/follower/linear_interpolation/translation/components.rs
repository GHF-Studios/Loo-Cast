use bevy::prelude::*;
use crate::{entity::id::structs::*, operations::InstanceID};

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TranslationLerpFollower {
    pub target: InstanceID<Entity>,
    pub smoothness: f32, // Higher values mean slower following (less smooth)
}