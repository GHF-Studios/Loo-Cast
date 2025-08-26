pub mod structs;

use bevy::prelude::*;
use crate::player::id::structs::*;

pub struct IDPlugin;

impl Plugin for IDPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<PlayerID>()
            .register_type::<PlayerRequestID>();
    }
}