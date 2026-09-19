//! Device-agnostic semantic item actions.
//!
//! Local input produces these messages once. Individual item plugins consume
//! only actions addressed to their item ID, which keeps item semantics usable
//! from keyboard/mouse, AI, replay, networking or mods without duplicating
//! device handling.

use bevy::prelude::*;

use super::ItemId;

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

/// Extensible logical action ID for an equipped item.
///
/// The built-in local input adapter emits the three conventional actions below.
/// Mods can define additional IDs and produce them from their own input systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ItemAction(pub &'static str);

impl ItemAction {
    pub const PRIMARY: Self = Self("primary");
    pub const SECONDARY: Self = Self("secondary");
    pub const RELOAD: Self = Self("reload");

    pub const fn new(value: &'static str) -> Self {
        Self(value)
    }
}

/// One frame's actor + aim snapshot for the built-in local item user.
#[derive(Debug, Clone, Copy)]
pub struct ItemAimContext {
    pub actor: Entity,
    pub ray: AimRay,
}

/// Shared local aiming state.
///
/// Presentation helpers (laser sights, previews, highlights) can read this
/// resource without depending on the concrete player camera or input devices.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct ItemAim {
    current: Option<ItemAimContext>,
}

impl ItemAim {
    pub fn current(&self) -> Option<ItemAimContext> {
        self.current
    }

    pub(crate) fn set(&mut self, current: Option<ItemAimContext>) {
        self.current = current;
    }
}

/// One semantic use of an equipped item.
#[derive(Message, Debug, Clone, Copy)]
pub struct UseItem {
    pub item: ItemId,
    pub action: ItemAction,
    pub actor: Entity,
    pub aim: AimRay,
}
