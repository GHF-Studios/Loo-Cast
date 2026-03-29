use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{
    USF_DPT_CATEGORIZER_IDS, USF_DPT_SAMPLER_IDS, USF_DPT_SCHEMAS_BY_SCALE, USF_SCALE_BINDINGS_BY_SCALE, USF_ZONE_TYPES,
};
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DptMetricId(pub u16);

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ZoneTypeId(pub String);
impl ZoneTypeId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub enum DptMetricValueType {
    U8,
    U16,
    I32,
    F32,
    F64,
}
impl DptMetricValueType {
    pub fn from_tag(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "u8" => Some(Self::U8),
            "u16" => Some(Self::U16),
            "i32" => Some(Self::I32),
            "f32" => Some(Self::F32),
            "f64" => Some(Self::F64),
            _ => None,
        }
    }
}
impl Default for DptMetricValueType {
    fn default() -> Self {
        Self::F32
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub enum DptMetricStorageClass {
    Uniform,
    Brick,
}
impl DptMetricStorageClass {
    pub fn from_tag(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "uniform" => Some(Self::Uniform),
            "brick" => Some(Self::Brick),
            _ => None,
        }
    }
}
impl Default for DptMetricStorageClass {
    fn default() -> Self {
        Self::Brick
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct DptMetricDefinition {
    pub id: DptMetricId,
    pub name: String,
    pub value_type: DptMetricValueType,
    pub semantics_tag: String,
    pub storage_class: DptMetricStorageClass,
    pub derived: bool,
    pub min_scale_index: u8,
    pub max_scale_index: u8,
}
impl DptMetricDefinition {
    pub fn applies_to_scale(&self, scale: Scale) -> bool {
        let index = scale.index_from_top();
        (self.min_scale_index..=self.max_scale_index).contains(&index)
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct DptSchema {
    pub revision: u64,
    pub metrics: Vec<DptMetricDefinition>,
    pub fallback_zone: ZoneTypeId,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Default)]
pub struct ScaleContentBinding {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
}
impl DptSchema {
    pub fn metric_index(&self, metric_id: DptMetricId) -> Option<usize> {
        self.metrics.iter().position(|metric| metric.id == metric_id)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.metrics.is_empty() {
            return Err("DPT schema must contain at least one metric".to_string());
        }
        if self.revision == 0 {
            return Err("DPT schema revision must be > 0".to_string());
        }
        if self.fallback_zone.0.trim().is_empty() {
            return Err("DPT schema fallback zone must not be empty".to_string());
        }

        let mut seen_metric_ids = HashSet::<DptMetricId>::new();
        let mut seen_metric_names = HashSet::<String>::new();
        for metric in &self.metrics {
            if !seen_metric_ids.insert(metric.id) {
                return Err(format!("duplicate DPT metric id {}", metric.id.0));
            }
            let normalized_name = metric.name.trim().to_ascii_lowercase();
            if normalized_name.is_empty() {
                return Err(format!("metric {} has an empty name", metric.id.0));
            }
            if !seen_metric_names.insert(normalized_name.clone()) {
                return Err(format!("duplicate DPT metric name '{normalized_name}'"));
            }
            let normalized_semantics = metric.semantics_tag.trim().to_ascii_lowercase();
            if normalized_semantics.is_empty() {
                return Err(format!("metric {} has an empty semantics_tag", metric.id.0));
            }
            if metric.min_scale_index >= Scale::SCALE_LEVEL_COUNT {
                return Err(format!(
                    "metric {} has min_scale_index={} outside [0..{}]",
                    metric.id.0,
                    metric.min_scale_index,
                    Scale::SCALE_LEVEL_COUNT.saturating_sub(1)
                ));
            }
            if metric.max_scale_index >= Scale::SCALE_LEVEL_COUNT {
                return Err(format!(
                    "metric {} has max_scale_index={} outside [0..{}]",
                    metric.id.0,
                    metric.max_scale_index,
                    Scale::SCALE_LEVEL_COUNT.saturating_sub(1)
                ));
            }
            if metric.min_scale_index > metric.max_scale_index {
                return Err(format!(
                    "metric {} has invalid scale range [{}..{}]",
                    metric.id.0, metric.min_scale_index, metric.max_scale_index
                ));
            }
        }

        Ok(())
    }
}

#[derive(Resource, Debug, Clone)]
pub struct DefinitionRegistry {
    pub schemas_by_scale: HashMap<Scale, DptSchema>,
    pub known_zone_types: HashSet<ZoneTypeId>,
}

#[derive(Resource, Debug, Clone)]
pub struct ScaleContentRegistry {
    pub bindings_by_scale: HashMap<Scale, ScaleContentBinding>,
    pub known_dpt_samplers: HashSet<String>,
    pub known_dpt_categorizers: HashSet<String>,
}
impl Default for ScaleContentRegistry {
    fn default() -> Self {
        let script_samplers = script_dpt_samplers();
        let script_categorizers = script_dpt_categorizers();
        let script_bindings = script_scale_bindings();
        let script_authored = !script_samplers.is_empty() || !script_categorizers.is_empty() || !script_bindings.is_empty();

        let registry = if script_authored {
            Self {
                bindings_by_scale: script_bindings,
                known_dpt_samplers: script_samplers,
                known_dpt_categorizers: script_categorizers,
            }
        } else {
            let mut bindings_by_scale = HashMap::new();
            for index in 0..Scale::SCALE_LEVEL_COUNT {
                let Some(scale) = Scale::from_index_from_top(index) else {
                    continue;
                };
                bindings_by_scale.insert(scale, baseline_scale_binding(scale));
            }
            Self {
                bindings_by_scale,
                known_dpt_samplers: default_dpt_samplers(),
                known_dpt_categorizers: default_dpt_categorizers(),
            }
        };

        if let Err(reason) = registry.validate() {
            panic!("USF scale content registry default validation failed: {reason}");
        }

        registry
    }
}
impl Default for DefinitionRegistry {
    fn default() -> Self {
        let script_zone_types = script_zone_types();
        let script_schemas = script_schema_overrides();
        let script_authored = !script_zone_types.is_empty() || !script_schemas.is_empty();

        let mut registry = Self {
            schemas_by_scale: HashMap::new(),
            known_zone_types: HashSet::new(),
        };

        if script_authored {
            for zone in script_zone_types {
                registry.known_zone_types.insert(zone);
            }
            for (scale, schema) in script_schemas {
                registry.register_scale_schema(scale, schema);
            }
        } else {
            for zone in default_zone_types() {
                registry.known_zone_types.insert(zone);
            }
            for index in 0..Scale::SCALE_LEVEL_COUNT {
                let Some(scale) = Scale::from_index_from_top(index) else {
                    continue;
                };
                registry.register_scale_schema(scale, baseline_schema_for_scale(scale));
            }
        }

        if let Err(reason) = registry.validate() {
            panic!("USF definition registry default validation failed: {reason}");
        }

        registry
    }
}

fn normalize_zone_type(value: &str) -> ZoneTypeId {
    ZoneTypeId::new(value.trim().to_ascii_lowercase())
}

fn default_zone_types() -> Vec<ZoneTypeId> {
    vec![
        normalize_zone_type("void"),
        normalize_zone_type("forest"),
        normalize_zone_type("wetland"),
        normalize_zone_type("arid"),
        normalize_zone_type("alpine"),
    ]
}

fn baseline_schema_for_scale(_scale: Scale) -> DptSchema {
    let min_scale_index = 0;
    let max_scale_index = Scale::SCALE_LEVEL_COUNT.saturating_sub(1);
    DptSchema {
        revision: 1,
        metrics: vec![
            DptMetricDefinition {
                id: DptMetricId(0),
                name: "temperature".to_string(),
                value_type: DptMetricValueType::F32,
                semantics_tag: "climate.temperature.normalized".to_string(),
                storage_class: DptMetricStorageClass::Brick,
                derived: false,
                min_scale_index,
                max_scale_index,
            },
            DptMetricDefinition {
                id: DptMetricId(1),
                name: "humidity".to_string(),
                value_type: DptMetricValueType::F32,
                semantics_tag: "climate.humidity.normalized".to_string(),
                storage_class: DptMetricStorageClass::Brick,
                derived: false,
                min_scale_index,
                max_scale_index,
            },
            DptMetricDefinition {
                id: DptMetricId(2),
                name: "elevation".to_string(),
                value_type: DptMetricValueType::F32,
                semantics_tag: "terrain.elevation.normalized".to_string(),
                storage_class: DptMetricStorageClass::Brick,
                derived: false,
                min_scale_index,
                max_scale_index,
            },
            DptMetricDefinition {
                id: DptMetricId(3),
                name: "vegetation_density".to_string(),
                value_type: DptMetricValueType::F32,
                semantics_tag: "biosphere.vegetation_density.normalized".to_string(),
                storage_class: DptMetricStorageClass::Brick,
                derived: true,
                min_scale_index,
                max_scale_index,
            },
        ],
        fallback_zone: normalize_zone_type("void"),
    }
}

fn default_dpt_samplers() -> HashSet<String> {
    HashSet::from(["dpt_sampler.debug.default.v1".to_string()])
}

fn default_dpt_categorizers() -> HashSet<String> {
    HashSet::from(["dpt_categorizer.debug.zlm_lookup.v1".to_string()])
}

fn baseline_scale_binding(_scale: Scale) -> ScaleContentBinding {
    ScaleContentBinding {
        dpt_sampler_id: "dpt_sampler.debug.default.v1".to_string(),
        dpt_categorizer_id: "dpt_categorizer.debug.zlm_lookup.v1".to_string(),
        chunk_store_key: "chunk_store.default".to_string(),
    }
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

fn script_dpt_samplers() -> HashSet<String> {
    USF_DPT_SAMPLER_IDS().lock().unwrap().clone()
}

fn script_dpt_categorizers() -> HashSet<String> {
    USF_DPT_CATEGORIZER_IDS().lock().unwrap().clone()
}

fn script_scale_bindings() -> HashMap<Scale, ScaleContentBinding> {
    let bindings = USF_SCALE_BINDINGS_BY_SCALE().lock().unwrap().clone();
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
                ScaleContentBinding {
                    dpt_sampler_id: binding.dpt_sampler_id.trim().to_ascii_lowercase(),
                    dpt_categorizer_id: binding.dpt_categorizer_id.trim().to_ascii_lowercase(),
                    chunk_store_key: binding.chunk_store_key.trim().to_ascii_lowercase(),
                },
            ))
        })
        .collect()
}
impl DefinitionRegistry {
    pub fn register_scale_schema(&mut self, scale: Scale, schema: DptSchema) {
        self.schemas_by_scale.insert(scale, schema);
    }

    pub fn schema_for_scale(&self, scale: Scale) -> Option<&DptSchema> {
        self.schemas_by_scale.get(&scale)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.schemas_by_scale.is_empty() {
            return Err("USF definition registry has no schemas".to_string());
        }

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
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

impl ScaleContentRegistry {
    pub fn binding_for_scale(&self, scale: Scale) -> Option<&ScaleContentBinding> {
        self.bindings_by_scale.get(&scale)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.known_dpt_samplers.is_empty() {
            return Err("no DPT samplers registered".to_string());
        }
        if self.known_dpt_categorizers.is_empty() {
            return Err("no DPT categorizers registered".to_string());
        }
        if self.bindings_by_scale.is_empty() {
            return Err("no scale content bindings registered".to_string());
        }

        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(binding) = self.bindings_by_scale.get(&scale) else {
                return Err(format!("missing scale content binding for scale index {}", scale.index_from_top()));
            };
            if binding.dpt_sampler_id.trim().is_empty() {
                return Err(format!("empty dpt_sampler_id for scale index {}", scale.index_from_top()));
            }
            if binding.dpt_categorizer_id.trim().is_empty() {
                return Err(format!("empty dpt_categorizer_id for scale index {}", scale.index_from_top()));
            }
            if binding.chunk_store_key.trim().is_empty() {
                return Err(format!("empty chunk_store_key for scale index {}", scale.index_from_top()));
            }
            if !self.known_dpt_samplers.contains(&binding.dpt_sampler_id) {
                return Err(format!(
                    "unknown dpt_sampler_id '{}' for scale index {}",
                    binding.dpt_sampler_id,
                    scale.index_from_top()
                ));
            }
            if !self.known_dpt_categorizers.contains(&binding.dpt_categorizer_id) {
                return Err(format!(
                    "unknown dpt_categorizer_id '{}' for scale index {}",
                    binding.dpt_categorizer_id,
                    scale.index_from_top()
                ));
            }
        }

        Ok(())
    }
}

fn validate_definition_registry_system(registry: Res<DefinitionRegistry>) {
    if let Err(reason) = registry.validate() {
        panic!("USF definition registry validation failed: {reason}");
    }
}

fn validate_scale_content_registry_system(registry: Res<ScaleContentRegistry>) {
    if let Err(reason) = registry.validate() {
        panic!("USF scale content registry validation failed: {reason}");
    }
}

pub(crate) struct DefinitionPlugin;
impl Plugin for DefinitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefinitionRegistry>().init_resource::<ScaleContentRegistry>().add_systems(
            Startup,
            (validate_definition_registry_system, validate_scale_content_registry_system).in_set(AppSet::Diagnostics),
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn baseline_scale_content_registry() -> ScaleContentRegistry {
        let mut bindings_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            bindings_by_scale.insert(scale, baseline_scale_binding(scale));
        }

        ScaleContentRegistry {
            bindings_by_scale,
            known_dpt_samplers: default_dpt_samplers(),
            known_dpt_categorizers: default_dpt_categorizers(),
        }
    }

    #[test]
    fn scale_content_registry_baseline_is_valid() {
        let registry = baseline_scale_content_registry();
        assert!(registry.validate().is_ok());
    }

    #[test]
    fn scale_content_registry_rejects_missing_scale_binding() {
        let mut registry = baseline_scale_content_registry();
        registry.bindings_by_scale.remove(&Scale::MAX);
        let error = registry.validate().unwrap_err();
        assert!(error.contains("missing scale content binding"));
    }

    #[test]
    fn scale_content_registry_rejects_unknown_sampler_id() {
        let mut registry = baseline_scale_content_registry();
        let binding = registry.bindings_by_scale.get_mut(&Scale::MAX).unwrap();
        binding.dpt_sampler_id = "dpt_sampler.unknown".to_string();
        let error = registry.validate().unwrap_err();
        assert!(error.contains("unknown dpt_sampler_id"));
    }
}
