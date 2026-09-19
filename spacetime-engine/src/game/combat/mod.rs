//! Health, weapons, projectiles, hits, damage and death.
//!
//! `Projectile -> Hit -> Damage -> Health -> Died`

use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

use super::{
    GameAssets, GameSet, SimulationSet,
    portal::{PortalTraveler, PortalVelocity},
};

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

#[derive(Component, Debug, Clone, Copy)]
pub struct Weapon {
    pub projectile_speed: f32,
    pub projectile_lifetime: f32,
    pub damage: f32,
}

impl Default for Weapon {
    fn default() -> Self {
        Self {
            projectile_speed: 20.0,
            projectile_lifetime: 3.0,
            damage: 25.0,
        }
    }
}

#[derive(Message, Debug, Clone, Copy)]
pub struct FireWeapon {
    pub wielder: Entity,
    pub origin: Vec3,
    pub direction: Vec3,
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Projectile {
    pub instigator: Entity,
    pub remaining_lifetime: f32,
    pub damage: f32,
}

#[derive(Message, Debug, Clone, Copy)]
pub struct Hit {
    pub projectile: Entity,
    pub instigator: Entity,
    pub target: Entity,
    pub position: Vec3,
    pub damage: f32,
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
mod projectile;
mod weapon;

use damage::{apply_damage, hits_to_damage};
use projectile::{detect_projectile_hits, move_projectiles};
use weapon::fire_weapons;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, fire_weapons.in_set(GameSet::Action))
            .add_systems(Update, move_projectiles.in_set(SimulationSet::Motion))
            .add_systems(
                Update,
                detect_projectile_hits.in_set(SimulationSet::Collision),
            )
            .add_systems(
                Update,
                (hits_to_damage, apply_damage)
                    .chain()
                    .in_set(GameSet::Consequence),
            );
    }
}
