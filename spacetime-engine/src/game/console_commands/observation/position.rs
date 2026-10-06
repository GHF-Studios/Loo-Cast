//! Canonical position and authored landmark inspection.

use super::*;

pub(in crate::game::console_commands) fn where_command(
    world: &mut World,
    _: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
    let controlled = {
        let mut query = world
            .query_filtered::<(Entity, &Transform, &UsfScaleLayer), With<LocalControlSubject>>();
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
    let locomotion = world
        .get::<ControlledSubjectLocomotion>(controlled_entity)
        .copied();
    let execution = world.get::<MotionExecution>(controlled_entity).copied();
    let probe = *world.resource::<UsfPresentationDomainProbe>();
    let coverage_status = semantic_position.map_or_else(
        || "coverage = <canonical position unavailable>".to_string(),
        |position| {
            let coverage = world.resource::<UsfScaleCoverageSnapshot>();
            let scale = interaction.scale();
            let realized = coverage.has_near(scale, &position, UsfScaleRoleMask::REALIZATION, 0.0);
            let presented =
                coverage.has_near(scale, &position, UsfScaleRoleMask::PRESENTATION, 0.0);
            let collision = coverage.has_near(scale, &position, UsfScaleRoleMask::COLLISION, 0.0);
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
        locomotion.zip(execution).map_or_else(
            || "locomotion = <unavailable>".to_string(),
            |(state, execution)| {
                format!(
                    "locomotion = {:?} / {:?} / {:?}",
                    state.regime(),
                    execution.kernel(),
                    execution.collision_policy(),
                )
            },
        ),
        format!("presentation probe = {}", probe.label()),
        format!("canonical = {semantic}"),
        coverage_status,
        {
            match (semantic_position, interaction.requested_scale()) {
                (Some(position), Some(requested)) => {
                    let coverage = world.resource::<UsfScaleCoverageSnapshot>();
                    let realized =
                        coverage.has_near(requested, &position, UsfScaleRoleMask::REALIZATION, 0.0);
                    let presented = coverage.has_near(
                        requested,
                        &position,
                        UsfScaleRoleMask::PRESENTATION,
                        0.0,
                    );
                    let collision =
                        coverage.has_near(requested, &position, UsfScaleRoleMask::COLLISION, 0.0);
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

pub(in crate::game::console_commands) fn locate_command(
    world: &mut World,
    invocation: &ConsoleCommandInvocation,
) -> ConsoleCommandResult {
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
            landmark.coordinate_label(world.get::<UsfPosition>(landmark.body).copied()),
            landmark.description
        )
    }))
}
