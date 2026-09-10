//! Lifecycle of the game's single test target.
//!
//! [`TargetState`] is the sole authority over which entity, if any, is the
//! current target. Other mechanics do not mutate this resource.

use bevy::prelude::*;

use super::{
    GameAssets, GameSet,
    combat::{Died, Health, Hitbox},
};

/// Marks the test target.
#[derive(Component)]
pub struct TargetCube;

/// Requests creation of the target.
///
/// Multiple requests cannot create multiple targets.
#[derive(Message, Debug, Clone, Copy)]
pub struct SpawnTarget;

/// Configurable properties of newly spawned targets.
#[derive(Resource, Debug, Clone, Copy)]
pub struct TargetConfig {
    pub position: Vec3,
    pub size: f32,
    pub maximum_health: f32,
}

impl Default for TargetConfig {
    fn default() -> Self {
        Self {
            position: Vec3::new(0.0, 1.0, -5.0),
            size: 2.0,
            maximum_health: 100.0,
        }
    }
}

/// Tracks the one permitted target.
///
/// Mutation is intentionally private to this module.
#[derive(Resource, Default, Debug)]
pub struct TargetState {
    entity: Option<Entity>,
}

impl TargetState {
    pub fn entity(&self) -> Option<Entity> {
        self.entity
    }
}

pub struct TargetPlugin;

impl Plugin for TargetPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<TargetConfig>()
            .init_resource::<TargetState>()
            .add_systems(
                Update,
                spawn_target.in_set(GameSet::Action),
            )
            .add_systems(
                Update,
                (despawn_dead_target, repair_target_state)
                    .chain()
                    .in_set(GameSet::Cleanup),
            );
    }
}

fn spawn_target(
    mut commands: Commands,
    mut requests: MessageReader<SpawnTarget>,
    config: Res<TargetConfig>,
    assets: Res<GameAssets>,
    mut state: ResMut<TargetState>,
) {
    if requests.is_empty() {
        return;
    }

    // A burst of requests still means "ensure a target exists".
    requests.clear();

    if state.entity.is_some() {
        return;
    }

    let entity = commands
        .spawn((
            Name::new("Target Cube"),
            TargetCube,
            Health::new(config.maximum_health),
            Hitbox::cube(config.size),
            Mesh3d(assets.target_mesh.clone()),
            MeshMaterial3d(assets.target_material.clone()),
            Transform::from_translation(config.position),
        ))
        .id();

    state.entity = Some(entity);
}

fn despawn_dead_target(
    mut commands: Commands,
    mut deaths: MessageReader<Died>,
    mut state: ResMut<TargetState>,
) {
    for death in deaths.read() {
        if state.entity != Some(death.entity) {
            continue;
        }

        commands.entity(death.entity).despawn();
        state.entity = None;
    }
}

/// Repairs the singleton handle if some external mechanic or mod despawned
/// the target directly.
fn repair_target_state(
    mut state: ResMut<TargetState>,
    targets: Query<(), With<TargetCube>>,
) {
    let Some(entity) = state.entity else {
        return;
    };

    if !targets.contains(entity) {
        state.entity = None;
    }
}