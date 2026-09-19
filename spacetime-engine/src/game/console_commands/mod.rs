//! Loo Cast commands layered on the generic developer console.

use avian3d::prelude::LinearVelocity;
use bevy::{math::DVec3, prelude::*};

use crate::{
    console::{
        AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
    },
    ecs::UsfManifestationOf,
    physics::character::{
        CharacterGroundState, CharacterMotor, CharacterMovementInput,
    },
    portal::PortalTraveler,
    spatial::{
        SpatialScale, UsfActiveScaleLayer, UsfPosition, UsfScaleLayer, UsfScaleLayerFrames,
        UsfSpatialSet, UsfViewFrame,
    },
};

use super::{
    player::{Player, PlayerAim, PlayerNoclip},
    world::{UniverseLandmark, UniverseLandmarkIndex},
};

#[derive(Debug, Clone)]
struct ConsoleTeleport {
    label: String,
    scale: SpatialScale,
    arrival: DVec3,
    look_at: Option<DVec3>,
}

#[derive(Resource, Default)]
struct PendingConsoleTeleport(Option<ConsoleTeleport>);

pub(super) fn configure(app: &mut App) {
    app.init_resource::<PendingConsoleTeleport>()
        .add_systems(
            PostUpdate,
            apply_pending_teleport
                .after(UsfSpatialSet::SyncSemantic)
                .before(UsfSpatialSet::ViewAnchor),
        );

    app.register_console_command(
        ConsoleCommandSpec {
            name: "where",
            aliases: &["pos", "position"],
            usage: "where",
            summary: "Show player runtime, canonical and observer-scale position.",
        },
        where_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "locate",
            aliases: &["find", "landmarks"],
            usage: "locate [name|kind]",
            summary: "Locate generated structures in scale-local coordinates.",
        },
        locate_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "teleport",
            aliases: &["tp", "goto"],
            usage: "teleport <landmark> | teleport <scale> <x> <y> <z>",
            summary: "Teleport across USF scale layers using bounded native coordinates.",
        },
        teleport_command,
    );
}

fn where_command(world: &mut World, _: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    let player = {
        let mut query = world.query_filtered::<
            (&Transform, &UsfScaleLayer, &UsfManifestationOf),
            With<Player>,
        >();
        query
            .iter(world)
            .next()
            .map(|(transform, layer, manifestation)| {
                (transform.translation, layer.scale(), manifestation.0)
            })
    };

    let Some((runtime, scale, semantic_entity)) = player else {
        return ConsoleCommandResult::error("player manifestation is unavailable");
    };

    let semantic = world
        .get::<UsfPosition>(semantic_entity)
        .map(UsfPosition::format_stack)
        .unwrap_or_else(|| "<semantic position unavailable>".to_string());
    let view = world.resource::<UsfViewFrame>();

    ConsoleCommandResult::lines([
        format!(
            "runtime S{} = ({:.3}, {:.3}, {:.3})",
            scale, runtime.x, runtime.y, runtime.z
        ),
        format!(
            "observer = {:+.3} (lower S{}, transition {:.3})",
            view.continuous_exponent(),
            view.scale(),
            view.zoom()
        ),
        format!("canonical = {semantic}"),
    ])
}

fn locate_command(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    let query = invocation.args().join(" ");
    let index = world.resource::<UniverseLandmarkIndex>();
    let matches = index.find(&query);

    if matches.is_empty() {
        return ConsoleCommandResult::error(format!("no landmark matches `{query}`"));
    }

    ConsoleCommandResult::lines(matches.into_iter().map(|landmark| {
        format!(
            "{:<14} {:<12} {} — {}",
            landmark.id,
            format!("[{}]", landmark.kind),
            landmark.coordinate_label(),
            landmark.description
        )
    }))
}

fn teleport_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let args = invocation.args();
    if args.is_empty() {
        return ConsoleCommandResult::error(
            "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
        );
    }

    let target = if args.len() == 1 {
        let landmark = {
            let index = world.resource::<UniverseLandmarkIndex>();
            let matches = index.find(&args[0]);
            if matches.is_empty() {
                return ConsoleCommandResult::error(format!(
                    "no landmark matches `{}`; use `locate`",
                    args[0]
                ));
            }
            if matches.len() > 1 {
                return ConsoleCommandResult::error(format!(
                    "`{}` is ambiguous: {}",
                    args[0],
                    matches
                        .iter()
                        .map(|landmark| landmark.id)
                        .collect::<Vec<_>>()
                        .join(", ")
                ));
            }
            (*matches[0]).clone()
        };
        target_from_landmark(&landmark)
    } else if args.len() == 4 {
        let Some(scale) = parse_scale(&args[0]) else {
            return ConsoleCommandResult::error(format!("invalid USF scale `{}`", args[0]));
        };
        let coordinates = args[1..]
            .iter()
            .map(|value| value.parse::<f64>())
            .collect::<Result<Vec<_>, _>>();
        let Ok(coordinates) = coordinates else {
            return ConsoleCommandResult::error("teleport coordinates must be finite numbers");
        };
        if coordinates.iter().any(|value| !value.is_finite()) {
            return ConsoleCommandResult::error("teleport coordinates must be finite numbers");
        }

        ConsoleTeleport {
            label: format!("S{scale} coordinate"),
            scale,
            arrival: DVec3::new(coordinates[0], coordinates[1], coordinates[2]),
            look_at: None,
        }
    } else {
        return ConsoleCommandResult::error(
            "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
        );
    };

    world
        .resource_mut::<UsfViewFrame>()
        .set_continuous_exponent(target.scale.exponent() as f32);
    world.resource_mut::<PendingConsoleTeleport>().0 = Some(target.clone());

    ConsoleCommandResult::success(format!(
        "teleport queued: {} @ S{} ({:.3}, {:.3}, {:.3})",
        target.label,
        target.scale,
        target.arrival.x,
        target.arrival.y,
        target.arrival.z
    ))
}

fn target_from_landmark(landmark: &UniverseLandmark) -> ConsoleTeleport {
    ConsoleTeleport {
        label: landmark.id.to_string(),
        scale: landmark.scale,
        arrival: landmark.arrival,
        look_at: Some(landmark.look_at),
    }
}

fn parse_scale(value: &str) -> Option<SpatialScale> {
    let value = value
        .trim()
        .trim_start_matches('S')
        .trim_start_matches('s')
        .trim_start_matches('+');
    SpatialScale::new(value.parse().ok()?)
}

fn apply_pending_teleport(
    mut commands: Commands,
    mut pending: ResMut<PendingConsoleTeleport>,
    active: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    mut player: Single<
        (
            Entity,
            &mut Transform,
            &mut LinearVelocity,
            &mut PortalTraveler,
            &UsfManifestationOf,
            &mut PlayerAim,
            &mut PlayerNoclip,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<Player>,
    >,
    mut semantic_positions: Query<&mut UsfPosition>,
) {
    let Some(request) = pending.0.as_ref() else {
        return;
    };
    if active.scale() != request.scale {
        return;
    }
    let request = pending.0.take().expect("pending teleport still exists");

    let active_scale = active.scale();
    let arrival_absolute =
        frames.convert_absolute(request.arrival, request.scale, active_scale);
    let arrival_runtime = frames.runtime_from_absolute(active_scale, arrival_absolute);

    let (
        entity,
        mut transform,
        mut velocity,
        mut traveler,
        manifestation,
        mut aim,
        mut noclip,
        mut input,
        mut ground,
    ) = player.into_inner();

    transform.translation = arrival_runtime;
    velocity.0 = Vec3::ZERO;
    traveler.commit_position(arrival_runtime);
    input.clear();
    ground.grounded = false;
    ground.ground_entity = None;

    if active_scale.exponent() > 4 {
        noclip.active = true;
        commands.entity(entity).remove::<CharacterMotor>();
    }

    if let Some(look_at) = request.look_at {
        let look_absolute = frames.convert_absolute(look_at, request.scale, active_scale);
        let look_runtime = frames.runtime_from_absolute(active_scale, look_absolute);
        let direction = (look_runtime - arrival_runtime).normalize_or_zero();
        if direction != Vec3::ZERO {
            aim.yaw = (-direction.x).atan2(-direction.z);
            aim.pitch = direction.y.asin().clamp(aim.min_pitch, aim.max_pitch);
        }
    }

    let canonical_absolute = Vec3::new(
        arrival_absolute.x as f32,
        arrival_absolute.y as f32,
        arrival_absolute.z as f32,
    );
    if canonical_absolute.is_finite()
        && let Ok(canonical) =
            UsfPosition::zero(active_scale).translated_native(canonical_absolute)
        && let Ok(mut semantic) = semantic_positions.get_mut(manifestation.0)
    {
        *semantic = canonical;
    }
}
