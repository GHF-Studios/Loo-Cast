use bevy::prelude::*;

/// Spatial details that only exist when focus came from a world-space hit.
#[derive(Debug, Clone, Copy)]
pub struct FocusHit {
    pub position: Vec3,
    pub normal: Option<Vec3>,
    pub distance_meters: f32,
}

/// One canonical tooling focus target.
///
/// `spatial_entity` is the concrete ECS entity selected/hit. `semantic_entity`
/// is the logical owner of shared state; for a USF manifestation these differ.
/// `hit` is intentionally optional: Hierarchy and Structure selection are every
/// bit as real as ray focus, but inventing a fake world-space hit for them would
/// make downstream tooling subtly wrong.
#[derive(Debug, Clone, Copy)]
pub struct FocusTarget {
    pub spatial_entity: Entity,
    pub semantic_entity: Entity,
    pub hit: Option<FocusHit>,
}

impl FocusTarget {
    pub const fn entity(spatial_entity: Entity, semantic_entity: Entity) -> Self {
        Self {
            spatial_entity,
            semantic_entity,
            hit: None,
        }
    }

    pub const fn hit(
        spatial_entity: Entity,
        semantic_entity: Entity,
        hit: FocusHit,
    ) -> Self {
        Self {
            spatial_entity,
            semantic_entity,
            hit: Some(hit),
        }
    }
}

/// Canonical developer/editor entity focus.
///
/// Hover, persistent editor selection and pinning are acquisition modes for the
/// same target model rather than separate selection systems. Explicit/pinned
/// focus wins over hover; leaving the embedded editor can clear the explicit
/// selection and immediately return to live immersive hover.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct DeveloperFocus {
    hovered: Option<FocusTarget>,
    selected: Option<FocusTarget>,
    pinned: Option<FocusTarget>,
}

impl DeveloperFocus {
    pub fn hovered(&self) -> Option<FocusTarget> {
        self.hovered
    }

    pub fn selected(&self) -> Option<FocusTarget> {
        self.selected
    }

    pub fn pinned(&self) -> Option<FocusTarget> {
        self.pinned
    }

    pub fn current(&self) -> Option<FocusTarget> {
        self.pinned.or(self.selected).or(self.hovered)
    }

    pub fn set_hovered(&mut self, target: Option<FocusTarget>) {
        self.hovered = target;
    }

    pub fn select(&mut self, target: FocusTarget) {
        self.selected = Some(target);
    }

    pub fn clear_selection(&mut self) {
        self.selected = None;
    }

    pub fn pin(&mut self, target: FocusTarget) {
        self.pinned = Some(target);
    }

    pub fn pin_current(&mut self) {
        if let Some(target) = self.current() {
            self.pinned = Some(target);
        }
    }

    pub fn clear_pin(&mut self) {
        self.pinned = None;
    }
}
/// Drops tooling focus modes whose concrete or semantic entity no longer exists.
pub(super) fn prune_focus(mut focus: ResMut<DeveloperFocus>, entities: Query<Entity>) {
    let alive = |target: FocusTarget| {
        entities.contains(target.spatial_entity) && entities.contains(target.semantic_entity)
    };

    if focus.hovered.is_some_and(|target| !alive(target)) {
        focus.hovered = None;
    }
    if focus.selected.is_some_and(|target| !alive(target)) {
        focus.selected = None;
    }
    if focus.pinned.is_some_and(|target| !alive(target)) {
        focus.pinned = None;
    }
}
