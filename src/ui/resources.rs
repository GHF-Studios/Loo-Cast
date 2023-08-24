use bevy::prelude::*;

#[derive(Resource)]
pub struct FocusManager {
    pub focus: Option<Entity>,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self { focus: None }
    }
}
