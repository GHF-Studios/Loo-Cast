//! Generic health, weapons, projectiles, hits, damage and death.
//!
//! The important dependency direction is:
//!
//! `Projectile -> Hit -> Damage -> Health -> Died`
//!
//! Projectiles therefore do not know how health works, and health does not
//! know what caused damage.

use bevy::prelude::*;

use super::{GameAssets, GameSet};

/// Generic finite health state.
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

    /// Applies positive damage.
    ///
    /// Returns `true` only when this operation causes the transition from
    /// alive to dead.
    fn damage(&mut self, amount: f32) -> bool {
        if amount <= 0.0 || !amount.is_finite() || !self.is_alive() {
            return false;
        }

        self.current = (self.current - amount).max(0.0);

        self.current == 0.0
    }
}

/// Simple world-axis-aligned interaction volume.
///
/// This is intentionally not a physics-engine abstraction. It exists only to
/// give the baseline game a deterministic collision surface.
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

    fn contains(&self, point: Vec3, transform: &Transform) -> bool {
        let offset = point - transform.translation;

        offset.x.abs() <= self.half_extents.x
            && offset.y.abs() <= self.half_extents.y
            && offset.z.abs() <= self.half_extents.z
    }
}

/// Configures projectile creation for an entity capable of firing.
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

/// A request for an entity to fire its weapon.
///
/// Input, AI or mods may all issue the same request.
#[derive(Message, Debug, Clone, Copy)]
pub struct FireWeapon {
    pub wielder: Entity,
}

/// A concrete projectile currently travelling through the world.
#[derive(Component, Debug, Clone, Copy)]
pub struct Projectile {
    pub instigator: Entity,
    pub velocity: Vec3,
    pub remaining_lifetime: f32,
    pub damage: f32,
}

/// States that a projectile physically hit another entity.
///
/// This does not itself modify health.
#[derive(Message, Debug, Clone, Copy)]
pub struct Hit {
    pub projectile: Entity,
    pub instigator: Entity,
    pub target: Entity,
    pub position: Vec3,
    pub damage: f32,
}

/// Requests a semantic health reduction.
///
/// Damage is intentionally independent of bullets: arbitrary mechanics may
/// produce this message.
#[derive(Message, Debug, Clone, Copy)]
pub struct Damage {
    pub target: Entity,
    pub instigator: Option<Entity>,
    pub amount: f32,
}

/// States that an entity's [`Health`] transitioned from alive to dead.
#[derive(Message, Debug, Clone, Copy)]
pub struct Died {
    pub entity: Entity,
    pub instigator: Option<Entity>,
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            fire_weapons.in_set(GameSet::Action),
        )
            .add_systems(
                Update,
                simulate_projectiles.in_set(GameSet::Simulation),
            )
            .add_systems(
                Update,
                (hits_to_damage, apply_damage)
                    .chain()
                    .in_set(GameSet::Consequence),
            );
    }
}

fn fire_weapons(
    mut commands: Commands,
    mut requests: MessageReader<FireWeapon>,
    weapons: Query<(&Weapon, &Transform)>,
    assets: Res<GameAssets>,
) {
    for request in requests.read() {
        let Ok((weapon, transform)) =
            weapons.get(request.wielder)
        else {
            continue;
        };

        let forward = transform.rotation * Vec3::NEG_Z;

        commands.spawn((
            Name::new("Projectile"),
            Projectile {
                instigator: request.wielder,
                velocity: forward * weapon.projectile_speed,
                remaining_lifetime: weapon.projectile_lifetime,
                damage: weapon.damage,
            },
            Mesh3d(assets.projectile_mesh.clone()),
            MeshMaterial3d(
                assets.projectile_material.clone(),
            ),
            Transform::from_translation(
                transform.translation + forward * 0.5,
            ),
        ));
    }
}

fn simulate_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut hits: MessageWriter<Hit>,
    mut projectiles: Query<(
        Entity,
        &mut Projectile,
        &mut Transform,
    )>,
    hitboxes: Query<
        (Entity, &Hitbox, &Transform),
        Without<Projectile>,
    >,
) {
    let delta = time.delta_secs();

    for (entity, mut projectile, mut transform) in
        &mut projectiles
    {
        transform.translation += projectile.velocity * delta;
        projectile.remaining_lifetime -= delta;

        let impact = hitboxes
            .iter()
            .filter(|(target, _, _)| {
                *target != projectile.instigator
            })
            .filter_map(|(target, hitbox, target_transform)| {
                hitbox
                    .contains(transform.translation, target_transform)
                    .then_some((
                        target,
                        transform
                            .translation
                            .distance_squared(
                                target_transform.translation,
                            ),
                    ))
            })
            .min_by(|a, b| a.1.total_cmp(&b.1));

        if let Some((target, _)) = impact {
            hits.write(Hit {
                projectile: entity,
                instigator: projectile.instigator,
                target,
                position: transform.translation,
                damage: projectile.damage,
            });

            commands.entity(entity).despawn();
            continue;
        }

        if projectile.remaining_lifetime <= 0.0 {
            commands.entity(entity).despawn();
        }
    }
}

fn hits_to_damage(
    mut hits: MessageReader<Hit>,
    mut damage: MessageWriter<Damage>,
) {
    for hit in hits.read() {
        damage.write(Damage {
            target: hit.target,
            instigator: Some(hit.instigator),
            amount: hit.damage,
        });
    }
}

fn apply_damage(
    mut damage: MessageReader<Damage>,
    mut health: Query<&mut Health>,
    mut died: MessageWriter<Died>,
) {
    for damage in damage.read() {
        let Ok(mut health) = health.get_mut(damage.target) else {
            continue;
        };

        if health.damage(damage.amount) {
            died.write(Died {
                entity: damage.target,
                instigator: damage.instigator,
            });
        }
    }
}