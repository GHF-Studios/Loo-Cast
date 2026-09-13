use bevy::prelude::*;

/// Viewpoint used by developer tools that need observer-relative presentation.
///
/// This is deliberately separate from [`super::DeveloperFocus`]: the observer
/// answers "where are developer tools viewed from?", while focus answers
/// "which thing is being inspected?".
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct DeveloperView {
    observer: Option<Entity>,
}

impl DeveloperView {
    pub fn observer(&self) -> Option<Entity> {
        self.observer
    }

    pub fn set_observer(&mut self, observer: Option<Entity>) {
        self.observer = observer;
    }
}
