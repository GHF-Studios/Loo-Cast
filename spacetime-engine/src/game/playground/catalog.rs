use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaygroundItemId(pub &'static str);

impl PlaygroundItemId {
    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PlaygroundItem {
    pub id: PlaygroundItemId,
    pub name: &'static str,
    pub description: &'static str,
}

#[derive(Resource, Default)]
pub struct PlaygroundCatalog {
    items: Vec<PlaygroundItem>,
}

impl PlaygroundCatalog {
    pub fn register(&mut self, item: PlaygroundItem) {
        assert!(
            !self.items.iter().any(|existing| existing.id == item.id),
            "duplicate playground item id: {}",
            item.id.0,
        );

        self.items.push(item);
    }

    pub fn items(&self) -> &[PlaygroundItem] {
        &self.items
    }

    pub fn find(&self, id: PlaygroundItemId) -> Option<&PlaygroundItem> {
        self.items.iter().find(|item| item.id == id)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct AimRay {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl AimRay {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction: direction.normalize_or_zero(),
        }
    }

    pub fn horizontal_plane(self, y: f32, max_distance: f32) -> Option<Vec3> {
        if self.direction.y.abs() <= f32::EPSILON {
            return None;
        }

        let distance = (y - self.origin.y) / self.direction.y;

        if distance < 0.0 || distance > max_distance {
            return None;
        }

        Some(self.origin + self.direction * distance)
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct UsePlaygroundItem {
    pub item: PlaygroundItemId,
    pub actor: Entity,
    pub aim: AimRay,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct ErasePlaygroundObject {
    pub aim: AimRay,
}
