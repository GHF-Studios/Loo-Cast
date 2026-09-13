//! Lightweight engine diagnostics surfaced through the debug-view system.
//!
//! This is intentionally the beginning of the performance instrumentation
//! layer, not an attempt to fake metrics the engine/backend does not expose yet.

use std::collections::HashSet;

use bevy::{
    diagnostic::{
        DiagnosticsStore, EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin,
        SystemInformationDiagnosticsPlugin,
    },
    prelude::*,
    text::FontSize,
};

use super::{AppDebugExt, DebugView, DebugViews};

const FRAME_HISTORY_LENGTH: usize = 3_600;

struct PerformanceStatsDebugView;

impl DebugView for PerformanceStatsDebugView {
    const NAME: &'static str = "Performance / Basic";
    const DESCRIPTION: &'static str =
        "FPS/frame-time lows, ECS size, process/system CPU and RAM diagnostics.";
    const ENABLED_BY_DEFAULT: bool = false;
}

#[derive(Resource, Debug, Default, Clone, Copy)]
struct EcsStructureStats {
    component_instances: usize,
    component_registrations: usize,
    resource_count: usize,
    archetype_count: usize,
}

#[derive(Component)]
struct PerformanceStatsOverlay;

pub(super) fn configure(app: &mut App) {
    app.register_debug_view::<PerformanceStatsDebugView>()
        .init_resource::<EcsStructureStats>()
        .add_plugins((
            FrameTimeDiagnosticsPlugin::new(FRAME_HISTORY_LENGTH),
            EntityCountDiagnosticsPlugin::new(FRAME_HISTORY_LENGTH),
            SystemInformationDiagnosticsPlugin,
        ))
        .add_systems(PostStartup, spawn_performance_overlay)
        .add_systems(Update, update_performance_overlay)
        .add_systems(Last, collect_ecs_structure_stats);
}

fn spawn_performance_overlay(mut commands: Commands) {
    commands.spawn((
        Name::new("Performance Stats Overlay"),
        PerformanceStatsOverlay,
        Text::new("PERFORMANCE\ncollecting diagnostics..."),
        TextFont {
            font_size: FontSize::Px(14.0),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            right: px(20),
            bottom: px(20),
            padding: UiRect::all(px(10)),
            display: Display::None,
            ..default()
        },
        BackgroundColor(Color::srgba(0.02, 0.025, 0.035, 0.88)),
        GlobalZIndex(900),
    ));
}

fn update_performance_overlay(
    views: Res<DebugViews>,
    diagnostics: Res<DiagnosticsStore>,
    ecs: Res<EcsStructureStats>,
    mut overlay: Query<(&mut Text, &mut Node), With<PerformanceStatsOverlay>>,
) {
    let enabled = views.enabled::<PerformanceStatsDebugView>();

    for (mut text, mut node) in &mut overlay {
        node.display = if enabled { Display::Flex } else { Display::None };
        if !enabled {
            continue;
        }

        let frame_time = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
        let mut frame_times = frame_time
            .into_iter()
            .flat_map(|diagnostic| diagnostic.values().copied())
            .filter(|value| value.is_finite() && *value > 0.0)
            .collect::<Vec<_>>();

        let frame_current = frame_time.and_then(|diagnostic| diagnostic.value());
        let frame_average = frame_time.and_then(|diagnostic| diagnostic.average());
        let frame_min = frame_times.iter().copied().reduce(f64::min);
        let frame_max = frame_times.iter().copied().reduce(f64::max);
        let fps_current = diagnostics
            .get(&FrameTimeDiagnosticsPlugin::FPS)
            .and_then(|diagnostic| diagnostic.value());
        let fps_average = frame_average.and_then(fps_from_frame_ms);
        let fps_one_percent_low = low_fps(&mut frame_times, 0.01);
        let fps_point_one_percent_low = low_fps(&mut frame_times, 0.001);

        let entities = diagnostic_value(&diagnostics, &EntityCountDiagnosticsPlugin::ENTITY_COUNT);
        let process_cpu = diagnostic_value(
            &diagnostics,
            &SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE,
        );
        let system_cpu = diagnostic_value(
            &diagnostics,
            &SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
        );
        let process_memory_gib = diagnostic_value(
            &diagnostics,
            &SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE,
        );
        let system_memory_percent = diagnostic_value(
            &diagnostics,
            &SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
        );

        text.0 = format!(
            concat!(
                "PERFORMANCE\n",
                "FPS                 {}   avg {}\n",
                "Frame time          {} ms\n",
                "Frame min/avg/max   {} / {} / {} ms\n",
                "FPS 1% low          {}\n",
                "FPS 0.1% low        {}\n",
                "\n",
                "Entities            {}\n",
                "Component instances {}\n",
                "Component types     {}\n",
                "Resources           {}\n",
                "Archetypes          {}\n",
                "\n",
                "CPU process/system  {} / {} %\n",
                "RAM process         {} GiB\n",
                "RAM system          {} %",
            ),
            format_optional(fps_current, 1),
            format_optional(fps_average, 1),
            format_optional(frame_current, 2),
            format_optional(frame_min, 2),
            format_optional(frame_average, 2),
            format_optional(frame_max, 2),
            format_optional(fps_one_percent_low, 1),
            format_optional(fps_point_one_percent_low, 1),
            format_optional(entities, 0),
            ecs.component_instances,
            ecs.component_registrations,
            ecs.resource_count,
            ecs.archetype_count,
            format_optional(process_cpu, 1),
            format_optional(system_cpu, 1),
            format_optional(process_memory_gib, 2),
            format_optional(system_memory_percent, 1),
        );
    }
}

fn collect_ecs_structure_stats(world: &mut World) {
    let resource_ids = world
        .iter_resources()
        .map(|(info, _)| info.id())
        .collect::<HashSet<_>>();
    let component_instances = world
        .archetypes()
        .iter()
        .map(|archetype| {
            let ordinary_components = archetype
                .components()
                .iter()
                .filter(|component| !resource_ids.contains(component))
                .count();
            archetype.len() as usize * ordinary_components
        })
        .sum();
    let stats = EcsStructureStats {
        component_instances,
        component_registrations: world
            .components()
            .num_registered()
            .saturating_sub(resource_ids.len()),
        resource_count: resource_ids.len(),
        archetype_count: world.archetypes().len(),
    };

    *world.resource_mut::<EcsStructureStats>() = stats;
}

fn diagnostic_value(
    diagnostics: &DiagnosticsStore,
    path: &bevy::diagnostic::DiagnosticPath,
) -> Option<f64> {
    diagnostics.get(path).and_then(|diagnostic| diagnostic.value())
}

fn fps_from_frame_ms(frame_ms: f64) -> Option<f64> {
    (frame_ms.is_finite() && frame_ms > 0.0).then_some(1_000.0 / frame_ms)
}

fn low_fps(frame_times: &mut [f64], fraction: f64) -> Option<f64> {
    if frame_times.is_empty() {
        return None;
    }

    // Worst frames first. Averaging frame *times* before inverting avoids the
    // bias introduced by averaging instantaneous FPS values.
    frame_times.sort_by(|a, b| b.total_cmp(a));
    let count = ((frame_times.len() as f64 * fraction).ceil() as usize)
        .clamp(1, frame_times.len());
    let average_ms = frame_times[..count].iter().sum::<f64>() / count as f64;
    fps_from_frame_ms(average_ms)
}

fn format_optional(value: Option<f64>, decimals: usize) -> String {
    value
        .filter(|value| value.is_finite())
        .map(|value| format!("{:.*}", decimals, value))
        .unwrap_or_else(|| "--".to_owned())
}
