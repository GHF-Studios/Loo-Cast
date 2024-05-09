pub mod structs;

use bevy::prelude::*;
use crate::entity::id::structs::*;

pub struct IDPlugin;

impl Plugin for IDPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<EntityID>();
    }
}