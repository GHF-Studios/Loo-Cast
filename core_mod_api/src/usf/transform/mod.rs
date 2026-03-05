pub mod types;

#[cfg(test)]
mod tests;

use crate::bevy::prelude::*;
use types::{UsfFloat, UsfFloatPolicy, UsfRotation, UsfScale, UsfTransform, UsfTranslation};

pub(crate) struct TransformPlugin;
impl Plugin for TransformPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UsfFloatPolicy>()
            .register_type::<UsfFloat>()
            .register_type::<UsfTranslation>()
            .register_type::<UsfScale>()
            .register_type::<UsfRotation>()
            .register_type::<UsfTransform>();
    }
}

