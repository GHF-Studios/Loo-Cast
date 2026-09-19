//! Weapons, projectiles and manifestation-space hit detection.
//!
//! `Projectile -> Hit -> health::Damage`

use avian3d::prelude::Collider;
use bevy::prelude::*;

use crate::{
    ecs::UsfManifestationOf,
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
    portal::{PortalTraveler, PortalVelocity},
};

use super::{
    GameSet, SimulationSet,
    health::{Damage, HealthSet, Hitbox},
};

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

mod assets;
mod damage;
mod projectile;
mod weapon;

use assets::{CombatPresentationAssets, setup_combat_assets};
use damage::hits_to_damage;
use projectile::{detect_projectile_hits, move_projectiles};
use weapon::fire_weapons;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<FireWeapon>()
            .add_message::<Hit>()
            .add_systems(Startup, setup_combat_assets)
            .add_systems(Update, fire_weapons.in_set(GameSet::Action))
            .add_systems(Update, move_projectiles.in_set(SimulationSet::Motion))
            .add_systems(
                Update,
                detect_projectile_hits.in_set(SimulationSet::Collision),
            )
            .add_systems(
                Update,
                hits_to_damage
                    .in_set(GameSet::Consequence)
                    .before(HealthSet::ApplyDamage),
            );
    }
}
