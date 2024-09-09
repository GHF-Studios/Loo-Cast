pub mod structs;

use bevy::prelude::*;

pub(in crate) struct MathPlugin;

impl Plugin for MathPlugin {
    fn build(&self, app: &mut App) {
        app
            .register_type::<structs::I16Vec2>()
            .register_type::<Option<Vec2>>()
            .register_type::<Option<bevy::math::Rect>>();
    }
}