//! Loo Cast commands layered on the generic developer console.
//!
//! Navigation resolves destinations into canonical USF transitions. The console
//! never mutates runtime Transform coordinates directly.

use bevy::{math::DVec3, prelude::*};

use crate::{
    console::{
        AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
    },
    ecs::UsfManifestationOf,
    physics::character::{CharacterGroundState, CharacterMovementInput},
    portal::{PortalSplitTraveler, PortalTraveler},
    spatial::{
        SpatialScale, UsfPosition, UsfScaleLayer, UsfSpatialSet, UsfSpatialTransition,
        UsfSpatialTransitionApplied, UsfSpatialTransitionQueue,
        UsfTransitionVelocity, UsfViewFrame,
    },
};

use super::{
    player::{Player, PlayerAim},
    world::UniverseLandmarkIndex,
};

pub(super) fn configure(app: &mut App) {
    app.add_systems(
        PostUpdate,
        reconcile_player_spatial_transition
            .after(UsfSpatialSet::SyncSemantic)
            .before(UsfSpatialSet::Rebase),
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
            name: "zoom",
            aliases: &["scale"],
            usage: "zoom <scale|continuous-exponent>",
            summary: "Change observer scale without changing canonical position.",
        },
        zoom_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "teleport",
            aliases: &["tp", "goto"],
            usage: "teleport <landmark> | teleport <scale> <x> <y> <z>",
            summary: "Request a canonical USF relocation and matching observer scale.",
        },
        teleport_command,
    );
}

fn player_semantic_entity(world: &mut World) -> Option<Entity> {
    let mut query = world.query_filtered::<&UsfManifestationOf, With<Player>>();
    query.iter(world).next().map(|manifestation| manifestation.0)
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
            "observer = {:+.3} (lower S{}, transition {:.3}, interaction S{})",
            view.continuous_exponent(),
            view.scale(),
            view.zoom(),
            view.interaction_scale(),
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

fn zoom_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let Some(value) = invocation.args().first() else {
        let view = world.resource::<UsfViewFrame>();
        return ConsoleCommandResult::success(format!(
            "observer scale = {:+.3} (interaction S{})",
            view.continuous_exponent(),
            view.interaction_scale(),
        ));
    };

    let raw = value
        .trim()
        .trim_start_matches('S')
        .trim_start_matches('s')
        .trim_start_matches('+');
    let Ok(exponent) = raw.parse::<f32>() else {
        return ConsoleCommandResult::error(format!("invalid observer scale `{value}`"));
    };
    if !exponent.is_finite() {
        return ConsoleCommandResult::error("observer scale must be finite");
    }

    world
        .resource_mut::<UsfViewFrame>()
        .set_continuous_exponent(exponent);
    let view = world.resource::<UsfViewFrame>();
    ConsoleCommandResult::success(format!(
        "observer scale requested -> {:+.3} (interaction S{})",
        view.continuous_exponent(),
        view.interaction_scale(),
    ))
}

fn teleport_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let Some(subject) = player_semantic_entity(world) else {
        return ConsoleCommandResult::error("player semantic entity is unavailable");
    };

    let args = invocation.args();
    if args.is_empty() {
        return ConsoleCommandResult::error(
            "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
        );
    }

    let (label, scale, view_exponent, arrival, look_at) = if args.len() == 1 {
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
        (
            landmark.id.to_string(),
            landmark.scale,
            landmark.view_exponent,
            landmark.arrival,
            Some(landmark.look_at),
        )
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

        (
            format!("S{scale} coordinate"),
            scale,
            scale.exponent() as f32,
            DVec3::new(coordinates[0], coordinates[1], coordinates[2]),
            None,
        )
    } else {
        return ConsoleCommandResult::error(
            "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
        );
    };

    let arrival_f32 = Vec3::new(arrival.x as f32, arrival.y as f32, arrival.z as f32);
    if !arrival_f32.is_finite() {
        return ConsoleCommandResult::error("destination is not representable in its native chart");
    }
    let Ok(position) = UsfPosition::zero(scale).translated_native(arrival_f32) else {
        return ConsoleCommandResult::error("destination could not become a canonical USF position");
    };

    world
        .resource_mut::<UsfSpatialTransitionQueue>()
        .request(
            UsfSpatialTransition::new(subject, position)
                .with_view_exponent(view_exponent)
                .with_velocity(UsfTransitionVelocity::Zero),
        );

    if let Some(look_at) = look_at {
        let direction = Vec3::new(
            (look_at.x - arrival.x) as f32,
            (look_at.y - arrival.y) as f32,
            (look_at.z - arrival.z) as f32,
        )
        .normalize_or_zero();

        if direction != Vec3::ZERO {
            let mut query = world.query_filtered::<&mut PlayerAim, With<Player>>();
            if let Some(mut aim) = query.iter_mut(world).next() {
                aim.yaw = (-direction.x).atan2(-direction.z);
                aim.pitch = direction.y.asin().clamp(aim.min_pitch, aim.max_pitch);
            }
        }
    }

    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "spatial transition requested: {label} @ S{scale} ({:.3}, {:.3}, {:.3}), view {view_exponent:+.1}",
        arrival.x, arrival.y, arrival.z,
    ))
}

fn parse_scale(value: &str) -> Option<SpatialScale> {
    let value = value
        .trim()
        .trim_start_matches('S')
        .trim_start_matches('s')
        .trim_start_matches('+');
    SpatialScale::new(value.parse().ok()?)
}

fn reconcile_player_spatial_transition(
    mut transitions: MessageReader<UsfSpatialTransitionApplied>,
    mut players: Query<
        (
            &Transform,
            &UsfManifestationOf,
            &mut PortalTraveler,
            &mut PortalSplitTraveler,
            &mut CharacterMovementInput,
            &mut CharacterGroundState,
        ),
        With<Player>,
    >,
) {
    for transition in transitions.read() {
        for (
            transform,
            manifestation,
            mut traveler,
            mut split,
            mut input,
            mut ground,
        ) in &mut players
        {
            if manifestation.0 != transition.subject {
                continue;
            }

            traveler.reset_spatial_transition(transform.translation);
            split.reset_spatial_transition(*transform);
            input.clear();
            ground.grounded = false;
            ground.ground_entity = None;
        }
    }
}
