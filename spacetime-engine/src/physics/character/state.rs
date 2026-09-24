use bevy::prelude::*;

/// One walkable support contact currently held by the character motor.
///
/// `collider` is the actual collider hit by the shape query. It is deliberately
/// not called a "ground entity": a future moving-platform adapter may resolve
/// that collider to another rigid-body/semantic owner.
#[derive(Reflect, Clone, Copy, Debug, PartialEq)]
pub struct CharacterGroundContact {
    collider: Entity,
    normal: Vec3,
}

impl CharacterGroundContact {
    pub(crate) fn new(collider: Entity, normal: Vec3) -> Self {
        Self {
            collider,
            normal: normal.normalize_or_zero(),
        }
    }

    pub const fn collider(self) -> Entity {
        self.collider
    }

    pub const fn normal(self) -> Vec3 {
        self.normal
    }
}

/// Runtime walkable-support state produced by [`super::CharacterMotor`].
///
/// Groundedness has exactly one source of truth: `contact.is_some()`. Transition
/// flags describe changes during the current fixed tick and cannot disagree with
/// a second grounded boolean/entity/normal tuple.
#[derive(Component, Reflect, Clone, Debug, Default)]
#[reflect(Component)]
pub struct CharacterGroundState {
    contact: Option<CharacterGroundContact>,
    just_landed: bool,
    just_left_ground: bool,
    just_jumped: bool,
}

impl CharacterGroundState {
    pub const fn contact(&self) -> Option<CharacterGroundContact> {
        self.contact
    }

    pub const fn is_grounded(&self) -> bool {
        self.contact.is_some()
    }

    pub const fn just_landed(&self) -> bool {
        self.just_landed
    }

    pub const fn just_left_ground(&self) -> bool {
        self.just_left_ground
    }

    pub const fn just_jumped(&self) -> bool {
        self.just_jumped
    }

    pub(crate) fn begin_tick(&mut self) {
        self.just_landed = false;
        self.just_left_ground = false;
        self.just_jumped = false;
    }

    pub(crate) fn set_contact(&mut self, contact: Option<CharacterGroundContact>) {
        self.contact = contact;
    }

    /// Clears local support/contact and transient transition outputs.
    ///
    /// Use this for explicit runtime discontinuities such as teleport, control
    /// mode changes, death or Scale-Slice recharting.
    pub(crate) fn clear_contact(&mut self) {
        self.contact = None;
        self.begin_tick();
    }

    pub(crate) fn clear_for_rechart(&mut self) {
        self.clear_contact();
    }

    pub(crate) fn jump_from_ground(&mut self) {
        debug_assert!(self.is_grounded());
        self.contact = None;
        self.just_jumped = true;
        self.just_left_ground = true;
    }

    pub(crate) fn finish_tick(&mut self, was_grounded: bool) {
        let grounded = self.is_grounded();
        if !was_grounded && grounded {
            self.just_landed = true;
        }
        if was_grounded && !grounded && !self.just_jumped {
            self.just_left_ground = true;
        }
    }

    /// Invalidates local support because a topology transaction moved the body
    /// into another collision space.
    pub(crate) fn invalidate_contact(&mut self) {
        let was_grounded = self.contact.take().is_some();
        self.just_landed = false;
        if was_grounded {
            self.just_left_ground = true;
        }
    }
}
