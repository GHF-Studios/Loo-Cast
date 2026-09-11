use bevy::prelude::*;

/// Marks an entity whose movement may cross portal apertures.
///
/// The previous position enables segment/plane crossing detection rather than
/// relying on instantaneous overlap.
#[derive(Component, Debug, Default)]
pub struct PortalTraveler {
    previous_position: Option<Vec3>,
}

impl PortalTraveler {
    pub fn new(position: Vec3) -> Self {
        Self {
            previous_position: Some(position),
        }
    }

    pub(crate) fn previous_position(&self) -> Option<Vec3> {
        self.previous_position
    }

    pub(crate) fn commit_position(&mut self, position: Vec3) {
        self.previous_position = Some(position);
    }
}

/// Optional linear velocity transformed alongside a traveler.
#[derive(Component, Debug, Clone, Copy)]
pub struct PortalVelocity(pub Vec3);
