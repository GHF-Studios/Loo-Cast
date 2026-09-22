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
        SpatialScale, UsfPosition, UsfPrimaryInteractionSlice, UsfScaleLayer,
        UsfSpatialFrame, UsfSpatialSet, UsfSpatialTransition,
        UsfSpatialTransitionApplied, UsfSpatialTransitionQueue,
        UsfTransitionVelocity, UsfViewContext, UsfViewRenderAnchor,
    },
};

use super::{
    control::LocalControlSubject,
    player::{
        ControlledSubjectLocomotion, Player, PlayerAdaptiveCruise, PlayerAim,
        PlayerLocomotionRegime, PlayerLocomotionRequest, PlayerTravelSpeed,
    },
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
            name: "speed",
            aliases: &["movespeed", "travel-speed"],
            usage: "speed [<multiplier>|reset]",
            summary: "Show or set manual locomotion pace; 1.0 is the natural baseline for the active locomotion mode.",
        },
        speed_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "cruise",
            aliases: &["supercruise"],
            usage: "cruise [on|off]",
            summary: "Toggle adaptive long-distance travel; W/S control throttle.",
        },
        cruise_command,
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

fn primary_view_context(world: &mut World) -> Option<UsfViewContext> {
    let mut query =
        world.query_filtered::<&UsfViewContext, With<UsfViewRenderAnchor>>();
    query.iter(world).next().cloned()
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
    let Some(view) = primary_view_context(world) else {
        return ConsoleCommandResult::error("primary USF view context is unavailable");
    };

    ConsoleCommandResult::lines([
        format!(
            "runtime S{} = ({:.3}, {:.3}, {:.3})",
            scale, runtime.x, runtime.y, runtime.z
        ),
        format!(
            "observer = {:+.3} (lower S{}, transition {:.3})",
            view.continuous_exponent(),
            view.scale(),
            view.zoom(),
        ),
        {
            let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
            match interaction.requested_scale() {
                Some(requested) => format!(
                    "interaction = S{} -> S{} (pending)",
                    interaction.scale(),
                    requested,
                ),
                None => format!("interaction = S{}", interaction.scale()),
            }
        },
        format!("canonical = {semantic}"),
        {
            let frame = world.resource::<UsfSpatialFrame>();
            format!(
                "rebases = {} | last local shift = ({:.3}, {:.3}, {:.3})",
                frame.rebase_count(),
                frame.last_shift().x,
                frame.last_shift().y,
                frame.last_shift().z,
            )
        },
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
        let Some(view) = primary_view_context(world) else {
            return ConsoleCommandResult::error("primary USF view context is unavailable");
        };
        let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
        return ConsoleCommandResult::success(format!(
            "observer scale = {:+.3} | interaction S{}{}",
            view.continuous_exponent(),
            interaction.scale(),
            if interaction.handoff_pending() { " (handoff pending)" } else { "" },
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

    {
        let mut query =
            world.query_filtered::<&mut UsfViewContext, With<UsfViewRenderAnchor>>();
        let Some(mut view) = query.iter_mut(world).next() else {
            return ConsoleCommandResult::error("primary USF view context is unavailable");
        };
        view.set_continuous_exponent(exponent);
    }
    let Some(view) = primary_view_context(world) else {
        return ConsoleCommandResult::error("primary USF view context is unavailable");
    };
    let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
    ConsoleCommandResult::success(format!(
        "observer scale requested -> {:+.3} | interaction remains S{}{}",
        view.continuous_exponent(),
        interaction.scale(),
        if interaction.handoff_pending() { " (handoff pending)" } else { "" },
    ))
}


fn speed_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let requested = invocation.args().first().map(String::as_str);

    let mut query = world.query_filtered::<&mut PlayerTravelSpeed, With<LocalControlSubject>>();
    let Some(mut speed) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("player travel-speed state is unavailable");
    };

    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: speed [<multiplier>|reset]");
    }

    if let Some(raw) = requested {
        if raw.eq_ignore_ascii_case("reset") {
            *speed = PlayerTravelSpeed::default();
        } else {
            let Ok(parsed) = raw.parse::<f32>() else {
                return ConsoleCommandResult::error(format!(
                    "invalid speed multiplier `{raw}`"
                ));
            };
            if !parsed.is_finite() || parsed < 0.0 {
                return ConsoleCommandResult::error(
                    "speed multiplier must be finite and non-negative",
                );
            }
            speed.multiplier = parsed;
        }
    }

    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "manual locomotion pace = {:.3}x",
        speed.multiplier,
    ))
}

fn cruise_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: cruise [on|off]");
    }

    let mut query = world.query_filtered::<
        (&mut ControlledSubjectLocomotion, &mut PlayerAdaptiveCruise),
        With<LocalControlSubject>,
    >();
    let Some((mut locomotion, mut cruise)) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("player locomotion state is unavailable");
    };

    let active = match invocation.args().first().map(String::as_str) {
        None => {
            locomotion.request()
                != PlayerLocomotionRequest::Regime(PlayerLocomotionRegime::Cruise)
        }
        Some(value) if value.eq_ignore_ascii_case("on") => true,
        Some(value) if value.eq_ignore_ascii_case("off") => false,
        Some(value) => {
            return ConsoleCommandResult::error(format!(
                "invalid Cruise state `{value}`; expected on or off"
            ));
        }
    };

    if active {
        locomotion.request_regime(PlayerLocomotionRegime::Cruise);
        locomotion.set_thrusters_enabled(false);
    } else {
        locomotion.request_automatic();
    }
    cruise.throttle = 0.0;
    cruise.speed_scale0 = 0.0;

    ConsoleCommandResult::success_and_return_to_gameplay(if active {
        "adaptive Cruise enabled — W/S throttle, mouse steers"
    } else {
        "adaptive Cruise disabled"
    })
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

    let leaf_scale = scale.min(SpatialScale::ZERO);
    let Ok(position) = UsfPosition::from_scale_native_f64(arrival, scale, leaf_scale) else {
        return ConsoleCommandResult::error("destination could not become a canonical USF position");
    };

    world
        .resource_mut::<UsfSpatialTransitionQueue>()
        .request(
            UsfSpatialTransition::new(subject, position, UsfTransitionVelocity::Zero)
                .with_view_exponent(view_exponent),
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
            Option<&mut CharacterMovementInput>,
            Option<&mut CharacterGroundState>,
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
            input,
            ground,
        ) in &mut players
        {
            if manifestation.0 != transition.subject {
                continue;
            }

            traveler.reset_spatial_transition(transform.translation);
            split.reset_spatial_transition(*transform);

            if let Some(mut input) = input {
                input.clear();
            }
            if let Some(mut ground) = ground {
                ground.grounded = false;
                ground.ground_entity = None;
            }
        }
    }
}
