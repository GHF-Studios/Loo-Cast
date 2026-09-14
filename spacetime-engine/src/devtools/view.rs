use bevy::prelude::*;

use crate::view::ViewRay;

/// Viewpoint and interaction ray used by developer tools.
///
/// This is deliberately separate from [`super::DeveloperFocus`]: the view
/// answers "where/from what surface are tools observing?", while focus answers
/// "which thing is being inspected?". The interaction ray is already world-space,
/// so consumers never need to know whether the view fills a window or is embedded.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct DeveloperView {
    observer: Option<Entity>,
    interaction_ray: Option<ViewRay>,
}

impl DeveloperView {
    pub fn observer(&self) -> Option<Entity> {
        self.observer
    }

    pub fn interaction_ray(&self) -> Option<ViewRay> {
        self.interaction_ray
    }

    pub fn set_observer(&mut self, observer: Option<Entity>) {
        self.observer = observer;
    }

    pub fn set_interaction_ray(&mut self, interaction_ray: Option<ViewRay>) {
        self.interaction_ray = interaction_ray;
    }
}
