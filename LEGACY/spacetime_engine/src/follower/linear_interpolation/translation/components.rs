use bevy::prelude::*;
use crate::entity::id::structs::*;

#[derive(Component, Debug, Reflect)]
#[reflect(Component)]
pub struct TranslationLerpFollower {
    pub target: EntityID,
    pub smoothness: f32, // Higher values mean slower following (less smooth)
}