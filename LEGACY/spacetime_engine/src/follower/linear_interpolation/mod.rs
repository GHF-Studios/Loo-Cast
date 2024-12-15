pub mod translation;

use translation::*;
use bevy::prelude::*;

pub(in crate) struct LinearInterpolationPlugin;

impl Plugin for LinearInterpolationPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(TranslationPlugin);
    }
}