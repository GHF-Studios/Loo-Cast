//! Bounded Rhai host configuration and script logging.

use rhai::{Engine, ImmutableString};
use std::{
    sync::{Mutex, OnceLock},
    time::{Duration, Instant},
};

const SCRIPT_MAX_OPERATIONS: u64 = 5_000;
const SCRIPT_MAX_CALL_LEVELS: usize = 12;
const SCRIPT_MAX_VARIABLES: usize = 96;
const SCRIPT_MAX_FUNCTIONS: usize = 48;
const SCRIPT_MAX_STRING_BYTES: usize = 16 * 1024;
const SCRIPT_MAX_ARRAY_SIZE: usize = 256;
const SCRIPT_MAX_MAP_SIZE: usize = 128;

const SCRIPT_LOGS_PER_SECOND: usize = 32;

struct ScriptLogWindow {
    started: Instant,
    emitted: usize,
    suppression_reported: bool,
}

static SCRIPT_LOG_WINDOW: OnceLock<Mutex<ScriptLogWindow>> = OnceLock::new();

#[derive(Clone, Copy)]
enum ScriptLogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

fn emit_script_log(level: ScriptLogLevel, text: &str) {
    let limiter = SCRIPT_LOG_WINDOW.get_or_init(|| {
        Mutex::new(ScriptLogWindow {
            started: Instant::now(),
            emitted: 0,
            suppression_reported: false,
        })
    });
    let mut window = limiter
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner());
    if window.started.elapsed() >= Duration::from_secs(1) {
        window.started = Instant::now();
        window.emitted = 0;
        window.suppression_reported = false;
    }

    if window.emitted >= SCRIPT_LOGS_PER_SECOND {
        if !window.suppression_reported {
            window.suppression_reported = true;
            bevy::log::warn!(
                target: "rhai",
                "script log rate limit reached; suppressing additional script output this second"
            );
        }
        return;
    }
    window.emitted += 1;
    drop(window);

    match level {
        ScriptLogLevel::Trace => bevy::log::trace!(target: "rhai", "{text}"),
        ScriptLogLevel::Debug => bevy::log::debug!(target: "rhai", "{text}"),
        ScriptLogLevel::Info => bevy::log::info!(target: "rhai", "{text}"),
        ScriptLogLevel::Warn => bevy::log::warn!(target: "rhai", "{text}"),
        ScriptLogLevel::Error => bevy::log::error!(target: "rhai", "{text}"),
    }
}

fn log_trace(message: ImmutableString) {
    emit_script_log(ScriptLogLevel::Trace, message.as_str());
}
fn log_debug(message: ImmutableString) {
    emit_script_log(ScriptLogLevel::Debug, message.as_str());
}
fn log_info(message: ImmutableString) {
    emit_script_log(ScriptLogLevel::Info, message.as_str());
}
fn log_warn(message: ImmutableString) {
    emit_script_log(ScriptLogLevel::Warn, message.as_str());
}
fn log_error(message: ImmutableString) {
    emit_script_log(ScriptLogLevel::Error, message.as_str());
}

fn pow_int(value: &mut f64, exponent: i64) -> f64 {
    value.powi(exponent.clamp(i64::from(i32::MIN), i64::from(i32::MAX)) as i32)
}

fn pow_float(value: &mut f64, exponent: f64) -> f64 {
    value.powf(exponent)
}

fn lerp(a: f64, b: f64, t: f64) -> f64 {
    a + (b - a) * t
}

fn saturate(value: f64) -> f64 {
    value.clamp(0.0, 1.0)
}

fn smoothstep(edge0: f64, edge1: f64, value: f64) -> f64 {
    if !edge0.is_finite() || !edge1.is_finite() || !value.is_finite() {
        return 0.0;
    }
    if (edge1 - edge0).abs() <= f64::EPSILON {
        return if value < edge0 { 0.0 } else { 1.0 };
    }
    let t = saturate((value - edge0) / (edge1 - edge0));
    t * t * (3.0 - 2.0 * t)
}

fn remap(value: f64, input_min: f64, input_max: f64, output_min: f64, output_max: f64) -> f64 {
    if (input_max - input_min).abs() <= f64::EPSILON {
        return output_min;
    }
    let t = (value - input_min) / (input_max - input_min);
    lerp(output_min, output_max, t)
}

fn configure_engine(mut engine: Engine) -> Engine {
    engine
        .set_max_operations(SCRIPT_MAX_OPERATIONS)
        .set_max_call_levels(SCRIPT_MAX_CALL_LEVELS)
        .set_max_expr_depths(32, 16)
        .set_max_variables(SCRIPT_MAX_VARIABLES)
        .set_max_functions(SCRIPT_MAX_FUNCTIONS)
        .set_max_modules(0)
        .set_max_string_size(SCRIPT_MAX_STRING_BYTES)
        .set_max_array_size(SCRIPT_MAX_ARRAY_SIZE)
        .set_max_map_size(SCRIPT_MAX_MAP_SIZE);

    // Engine::new already installs Rhai's StandardPackage, including normal
    // math (`sin`, `cos`, `sqrt`, `exp`, `ln`, `log`, `min`, `max`, `abs`,
    // PI, E, etc.). We add only project-useful helpers and host capabilities.
    engine.on_print(|text| emit_script_log(ScriptLogLevel::Info, text));
    engine.on_debug(|text, source, position| {
        let location = source.map_or_else(
            || format!("{position:?}"),
            |source| format!("{source} @ {position:?}"),
        );
        emit_script_log(ScriptLogLevel::Debug, &format!("{location}: {text}"));
    });

    engine
        .register_fn("log_trace", log_trace)
        .register_fn("log_debug", log_debug)
        .register_fn("log_info", log_info)
        .register_fn("log_warn", log_warn)
        .register_fn("log_error", log_error)
        .register_fn("pow", pow_int)
        .register_fn("pow", pow_float)
        .register_fn("lerp", lerp)
        .register_fn("saturate", saturate)
        .register_fn("smoothstep", smoothstep)
        .register_fn("remap", remap);

    engine
}

pub(super) fn bounded_engine() -> Engine {
    configure_engine(Engine::new())
}
