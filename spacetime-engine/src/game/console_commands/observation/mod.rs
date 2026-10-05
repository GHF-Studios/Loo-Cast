//! Observation and diagnostic console commands for the current world view.

use std::collections::BTreeMap;

use bevy::prelude::*;

use crate::{
    console::{ConsoleCommandInvocation, ConsoleCommandResult},
    physics::topology::runtime_semantic_of_world,
    spatial::{
        UsfCapabilityRealization, UsfInteractionScaleAffinity, UsfPosition,
        UsfPresentationProbe, UsfPrimaryInteractionSlice, UsfRuntimeChartState,
        UsfScaleCoverageSnapshot, UsfScaleLayer, UsfScalePresentation,
        UsfScaleRoleMask, UsfSceneryPresentation, UsfViewRenderAnchor,
    },
    view::PrimaryGameView,
    voxel::VoxelStreamingTelemetry,
};
use super::{controlled_semantic_entity, primary_view_context};
use super::super::{
    control::LocalControlSubject,
    locomotion::ControlledSubjectLocomotion,
    navigation::{NavigationAudit, NavigationFlightRecorder},
    world::UniverseLandmarkIndex,
};

pub(super) fn where_command(world: &mut World, _: &ConsoleCommandInvocation) -> ConsoleCommandResult {
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
            let frame = world.resource::<UsfRuntimeChartState>();
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


pub(super) fn presentation_command(
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
    let interaction_affinity = {
        let mut query = world.query_filtered::<
            &UsfInteractionScaleAffinity,
            With<LocalControlSubject>,
        >();
        query.iter(world).next().copied()
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
            "control/navigation: affinity={} | clearance={} | approach={} | refinement target={}",
            interaction_affinity.map_or_else(
                || "<none>".to_string(),
                |affinity| format!("S{}", affinity.scale()),
            ),
            navigation.primary_clearance_metres.map_or_else(
                || "<none>".to_string(),
                |value| format!("{value:.3} m"),
            ),
            navigation.approach_active,
            navigation.realization_target_scale.map_or_else(
                || "<none>".to_string(),
                |scale| format!("S{scale}"),
            ),
        ),
    ];

    if let (
        Some(authority),
        Some(position),
        Some(affinity),
    ) = (
        navigation.primary_body,
        controlled_position,
        interaction_affinity,
    ) {
        let target = affinity.scale();
        let radius_native = affinity.coverage_radius_native();
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
            "affinity readiness S{} r={:.3} native: near R={} P={} C={} | authority entries total={} R={} P={} C={}",
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
            "affinity readiness = <insufficient primary-body/control/canonical state>".to_string(),
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

pub(super) fn navtrace_command(
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

pub(super) fn voxelstream_command(
    world: &mut World,
    _: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    ConsoleCommandResult::success(
        world.resource::<VoxelStreamingTelemetry>().summary(),
    )
}

pub(super) fn locate_command(world: &mut World, invocation: &ConsoleCommandInvocation) -> ConsoleCommandResult {
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
