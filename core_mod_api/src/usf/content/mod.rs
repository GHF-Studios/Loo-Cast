use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::config::{statics::CONFIG, types::ConfigValue};
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{USF_DPT_SCHEMAS_BY_SCALE, USF_MODPACKS_BY_ID, USF_MODS_BY_ID, USF_SCALES_BY_INDEX, USF_ZONE_TYPES};
use crate::usf::definition::{DptMetricDefinition, DptMetricId, DptMetricStorageClass, DptMetricValueType, DptSchema, ZoneTypeId};
use crate::usf::scale::Scale;

pub const DEFAULT_DEMO_MOD_ID: &str = "demo";
pub const DPT_SAMPLER_KERNEL_DEFAULT_ID: &str = "dpt_sampler.kernel.default.v1";
pub const DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID: &str = "dpt_categorizer.kernel.zlm_lookup.v1";

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfScaleDefinition {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfActiveModpack {
    pub modpack_id: String,
    pub configured_mods: Vec<UsfConfiguredMod>,
    pub enabled_mods: HashSet<String>,
    pub resolved_enabled_mods: Vec<String>,
    pub scales_by_index: HashMap<Scale, UsfScaleDefinition>,
    pub known_dpt_samplers: HashSet<String>,
    pub known_dpt_categorizers: HashSet<String>,
    pub schemas_by_scale: HashMap<Scale, DptSchema>,
    pub known_zone_types: HashSet<ZoneTypeId>,
}
impl Default for UsfActiveModpack {
    fn default() -> Self {
        let modpack_id = active_usf_modpack_id_from_config();
        let (configured_mods, resolved_enabled_mods) = configured_mods_for_modpack(modpack_id.as_str());
        let enabled_mods = configured_mods.iter().map(|mod_entry| mod_entry.mod_id.clone()).collect::<HashSet<_>>();

        let mut active_modpack = Self {
            modpack_id,
            configured_mods,
            enabled_mods,
            resolved_enabled_mods,
            scales_by_index: HashMap::new(),
            known_dpt_samplers: script_dpt_samplers(),
            known_dpt_categorizers: script_dpt_categorizers(),
            schemas_by_scale: HashMap::new(),
            known_zone_types: HashSet::new(),
        };

        for (scale, definition) in script_scales() {
            active_modpack.scales_by_index.insert(scale, definition);
        }
        for zone in script_zone_types() {
            active_modpack.known_zone_types.insert(zone);
        }
        for (scale, schema) in script_schema_overrides() {
            active_modpack.schemas_by_scale.insert(scale, schema);
        }

        if let Err(reason) = active_modpack.validate() {
            panic!("USF active modpack default validation failed: {reason}");
        }

        active_modpack
    }
}
impl UsfActiveModpack {
    pub fn schema_for_scale(&self, scale: Scale) -> Option<&DptSchema> {
        self.schemas_by_scale.get(&scale)
    }

    pub fn scale_definition_for_scale(&self, scale: Scale) -> Option<&UsfScaleDefinition> {
        self.scales_by_index.get(&scale)
    }

    pub fn enabled_mods_in_profile_order(&self) -> Vec<String> {
        self.resolved_enabled_mods.clone()
    }

    pub fn is_mod_enabled(&self, mod_id: &str) -> bool {
        self.enabled_mods.contains(mod_id)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.modpack_id.trim().is_empty() {
            return Err("USF active modpack id must not be empty".to_string());
        }
        if self.enabled_mods.is_empty() {
            return Err(format!("USF active modpack '{}' resolved to zero enabled mods", self.modpack_id));
        }
        if self.resolved_enabled_mods.is_empty() {
            return Err(format!("USF active modpack '{}' resolved no ordered enabled mods", self.modpack_id));
        }
        if self.configured_mods.is_empty() {
            return Err(format!("USF active modpack '{}' has no configured mods", self.modpack_id));
        }
        if self.schemas_by_scale.is_empty() {
            return Err(format!("USF active modpack '{}' has no DPT schemas", self.modpack_id));
        }
        if self.scales_by_index.is_empty() {
            return Err(format!("USF active modpack '{}' has no scale definitions", self.modpack_id));
        }
        if self.known_dpt_samplers.is_empty() {
            return Err(format!("USF active modpack '{}' has no known DPT samplers", self.modpack_id));
        }
        if self.known_dpt_categorizers.is_empty() {
            return Err(format!("USF active modpack '{}' has no known DPT categorizers", self.modpack_id));
        }
        let mut configured_mod_ids = HashSet::<String>::new();
        for mod_entry in &self.configured_mods {
            if mod_entry.mod_id.trim().is_empty() {
                return Err(format!("USF active modpack '{}' has configured mod with empty id", self.modpack_id));
            }
            if !configured_mod_ids.insert(mod_entry.mod_id.clone()) {
                return Err(format!(
                    "USF active modpack '{}' has duplicate configured mod '{}'",
                    self.modpack_id, mod_entry.mod_id
                ));
            }
        }
        for mod_id in &self.enabled_mods {
            if !configured_mod_ids.contains(mod_id) {
                return Err(format!("USF active modpack '{}' marks unknown mod '{}' as enabled", self.modpack_id, mod_id));
            }
        }
        for mod_id in &self.resolved_enabled_mods {
            if !self.enabled_mods.contains(mod_id) {
                return Err(format!(
                    "USF active modpack '{}' has ordered enabled mod '{}' that is not in enabled_mods",
                    self.modpack_id, mod_id
                ));
            }
        }
        let resolved_enabled_set = self.resolved_enabled_mods.iter().cloned().collect::<HashSet<_>>();
        if resolved_enabled_set != self.enabled_mods {
            return Err(format!(
                "USF active modpack '{}' has mismatch between enabled_mods and resolved_enabled_mods",
                self.modpack_id
            ));
        }

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(scale_definition) = self.scales_by_index.get(&scale) else {
                return Err(format!("missing scale definition for scale index {}", scale.index_from_top()));
            };
            if scale_definition.dpt_sampler_id.trim().is_empty() {
                return Err(format!("empty dpt_sampler_id for scale index {}", scale.index_from_top()));
            }
            if scale_definition.dpt_categorizer_id.trim().is_empty() {
                return Err(format!("empty dpt_categorizer_id for scale index {}", scale.index_from_top()));
            }
            if scale_definition.chunk_store_key.trim().is_empty() {
                return Err(format!("empty chunk_store_key for scale index {}", scale.index_from_top()));
            }
            if !self.known_dpt_samplers.contains(&scale_definition.dpt_sampler_id) {
                return Err(format!(
                    "unknown dpt_sampler_id '{}' for scale index {}",
                    scale_definition.dpt_sampler_id,
                    scale.index_from_top()
                ));
            }
            if !self.known_dpt_categorizers.contains(&scale_definition.dpt_categorizer_id) {
                return Err(format!(
                    "unknown dpt_categorizer_id '{}' for scale index {}",
                    scale_definition.dpt_categorizer_id,
                    scale.index_from_top()
                ));
            }
            let Some(schema) = self.schemas_by_scale.get(&scale) else {
                return Err(format!("missing DPT schema for scale index {}", scale.index_from_top()));
            };
            schema.validate()?;
            for metric in &schema.metrics {
                if !metric.applies_to_scale(scale) {
                    return Err(format!(
                        "metric '{}' (id={}) is not valid for scale index {} (range=[{}..{}])",
                        metric.name,
                        metric.id.0,
                        scale.index_from_top(),
                        metric.min_scale_index,
                        metric.max_scale_index
                    ));
                }
            }
            if !self.known_zone_types.contains(&schema.fallback_zone) {
                return Err(format!(
                    "fallback zone '{}' for scale {} is not declared in known_zone_types",
                    schema.fallback_zone.0,
                    scale.index_from_top()
                ));
            }
        }

        Ok(())
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfConfiguredMod {
    pub mod_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct UsfScriptModpackDefinition {
    pub mod_ids: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct UsfScriptModDefinition {
    pub priority: i32,
    pub dependencies: HashSet<String>,
    pub load_after: HashSet<String>,
    pub conflicts_with: HashSet<String>,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfScaleExecutionRoute {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
    pub usf_modpack_id: String,
    pub mod_ids: Vec<String>,
}

#[derive(Resource, Reflect, Debug, Clone, Default)]
#[reflect(Resource)]
pub struct UsfExecutionPlan {
    pub routes_by_scale: HashMap<Scale, UsfScaleExecutionRoute>,
}
impl UsfExecutionPlan {
    pub fn route_for_scale(&self, scale: Scale) -> Option<&UsfScaleExecutionRoute> {
        self.routes_by_scale.get(&scale)
    }
}

fn script_dpt_samplers() -> HashSet<String> {
    HashSet::from([DPT_SAMPLER_KERNEL_DEFAULT_ID.to_string()])
}

fn script_dpt_categorizers() -> HashSet<String> {
    HashSet::from([DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string()])
}

fn normalize_zone_type(value: &str) -> ZoneTypeId {
    ZoneTypeId::new(value.trim().to_ascii_lowercase())
}

fn script_zone_types() -> Vec<ZoneTypeId> {
    let zone_types = USF_ZONE_TYPES().lock().unwrap().clone();
    let mut ordered = zone_types.into_iter().collect::<Vec<_>>();
    ordered.sort();
    ordered.into_iter().map(|zone_type| normalize_zone_type(&zone_type)).collect()
}

fn script_schema_overrides() -> Vec<(Scale, DptSchema)> {
    let schema_map = USF_DPT_SCHEMAS_BY_SCALE().lock().unwrap().clone();
    let mut ordered = schema_map.into_iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(scale_index, _)| *scale_index);

    ordered
        .into_iter()
        .filter_map(|(scale_index, script_schema)| {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                return None;
            };
            let metrics = script_schema
                .metrics
                .into_iter()
                .map(|metric| {
                    let value_type = DptMetricValueType::from_tag(&metric.value_type).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid value_type '{}'; expected one of: u8, u16, i32, f32, f64",
                            metric.name, metric.value_type
                        )
                    });
                    let storage_class = DptMetricStorageClass::from_tag(&metric.storage_class).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid storage_class '{}'; expected one of: uniform, brick",
                            metric.name, metric.storage_class
                        )
                    });

                    DptMetricDefinition {
                        id: DptMetricId(metric.id),
                        name: metric.name.trim().to_string(),
                        value_type,
                        semantics_tag: metric.semantics_tag.trim().to_string(),
                        storage_class,
                        derived: metric.derived,
                        min_scale_index: metric.min_scale_index,
                        max_scale_index: metric.max_scale_index,
                    }
                })
                .collect::<Vec<_>>();
            Some((
                scale,
                DptSchema {
                    revision: script_schema.revision,
                    metrics,
                    fallback_zone: normalize_zone_type(&script_schema.fallback_zone),
                },
            ))
        })
        .collect()
}

fn script_scales() -> HashMap<Scale, UsfScaleDefinition> {
    let bindings = USF_SCALES_BY_INDEX().lock().unwrap().clone();
    let mut ordered = bindings.into_iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(scale_index, _)| *scale_index);

    ordered
        .into_iter()
        .filter_map(|(scale_index, binding)| {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                return None;
            };
            Some((
                scale,
                UsfScaleDefinition {
                    dpt_sampler_id: binding.dpt_sampler_id.trim().to_ascii_lowercase(),
                    dpt_categorizer_id: binding.dpt_categorizer_id.trim().to_ascii_lowercase(),
                    chunk_store_key: binding.chunk_store_key.trim().to_ascii_lowercase(),
                },
            ))
        })
        .collect()
}

fn script_mods() -> HashMap<String, UsfScriptModDefinition> {
    USF_MODS_BY_ID()
        .lock()
        .unwrap()
        .iter()
        .map(|(mod_id, mod_definition)| {
            (
                mod_id.clone(),
                UsfScriptModDefinition {
                    priority: mod_definition.priority,
                    dependencies: mod_definition.dependencies.iter().map(|value| value.trim().to_ascii_lowercase()).collect(),
                    load_after: mod_definition.load_after.iter().map(|value| value.trim().to_ascii_lowercase()).collect(),
                    conflicts_with: mod_definition.conflicts_with.iter().map(|value| value.trim().to_ascii_lowercase()).collect(),
                },
            )
        })
        .collect()
}

fn script_modpacks() -> HashMap<String, UsfScriptModpackDefinition> {
    USF_MODPACKS_BY_ID()
        .lock()
        .unwrap()
        .iter()
        .map(|(modpack_id, profile)| {
            (
                modpack_id.clone(),
                UsfScriptModpackDefinition {
                    mod_ids: profile.mod_ids.iter().map(|mod_id| mod_id.trim().to_ascii_lowercase()).collect(),
                },
            )
        })
        .collect()
}

fn active_usf_modpack_id_from_config() -> String {
    match CONFIG().data.get("usf/active_modpack_id") {
        Some(ConfigValue::String(value)) => {
            let normalized = value.trim().to_ascii_lowercase();
            if normalized.is_empty() {
                panic!("USF active modpack resolve failed: 'usf/active_modpack_id' must not be empty");
            }
            normalized
        }
        Some(other) => panic!("USF active modpack resolve failed: 'usf/active_modpack_id' must be a string, got {:?}", other),
        None => panic!("USF active modpack resolve failed: 'usf/active_modpack_id' must be configured explicitly"),
    }
}

fn configured_mods_for_modpack(modpack_id: &str) -> (Vec<UsfConfiguredMod>, Vec<String>) {
    let modpacks = script_modpacks();
    let mods_by_id = script_mods();
    let Some(modpack) = modpacks.get(modpack_id) else {
        panic!("USF active modpack resolve failed: modpack '{}' is not registered", modpack_id);
    };
    if modpack.mod_ids.is_empty() {
        panic!("USF active modpack resolve failed: modpack '{}' contains no mods", modpack_id);
    }

    let mut configured_mods = Vec::<UsfConfiguredMod>::new();
    let mut modpack_index_by_mod = HashMap::<String, usize>::new();
    let mut seen_mod_ids = HashSet::<String>::new();
    for (index, mod_id) in modpack.mod_ids.iter().enumerate() {
        if !seen_mod_ids.insert(mod_id.clone()) {
            panic!(
                "USF active modpack resolve failed: modpack '{}' contains duplicate mod '{}'",
                modpack_id, mod_id
            );
        }
        modpack_index_by_mod.insert(mod_id.clone(), index);
        let Some(_mod_definition) = mods_by_id.get(mod_id) else {
            panic!(
                "USF active modpack resolve failed: modpack '{}' references unknown mod '{}'",
                modpack_id, mod_id
            );
        };
        configured_mods.push(UsfConfiguredMod { mod_id: mod_id.clone() });
    }

    let enabled_mod_ids = configured_mods.iter().map(|entry| entry.mod_id.clone()).collect::<Vec<_>>();
    let enabled_mod_set = enabled_mod_ids.iter().cloned().collect::<HashSet<_>>();

    for mod_id in &enabled_mod_ids {
        let mod_definition = mods_by_id
            .get(mod_id)
            .unwrap_or_else(|| panic!("USF active modpack resolve failed: mod '{}' definition missing unexpectedly", mod_id));

        for dependency in &mod_definition.dependencies {
            if !mods_by_id.contains_key(dependency) {
                panic!("USF active modpack resolve failed: mod '{}' depends_on unknown mod '{}'", mod_id, dependency);
            }
            if !enabled_mod_set.contains(dependency) {
                panic!(
                    "USF active modpack resolve failed: mod '{}' depends_on '{}' but dependency is not enabled in modpack '{}'",
                    mod_id, dependency, modpack_id
                );
            }
        }

        for conflict in &mod_definition.conflicts_with {
            if enabled_mod_set.contains(conflict) {
                panic!(
                    "USF active modpack resolve failed: mod '{}' conflicts_with '{}' and both are enabled in modpack '{}'",
                    mod_id, conflict, modpack_id
                );
            }
        }
    }

    let mut indegree = HashMap::<String, usize>::new();
    let mut edges = HashMap::<String, HashSet<String>>::new();
    for mod_id in &enabled_mod_ids {
        indegree.insert(mod_id.clone(), 0);
        edges.insert(mod_id.clone(), HashSet::new());
    }

    for mod_id in &enabled_mod_ids {
        let mod_definition = mods_by_id
            .get(mod_id)
            .unwrap_or_else(|| panic!("USF active modpack resolve failed: mod '{}' definition missing unexpectedly", mod_id));

        for dependency in &mod_definition.dependencies {
            if !enabled_mod_set.contains(dependency) {
                continue;
            }
            let adjacency = edges
                .get_mut(dependency.as_str())
                .unwrap_or_else(|| panic!("USF active modpack resolve failed: graph missing dependency node '{}'", dependency));
            if adjacency.insert(mod_id.clone()) {
                *indegree
                    .get_mut(mod_id.as_str())
                    .unwrap_or_else(|| panic!("USF active modpack resolve failed: graph missing indegree for '{}'", mod_id)) += 1;
            }
        }

        for after_mod_id in &mod_definition.load_after {
            if !mods_by_id.contains_key(after_mod_id) {
                panic!("USF active modpack resolve failed: mod '{}' load_after unknown mod '{}'", mod_id, after_mod_id);
            }
            if !enabled_mod_set.contains(after_mod_id) {
                continue;
            }
            let adjacency = edges
                .get_mut(after_mod_id.as_str())
                .unwrap_or_else(|| panic!("USF active modpack resolve failed: graph missing load_after node '{}'", after_mod_id));
            if adjacency.insert(mod_id.clone()) {
                *indegree
                    .get_mut(mod_id.as_str())
                    .unwrap_or_else(|| panic!("USF active modpack resolve failed: graph missing indegree for '{}'", mod_id)) += 1;
            }
        }
    }

    let mut resolved_enabled = Vec::<String>::new();
    let mut ready = indegree
        .iter()
        .filter_map(|(mod_id, degree)| if *degree == 0 { Some(mod_id.clone()) } else { None })
        .collect::<Vec<_>>();

    while !ready.is_empty() {
        ready.sort_by(|left, right| {
            let left_definition = mods_by_id
                .get(left)
                .unwrap_or_else(|| panic!("USF active modpack resolve failed: mod '{}' missing during dependency resolution", left));
            let right_definition = mods_by_id
                .get(right)
                .unwrap_or_else(|| panic!("USF active modpack resolve failed: mod '{}' missing during dependency resolution", right));
            right_definition
                .priority
                .cmp(&left_definition.priority)
                .then_with(|| {
                    modpack_index_by_mod
                        .get(left)
                        .copied()
                        .unwrap_or(usize::MAX)
                        .cmp(&modpack_index_by_mod.get(right).copied().unwrap_or(usize::MAX))
                })
                .then_with(|| left.cmp(right))
        });
        let current = ready.remove(0);
        resolved_enabled.push(current.clone());
        let outgoing = edges
            .get(current.as_str())
            .cloned()
            .unwrap_or_else(|| panic!("USF active modpack resolve failed: graph missing adjacency for mod '{}'", current));
        for downstream in outgoing {
            let degree = indegree
                .get_mut(downstream.as_str())
                .unwrap_or_else(|| panic!("USF active modpack resolve failed: graph missing indegree for mod '{}'", downstream));
            *degree = degree.saturating_sub(1);
            if *degree == 0 {
                ready.push(downstream);
            }
        }
    }

    if resolved_enabled.len() != enabled_mod_ids.len() {
        let unresolved = indegree
            .iter()
            .filter_map(|(mod_id, degree)| if *degree > 0 { Some(mod_id.clone()) } else { None })
            .collect::<Vec<_>>();
        panic!(
            "USF active modpack resolve failed: dependency cycle detected in modpack '{}'; unresolved mods: {:?}",
            modpack_id, unresolved
        );
    }

    (configured_mods, resolved_enabled)
}

fn validate_usf_active_modpack_system(active_modpack: Res<UsfActiveModpack>) {
    if let Err(reason) = active_modpack.validate() {
        panic!("USF active modpack validation failed: {reason}");
    }
}

fn rebuild_usf_execution_plan_system(mut execution_plan: ResMut<UsfExecutionPlan>, active_modpack: Res<UsfActiveModpack>) {
    execution_plan.routes_by_scale.clear();
    let enabled_mod_ids = active_modpack.enabled_mods_in_profile_order();
    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let scale_definition = active_modpack
            .scale_definition_for_scale(scale)
            .unwrap_or_else(|| panic!("USF execution plan rebuild missing scale definition for scale index {}", scale.index_from_top()));
        if enabled_mod_ids.is_empty() {
            panic!(
                "USF execution plan rebuild failed: active modpack '{}' has no enabled mods",
                active_modpack.modpack_id
            );
        }

        execution_plan.routes_by_scale.insert(
            scale,
            UsfScaleExecutionRoute {
                dpt_sampler_id: scale_definition.dpt_sampler_id.clone(),
                dpt_categorizer_id: scale_definition.dpt_categorizer_id.clone(),
                chunk_store_key: scale_definition.chunk_store_key.clone(),
                usf_modpack_id: active_modpack.modpack_id.clone(),
                mod_ids: enabled_mod_ids.clone(),
            },
        );
    }
}

fn validate_usf_execution_plan_system(execution_plan: Res<UsfExecutionPlan>, active_modpack: Res<UsfActiveModpack>) {
    if active_modpack.enabled_mods.is_empty() {
        panic!("USF execution plan validation failed: active modpack has no enabled mods");
    }

    for index in 0..Scale::SCALE_LEVEL_COUNT {
        let Some(scale) = Scale::from_index_from_top(index) else {
            continue;
        };
        let Some(route) = execution_plan.route_for_scale(scale) else {
            panic!(
                "USF execution plan missing route for scale index {} (modpack='{}')",
                scale.index_from_top(),
                active_modpack.modpack_id
            );
        };
        let Some(scale_definition) = active_modpack.scale_definition_for_scale(scale) else {
            panic!(
                "USF execution plan validation failed: missing active modpack scale definition for scale {}",
                scale.index_from_top()
            );
        };
        if route.dpt_sampler_id != scale_definition.dpt_sampler_id
            || route.dpt_categorizer_id != scale_definition.dpt_categorizer_id
            || route.chunk_store_key != scale_definition.chunk_store_key
        {
            panic!(
                "USF execution plan route mismatch at scale {} against active modpack scale definition",
                scale.index_from_top()
            );
        }
        if route.usf_modpack_id != active_modpack.modpack_id {
            panic!(
                "USF execution plan route mismatch at scale {}: active modpack='{}', route modpack='{}'",
                scale.index_from_top(),
                active_modpack.modpack_id,
                route.usf_modpack_id
            );
        }
        if route.mod_ids.is_empty() {
            panic!(
                "USF execution plan route has no mods at scale {} for modpack '{}'",
                scale.index_from_top(),
                route.usf_modpack_id
            );
        }
        for mod_id in &route.mod_ids {
            if !active_modpack.enabled_mods.contains(mod_id) {
                panic!(
                    "USF execution plan route references mod '{}' at scale {} \
                     but it is not enabled by active modpack '{}'",
                    mod_id,
                    scale.index_from_top(),
                    active_modpack.modpack_id
                );
            }
        }
    }
}

pub(crate) struct ContentPlugin;
impl Plugin for ContentPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfActiveModpack>().init_resource::<UsfExecutionPlan>().add_systems(
            Startup,
            (
                validate_usf_active_modpack_system,
                rebuild_usf_execution_plan_system,
                validate_usf_execution_plan_system,
            )
                .chain()
                .in_set(AppSet::Diagnostics),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scripted_scales_and_kernels() -> (HashMap<Scale, UsfScaleDefinition>, HashSet<String>, HashSet<String>) {
        let sampler_id = "dpt_sampler.test.default.v1".to_string();
        let categorizer_id = "dpt_categorizer.test.default.v1".to_string();
        let mut scales_by_index = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            scales_by_index.insert(
                scale,
                UsfScaleDefinition {
                    dpt_sampler_id: sampler_id.clone(),
                    dpt_categorizer_id: categorizer_id.clone(),
                    chunk_store_key: "chunk_store.test.default".to_string(),
                },
            );
        }

        (scales_by_index, HashSet::from([sampler_id]), HashSet::from([categorizer_id]))
    }

    fn scripted_active_modpack() -> UsfActiveModpack {
        let mut known_zone_types = HashSet::new();
        known_zone_types.insert(ZoneTypeId::new("void"));
        let (scales_by_index, known_dpt_samplers, known_dpt_categorizers) = scripted_scales_and_kernels();

        let mut schemas_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let scale_index = scale.index_from_top();
            schemas_by_scale.insert(
                scale,
                DptSchema {
                    revision: 1,
                    metrics: vec![DptMetricDefinition {
                        id: DptMetricId(0),
                        name: "density".to_string(),
                        value_type: DptMetricValueType::F32,
                        semantics_tag: "matter.density.normalized".to_string(),
                        storage_class: DptMetricStorageClass::Brick,
                        derived: false,
                        min_scale_index: scale_index,
                        max_scale_index: scale_index,
                    }],
                    fallback_zone: ZoneTypeId::new("void"),
                },
            );
        }

        UsfActiveModpack {
            modpack_id: "modpack.test.default".to_string(),
            configured_mods: vec![UsfConfiguredMod {
                mod_id: "mod.test.default".to_string(),
            }],
            enabled_mods: HashSet::from(["mod.test.default".to_string()]),
            resolved_enabled_mods: vec!["mod.test.default".to_string()],
            scales_by_index,
            known_dpt_samplers,
            known_dpt_categorizers,
            schemas_by_scale,
            known_zone_types,
        }
    }

    #[test]
    fn active_modpack_is_valid() {
        let active_modpack = scripted_active_modpack();
        assert!(active_modpack.validate().is_ok());
    }

    #[test]
    fn active_modpack_rejects_missing_scale_definition() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.scales_by_index.remove(&Scale::MAX);
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("missing scale definition"));
    }

    #[test]
    fn active_modpack_rejects_unknown_sampler_id() {
        let mut active_modpack = scripted_active_modpack();
        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.dpt_sampler_id = "dpt_sampler.unknown".to_string();
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("unknown dpt_sampler_id"));
    }

    #[test]
    fn active_modpack_rejects_missing_scale_schema() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.schemas_by_scale.remove(&Scale::MAX);
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("missing DPT schema"));
    }

    #[test]
    fn active_modpack_rejects_duplicate_configured_mod_ids() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.configured_mods.push(UsfConfiguredMod {
            mod_id: "mod.test.default".to_string(),
        });
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("duplicate configured mod"));
    }

    #[test]
    fn active_modpack_rejects_enabled_mod_not_configured() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.enabled_mods.insert("mod.test.unknown".to_string());
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("marks unknown mod"));
    }
}
