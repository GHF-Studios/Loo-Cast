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

    let [local_camera, context_camera] = presentation_camera_lines(world);
    let mut lines = vec![
        format!("presentation probe = {}", probe.label()),
        format!(
            "observer = {:+.3} | context floor S{} | interaction S{}{}",
            view.continuous_exponent(),
            view.scale(),
            interaction.scale(),
            interaction.requested_scale().map_or(String::new(), |s| format!(" -> S{} pending", s)),
        ),
        local_camera,
        context_camera,
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

    lines.push(affinity_readiness_line(world, navigation, controlled_position, interaction_affinity));
    lines.extend(terrain_presentation_lines(world, interaction.scale()));
    lines.extend(scenery_presentation_lines(world));

    ConsoleCommandResult::lines(lines)
}

fn presentation_camera_lines(world: &mut World) -> [String; 2] {
    let local = {
        let mut query = world.query_filtered::<(&Transform, &Camera, &Camera3d), With<PrimaryGameView>>();
        query.iter(world).next().map(|(transform, camera, camera3d)|
            (transform.translation, camera.order, format!("{:?}", camera3d.depth_load_op)))
    };
    let context = {
        let mut query = world.query_filtered::<(&Transform, &Camera, &Camera3d), With<UsfViewRenderAnchor>>();
        query.iter(world).next().map(|(transform, camera, camera3d)|
            (transform.translation, camera.order, format!("{:?}", camera3d.depth_load_op)))
    };
    let format_camera = |name: &str, value: Option<(Vec3, isize, String)>| {
        value.map_or_else(
            || format!("{name} camera = <unavailable>"),
            |(position, order, depth)| format!(
                "{name} camera: order={} origin=({:.4},{:.4},{:.4}) depth={}",
                order, position.x, position.y, position.z, depth
            ),
        )
    };
    [format_camera("local", local), format_camera("context", context)]
}

#[derive(Default)]
struct PresentationCounts {
    physical_total: usize,
    physical_visible: usize,
    context_total: usize,
    context_visible: usize,
}

fn terrain_presentation_lines(world: &mut World, interaction_scale: crate::spatial::SpatialScale) -> Vec<String> {
    let terrain = {
        let mut query = world.query::<(&UsfScalePresentation, Option<&ChildOf>, &Visibility)>();
        query.iter(world).map(|(presentation, parent, visibility)| (
            presentation.scale(), parent.map(|parent| parent.0),
            !matches!(*visibility, Visibility::Hidden),
        )).collect::<Vec<_>>()
    };
    let mut counts = BTreeMap::<i8, PresentationCounts>::new();
    for (scale, parent, visible) in terrain {
        let physical = parent
            .and_then(|entity| world.get::<UsfCapabilityRealization>(entity))
            .is_some() && scale == interaction_scale;
        let count = counts.entry(scale.exponent()).or_default();
        if physical {
            count.physical_total += 1;
            count.physical_visible += usize::from(visible);
        } else {
            count.context_total += 1;
            count.context_visible += usize::from(visible);
        }
    }
    if counts.is_empty() { return vec!["scale terrain = <none>".to_string()]; }
    counts.into_iter().map(|(exponent, count)| format!(
        "terrain S{:+}: physical {}/{} visible | context {}/{} visible",
        exponent, count.physical_visible, count.physical_total,
        count.context_visible, count.context_total,
    )).collect()
}

fn scenery_presentation_lines(world: &mut World) -> Vec<String> {
    let scenery = {
        let mut query = world.query::<(&UsfSceneryPresentation, &Visibility)>();
        query.iter(world).map(|(presentation, visibility)| (
            presentation.scale(), !matches!(*visibility, Visibility::Hidden)
        )).collect::<Vec<_>>()
    };
    let mut counts = BTreeMap::<i8, (usize, usize)>::new();
    for (scale, visible) in scenery {
        let count = counts.entry(scale.exponent()).or_default();
        count.0 += 1;
        count.1 += usize::from(visible);
    }
    counts.into_iter().map(|(exponent, (total, visible))|
        format!("scenery S{:+}: {}/{} visible", exponent, visible, total)
    ).collect()
}

fn affinity_readiness_line(
    world: &World,
    navigation: NavigationAudit,
    controlled_position: Option<UsfPosition>,
    interaction_affinity: Option<UsfInteractionScaleAffinity>,
) -> String {
    let (Some(authority), Some(position), Some(affinity)) = (
        navigation.primary_body, controlled_position, interaction_affinity,
    ) else {
        return "affinity readiness = <insufficient primary-body/control/canonical state>".to_string();
    };
    let target = affinity.scale();
    let radius_native = affinity.coverage_radius_native();
    let coverage = world.resource::<UsfScaleCoverageSnapshot>();
    let near = |role| coverage.has_near_for_authority(
        authority, target, &position, role, radius_native,
    );
    let mut counts = [0usize; 4];
    for entry in coverage.iter() {
        if entry.authority() != authority || entry.scale() != target { continue; }
        counts[0] += 1;
        let roles = entry.roles();
        counts[1] += usize::from(roles.contains(UsfScaleRoleMask::REALIZATION));
        counts[2] += usize::from(roles.contains(UsfScaleRoleMask::PRESENTATION));
        counts[3] += usize::from(roles.contains(UsfScaleRoleMask::COLLISION));
    }
    format!(
        "affinity readiness S{} r={:.3} native: near R={} P={} C={} | authority entries total={} R={} P={} C={}",
        target, radius_native,
        near(UsfScaleRoleMask::REALIZATION),
        near(UsfScaleRoleMask::PRESENTATION),
        near(UsfScaleRoleMask::COLLISION),
        counts[0], counts[1], counts[2], counts[3],
    )
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
