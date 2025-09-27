use bevy::prelude::{Resource, Reflect, ReflectResource};

use crate::config::statics::CONFIG;

#[derive(Resource, Reflect)]
#[reflect(Resource)]
pub(crate) struct ZoomFactor(pub f32);
impl Default for ZoomFactor {
    fn default() -> Self {
        Self(CONFIG().get::<f32>("camera/default_zoom"))
    }
}
