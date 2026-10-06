//! Canonical relocation requests and destination coverage gating.

use super::super::{
    control::LocalControlSubject,
    locomotion::ScaleInteractionProxy,
    player::{Player, PlayerAim},
    world::UniverseLandmarkIndex,
};
use super::controlled_semantic_entity;
use crate::{
    console::{ConsoleCommandInvocation, ConsoleCommandResult},
    physics::PhysicalBoxHull,
    spatial::{
        SpatialDemandSource, SpatialScale, UsfApproachRefinement, UsfPosition, UsfScaleRoleMask,
        UsfSemanticFrame, UsfSpatialTransition, UsfSpatialTransitionQueue, UsfTransitionVelocity,
        UsfTravelBoundaryResolver, UsfTravelInfluence, UsfTravelInfluenceKind,
    },
};
use bevy::{math::DVec3, prelude::*};

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
        let mut query = world.query_filtered::<&SpatialDemandSource, With<LocalControlSubject>>();
        query.iter(world).next()?.half_extent_native().max_element()
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

    for (entity, anchor, semantic_frame, influence, boundary, refinement) in influences.iter(world)
    {
        if refinement.is_none() || !matches!(influence.kind(), UsfTravelInfluenceKind::HardBody) {
            continue;
        }

        let Some(measurement) = influence.measure_from_at_scale(
            anchor,
            *semantic_frame,
            arrival,
            target_scale,
            boundary,
        ) else {
            continue;
        };
        let boundary_distance_scale0 =
            measurement.boundary_clearance_scale0() + measurement.penetration_depth_scale0();
        let distance_native = (boundary_distance_scale0 / scale0_per_native) as f32;

        if !distance_native.is_finite() || distance_native > demand_extent {
            continue;
        }

        if best.is_none_or(|(_, current)| distance_native < current) {
            best = Some((entity, distance_native.max(0.0)));
        }
    }

    best
}

fn controlled_collision_radius_native(world: &mut World, scale: SpatialScale) -> f32 {
    let mut query = world.query_filtered::<
        (&PhysicalBoxHull, Option<&ScaleInteractionProxy>),
        With<LocalControlSubject>,
    >();
    let Some((hull, proxy)) = query.iter(world).next() else {
        return 0.0;
    };

    let clearance_metres = proxy.map_or(0.0, |proxy| proxy.clearance_metres());
    scale
        .metres_to_native_f32(hull.bounding_radius_metres() + clearance_metres)
        .max(0.0)
}

/// Parsed relocation intent. Scale applies to the interaction chart only for
/// explicit coordinates; landmarks retain their authored display intent.
struct TeleportDestination {
    label: String,
    scale: SpatialScale,
    view_exponent: f32,
    arrival: UsfPosition,
    look_at: Option<UsfPosition>,
    explicit_scale: bool,
}

fn resolve_teleport_destination(
    world: &World,
    args: &[String],
) -> Result<TeleportDestination, ConsoleCommandResult> {
    if args.is_empty() {
        return Err(ConsoleCommandResult::error(
            "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
        ));
    }
    if args.len() == 1 {
        let landmark = {
            let index = world.resource::<UniverseLandmarkIndex>();
            let matches = index.find(&args[0]);
            if matches.is_empty() {
                return Err(ConsoleCommandResult::error(format!(
                    "no landmark matches `{}`; use `locate`",
                    args[0]
                )));
            }
            if matches.len() > 1 {
                return Err(ConsoleCommandResult::error(format!(
                    "`{}` is ambiguous: {}",
                    args[0],
                    matches
                        .iter()
                        .map(|landmark| landmark.id)
                        .collect::<Vec<_>>()
                        .join(", ")
                )));
            }
            (*matches[0]).clone()
        };
        return Ok(TeleportDestination {
            label: landmark.id.to_string(),
            scale: landmark.display_scale,
            view_exponent: landmark.view_exponent,
            arrival: landmark.arrival,
            look_at: Some(landmark.look_at),
            explicit_scale: false,
        });
    }
    if args.len() == 4 {
        let Some(scale) = parse_scale(&args[0]) else {
            return Err(ConsoleCommandResult::error(format!(
                "invalid USF scale `{}`",
                args[0]
            )));
        };
        let coordinates = args[1..]
            .iter()
            .map(|value| value.parse::<f64>())
            .collect::<Result<Vec<_>, _>>();
        let Ok(coordinates) = coordinates else {
            return Err(ConsoleCommandResult::error(
                "teleport coordinates must be finite numbers",
            ));
        };
        if coordinates.iter().any(|value| !value.is_finite()) {
            return Err(ConsoleCommandResult::error(
                "teleport coordinates must be finite numbers",
            ));
        }

        let authored = DVec3::new(coordinates[0], coordinates[1], coordinates[2]);
        let leaf_scale = scale.min(SpatialScale::ZERO);
        let Ok(position) = UsfPosition::from_scale_native_f64(authored, scale, leaf_scale) else {
            return Err(ConsoleCommandResult::error(
                "destination could not become a canonical USF position",
            ));
        };
        return Ok(TeleportDestination {
            label: format!("S{scale} coordinate"),
            scale,
            view_exponent: scale.exponent() as f32,
            arrival: position,
            look_at: None,
            explicit_scale: true,
        });
    }
    Err(ConsoleCommandResult::error(
        "usage: teleport <landmark> | teleport <scale> <x> <y> <z>",
    ))
}

fn aim_at_teleport_landmark(
    world: &mut World,
    arrival: UsfPosition,
    scale: SpatialScale,
    look_at: UsfPosition,
) {
    let direction = look_at
        .relative_at_scale_bounded(&arrival, scale, f32::MAX)
        .unwrap_or(Vec3::ZERO)
        .normalize_or_zero();
    if direction == Vec3::ZERO {
        return;
    }
    let mut query = world.query_filtered::<&mut PlayerAim, With<Player>>();
    if let Some(mut aim) = query.iter_mut(world).next() {
        aim.yaw = (-direction.x).atan2(-direction.z);
        aim.pitch = direction.y.asin().clamp(aim.min_pitch, aim.max_pitch);
    }
}

pub(super) fn teleport_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let Some(subject) = controlled_semantic_entity(world) else {
        return ConsoleCommandResult::error("controlled semantic entity is unavailable");
    };
    let TeleportDestination {
        label,
        scale,
        view_exponent,
        arrival,
        look_at,
        explicit_scale,
    } = match resolve_teleport_destination(world, invocation.args()) {
        Ok(destination) => destination,
        Err(error) => return error,
    };

    let mut transition = UsfSpatialTransition::new(subject, arrival, UsfTransitionVelocity::Zero)
        .with_view_exponent(view_exponent);
    let mut coverage_gated = false;
    if explicit_scale {
        transition = transition.with_scale(scale);

        if let Some((authority, boundary_distance_native)) =
            refinable_hard_body_transition_gate(world, &arrival, scale)
        {
            transition = transition.requiring_coverage_from(
                authority,
                UsfScaleRoleMask::REALIZATION.union(UsfScaleRoleMask::COLLISION),
                boundary_distance_native + controlled_collision_radius_native(world, scale),
            );
            coverage_gated = true;
        }
    }
    world
        .resource_mut::<UsfSpatialTransitionQueue>()
        .request(transition);

    if let Some(look_at) = look_at {
        aim_at_teleport_landmark(world, arrival, scale, look_at);
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
        } else if explicit_scale {
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
