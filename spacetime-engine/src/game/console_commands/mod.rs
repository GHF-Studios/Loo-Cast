//! Loo Cast commands layered on the generic developer console.
//!
//! Navigation resolves destinations into canonical USF transitions. The console
//! never mutates runtime Transform coordinates directly.

mod input_bindings;
mod observation;
mod runtime_variables;
mod teleport;
mod travel;

use bevy::prelude::*;

use super::control::LocalControlSubject;
use crate::{
    console::{AppConsoleExt, ConsoleCommandSpec},
    physics::{
        character::{CharacterGroundState, CharacterMovementInput},
        topology::runtime_semantic_of_world,
    },
    portal::{PortalSplitTraveler, PortalTraveler},
    spatial::{UsfSpatialSet, UsfSpatialTransitionApplied, UsfViewContext, UsfViewRenderAnchor},
};
use observation::{
    locate_command, navtrace_command, presentation_command, voxelstream_command, where_command,
};

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
        travel::zoom_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "speed",
            aliases: &["movespeed", "travel-speed"],
            usage: "speed [<multiplier>|reset]",
            summary: "Show or set manual locomotion pace; 1.0 is the natural baseline for the active locomotion mode.",
        },
        travel::speed_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "cruise",
            aliases: &["supercruise"],
            usage: "cruise [on|off]",
            summary: "Toggle adaptive long-distance travel; W/S control throttle.",
        },
        travel::cruise_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "attitude",
            aliases: &["sas"],
            usage: "attitude [hold|view]",
            summary: "Choose whether the ship holds attitude or follows the pilot view.",
        },
        travel::attitude_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "autopilot",
            aliases: &["ap"],
            usage: "autopilot [off|hold|prograde]",
            summary: "Select an attitude controller while retaining manual translation.",
        },
        travel::autopilot_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "motionstack",
            aliases: &["modality"],
            usage: "motionstack",
            summary: "Inspect each live motion layer and its measured context.",
        },
        travel::motion_stack_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "motionoverride",
            aliases: &["motiondev"],
            usage: "motionoverride <collision|gravity> <on|off>",
            summary: "Explicit developer override for controlled ship collision or gravity.",
        },
        travel::motion_override_command,
    )
    .register_console_command(
        ConsoleCommandSpec {
            name: "teleport",
            aliases: &["tp", "goto"],
            usage: "teleport <landmark> | teleport <scale> <x> <y> <z>",
            summary: "Request a canonical USF relocation and matching observer scale.",
        },
        teleport::teleport_command,
    );
}

fn primary_view_context(world: &mut World) -> Option<UsfViewContext> {
    let mut query = world.query_filtered::<&UsfViewContext, With<UsfViewRenderAnchor>>();
    query.iter(world).next().cloned()
}

fn controlled_semantic_entity(world: &mut World) -> Option<Entity> {
    let controlled = {
        let mut query = world.query_filtered::<Entity, With<LocalControlSubject>>();
        query.iter(world).next()
    }?;
    runtime_semantic_of_world(world, controlled)
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
        for (entity, transform, mut traveler, split, input, ground) in &mut subjects {
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
