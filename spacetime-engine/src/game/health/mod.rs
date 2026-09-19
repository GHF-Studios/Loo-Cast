//! Generic damageable-state, damage and death semantics.
//!
//! Combat, thermal hazards and other gameplay mechanisms may emit `Damage`.
//! This domain alone owns authoritative `Damage -> Health -> Died` resolution.

use bevy::prelude::*;

use super::GameSet;

#[derive(Component, Debug, Clone, Copy)]
pub struct Health {
    current: f32,
    maximum: f32,
}

impl Health {
    pub fn new(maximum: f32) -> Self {
        assert!(maximum.is_finite() && maximum > 0.0);

        Self {
            current: maximum,
            maximum,
        }
    }

    pub fn current(&self) -> f32 {
        self.current
    }

    pub fn maximum(&self) -> f32 {
        self.maximum
    }

    pub fn is_alive(&self) -> bool {
        self.current > 0.0
    }

    fn damage(&mut self, amount: f32) -> bool {
        if amount <= 0.0 || !amount.is_finite() || !self.is_alive() {
            return false;
        }

        self.current = (self.current - amount).max(0.0);

        self.current == 0.0
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Hitbox {
    pub half_extents: Vec3,
}

impl Hitbox {
    pub fn cube(size: f32) -> Self {
        Self {
            half_extents: Vec3::splat(size / 2.0),
        }
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct Damage {
    pub target: Entity,
    pub instigator: Option<Entity>,
    pub amount: f32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct Died {
    pub entity: Entity,
    pub instigator: Option<Entity>,
}

mod damage;
mod presentation;
mod thermal_injury;

use damage::apply_damage;

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HealthSet {
    ApplyDamage,
}

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<Damage>()
            .add_message::<Died>()
            .add_systems(
                Update,
                apply_damage
                    .in_set(GameSet::Consequence)
                    .in_set(HealthSet::ApplyDamage),
            );

        presentation::configure(app);
        thermal_injury::configure(app);
    }
}
