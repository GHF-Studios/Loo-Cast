//! Loo Cast commands layered on the generic developer console.
//!
//! Navigation resolves destinations into canonical USF transitions. The console
//! never mutates runtime Transform coordinates directly.

mod input_bindings;
mod observation;
mod runtime_variables;

use bevy::{math::DVec3, prelude::*};

use crate::{
    console::{
        AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
    },
    physics::{
        PhysicalBoxHull,
        character::{CharacterGroundState, CharacterMovementInput},
        topology::runtime_semantic_of_world,
    },
    portal::{PortalSplitTraveler, PortalTraveler},
    spatial::{
        SpatialDemandSource, SpatialScale, UsfApproachRefinement,
        UsfPosition,
        UsfPrimaryInteractionSlice, UsfSemanticFrame,
        UsfScaleRoleMask,
        UsfSpatialSet, UsfSpatialTransition,
        UsfSpatialTransitionApplied, UsfSpatialTransitionQueue, UsfTransitionVelocity,
        UsfTravelBoundaryResolver, UsfTravelInfluence, UsfTravelInfluenceKind,
        UsfViewContext, UsfViewRenderAnchor,
    },
};

use super::{
    control::LocalControlSubject,
    locomotion::{
        ControlledSubjectLocomotion, LocomotionRegime, LocomotionRequest,
        ScaleInteractionProxy,
    },
    navigation::{AdaptiveCruise, TravelPace},
    player::{Player, PlayerAim},
    world::UniverseLandmarkIndex,
};
use observation::{where_command, presentation_command, navtrace_command, voxelstream_command, locate_command};

pub(super) fn configure(app: &mut App) {
    input_bindings::configure(app);
    runtime_variables::configure(app);
    super::devtools::lab::configure(app);

    app.add_systems(
        PostUpdate,
        reconcile_controlled_spatial_transition
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
            name: "presentation",
            aliases: &["present", "viewpass"],
            usage: "presentation [all|physical|context]",
            summary: "Inspect or isolate physical vs contextual USF presentation passes.",
        },
        presentation_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "navtrace",
            aliases: &["ntrace", "flightrecorder"],
            usage: "navtrace [<count>|clear|on|off|status]",
            summary: "Dump recent navigation/interaction causal history.",
        },
        navtrace_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "voxelstream",
            aliases: &["vstream", "voxel-stream"],
            usage: "voxelstream",
            summary: "Show voxel worker/backpressure telemetry.",
        },
        voxelstream_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "locate",
            aliases: &["find", "landmarks"],
            usage: "locate [name|kind]",
            summary: "Locate instantiated fixture bodies in scale-local coordinates.",
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

fn controlled_semantic_entity(world: &mut World) -> Option<Entity> {
    let controlled = {
        let mut query = world.query_filtered::<Entity, With<LocalControlSubject>>();
        query.iter(world).next()
    }?;
    runtime_semantic_of_world(world, controlled)
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

    let mut query = world.query_filtered::<&mut TravelPace, With<LocalControlSubject>>();
    let Some(mut speed) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("player travel-speed state is unavailable");
    };

    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: speed [<multiplier>|reset]");
    }

    if let Some(raw) = requested {
        if raw.eq_ignore_ascii_case("reset") {
            *speed = TravelPace::default();
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
        (&mut ControlledSubjectLocomotion, &mut AdaptiveCruise),
        With<LocalControlSubject>,
    >();
    let Some((mut locomotion, mut cruise)) = query.iter_mut(world).next() else {
        return ConsoleCommandResult::error("player locomotion state is unavailable");
    };

    let active = match invocation.args().first().map(String::as_str) {
        None => {
            locomotion.request()
                != LocomotionRequest::Regime(LocomotionRegime::Cruise)
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
        locomotion.request_regime(LocomotionRegime::Cruise);
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

/// Finds a refinable hard-body boundary inside the controlled subject's
/// target-scale interest window.
///
/// This is generic semantic transition policy, not Earth/voxel policy.
fn refinable_hard_body_transition_gate(
    world: &mut World,
    arrival: &UsfPosition,
    target_scale: SpatialScale,
) -> Option<(Entity, f32)> {
    let demand_extent = {
        let mut query =
            world.query_filtered::<&SpatialDemandSource, With<LocalControlSubject>>();
        query
            .iter(world)
            .next()?
            .half_extent_native()
            .max_element()
    };
    if !demand_extent.is_finite() || demand_extent <= 0.0 {
        return None;
    }

    let scale0_per_native = target_scale.scale0_units_per_native();
    if !scale0_per_native.is_finite() || scale0_per_native <= 0.0 {
        return None;
    }

    let mut best = None::<(Entity, f32)>;
    let mut influences = world.query::<(
        Entity,
        &UsfPosition,
        &UsfSemanticFrame,
        &UsfTravelInfluence,
        Option<&UsfTravelBoundaryResolver>,
        Option<&UsfApproachRefinement>,
    )>();

    for (entity, anchor, semantic_frame, influence, boundary, refinement) in influences.iter(world) {
        if refinement.is_none()
            || !matches!(influence.kind(), UsfTravelInfluenceKind::HardBody)
        {
            continue;
        }

        let Some(measurement) =
            influence.measure_from_at_scale(
                anchor,
                *semantic_frame,
                arrival,
                target_scale,
                boundary,
            )
        else {
            continue;
        };
        let boundary_distance_scale0 =
            measurement.boundary_clearance_scale0()
                + measurement.penetration_depth_scale0();
        let distance_native =
            (boundary_distance_scale0 / scale0_per_native) as f32;

        if !distance_native.is_finite() || distance_native > demand_extent {
            continue;
        }

        if best.is_none_or(|(_, current)| distance_native < current) {
            best = Some((entity, distance_native.max(0.0)));
        }
    }

    best
}

fn controlled_collision_radius_native(
    world: &mut World,
    scale: SpatialScale,
) -> f32 {
    let mut query = world.query_filtered::<
        (&PhysicalBoxHull, Option<&ScaleInteractionProxy>),
        With<LocalControlSubject>,
    >();
    let Some((hull, proxy)) = query.iter(world).next() else {
        return 0.0;
    };

    let clearance_metres =
        proxy.map_or(0.0, |proxy| proxy.clearance_metres());
    scale.metres_to_native_f32(
        hull.bounding_radius_metres() + clearance_metres,
    )
    .max(0.0)
}

fn teleport_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let Some(subject) = controlled_semantic_entity(world) else {
        return ConsoleCommandResult::error("controlled semantic entity is unavailable");
    };

    let args = invocation.args();
    if args.is_empty() {
        return ConsoleCommandResult::error(
            "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
        );
    }

    let explicit_scale_transition = args.len() == 4;
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
            landmark.display_scale,
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

        let authored = DVec3::new(coordinates[0], coordinates[1], coordinates[2]);
        let leaf_scale = scale.min(SpatialScale::ZERO);
        let Ok(position) = UsfPosition::from_scale_native_f64(authored, scale, leaf_scale) else {
            return ConsoleCommandResult::error(
                "destination could not become a canonical USF position",
            );
        };

        (
            format!("S{scale} coordinate"),
            scale,
            scale.exponent() as f32,
            position,
            None,
        )
    } else {
        return ConsoleCommandResult::error(
            "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
        );
    };

    let mut transition =
        UsfSpatialTransition::new(subject, arrival, UsfTransitionVelocity::Zero)
            .with_view_exponent(view_exponent);
    let mut coverage_gated = false;
    if explicit_scale_transition {
        transition = transition.with_scale(scale);

        if let Some((authority, boundary_distance_native)) =
            refinable_hard_body_transition_gate(world, &arrival, scale)
        {
            transition = transition.requiring_coverage_from(
                authority,
                UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION),
                boundary_distance_native
                    + controlled_collision_radius_native(world, scale),
            );
            coverage_gated = true;
        }
    }
    world
        .resource_mut::<UsfSpatialTransitionQueue>()
        .request(transition);

    if let Some(look_at) = look_at {
        let direction = look_at
            .relative_at_scale_bounded(&arrival, scale, f32::MAX)
            .unwrap_or(Vec3::ZERO)
            .normalize_or_zero();

        if direction != Vec3::ZERO {
            let mut query = world.query_filtered::<&mut PlayerAim, With<Player>>();
            if let Some(mut aim) = query.iter_mut(world).next() {
                aim.yaw = (-direction.x).atan2(-direction.z);
                aim.pitch = direction.y.asin().clamp(aim.min_pitch, aim.max_pitch);
            }
        }
    }

    let coordinates = arrival
        .coordinate_at_scale_f64(scale)
        .unwrap_or(DVec3::splat(f64::NAN));

    ConsoleCommandResult::success_and_return_to_gameplay(format!(
        "spatial transition requested: {label} @ S{scale} ({:.3}, {:.3}, {:.3}), view {view_exponent:+.1}{}",
        coordinates.x,
        coordinates.y,
        coordinates.z,
        if coverage_gated {
            " with matching interaction scale (waiting for destination collision coverage)"
        } else if explicit_scale_transition {
            " with matching interaction scale"
        } else {
            ""
        },
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

fn reconcile_controlled_spatial_transition(
    mut transitions: MessageReader<UsfSpatialTransitionApplied>,
    ownership: crate::ecs::UsfOwnershipQuery,
    mut subjects: Query<
        (
            Entity,
            &Transform,
            &mut PortalTraveler,
            Option<&mut PortalSplitTraveler>,
            Option<&mut CharacterMovementInput>,
            Option<&mut CharacterGroundState>,
        ),
        With<LocalControlSubject>,
    >,
) {
    for transition in transitions.read() {
        for (
            entity,
            transform,
            mut traveler,
            split,
            input,
            ground,
        ) in &mut subjects
        {
            if ownership.semantic_of(entity) != Some(transition.subject) {
                continue;
            }

            traveler.reset_spatial_transition(transform.translation);
            if let Some(mut split) = split {
                split.reset_spatial_transition(*transform);
            }

            if let Some(mut input) = input {
                input.clear();
            }
            if let Some(mut ground) = ground {
                ground.clear_contact();
            }
        }
    }
}
