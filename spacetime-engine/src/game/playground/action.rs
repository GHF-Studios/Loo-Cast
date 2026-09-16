//! Device-agnostic playground action messages.
//!
//! Local input produces semantic item actions. Individual item plugins consume
//! only actions addressed to their item ID, which keeps item semantics usable
//! from keyboard/mouse, AI, replay, networking or mods without duplicating
//! device handling.

use bevy::prelude::*;

use super::PlaygroundItemId;

/// World-space origin and direction used by playground tools.
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

/// Extensible logical action ID for an equipped playground item.
///
/// The built-in local input adapter emits press and held variants for primary/secondary use.
/// Mods can define additional IDs and produce them from their own input systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlaygroundItemAction(pub &'static str);

impl PlaygroundItemAction {
    pub const PRIMARY: Self = Self("primary");
    pub const SECONDARY: Self = Self("secondary");
    pub const PRIMARY_HELD: Self = Self("primary_held");
    pub const SECONDARY_HELD: Self = Self("secondary_held");
    pub const RELOAD: Self = Self("reload");

    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

/// One frame's actor + aim snapshot for the built-in local playground user.
#[derive(Debug, Clone, Copy)]
pub struct PlaygroundAimContext {
    pub actor: Entity,
    pub ray: AimRay,
}

/// Shared local aiming state.
///
/// Presentation helpers (laser sights, previews, highlights) can read this
/// resource without depending on the concrete player camera or input devices.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct PlaygroundAim {
    current: Option<PlaygroundAimContext>,
}

impl PlaygroundAim {
    pub fn current(&self) -> Option<PlaygroundAimContext> {
        self.current
    }

    pub(crate) fn set(&mut self, current: Option<PlaygroundAimContext>) {
        self.current = current;
    }
}

/// One semantic use of an equipped item.
#[derive(Message, Debug, Clone, Copy)]
pub struct UsePlaygroundItem {
    pub item: PlaygroundItemId,
    pub action: PlaygroundItemAction,
    pub actor: Entity,
    pub aim: AimRay,
}

/// Global sandbox deletion request. This intentionally remains separate from
/// item actions; the built-in mouse adapter maps it to middle-click.
#[derive(Message, Debug, Clone, Copy)]
pub struct ErasePlaygroundObject {
    pub aim: AimRay,
}
