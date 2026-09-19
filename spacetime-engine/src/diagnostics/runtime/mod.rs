//! Frame/runtime/system diagnostic sampling and low-frame-rate calculation.

use super::*;

pub(super) fn collect_runtime_diagnostics(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut cadence: ResMut<DiagnosticsCadence>,
    mut snapshot: ResMut<RuntimeDiagnostics>,
    telemetry: Option<Res<VaporTelemetry>>,
) {
    cadence.runtime_elapsed += time.delta_secs();
    if cadence.runtime_elapsed < RUNTIME_SAMPLE_INTERVAL_SECONDS {
        return;
    }
    cadence.runtime_elapsed %= RUNTIME_SAMPLE_INTERVAL_SECONDS;

    let frame_time = diagnostics.get(&FrameTimeDiagnosticsPlugin::FRAME_TIME);
    snapshot.frame.fps = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|diagnostic| diagnostic.value());
    snapshot.frame.frame_time_ms = frame_time.and_then(|diagnostic| diagnostic.value());
    snapshot.frame.average_frame_time_ms = frame_time.and_then(|diagnostic| diagnostic.average());

    let mut frame_times = frame_time
        .into_iter()
        .flat_map(|diagnostic| diagnostic.values().copied())
        .filter(|value| value.is_finite() && *value > 0.0)
        .collect::<Vec<_>>();
    snapshot.frame.one_percent_low_fps = low_fps(&mut frame_times, 0.01);

    snapshot.system.process_cpu_percent = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::PROCESS_CPU_USAGE,
    );
    snapshot.system.system_cpu_percent = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::SYSTEM_CPU_USAGE,
    );
    snapshot.system.process_memory_gib = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::PROCESS_MEM_USAGE,
    );
    snapshot.system.system_memory_percent = diagnostic_value(
        &diagnostics,
        &SystemInformationDiagnosticsPlugin::SYSTEM_MEM_USAGE,
    );

    if let Some(telemetry) = telemetry {
        publish_runtime_telemetry(&telemetry, &snapshot);
    }
}

fn diagnostic_value(
    diagnostics: &DiagnosticsStore,
    path: &bevy::diagnostic::DiagnosticPath,
) -> Option<f64> {
    diagnostics
        .get(path)
        .and_then(|diagnostic| diagnostic.value())
}

pub(super) fn low_fps(frame_times: &mut [f64], fraction: f64) -> Option<f64> {
    if frame_times.is_empty() {
        return None;
    }

    let count = ((frame_times.len() as f64 * fraction).ceil() as usize).clamp(1, frame_times.len());
    if count < frame_times.len() {
        frame_times.select_nth_unstable_by(count - 1, |left, right| right.total_cmp(left));
    }
    let average_ms = frame_times[..count].iter().sum::<f64>() / count as f64;

    (average_ms > 0.0).then_some(1_000.0 / average_ms)
}
