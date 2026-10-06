//! Developer-console adapter for canonical controlled relocation requests.

use super::super::{
    navigation::ControlledRelocationRequest,
    player::{Player, PlayerAim},
    world::UniverseLandmarkIndex,
};
use crate::{
    console::{ConsoleCommandInvocation, ConsoleCommandResult},
    spatial::{SpatialScale, UsfPosition, UsfTransitionVelocity},
};
use bevy::{math::DVec3, prelude::*};

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
        let Some(center) = world.get::<UsfPosition>(landmark.body).copied() else {
            return Err(ConsoleCommandResult::error(format!(
                "landmark `{}` no longer has a live semantic position",
                landmark.id
            )));
        };
        let Some(arrival) = landmark.arrival_at(center) else {
            return Err(ConsoleCommandResult::error(format!(
                "landmark `{}` could not resolve a canonical arrival position",
                landmark.id
            )));
        };
        return Ok(TeleportDestination {
            label: landmark.id.to_string(),
            scale: landmark.display_scale,
            view_exponent: landmark.view_exponent,
            arrival,
            look_at: Some(center),
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

    let mut request = ControlledRelocationRequest::new(arrival, UsfTransitionVelocity::Zero)
        .with_view_exponent(view_exponent);
    if explicit_scale {
        request = request.with_interaction_scale(scale);
    }
    if world.write_message(request).is_none() {
        return ConsoleCommandResult::error("navigation relocation request channel is unavailable");
    }

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
        if explicit_scale {
            " with requested interaction scale; navigation owns destination coverage gating"
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
