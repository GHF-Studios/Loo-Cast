use bevy::prelude::*;

/// Spatial details of the concrete developer selection hit.
#[derive(Debug, Clone, Copy)]
pub struct FocusHit {
    pub position: Vec3,
    pub normal: Option<Vec3>,
    pub distance_meters: f32,
}

/// One inspectable thing.
///
/// `spatial_entity` is the concrete entity actually hit. `semantic_entity` is
/// the logical owner of shared state; for a USF manifestation these differ.
#[derive(Debug, Clone, Copy)]
pub struct FocusTarget {
    pub spatial_entity: Entity,
    pub semantic_entity: Entity,
    pub hit: FocusHit,
}

/// Developer selection state shared by Inspector and world visualization.
///
/// Hover follows the current look target. Pinning freezes the effective target
/// without destroying the live hover value, so unpinning resumes immediately.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct DeveloperFocus {
    hovered: Option<FocusTarget>,
    pinned: Option<FocusTarget>,
}

impl DeveloperFocus {
    pub fn hovered(&self) -> Option<FocusTarget> {
        self.hovered
    }

    pub fn pinned(&self) -> Option<FocusTarget> {
        self.pinned
    }

    pub fn current(&self) -> Option<FocusTarget> {
        self.pinned.or(self.hovered)
    }

    pub fn set_hovered(&mut self, target: Option<FocusTarget>) {
        self.hovered = target;
    }

    pub fn pin(&mut self, target: FocusTarget) {
        self.pinned = Some(target);
    }

    pub fn pin_hovered(&mut self) {
        if let Some(target) = self.hovered {
            self.pinned = Some(target);
        }
    }

    pub fn clear_pin(&mut self) {
        self.pinned = None;
    }
}
