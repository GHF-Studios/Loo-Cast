//! View, interaction and coverage diagnostic projection.

use super::*;

pub(in crate::game::console_commands) fn presentation_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    if invocation.args().len() > 1 {
        return ConsoleCommandResult::error("usage: presentation [all|physical|context]");
    }

    if let Some(raw) = invocation.args().first().map(String::as_str) {
        let mode = if raw.eq_ignore_ascii_case("all") {
            UsfPresentationDomainProbe::All
        } else if raw.eq_ignore_ascii_case("physical") || raw.eq_ignore_ascii_case("local") {
            UsfPresentationDomainProbe::Physical
        } else if raw.eq_ignore_ascii_case("context") || raw.eq_ignore_ascii_case("contextual") {
            UsfPresentationDomainProbe::Context
        } else {
            return ConsoleCommandResult::error(format!(
                "invalid presentation probe `{raw}`; expected all, physical or context"
            ));
        };
        *world.resource_mut::<UsfPresentationDomainProbe>() = mode;
    }

    let probe = *world.resource::<UsfPresentationDomainProbe>();
    let interaction = *world.resource::<UsfPrimaryInteractionSlice>();
    let navigation = *world.resource::<NavigationAudit>();
    let interaction_affinity = {
        let mut query =
            world.query_filtered::<&UsfInteractionScaleAffinity, With<LocalControlSubject>>();
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
            interaction
                .requested_scale()
                .map_or(String::new(), |s| format!(" -> S{} pending", s)),
        ),
        local_camera,
        context_camera,
        format!(
            "control/navigation: affinity={} | clearance={} | approach={} | refinement target={}",
            interaction_affinity.map_or_else(
                || "<none>".to_string(),
                |affinity| format!("S{}", affinity.scale()),
            ),
            navigation
                .primary_clearance_metres
                .map_or_else(|| "<none>".to_string(), |value| format!("{value:.3} m"),),
            navigation.approach_active,
            navigation
                .realization_target_scale
                .map_or_else(|| "<none>".to_string(), |scale| format!("S{scale}"),),
        ),
    ];

    lines.push(affinity_readiness_line(
        world,
        navigation,
        controlled_position,
        interaction_affinity,
    ));
    lines.extend(terrain_presentation_lines(world, interaction.scale()));

    ConsoleCommandResult::lines(lines)
}

fn presentation_camera_lines(world: &mut World) -> [String; 2] {
    let local = {
        let mut query =
            world.query_filtered::<(&Transform, &Camera, &Camera3d), With<PrimaryGameView>>();
        query
            .iter(world)
            .next()
            .map(|(transform, camera, camera3d)| {
                (
                    transform.translation,
                    camera.order,
                    format!("{:?}", camera3d.depth_load_op),
                )
            })
    };
    let context = {
        let mut query =
            world.query_filtered::<(&Transform, &Camera, &Camera3d), With<UsfViewRenderAnchor>>();
        query
            .iter(world)
            .next()
            .map(|(transform, camera, camera3d)| {
                (
                    transform.translation,
                    camera.order,
                    format!("{:?}", camera3d.depth_load_op),
                )
            })
    };
    let format_camera = |name: &str, value: Option<(Vec3, isize, String)>| {
        value.map_or_else(
            || format!("{name} camera = <unavailable>"),
            |(position, order, depth)| {
                format!(
                    "{name} camera: order={} origin=({:.4},{:.4},{:.4}) depth={}",
                    order, position.x, position.y, position.z, depth
                )
            },
        )
    };
    [
        format_camera("local", local),
        format_camera("context", context),
    ]
}

#[derive(Default)]
struct PresentationCounts {
    physical_total: usize,
    physical_visible: usize,
    context_total: usize,
    context_visible: usize,
}

fn terrain_presentation_lines(
    world: &mut World,
    interaction_scale: crate::spatial::SpatialScale,
) -> Vec<String> {
    let terrain = {
        let mut query = world.query::<(&UsfScalePresentation, Option<&ChildOf>, &Visibility)>();
        query
            .iter(world)
            .map(|(presentation, parent, visibility)| {
                (
                    presentation.scale(),
                    parent.map(|parent| parent.0),
                    !matches!(*visibility, Visibility::Hidden),
                )
            })
            .collect::<Vec<_>>()
    };
    let mut counts = BTreeMap::<i8, PresentationCounts>::new();
    for (scale, parent, visible) in terrain {
        let physical = parent
            .and_then(|entity| world.get::<UsfCapabilityRealization>(entity))
            .is_some()
            && scale == interaction_scale;
        let count = counts.entry(scale.exponent()).or_default();
        if physical {
            count.physical_total += 1;
            count.physical_visible += usize::from(visible);
        } else {
            count.context_total += 1;
            count.context_visible += usize::from(visible);
        }
    }
    if counts.is_empty() {
        return vec!["scale terrain = <none>".to_string()];
    }
    counts
        .into_iter()
        .map(|(exponent, count)| {
            format!(
                "terrain S{:+}: physical {}/{} visible | context {}/{} visible",
                exponent,
                count.physical_visible,
                count.physical_total,
                count.context_visible,
                count.context_total,
            )
        })
        .collect()
}

fn affinity_readiness_line(
    world: &World,
    navigation: NavigationAudit,
    controlled_position: Option<UsfPosition>,
    interaction_affinity: Option<UsfInteractionScaleAffinity>,
) -> String {
    let (Some(authority), Some(position), Some(affinity)) = (
        navigation.primary_body,
        controlled_position,
        interaction_affinity,
    ) else {
        return "affinity readiness = <insufficient primary-body/control/canonical state>"
            .to_string();
    };
    let target = affinity.scale();
    let radius_native = affinity.coverage_radius_native();
    let coverage = world.resource::<UsfScaleCoverageSnapshot>();
    let near =
        |role| coverage.has_near_for_authority(authority, target, &position, role, radius_native);
    let mut counts = [0usize; 4];
    for entry in coverage.iter() {
        if entry.authority() != authority || entry.scale() != target {
            continue;
        }
        counts[0] += 1;
        let roles = entry.roles();
        counts[1] += usize::from(roles.contains(UsfScaleRoleMask::REALIZATION));
        counts[2] += usize::from(roles.contains(UsfScaleRoleMask::PRESENTATION));
        counts[3] += usize::from(roles.contains(UsfScaleRoleMask::COLLISION));
    }
    format!(
        "affinity readiness S{} r={:.3} native: near R={} P={} C={} | authority entries total={} R={} P={} C={}",
        target,
        radius_native,
        near(UsfScaleRoleMask::REALIZATION),
        near(UsfScaleRoleMask::PRESENTATION),
        near(UsfScaleRoleMask::COLLISION),
        counts[0],
        counts[1],
        counts[2],
        counts[3],
    )
}
