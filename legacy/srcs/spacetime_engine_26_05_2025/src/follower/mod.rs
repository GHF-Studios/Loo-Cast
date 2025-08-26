pub mod linear_interpolation;

use linear_interpolation::*;
use bevy::prelude::*;

pub(in crate) struct FollowerPlugin;

impl Plugin for FollowerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(LinearInterpolationPlugin);
    }
}