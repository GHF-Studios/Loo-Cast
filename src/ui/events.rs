use bevy::prelude::*;

#[derive(Event)]
pub struct GainedFocus {
    pub entity: Entity,
}

#[derive(Event)]
pub struct LostFocus {
    pub entity: Entity,
}
