use std::collections::{HashMap, HashSet};

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::{USF_DPT_SCHEMAS_BY_SCALE, USF_ZONE_TYPES};
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
pub struct DptMetricDefinition {
    pub id: DptMetricId,
    pub name: String,
    pub primitive: bool,
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct DptSchema {
    pub revision: u64,
    pub metrics: Vec<DptMetricDefinition>,
    pub fallback_zone: ZoneTypeId,
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
        }

        Ok(())
    }
}

#[derive(Resource, Debug, Clone)]
pub struct DefinitionRegistry {
    pub schemas_by_scale: HashMap<Scale, DptSchema>,
    pub known_zone_types: HashSet<ZoneTypeId>,
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
    DptSchema {
        revision: 1,
        metrics: vec![
            DptMetricDefinition {
                id: DptMetricId(0),
                name: "temperature".to_string(),
                primitive: true,
            },
            DptMetricDefinition {
                id: DptMetricId(1),
                name: "humidity".to_string(),
                primitive: true,
            },
            DptMetricDefinition {
                id: DptMetricId(2),
                name: "elevation".to_string(),
                primitive: true,
            },
            DptMetricDefinition {
                id: DptMetricId(3),
                name: "vegetation_density".to_string(),
                primitive: false,
            },
        ],
        fallback_zone: normalize_zone_type("void"),
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
                .map(|metric| DptMetricDefinition {
                    id: DptMetricId(metric.id),
                    name: metric.name.trim().to_string(),
                    primitive: metric.primitive,
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

fn validate_definition_registry_system(registry: Res<DefinitionRegistry>) {
    if let Err(reason) = registry.validate() {
        panic!("USF definition registry validation failed: {reason}");
    }
}

pub(crate) struct DefinitionPlugin;
impl Plugin for DefinitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<DefinitionRegistry>()
            .add_systems(Startup, validate_definition_registry_system.in_set(AppSet::Diagnostics));
    }
}
