use std::fs;

use crate::bevy::prelude::*;
use crate::rhai::{AST, Array, Dynamic, Engine, ImmutableString, Map, Scope};
use crate::rhai_binding::engine::resources::MainScriptEngineHandle;
use crate::usf::scale::{DynScale, Scale};

const SCALE_SCRIPT_PATH: &str = "core_mod/assets/scale/35.rhai";
const METRIC_SCRIPT_PATH: &str = "core_mod/assets/metric/test_metric.rhai";
const PHENOMENON_SCRIPT_PATH: &str = "core_mod/assets/phenomenon/test_phenomenon.rhai";
const PHENOMENON_REALIZER_SCRIPT_PATH: &str = "core_mod/assets/phenomenon_realizer/35.rhai";

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ScriptAssetType {
    Scale,
    Metric,
    Phenomenon,
    PhenomenonRealizer,
}
impl ScriptAssetType {
    fn definition_hook(self) -> &'static str {
        match self {
            ScriptAssetType::Scale => "define_scale",
            ScriptAssetType::Metric => "define_metric",
            ScriptAssetType::Phenomenon => "define_phenomenon",
            ScriptAssetType::PhenomenonRealizer => "define_phenomenon_realizer",
        }
    }

    fn allowed_use_paths(self) -> &'static [&'static str] {
        match self {
            ScriptAssetType::Scale | ScriptAssetType::Metric | ScriptAssetType::Phenomenon | ScriptAssetType::PhenomenonRealizer => {
                &["ctx::math::scalar", "ctx::math::vector"]
            }
        }
    }
}

#[derive(Resource, Reflect, Debug, Clone, Default)]
#[reflect(Resource)]
pub struct UsfScriptMvpBootstrapState {
    pub definitions_frozen: bool,
    pub loaded_scale_script_path: String,
    pub loaded_metric_script_path: String,
    pub loaded_phenomenon_script_path: String,
    pub loaded_phenomenon_realizer_script_path: String,
    pub last_realized_metric_value: f64,
}

pub(crate) fn bootstrap_usf_script_mvp_system(
    mut script_engine_handle: ResMut<MainScriptEngineHandle>,
    mut bootstrap_state: ResMut<UsfScriptMvpBootstrapState>,
) {
    if bootstrap_state.definitions_frozen {
        return;
    }

    let engine = &mut script_engine_handle.0;

    let scale_ast = load_and_compile_script(engine, ScriptAssetType::Scale, SCALE_SCRIPT_PATH);
    let metric_ast = load_and_compile_script(engine, ScriptAssetType::Metric, METRIC_SCRIPT_PATH);
    let phenomenon_ast = load_and_compile_script(engine, ScriptAssetType::Phenomenon, PHENOMENON_SCRIPT_PATH);
    let phenomenon_realizer_ast = load_and_compile_script(engine, ScriptAssetType::PhenomenonRealizer, PHENOMENON_REALIZER_SCRIPT_PATH);

    let scale_definition = call_definition_hook(engine, &scale_ast, ScriptAssetType::Scale, SCALE_SCRIPT_PATH);
    let metric_definition = call_definition_hook(engine, &metric_ast, ScriptAssetType::Metric, METRIC_SCRIPT_PATH);
    let phenomenon_definition = call_definition_hook(engine, &phenomenon_ast, ScriptAssetType::Phenomenon, PHENOMENON_SCRIPT_PATH);
    let phenomenon_realizer_definition = call_definition_hook(
        engine,
        &phenomenon_realizer_ast,
        ScriptAssetType::PhenomenonRealizer,
        PHENOMENON_REALIZER_SCRIPT_PATH,
    );

    let scale_id = map_get_string(&scale_definition, "id", "scale definition");
    let scale_exponent = map_get_i64(&scale_definition, "scale_factor_exponent", "scale definition");
    let scale_index_from_top = map_get_i64(&scale_definition, "scale_index_from_top", "scale definition");

    let metric_id = map_get_string(&metric_definition, "id", "metric definition");
    let metric_scale_id = map_get_string(&metric_definition, "scale_id", "metric definition");
    let metric_baseline = map_get_f64(&metric_definition, "baseline", "metric definition");

    let phenomenon_id = map_get_string(&phenomenon_definition, "id", "phenomenon definition");
    let phenomenon_metric_id = map_get_string(&phenomenon_definition, "metric_id", "phenomenon definition");

    let phenomenon_realizer_id = map_get_string(&phenomenon_realizer_definition, "id", "phenomenon_realizer definition");
    let phenomenon_realizer_scale_id = map_get_string(&phenomenon_realizer_definition, "scale_id", "phenomenon_realizer definition");
    let phenomenon_realizer_phenomenon_id = map_get_string(&phenomenon_realizer_definition, "phenomenon_id", "phenomenon_realizer definition");

    if scale_id != "35" {
        panic!("USF script MVP expected scale id '35', got '{scale_id}'");
    }
    if scale_exponent != 35 {
        panic!("USF script MVP expected scale_factor_exponent = 35, got {scale_exponent}");
    }
    if scale_index_from_top != 0 {
        panic!("USF script MVP expected top scale index = 0, got {scale_index_from_top}");
    }
    if metric_scale_id != scale_id {
        panic!(
            "USF script MVP metric->scale mismatch: metric scale_id '{}' does not match scale id '{}'",
            metric_scale_id, scale_id
        );
    }
    if phenomenon_metric_id != metric_id {
        panic!(
            "USF script MVP phenomenon->metric mismatch: phenomenon metric_id '{}' does not match metric id '{}'",
            phenomenon_metric_id, metric_id
        );
    }
    if phenomenon_realizer_scale_id != scale_id {
        panic!(
            "USF script MVP realizer->scale mismatch: realizer scale_id '{}' does not match scale id '{}'",
            phenomenon_realizer_scale_id, scale_id
        );
    }
    if phenomenon_realizer_phenomenon_id != phenomenon_id {
        panic!(
            "USF script MVP realizer->phenomenon mismatch: realizer phenomenon_id '{}' does not match phenomenon id '{}'",
            phenomenon_realizer_phenomenon_id, phenomenon_id
        );
    }
    if phenomenon_realizer_id != "35" {
        panic!("USF script MVP expected phenomenon_realizer id '35', got '{}'", phenomenon_realizer_id);
    }

    let phenomenon_excitation = call_array_hook(engine, &phenomenon_ast, "emit_excitation", PHENOMENON_SCRIPT_PATH);
    let phenomenon_excitation_xyz = array_to_vec3(&phenomenon_excitation, "emit_excitation");

    let realized_value: f64 = call_f64_hook_with_vec3_arg(
        engine,
        &phenomenon_realizer_ast,
        "realize_metric_excitation",
        phenomenon_excitation.clone(),
        PHENOMENON_REALIZER_SCRIPT_PATH,
    );

    bootstrap_state.definitions_frozen = true;
    bootstrap_state.loaded_scale_script_path = SCALE_SCRIPT_PATH.to_string();
    bootstrap_state.loaded_metric_script_path = METRIC_SCRIPT_PATH.to_string();
    bootstrap_state.loaded_phenomenon_script_path = PHENOMENON_SCRIPT_PATH.to_string();
    bootstrap_state.loaded_phenomenon_realizer_script_path = PHENOMENON_REALIZER_SCRIPT_PATH.to_string();
    bootstrap_state.last_realized_metric_value = realized_value;

    let scale_value = Scale::MAX;
    info!(
        "USF script MVP loaded and frozen: scale_id='{}' exponent={} index_from_top={} scale_name='{}'",
        scale_id,
        scale_exponent,
        scale_index_from_top,
        scale_value.name(),
    );
    info!(
        "USF script MVP linkage: metric='{}' (baseline={:.6}) -> phenomenon='{}' -> realizer='{}'",
        metric_id, metric_baseline, phenomenon_id, phenomenon_realizer_id,
    );
    info!(
        "USF script MVP sample: excitation=[{:.6}, {:.6}, {:.6}] realized_metric_value={:.6}",
        phenomenon_excitation_xyz[0], phenomenon_excitation_xyz[1], phenomenon_excitation_xyz[2], realized_value,
    );
}

fn load_and_compile_script(engine: &Engine, script_asset_type: ScriptAssetType, path: &str) -> AST {
    let raw_source = fs::read_to_string(path).unwrap_or_else(|error| {
        panic!(
            "USF script MVP failed to read {} script at '{}': {}",
            script_asset_type_name(script_asset_type),
            path,
            error
        )
    });

    validate_use_capabilities(script_asset_type, raw_source.as_str(), path);

    let preprocessed = crate::rhai_binding::engine::preprocess_script_source(raw_source.as_str(), path);
    engine.compile(preprocessed.as_str()).unwrap_or_else(|error| {
        panic!(
            "USF script MVP failed to compile {} script '{}': {}",
            script_asset_type_name(script_asset_type),
            path,
            error
        )
    })
}

fn script_asset_type_name(script_asset_type: ScriptAssetType) -> &'static str {
    match script_asset_type {
        ScriptAssetType::Scale => "scale",
        ScriptAssetType::Metric => "metric",
        ScriptAssetType::Phenomenon => "phenomenon",
        ScriptAssetType::PhenomenonRealizer => "phenomenon_realizer",
    }
}

fn call_definition_hook(engine: &Engine, ast: &AST, script_asset_type: ScriptAssetType, path: &str) -> Map {
    let mut scope = Scope::new();
    engine
        .call_fn::<Map>(&mut scope, ast, script_asset_type.definition_hook(), ())
        .unwrap_or_else(|error| {
            panic!(
                "USF script MVP failed to execute definition hook '{}' for {} script '{}': {}",
                script_asset_type.definition_hook(),
                script_asset_type_name(script_asset_type),
                path,
                error
            )
        })
}

fn call_array_hook(engine: &Engine, ast: &AST, function_name: &str, path: &str) -> Array {
    let mut scope = Scope::new();
    engine
        .call_fn::<Array>(&mut scope, ast, function_name, ())
        .unwrap_or_else(|error| panic!("USF script MVP failed to execute '{}' in '{}': {}", function_name, path, error))
}

fn call_f64_hook_with_vec3_arg(engine: &Engine, ast: &AST, function_name: &str, arg: Array, path: &str) -> f64 {
    let mut scope = Scope::new();
    engine
        .call_fn::<f64>(&mut scope, ast, function_name, (arg,))
        .unwrap_or_else(|error| panic!("USF script MVP failed to execute '{}' in '{}': {}", function_name, path, error))
}

fn map_get_string(map: &Map, key: &str, context: &str) -> String {
    let value = map.get(key).unwrap_or_else(|| panic!("USF script MVP missing '{key}' in {context}")).clone();

    if let Some(text) = value.clone().try_cast::<String>() {
        return text;
    }
    if let Some(text) = value.try_cast::<ImmutableString>() {
        return text.to_string();
    }

    panic!("USF script MVP expected string '{key}' in {context}")
}

fn map_get_i64(map: &Map, key: &str, context: &str) -> i64 {
    let value = map.get(key).unwrap_or_else(|| panic!("USF script MVP missing '{key}' in {context}")).clone();

    if let Some(integer) = value.clone().try_cast::<i64>() {
        return integer;
    }
    if let Some(float) = value.try_cast::<f64>() {
        return float as i64;
    }

    panic!("USF script MVP expected numeric integer-like '{key}' in {context}")
}

fn map_get_f64(map: &Map, key: &str, context: &str) -> f64 {
    let value = map.get(key).unwrap_or_else(|| panic!("USF script MVP missing '{key}' in {context}")).clone();

    if let Some(float) = value.clone().try_cast::<f64>() {
        return float;
    }
    if let Some(integer) = value.try_cast::<i64>() {
        return integer as f64;
    }

    panic!("USF script MVP expected numeric '{key}' in {context}")
}

fn parse_use_path(line: &str, line_number: usize, source_name: &str) -> Option<String> {
    let trimmed = line.trim_start();
    if trimmed.starts_with("//") || trimmed.starts_with("/*") {
        return None;
    }
    if !trimmed.starts_with("use") {
        return None;
    }

    let mut rest = &trimmed["use".len()..];
    if !rest.starts_with(char::is_whitespace) {
        return None;
    }
    rest = rest.trim_start();

    let path_end = rest.find(char::is_whitespace).unwrap_or_else(|| {
        panic!(
            "USF script MVP invalid use declaration in '{}' at line {}: expected `use <path> as <alias>;`",
            source_name, line_number
        )
    });
    let use_path = rest[..path_end].trim();
    rest = rest[path_end..].trim_start();

    if !rest.starts_with("as") {
        panic!(
            "USF script MVP invalid use declaration in '{}' at line {}: expected `as`",
            source_name, line_number
        );
    }
    rest = &rest["as".len()..];
    if !rest.starts_with(char::is_whitespace) {
        panic!(
            "USF script MVP invalid use declaration in '{}' at line {}: expected alias after `as`",
            source_name, line_number
        );
    }
    rest = rest.trim_start();

    let alias_end = rest.find(|ch: char| ch.is_whitespace() || ch == ';').unwrap_or(rest.len());
    if alias_end == 0 {
        panic!(
            "USF script MVP invalid use declaration in '{}' at line {}: missing alias",
            source_name, line_number
        );
    }
    rest = rest[alias_end..].trim_start();

    let Some(trailing) = rest.strip_prefix(';') else {
        panic!(
            "USF script MVP invalid use declaration in '{}' at line {}: missing `;`",
            source_name, line_number
        );
    };
    if !(trailing.trim_start().is_empty() || trailing.trim_start().starts_with("//")) {
        panic!(
            "USF script MVP invalid use declaration in '{}' at line {}: unexpected trailing content",
            source_name, line_number
        );
    }

    Some(use_path.to_string())
}

fn validate_use_capabilities(script_asset_type: ScriptAssetType, source: &str, source_name: &str) {
    let allowed_use_paths = script_asset_type.allowed_use_paths();

    for (index, line) in source.lines().enumerate() {
        let line_number = index + 1;
        let Some(use_path) = parse_use_path(line, line_number, source_name) else {
            continue;
        };

        let allowed = allowed_use_paths
            .iter()
            .any(|allowed_prefix| use_path == *allowed_prefix || use_path.starts_with(&format!("{allowed_prefix}::")));

        if !allowed {
            panic!(
                "USF script MVP capability profile violation in '{}' ({} line {}): use path '{}' is not allowed. Allowed roots: [{}]",
                source_name,
                script_asset_type_name(script_asset_type),
                line_number,
                use_path,
                allowed_use_paths.join(", "),
            );
        }
    }
}

fn dynamic_to_f64(value: Dynamic, context: &str, index: usize) -> f64 {
    if let Some(float) = value.clone().try_cast::<f64>() {
        return float;
    }
    if let Some(integer) = value.try_cast::<i64>() {
        return integer as f64;
    }
    panic!("USF script MVP expected numeric vec3 value for '{context}' at index {index}")
}

fn array_to_vec3(array: &Array, context: &str) -> [f64; 3] {
    if array.len() != 3 {
        panic!("USF script MVP expected vec3 array from '{}', got length {}", context, array.len());
    }

    let x = dynamic_to_f64(array[0].clone(), context, 0);
    let y = dynamic_to_f64(array[1].clone(), context, 1);
    let z = dynamic_to_f64(array[2].clone(), context, 2);
    [x, y, z]
}
