//! Loo Cast commands layered on the generic developer console.
//!
//! Navigation resolves destinations into canonical USF transitions. The console
//! never mutates runtime Transform coordinates directly.

use std::collections::BTreeMap;

use bevy::{math::DVec3, prelude::*};

use crate::{
    console::{
        AppConsoleExt, ConsoleCommandInvocation, ConsoleCommandResult, ConsoleCommandSpec,
    },
    physics::{
        character::{CharacterGroundState, CharacterMovementInput},
        topology::runtime_semantic_of_world,
    },
    portal::{PortalSplitTraveler, PortalTraveler},
    spatial::{
        SpatialDemandSource, SpatialScale, UsfApproachRefinement, UsfCapabilityRealization,
        UsfPosition, UsfPresentationProbe, UsfPrimaryInteractionSlice,
        UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScalePresentation, UsfScaleRoleMask,
        UsfSceneryPresentation, UsfSpatialFrame, UsfSpatialSet, UsfSpatialTransition,
        UsfSpatialTransitionApplied, UsfSpatialTransitionQueue, UsfTransitionVelocity,
        UsfTravelBoundaryResolver, UsfTravelInfluence, UsfTravelInfluenceKind,
        UsfViewContext, UsfViewRenderAnchor,
    },
    voxel::VoxelStreamingTelemetry,
};

use super::{
    control::LocalControlSubject,
    locomotion::{ControlledSubjectLocomotion, LocomotionRegime, LocomotionRequest},
    navigation::{
        AdaptiveCruise, NavigationAudit, NavigationFlightRecorder, TravelPace, TravelProfile,
    },
    player::{Player, PlayerAim},
    world::UniverseLandmarkIndex,
};
use crate::view::PrimaryGameView;

pub(super) fn configure(app: &mut App) {
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

fn where_command(world: &mut World, _: &ConsoleCommandInvocation) -> ConsoleCommandResult {
    let controlled = {
        let mut query = world.query_filtered::<
            (Entity, &Transform, &UsfScaleLayer),
            With<LocalControlSubject>,
        >();
        query
            .iter(world)
            .next()
            .map(|(entity, transform, layer)| (entity, transform.translation, layer.scale()))
    };

    let Some((controlled_entity, runtime, scale)) = controlled else {
        return ConsoleCommandResult::error("controlled realization is unavailable");
    };
    let Some(semantic_entity) = runtime_semantic_of_world(world, controlled_entity) else {
        return ConsoleCommandResult::error("controlled semantic entity is unavailable");
    };

    let semantic_position = world.get::<UsfPosition>(semantic_entity).copied();
    let semantic = semantic_position
        .as_ref()
        .map(UsfPosition::format_stack)
        .unwrap_or_else(|| "<semantic position unavailable>".to_string());
    let Some(view) = primary_view_context(world) else {
        return ConsoleCommandResult::error("primary USF view context is unavailable");
    };

    let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
    let locomotion = world.get::<ControlledSubjectLocomotion>(controlled_entity).copied();
    let probe = *world.resource::<UsfPresentationProbe>();
    let coverage_status = semantic_position.map_or_else(
        || "coverage = <canonical position unavailable>".to_string(),
        |position| {
            let coverage = world.resource::<UsfScaleCoverageSnapshot>();
            let scale = interaction.scale();
            let realized = coverage.has_near(
                scale,
                &position,
                UsfScaleRoleMask::REALIZATION,
                0.0,
            );
            let presented = coverage.has_near(
                scale,
                &position,
                UsfScaleRoleMask::PRESENTATION,
                0.0,
            );
            let collision = coverage.has_near(
                scale,
                &position,
                UsfScaleRoleMask::COLLISION,
                0.0,
            );
            format!(
                "coverage @ S{}: realization={} presentation={} collision={}",
                scale, realized, presented, collision,
            )
        },
    );

    ConsoleCommandResult::lines([
        format!(
            "runtime S{} = ({:.3}, {:.3}, {:.3})",
            scale, runtime.x, runtime.y, runtime.z
        ),
        format!(
            "observer = {:+.3} (context floor S{}, transition {:.3})",
            view.continuous_exponent(),
            view.scale(),
            view.zoom(),
        ),
        {
            match interaction.requested_scale() {
                Some(requested) => format!(
                    "interaction = S{} -> S{} (pending)",
                    interaction.scale(),
                    requested,
                ),
                None => format!("interaction = S{}", interaction.scale()),
            }
        },
        locomotion.map_or_else(
            || "locomotion = <unavailable>".to_string(),
            |state| format!(
                "locomotion = {:?} / {:?} / {:?}",
                state.regime(), state.kernel(), state.collision_policy(),
            ),
        ),
        format!("presentation probe = {}", probe.label()),
        format!("canonical = {semantic}"),
        coverage_status,
        {
            match (semantic_position, interaction.requested_scale()) {
                (Some(position), Some(requested)) => {
                    let coverage = world.resource::<UsfScaleCoverageSnapshot>();
                    let realized = coverage.has_near(
                        requested,
                        &position,
                        UsfScaleRoleMask::REALIZATION,
                        0.0,
                    );
                    let presented = coverage.has_near(
                        requested,
                        &position,
                        UsfScaleRoleMask::PRESENTATION,
                        0.0,
                    );
                    let collision = coverage.has_near(
                        requested,
                        &position,
                        UsfScaleRoleMask::COLLISION,
                        0.0,
                    );
                    format!(
                        "requested coverage @ S{}: realization={} presentation={} collision={}",
                        requested, realized, presented, collision,
                    )
                }
                _ => "requested coverage = <none>".to_string(),
            }
        },
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


fn presentation_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: presentation [all|physical|context]");
    }

    if let Some(raw) = invocation.args().first().map(String::as_str) {
        let mode = if raw.eq_ignore_ascii_case("all") {
            UsfPresentationProbe::All
        } else if raw.eq_ignore_ascii_case("physical") || raw.eq_ignore_ascii_case("local") {
            UsfPresentationProbe::Physical
        } else if raw.eq_ignore_ascii_case("context") || raw.eq_ignore_ascii_case("contextual") {
            UsfPresentationProbe::Context
        } else {
            return ConsoleCommandResult::error(format!(
                "invalid presentation probe `{raw}`; expected all, physical or context"
            ));
        };
        *world.resource_mut::<UsfPresentationProbe>() = mode;
    }

    let probe = *world.resource::<UsfPresentationProbe>();
    let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
    let navigation = *world.resource::<NavigationAudit>();
    let handoff_radius_native = {
        let mut query =
            world.query_filtered::<&TravelProfile, With<LocalControlSubject>>();
        query
            .iter(world)
            .next()
            .map(|profile| profile.approach.interaction_handoff_coverage_radius_native)
    };
    let controlled_position = controlled_semantic_entity(world)
        .and_then(|entity| world.get::<UsfPosition>(entity).copied());

    let Some(view) = primary_view_context(world) else {
        return ConsoleCommandResult::error("primary USF view context is unavailable");
    };

    let camera_line = |name: &str, value: Option<(Vec3, isize, String)>| {
        value.map_or_else(
            || format!("{name} camera = <unavailable>"),
            |(p, order, depth)| format!(
                "{name} camera: order={} origin=({:.4},{:.4},{:.4}) depth={}",
                order, p.x, p.y, p.z, depth
            ),
        )
    };
    let local_camera = {
        let mut q = world.query_filtered::<(&Transform, &Camera, &Camera3d), With<PrimaryGameView>>();
        q.iter(world).next().map(|(t,c,c3)| (t.translation,c.order,format!("{:?}",c3.depth_load_op)))
    };
    let context_camera = {
        let mut q = world.query_filtered::<(&Transform, &Camera, &Camera3d), With<UsfViewRenderAnchor>>();
        q.iter(world).next().map(|(t,c,c3)| (t.translation,c.order,format!("{:?}",c3.depth_load_op)))
    };

    #[derive(Default)]
    struct Counts { pt: usize, pv: usize, ct: usize, cv: usize }

    let terrain = {
        let mut q = world.query::<(&UsfScalePresentation, Option<&ChildOf>, &Visibility)>();
        q.iter(world).map(|(p,parent,v)| (
            p.scale(), parent.map(|x| x.0), !matches!(*v, Visibility::Hidden)
        )).collect::<Vec<_>>()
    };
    let mut counts = BTreeMap::<i8, Counts>::new();
    for (scale, parent, visible) in terrain {
        let physical = parent
            .and_then(|e| world.get::<UsfCapabilityRealization>(e))
            .is_some() && scale == interaction.scale();
        let c = counts.entry(scale.exponent()).or_default();
        if physical {
            c.pt += 1;
            c.pv += usize::from(visible);
        } else {
            c.ct += 1;
            c.cv += usize::from(visible);
        }
    }

    let scenery = {
        let mut q = world.query::<(&UsfSceneryPresentation, &Visibility)>();
        q.iter(world).map(|(p,v)| (
            p.scale(), !matches!(*v, Visibility::Hidden)
        )).collect::<Vec<_>>()
    };
    let mut scenery_counts = BTreeMap::<i8,(usize,usize)>::new();
    for (scale, visible) in scenery {
        let c = scenery_counts.entry(scale.exponent()).or_default();
        c.0 += 1;
        c.1 += usize::from(visible);
    }

    let mut lines = vec![
        format!("presentation probe = {}", probe.label()),
        format!(
            "observer = {:+.3} | context floor S{} | interaction S{}{}",
            view.continuous_exponent(),
            view.scale(),
            interaction.scale(),
            interaction.requested_scale().map_or(String::new(), |s| format!(" -> S{} pending", s)),
        ),
        camera_line("local", local_camera),
        camera_line("context", context_camera),
        format!(
            "navigation: clearance={} | approach={} | interaction target={} | realization target={}",
            navigation.primary_clearance_metres.map_or_else(
                || "<none>".to_string(),
                |value| format!("{value:.3} m"),
            ),
            navigation.approach_active,
            navigation.interaction_target_scale.map_or_else(
                || "<none>".to_string(),
                |scale| format!("S{scale}"),
            ),
            navigation.realization_target_scale.map_or_else(
                || "<none>".to_string(),
                |scale| format!("S{scale}"),
            ),
        ),
    ];

    if let (
        Some(authority),
        Some(position),
        Some(target),
        Some(radius_native),
    ) = (
        navigation.primary_body,
        controlled_position,
        navigation.interaction_target_scale,
        handoff_radius_native,
    ) {
        let coverage = world.resource::<UsfScaleCoverageSnapshot>();
        let realization_near = coverage.has_near_for_authority(
            authority,
            target,
            &position,
            UsfScaleRoleMask::REALIZATION,
            radius_native,
        );
        let presentation_near = coverage.has_near_for_authority(
            authority,
            target,
            &position,
            UsfScaleRoleMask::PRESENTATION,
            radius_native,
        );
        let collision_near = coverage.has_near_for_authority(
            authority,
            target,
            &position,
            UsfScaleRoleMask::COLLISION,
            radius_native,
        );

        let mut total_entries = 0usize;
        let mut realization_entries = 0usize;
        let mut presentation_entries = 0usize;
        let mut collision_entries = 0usize;
        for entry in coverage.iter() {
            if entry.authority() != authority || entry.scale() != target {
                continue;
            }
            total_entries += 1;
            let roles = entry.roles();
            realization_entries +=
                usize::from(roles.contains(UsfScaleRoleMask::REALIZATION));
            presentation_entries +=
                usize::from(roles.contains(UsfScaleRoleMask::PRESENTATION));
            collision_entries +=
                usize::from(roles.contains(UsfScaleRoleMask::COLLISION));
        }

        lines.push(format!(
            "handoff gate S{} r={:.3} native: near R={} P={} C={} | authority entries total={} R={} P={} C={}",
            target,
            radius_native,
            realization_near,
            presentation_near,
            collision_near,
            total_entries,
            realization_entries,
            presentation_entries,
            collision_entries,
        ));
    } else {
        lines.push(
            "handoff gate = <insufficient primary-body/planner/canonical state>".to_string(),
        );
    }

    if counts.is_empty() {
        lines.push("scale terrain = <none>".to_string());
    } else {
        for (e,c) in counts {
            lines.push(format!(
                "terrain S{:+}: physical {}/{} visible | context {}/{} visible",
                e,c.pv,c.pt,c.cv,c.ct
            ));
        }
    }
    for (e,(total,visible)) in scenery_counts {
        lines.push(format!("scenery S{:+}: {}/{} visible", e,visible,total));
    }

    ConsoleCommandResult::lines(lines)
}

fn navtrace_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error(
            "usage: navtrace [<count>|clear|on|off|status]",
        );
    }

    let arg = invocation.args().first().map(String::as_str);
    match arg {
        Some(value) if value.eq_ignore_ascii_case("clear") => {
            world.resource_mut::<NavigationFlightRecorder>().clear();
            ConsoleCommandResult::success("navtrace cleared")
        }
        Some(value) if value.eq_ignore_ascii_case("on") => {
            world.resource_mut::<NavigationFlightRecorder>().set_enabled(true);
            ConsoleCommandResult::success("navtrace enabled")
        }
        Some(value) if value.eq_ignore_ascii_case("off") => {
            world.resource_mut::<NavigationFlightRecorder>().set_enabled(false);
            ConsoleCommandResult::success("navtrace disabled")
        }
        Some(value) if value.eq_ignore_ascii_case("status") => {
            let recorder = world.resource::<NavigationFlightRecorder>();
            ConsoleCommandResult::success(format!(
                "navtrace {} | {} samples retained",
                if recorder.enabled() { "enabled" } else { "disabled" },
                recorder.len(),
            ))
        }
        Some(value) => {
            let Ok(count) = value.parse::<usize>() else {
                return ConsoleCommandResult::error(
                    "navtrace count must be a positive integer",
                );
            };
            ConsoleCommandResult::lines(
                world.resource::<NavigationFlightRecorder>().lines(count),
            )
        }
        None => ConsoleCommandResult::lines(
            world.resource::<NavigationFlightRecorder>().lines(40),
        ),
    }
}

fn voxelstream_command(
    world: &mut World,
    _: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    ConsoleCommandResult::success(
        world.resource::<VoxelStreamingTelemetry>().summary(),
    )
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
        &UsfTravelInfluence,
        Option<&UsfTravelBoundaryResolver>,
        Option<&UsfApproachRefinement>,
    )>();

    for (entity, influence, boundary, refinement) in influences.iter(world) {
        if refinement.is_none()
            || !matches!(influence.kind(), UsfTravelInfluenceKind::HardBody)
        {
            continue;
        }

        let Some(measurement) =
            influence.measure_from_at_scale(arrival, target_scale, boundary)
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
                    + super::locomotion::ScaleInteractionProxy::DEFAULT_RADIUS_NATIVE,
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
