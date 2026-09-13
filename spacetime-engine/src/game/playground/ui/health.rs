use std::collections::{HashMap, HashSet};

use bevy::{
    light::{NotShadowCaster, NotShadowReceiver},
    prelude::*,
    transform::TransformSystems,
};

use crate::{
    ecs::UsfManifestationOf,
    game::{
        GameSet,
        combat::{Health, Hitbox},
        player::{Player, PlayerCamera},
    },
    physics::topology::{SpatialSplitPeer, SpatialSplitPeerActive},
};

const PLAYER_BAR_WIDTH_PX: f32 = 320.0;
const PLAYER_BAR_HEIGHT_PX: f32 = 16.0;
const PLAYER_BAR_BOTTOM_PX: f32 = 78.0;
const PLAYER_BAR_BORDER_PX: f32 = 2.0;

const WORLD_BAR_WIDTH: f32 = 1.1;
const WORLD_BAR_HEIGHT: f32 = 0.11;
const WORLD_BAR_DEPTH: f32 = 0.025;
const WORLD_BAR_BORDER: f32 = 0.018;
const WORLD_BAR_GAP: f32 = 0.28;
const WORLD_BAR_DEFAULT_HALF_HEIGHT: f32 = 0.5;
const WORLD_BAR_FILL_Z_OFFSET: f32 = 0.02;

const FRAME_COLOR: Color = Color::srgb(0.035, 0.035, 0.04);
const FRAME_BORDER_COLOR: Color = Color::srgb(0.65, 0.65, 0.68);
const FILL_COLOR: Color = Color::srgb(0.82, 0.025, 0.035);

#[derive(Component)]
struct PlayerHealthFill;

#[derive(Component)]
struct WorldHealthBarVisual;

#[derive(Resource)]
struct WorldHealthBarAssets {
    mesh: Handle<Mesh>,
    frame_material: Handle<StandardMaterial>,
    fill_material: Handle<StandardMaterial>,
}

#[derive(Debug, Clone, Copy)]
struct CachedWorldHealthBar {
    frame: Entity,
    fill: Entity,
}

#[derive(Resource, Default)]
struct WorldHealthBarCache {
    bars: HashMap<Entity, CachedWorldHealthBar>,
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<WorldHealthBarCache>()
        .add_systems(
            Startup,
            (spawn_player_health_bar, setup_world_health_bar_assets),
        )
        .add_systems(
            Update,
            sync_player_health_bar.in_set(GameSet::Presentation),
        )
        .add_systems(
            PostUpdate,
            sync_world_health_bars.after(TransformSystems::Propagate),
        );
}

fn spawn_player_health_bar(mut commands: Commands) {
    commands
        .spawn((
            Name::new("Player Health Bar"),
            Node {
                position_type: PositionType::Absolute,
                bottom: px(PLAYER_BAR_BOTTOM_PX),
                left: percent(50.0),
                margin: UiRect::left(px(-PLAYER_BAR_WIDTH_PX * 0.5)),
                width: px(PLAYER_BAR_WIDTH_PX),
                height: px(PLAYER_BAR_HEIGHT_PX),
                border: UiRect::all(px(PLAYER_BAR_BORDER_PX)),
                ..default()
            },
            BackgroundColor(FRAME_COLOR),
            BorderColor::all(FRAME_BORDER_COLOR),
        ))
        .with_children(|bar| {
            bar.spawn((
                Name::new("Player Health Fill"),
                PlayerHealthFill,
                Node {
                    width: percent(100.0),
                    height: percent(100.0),
                    ..default()
                },
                BackgroundColor(FILL_COLOR),
            ));
        });
}

fn sync_player_health_bar(
    player: Query<&UsfManifestationOf, With<Player>>,
    health: Query<&Health>,
    mut fill: Single<&mut Node, With<PlayerHealthFill>>,
) {
    let fraction = player
        .iter()
        .next()
        .and_then(|manifestation| health.get(manifestation.0).ok())
        .map(health_fraction)
        .unwrap_or(0.0);

    fill.width = percent(fraction * 100.0);
}

fn setup_world_health_bar_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mesh = meshes.add(Cuboid::from_length(1.0));
    let frame_material = materials.add(StandardMaterial {
        base_color: FRAME_COLOR,
        unlit: true,
        cull_mode: None,
        ..default()
    });
    let fill_material = materials.add(StandardMaterial {
        base_color: FILL_COLOR,
        unlit: true,
        cull_mode: None,
        ..default()
    });

    commands.insert_resource(WorldHealthBarAssets {
        mesh,
        frame_material,
        fill_material,
    });
}

fn sync_world_health_bars(
    mut commands: Commands,
    assets: Res<WorldHealthBarAssets>,
    mut cache: ResMut<WorldHealthBarCache>,
    cameras: Query<&GlobalTransform, (With<PlayerCamera>, Without<WorldHealthBarVisual>)>,
    player: Query<&UsfManifestationOf, With<Player>>,
    health: Query<&Health>,
    manifestations: Query<
        (Entity, &UsfManifestationOf, &GlobalTransform, Option<&Hitbox>),
        (
            Or<(Without<SpatialSplitPeer>, With<SpatialSplitPeerActive>)>,
            Without<WorldHealthBarVisual>,
        ),
    >,
    mut visuals: Query<
        (&mut Transform, &mut GlobalTransform, &mut Visibility),
        With<WorldHealthBarVisual>,
    >,
) {
    let Some(camera_rotation) = cameras
        .iter()
        .next()
        .map(|camera| camera.compute_transform().rotation)
    else {
        return;
    };
    let player_semantic = player.iter().next().map(|manifestation| manifestation.0);

    let mut seen = HashSet::new();

    // Health is semantic state. Render one bar for each currently active
    // spatial manifestation rather than copying Health onto presentation entities.
    for (entity, manifestation, transform, hitbox) in &manifestations {
        if Some(manifestation.0) == player_semantic {
            continue;
        }
        let Ok(health) = health.get(manifestation.0) else {
            continue;
        };

        seen.insert(entity);
        let fraction = health_fraction(health);
        let (frame_transform, fill_transform) = world_bar_transforms(
            transform.compute_transform(),
            hitbox.map(|hitbox| hitbox.half_extents),
            fraction,
            camera_rotation,
        );
        let fill_visible = fraction > 0.0;

        if let Some(cached) = cache.bars.get(&entity).copied() {
            let frame_alive = if let Ok((mut transform, mut global, mut visibility)) =
                visuals.get_mut(cached.frame)
            {
                *transform = frame_transform;
                *global = GlobalTransform::from(frame_transform);
                *visibility = Visibility::Visible;
                true
            } else {
                false
            };

            let fill_alive = if let Ok((mut transform, mut global, mut visibility)) =
                visuals.get_mut(cached.fill)
            {
                *transform = fill_transform;
                *global = GlobalTransform::from(fill_transform);
                *visibility = if fill_visible {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                };
                true
            } else {
                false
            };

            if frame_alive && fill_alive {
                continue;
            }

            if frame_alive {
                commands.entity(cached.frame).despawn();
            }
            if fill_alive {
                commands.entity(cached.fill).despawn();
            }
            cache.bars.remove(&entity);
        }

        let frame = commands
            .spawn((
                Name::new(format!("World Health Bar Frame {entity:?}")),
                WorldHealthBarVisual,
                NotShadowCaster,
                NotShadowReceiver,
                Mesh3d(assets.mesh.clone()),
                MeshMaterial3d(assets.frame_material.clone()),
                frame_transform,
                GlobalTransform::from(frame_transform),
                Visibility::Visible,
            ))
            .id();
        let fill = commands
            .spawn((
                Name::new(format!("World Health Bar Fill {entity:?}")),
                WorldHealthBarVisual,
                NotShadowCaster,
                NotShadowReceiver,
                Mesh3d(assets.mesh.clone()),
                MeshMaterial3d(assets.fill_material.clone()),
                fill_transform,
                GlobalTransform::from(fill_transform),
                if fill_visible {
                    Visibility::Visible
                } else {
                    Visibility::Hidden
                },
            ))
            .id();

        cache
            .bars
            .insert(entity, CachedWorldHealthBar { frame, fill });
    }

    cache.bars.retain(|source, cached| {
        if seen.contains(source) {
            true
        } else {
            commands.entity(cached.frame).despawn();
            commands.entity(cached.fill).despawn();
            false
        }
    });
}

fn world_bar_transforms(
    source: Transform,
    half_extents: Option<Vec3>,
    fraction: f32,
    camera_rotation: Quat,
) -> (Transform, Transform) {
    let half_height = half_extents
        .map(|half_extents| {
            projected_world_half_height(source.rotation, half_extents * source.scale.abs())
        })
        .unwrap_or(WORLD_BAR_DEFAULT_HALF_HEIGHT);
    let center = source.translation + Vec3::Y * (half_height + WORLD_BAR_GAP);

    let frame = Transform {
        translation: center,
        rotation: camera_rotation,
        scale: Vec3::new(WORLD_BAR_WIDTH, WORLD_BAR_HEIGHT, WORLD_BAR_DEPTH),
    };

    let inner_width = WORLD_BAR_WIDTH - WORLD_BAR_BORDER * 2.0;
    let inner_height = WORLD_BAR_HEIGHT - WORLD_BAR_BORDER * 2.0;
    let fill_width = inner_width * fraction;
    let rendered_fill_width = fill_width.max(0.001);
    let local_x = -(inner_width - fill_width) * 0.5;
    let fill_center = center
        + camera_rotation * Vec3::new(local_x, 0.0, WORLD_BAR_FILL_Z_OFFSET);
    let fill = Transform {
        translation: fill_center,
        rotation: camera_rotation,
        scale: Vec3::new(rendered_fill_width, inner_height, WORLD_BAR_DEPTH * 0.45),
    };

    (frame, fill)
}

fn projected_world_half_height(rotation: Quat, half_extents: Vec3) -> f32 {
    let x = rotation * Vec3::X;
    let y = rotation * Vec3::Y;
    let z = rotation * Vec3::Z;

    x.y.abs() * half_extents.x + y.y.abs() * half_extents.y + z.y.abs() * half_extents.z
}

fn health_fraction(health: &Health) -> f32 {
    (health.current() / health.maximum()).clamp(0.0, 1.0)
}
