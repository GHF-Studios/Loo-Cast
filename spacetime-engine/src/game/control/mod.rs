//! Local control authority.
//!
//! `Player` is identity. `LocalControlSubject` is merely the runtime
//! manifestation currently receiving local input.

use bevy::prelude::*;

#[derive(Component, Reflect, Debug, Default, Clone, Copy)]
#[reflect(Component)]
pub struct LocalControlSubject;

#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct LocalControlState {
    controller: Option<Entity>,
    subject: Option<Entity>,
    manifestation: Option<Entity>,
}

impl LocalControlState {
    pub const fn controller(&self) -> Option<Entity> { self.controller }
    pub const fn subject(&self) -> Option<Entity> { self.subject }
    pub const fn manifestation(&self) -> Option<Entity> { self.manifestation }

    pub fn set(&mut self, controller: Entity, subject: Entity, manifestation: Entity) {
        self.controller = Some(controller);
        self.subject = Some(subject);
        self.manifestation = Some(manifestation);
    }
}

pub struct ControlPlugin;

impl Plugin for ControlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LocalControlState>()
            .register_type::<LocalControlSubject>();
    }
}
