use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{ScriptUsfConceptCatalog, USF_CONCEPT_CATALOG};
use crate::usf::metric::{MetricDefinition, MetricId, MetricStorageClass, MetricValueType};
use crate::usf::metric_container::{MetricContainerLayout, is_sampler_kernel_id_supported};
use crate::usf::mods::UsfConfiguredMod;
use crate::usf::scale::Scale;
use crate::usf::zlm::is_categorizer_kernel_id_supported;
use crate::usf::zone::ZoneTypeId;

pub const DEFAULT_DEMO_MOD_ID: &str = "demo";
#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct UsfScaleDefinition {
    pub metric_sampler_id: String,
    pub metric_categorizer_id: String,
    pub chunk_store_key: String,
}

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct UsfActiveModPack {
    pub mod_pack_id: String,
    pub configured_mods: Vec<UsfConfiguredMod>,
    pub enabled_mods: HashSet<String>,
    pub resolved_enabled_mods: Vec<String>,
    pub scales_by_index: HashMap<Scale, UsfScaleDefinition>,
    pub known_metric_samplers: HashSet<String>,
    pub known_metric_categorizers: HashSet<String>,
    pub schemas_by_scale: HashMap<Scale, MetricContainerLayout>,
    pub known_zone_types: HashSet<ZoneTypeId>,
}
impl Default for UsfActiveModPack {
    fn default() -> Self {
        let (mod_pack_id, configured_mods, resolved_enabled_mods) = configured_mods_for_active_modpack_from_catalog();
        let enabled_mods = configured_mods.iter().map(|mod_entry| mod_entry.mod_id.clone()).collect::<HashSet<_>>();

        let mut active_modpack = Self {
            mod_pack_id,
            configured_mods,
            enabled_mods,
            resolved_enabled_mods,
            scales_by_index: HashMap::new(),
            known_metric_samplers: script_metric_samplers(),
            known_metric_categorizers: script_metric_categorizers(),
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
impl UsfActiveModPack {
    pub fn schema_for_scale(&self, scale: Scale) -> Option<&MetricContainerLayout> {
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
        if self.mod_pack_id.trim().is_empty() {
            return Err("USF active modpack id must not be empty".to_string());
        }
        if self.enabled_mods.is_empty() {
            return Err(format!("USF active modpack '{}' resolved to zero enabled mods", self.mod_pack_id));
        }
        if self.resolved_enabled_mods.is_empty() {
            return Err(format!("USF active modpack '{}' resolved no ordered enabled mods", self.mod_pack_id));
        }
        if self.configured_mods.is_empty() {
            return Err(format!("USF active modpack '{}' has no configured mods", self.mod_pack_id));
        }
        if self.schemas_by_scale.is_empty() {
            return Err(format!("USF active modpack '{}' has no metric container layouts", self.mod_pack_id));
        }
        if self.scales_by_index.is_empty() {
            return Err(format!("USF active modpack '{}' has no scale definitions", self.mod_pack_id));
        }
        if self.known_metric_samplers.is_empty() {
            return Err(format!("USF active modpack '{}' has no known metric samplers", self.mod_pack_id));
        }
        if self.known_metric_categorizers.is_empty() {
            return Err(format!("USF active modpack '{}' has no known metric categorizers", self.mod_pack_id));
        }
        let mut configured_mod_ids = HashSet::<String>::new();
        for mod_entry in &self.configured_mods {
            if mod_entry.mod_id.trim().is_empty() {
                return Err(format!("USF active modpack '{}' has configured mod with empty id", self.mod_pack_id));
            }
            if !configured_mod_ids.insert(mod_entry.mod_id.clone()) {
                return Err(format!(
                    "USF active modpack '{}' has duplicate configured mod '{}'",
                    self.mod_pack_id, mod_entry.mod_id
                ));
            }
        }
        for mod_id in &self.enabled_mods {
            if !configured_mod_ids.contains(mod_id) {
                return Err(format!("USF active modpack '{}' marks unknown mod '{}' as enabled", self.mod_pack_id, mod_id));
            }
        }
        for mod_id in &self.resolved_enabled_mods {
            if !self.enabled_mods.contains(mod_id) {
                return Err(format!(
                    "USF active modpack '{}' has ordered enabled mod '{}' that is not in enabled_mods",
                    self.mod_pack_id, mod_id
                ));
            }
        }
        let resolved_enabled_set = self.resolved_enabled_mods.iter().cloned().collect::<HashSet<_>>();
        if resolved_enabled_set != self.enabled_mods {
            return Err(format!(
                "USF active modpack '{}' has mismatch between enabled_mods and resolved_enabled_mods",
                self.mod_pack_id
            ));
        }

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(scale_definition) = self.scales_by_index.get(&scale) else {
                return Err(format!("missing scale definition for scale index {}", scale.index_from_top()));
            };
            if scale_definition.metric_sampler_id.trim().is_empty() {
                return Err(format!("empty metric_sampler_id for scale index {}", scale.index_from_top()));
            }
            if scale_definition.metric_categorizer_id.trim().is_empty() {
                return Err(format!("empty metric_categorizer_id for scale index {}", scale.index_from_top()));
            }
            if scale_definition.chunk_store_key.trim().is_empty() {
                return Err(format!("empty chunk_store_key for scale index {}", scale.index_from_top()));
            }
            if !self.known_metric_samplers.contains(&scale_definition.metric_sampler_id) {
                return Err(format!(
                    "unknown metric_sampler_id '{}' for scale index {}",
                    scale_definition.metric_sampler_id,
                    scale.index_from_top()
                ));
            }
            if !self.known_metric_categorizers.contains(&scale_definition.metric_categorizer_id) {
                return Err(format!(
                    "unknown metric_categorizer_id '{}' for scale index {}",
                    scale_definition.metric_categorizer_id,
                    scale.index_from_top()
                ));
            }
            if !is_sampler_kernel_id_supported(scale_definition.metric_sampler_id.as_str()) {
                return Err(format!(
                    "metric_sampler_id '{}' for scale index {} is registered but not runtime-supported",
                    scale_definition.metric_sampler_id,
                    scale.index_from_top()
                ));
            }
            if !is_categorizer_kernel_id_supported(scale_definition.metric_categorizer_id.as_str()) {
                return Err(format!(
                    "metric_categorizer_id '{}' for scale index {} is registered but not runtime-supported",
                    scale_definition.metric_categorizer_id,
                    scale.index_from_top()
                ));
            }
            let Some(schema) = self.schemas_by_scale.get(&scale) else {
                return Err(format!("missing metric container layout for scale index {}", scale.index_from_top()));
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
pub struct UsfScaleExecutionRoute {
    pub metric_sampler_id: String,
    pub metric_categorizer_id: String,
    pub chunk_store_key: String,
    pub usf_mod_pack_id: String,
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

fn script_concept_catalog() -> ScriptUsfConceptCatalog {
    USF_CONCEPT_CATALOG().lock().unwrap().clone()
}

fn configured_mods_for_active_modpack_from_catalog() -> (String, Vec<UsfConfiguredMod>, Vec<String>) {
    let catalog = script_concept_catalog();
    let mod_pack_id = catalog.active_modpack_id.trim().to_ascii_lowercase();
    if mod_pack_id.is_empty() {
        panic!("USF active modpack resolve failed: concept catalog has no active modpack id");
    }

    let Some(active_modpack) = catalog.modpacks_by_id.get(&mod_pack_id) else {
        panic!("USF active modpack resolve failed: modpack '{}' is not present in concept catalog", mod_pack_id);
    };
    if active_modpack.mod_ids.is_empty() {
        panic!("USF active modpack resolve failed: modpack '{}' contains no mods", mod_pack_id);
    }

    let configured_mods = active_modpack
        .mod_ids
        .iter()
        .map(|mod_id| mod_id.trim().to_ascii_lowercase())
        .map(|mod_id| {
            if !catalog.mods_by_id.contains_key(&mod_id) {
                panic!(
                    "USF active modpack resolve failed: modpack '{}' references unknown mod '{}' in concept catalog",
                    mod_pack_id, mod_id
                );
            }
            UsfConfiguredMod { mod_id }
        })
        .collect::<Vec<_>>();

    let resolved_enabled_mods = catalog
        .resolved_mod_ids
        .iter()
        .map(|mod_id| mod_id.trim().to_ascii_lowercase())
        .collect::<Vec<_>>();
    if resolved_enabled_mods.is_empty() {
        panic!(
            "USF active modpack resolve failed: concept catalog has no resolved enabled mods for modpack '{}'",
            mod_pack_id
        );
    }
    let configured_mod_set = configured_mods.iter().map(|entry| entry.mod_id.clone()).collect::<HashSet<_>>();
    for mod_id in &resolved_enabled_mods {
        if !configured_mod_set.contains(mod_id) {
            panic!(
                "USF active modpack resolve failed: resolved mod '{}' is not configured by modpack '{}'",
                mod_id, mod_pack_id
            );
        }
    }

    (mod_pack_id, configured_mods, resolved_enabled_mods)
}

fn script_metric_samplers() -> HashSet<String> {
    let kernels = script_concept_catalog().composed.metric_sampler_kernel_ids;
    if kernels.is_empty() {
        panic!("USF content bootstrap failed: no metric sampler kernels in concept catalog");
    }
    kernels
}

fn script_metric_categorizers() -> HashSet<String> {
    let kernels = script_concept_catalog().composed.metric_categorizer_kernel_ids;
    if kernels.is_empty() {
        panic!("USF content bootstrap failed: no metric categorizer kernels in concept catalog");
    }
    kernels
}

fn normalize_zone_type(value: &str) -> ZoneTypeId {
    ZoneTypeId::new(value.trim().to_ascii_lowercase())
}

fn script_zone_types() -> Vec<ZoneTypeId> {
    let mut ordered = script_concept_catalog().composed.zone_types.into_iter().collect::<Vec<_>>();
    ordered.sort();
    ordered.into_iter().map(|zone_type| normalize_zone_type(&zone_type)).collect()
}

fn script_schema_overrides() -> Vec<(Scale, MetricContainerLayout)> {
    let mut ordered = script_concept_catalog()
        .composed
        .metric_container_layouts_by_scale
        .into_iter()
        .collect::<Vec<_>>();
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
                    let value_type = MetricValueType::from_tag(&metric.value_type).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid value_type '{}'; expected one of: u8, u16, i32, f32, f64",
                            metric.name, metric.value_type
                        )
                    });
                    let storage_class = MetricStorageClass::from_tag(&metric.storage_class).unwrap_or_else(|| {
                        panic!(
                            "USF script metric '{}' has invalid storage_class '{}'; expected one of: uniform, brick",
                            metric.name, metric.storage_class
                        )
                    });

                    MetricDefinition {
                        id: MetricId(metric.id),
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
                MetricContainerLayout {
                    revision: script_schema.revision,
                    metrics,
                    fallback_zone: normalize_zone_type(&script_schema.fallback_zone),
                },
            ))
        })
        .collect()
}

fn script_scales() -> HashMap<Scale, UsfScaleDefinition> {
    let mut ordered = script_concept_catalog().composed.scales_by_index.into_iter().collect::<Vec<_>>();
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
                    metric_sampler_id: binding.metric_sampler_id.trim().to_ascii_lowercase(),
                    metric_categorizer_id: binding.metric_categorizer_id.trim().to_ascii_lowercase(),
                    chunk_store_key: binding.chunk_store_key.trim().to_ascii_lowercase(),
                },
            ))
        })
        .collect()
}

fn validate_usf_active_mod_pack_system(active_modpack: Res<UsfActiveModPack>) {
    if let Err(reason) = active_modpack.validate() {
        panic!("USF active modpack validation failed: {reason}");
    }
}

fn rebuild_usf_execution_plan_system(mut execution_plan: ResMut<UsfExecutionPlan>, active_modpack: Res<UsfActiveModPack>) {
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
                active_modpack.mod_pack_id
            );
        }

        execution_plan.routes_by_scale.insert(
            scale,
            UsfScaleExecutionRoute {
                metric_sampler_id: scale_definition.metric_sampler_id.clone(),
                metric_categorizer_id: scale_definition.metric_categorizer_id.clone(),
                chunk_store_key: scale_definition.chunk_store_key.clone(),
                usf_mod_pack_id: active_modpack.mod_pack_id.clone(),
                mod_ids: enabled_mod_ids.clone(),
            },
        );
    }
}

fn validate_usf_execution_plan_system(execution_plan: Res<UsfExecutionPlan>, active_modpack: Res<UsfActiveModPack>) {
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
                active_modpack.mod_pack_id
            );
        };
        let Some(scale_definition) = active_modpack.scale_definition_for_scale(scale) else {
            panic!(
                "USF execution plan validation failed: missing active modpack scale definition for scale {}",
                scale.index_from_top()
            );
        };
        if route.metric_sampler_id != scale_definition.metric_sampler_id
            || route.metric_categorizer_id != scale_definition.metric_categorizer_id
            || route.chunk_store_key != scale_definition.chunk_store_key
        {
            panic!(
                "USF execution plan route mismatch at scale {} against active modpack scale definition",
                scale.index_from_top()
            );
        }
        if route.usf_mod_pack_id != active_modpack.mod_pack_id {
            panic!(
                "USF execution plan route mismatch at scale {}: active modpack='{}', route modpack='{}'",
                scale.index_from_top(),
                active_modpack.mod_pack_id,
                route.usf_mod_pack_id
            );
        }
        if route.mod_ids.is_empty() {
            panic!(
                "USF execution plan route has no mods at scale {} for modpack '{}'",
                scale.index_from_top(),
                route.usf_mod_pack_id
            );
        }
        for mod_id in &route.mod_ids {
            if !active_modpack.enabled_mods.contains(mod_id) {
                panic!(
                    "USF execution plan route references mod '{}' at scale {} \
                     but it is not enabled by active modpack '{}'",
                    mod_id,
                    scale.index_from_top(),
                    active_modpack.mod_pack_id
                );
            }
        }
    }
}

pub(crate) struct ModPacksPlugin;
impl Plugin for ModPacksPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<UsfActiveModPack>().init_resource::<UsfExecutionPlan>().add_systems(
            Startup,
            (
                validate_usf_active_mod_pack_system,
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
        let sampler_id = crate::usf::metric_container::METRIC_SAMPLER_KERNEL_DEFAULT_ID.to_string();
        let categorizer_id = crate::usf::zlm::METRIC_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string();
        let mut scales_by_index = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            scales_by_index.insert(
                scale,
                UsfScaleDefinition {
                    metric_sampler_id: sampler_id.clone(),
                    metric_categorizer_id: categorizer_id.clone(),
                    chunk_store_key: "chunk_store.test.default".to_string(),
                },
            );
        }

        (scales_by_index, HashSet::from([sampler_id]), HashSet::from([categorizer_id]))
    }

    fn scripted_active_modpack() -> UsfActiveModPack {
        let mut known_zone_types = HashSet::new();
        known_zone_types.insert(ZoneTypeId::new("void"));
        let (scales_by_index, known_metric_samplers, known_metric_categorizers) = scripted_scales_and_kernels();

        let mut schemas_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let scale_index = scale.index_from_top();
            schemas_by_scale.insert(
                scale,
                MetricContainerLayout {
                    revision: 1,
                    metrics: vec![MetricDefinition {
                        id: MetricId(0),
                        name: "density".to_string(),
                        value_type: MetricValueType::F32,
                        semantics_tag: "matter.density.normalized".to_string(),
                        storage_class: MetricStorageClass::Brick,
                        derived: false,
                        min_scale_index: scale_index,
                        max_scale_index: scale_index,
                    }],
                    fallback_zone: ZoneTypeId::new("void"),
                },
            );
        }

        UsfActiveModPack {
            mod_pack_id: "modpack.test.default".to_string(),
            configured_mods: vec![UsfConfiguredMod {
                mod_id: "mod.test.default".to_string(),
            }],
            enabled_mods: HashSet::from(["mod.test.default".to_string()]),
            resolved_enabled_mods: vec!["mod.test.default".to_string()],
            scales_by_index,
            known_metric_samplers,
            known_metric_categorizers,
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
        scale_definition.metric_sampler_id = "metric_sampler.unknown".to_string();
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("unknown metric_sampler_id"));
    }

    #[test]
    fn active_modpack_rejects_registered_but_runtime_unsupported_sampler_id() {
        let mut active_modpack = scripted_active_modpack();
        let unsupported = "metric_sampler.custom_legacy_id".to_string();
        active_modpack.known_metric_samplers.insert(unsupported.clone());
        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.metric_sampler_id = unsupported;
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("not runtime-supported"));
    }

    #[test]
    fn active_modpack_rejects_registered_but_runtime_unsupported_categorizer_id() {
        let mut active_modpack = scripted_active_modpack();
        let unsupported = "metric_categorizer.custom_legacy_id".to_string();
        active_modpack.known_metric_categorizers.insert(unsupported.clone());
        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.metric_categorizer_id = unsupported;
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("not runtime-supported"));
    }

    #[test]
    fn active_modpack_accepts_runtime_supported_custom_kernel_ids() {
        let mut active_modpack = scripted_active_modpack();
        let custom_sampler = "metric_sampler.kernel.scale_max.experimental.v1".to_string();
        let custom_categorizer = "metric_categorizer.kernel.scale_max.experimental.v1".to_string();
        active_modpack.known_metric_samplers.insert(custom_sampler.clone());
        active_modpack.known_metric_categorizers.insert(custom_categorizer.clone());

        let scale_definition = active_modpack.scales_by_index.get_mut(&Scale::MAX).unwrap();
        scale_definition.metric_sampler_id = custom_sampler;
        scale_definition.metric_categorizer_id = custom_categorizer;

        let validation = active_modpack.validate();
        assert!(validation.is_ok(), "expected custom runtime-supported kernel ids to validate, got: {validation:?}");
    }

    #[test]
    fn active_modpack_rejects_missing_scale_schema() {
        let mut active_modpack = scripted_active_modpack();
        active_modpack.schemas_by_scale.remove(&Scale::MAX);
        let error = active_modpack.validate().unwrap_err();
        assert!(error.contains("missing metric container layout"));
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
