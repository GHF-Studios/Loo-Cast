use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::rhai_binding::engine::statics::{USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODELS_BY_ID, USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID};

use super::types::{MetricSurfaceDebugFieldDefinition, PhenomenonKind};

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct PhenomenonDefinitionRegistry {
    pub kind_by_phenomenon_id: HashMap<String, PhenomenonKind>,
    pub metric_surface_debug_by_model_id: HashMap<String, MetricSurfaceDebugFieldDefinition>,
    pub primary_model_by_phenomenon_id: HashMap<String, String>,
    pub phenomenon_by_model_id: HashMap<String, String>,
}

impl Default for PhenomenonDefinitionRegistry {
    fn default() -> Self {
        let script_phenomena = USF_PHENOMENA_BY_ID().lock().unwrap().clone();
        let script_models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().clone();
        let script_primary_models = USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID().lock().unwrap().clone();

        if script_phenomena.is_empty() {
            panic!("USF phenomenon bootstrap failed: no phenomena registered. Define at least one '*.phenomenon.rhai' file.");
        }
        if script_models.is_empty() {
            panic!("USF phenomenon bootstrap failed: no phenomenon models registered. Define at least one '*.phenomenon_model.rhai' file.");
        }
        if script_primary_models.is_empty() {
            panic!("USF phenomenon bootstrap failed: no primary model assignments registered. Assign at least one via set_primary_model(...).");
        }

        let mut kind_by_phenomenon_id = HashMap::new();
        for (phenomenon_id, phenomenon) in script_phenomena {
            let normalized_phenomenon_id = normalize_identifier(&phenomenon_id);
            let kind = PhenomenonKind::from_config_value(phenomenon.kind.as_str());
            kind_by_phenomenon_id.insert(normalized_phenomenon_id, kind);
        }

        let mut phenomenon_by_model_id = HashMap::new();
        let mut metric_surface_debug_by_model_id = HashMap::new();
        for (model_id, model) in script_models {
            let normalized_model_id = normalize_identifier(&model_id);
            let normalized_phenomenon_id = normalize_identifier(&model.phenomenon_id);
            let Some(kind) = kind_by_phenomenon_id.get(&normalized_phenomenon_id).copied() else {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' references unknown phenomenon '{}'.",
                    normalized_model_id, normalized_phenomenon_id
                );
            };
            if kind == PhenomenonKind::MetricSurfaceDebug {
                let Some(field) = model.metric_surface_debug else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to metric_surface_debug phenomenon '{}' \
                         but has no field definition. Call set_metric_surface_debug_model_field(...) in the model script.",
                        normalized_model_id, normalized_phenomenon_id
                    );
                };
                if !field.coarse_span_units.is_finite() || field.coarse_span_units <= 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid coarse_span_units={}.",
                        normalized_model_id, field.coarse_span_units
                    );
                }
                if !field.detail_span_units.is_finite() || field.detail_span_units <= 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid detail_span_units={}.",
                        normalized_model_id, field.detail_span_units
                    );
                }
                if !field.coarse_weight.is_finite()
                    || field.coarse_weight < 0.0
                    || !field.detail_weight.is_finite()
                    || field.detail_weight < 0.0
                    || field.coarse_weight + field.detail_weight <= 0.0
                {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid noise weights coarse={} detail={}.",
                        normalized_model_id, field.coarse_weight, field.detail_weight
                    );
                }
                if !field.bias.is_finite() || !field.gain.is_finite() || field.gain <= 0.0 || !field.center.is_finite() {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid shaping params bias={} gain={} center={}.",
                        normalized_model_id, field.bias, field.gain, field.center
                    );
                }
                metric_surface_debug_by_model_id.insert(
                    normalized_model_id.clone(),
                    MetricSurfaceDebugFieldDefinition {
                        coarse_span_units: field.coarse_span_units,
                        detail_span_units: field.detail_span_units,
                        coarse_weight: field.coarse_weight,
                        detail_weight: field.detail_weight,
                        bias: field.bias,
                        gain: field.gain,
                        center: field.center,
                        seed_salt_primary: field.seed_salt_primary,
                        seed_salt_detail: field.seed_salt_detail,
                    },
                );
            }
            phenomenon_by_model_id.insert(normalized_model_id, normalized_phenomenon_id);
        }

        let mut primary_model_by_phenomenon_id = HashMap::new();
        for (phenomenon_id, model_id) in script_primary_models {
            let normalized_phenomenon_id = normalize_identifier(&phenomenon_id);
            let normalized_model_id = normalize_identifier(&model_id);
            if !kind_by_phenomenon_id.contains_key(&normalized_phenomenon_id) {
                panic!(
                    "USF phenomenon bootstrap failed: primary model assignment references unknown phenomenon '{}'.",
                    normalized_phenomenon_id
                );
            }
            let Some(model_phenomenon_id) = phenomenon_by_model_id.get(&normalized_model_id) else {
                panic!(
                    "USF phenomenon bootstrap failed: primary model assignment references unknown model '{}'.",
                    normalized_model_id
                );
            };
            if model_phenomenon_id != &normalized_phenomenon_id {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' belongs to '{}', but is assigned as primary for '{}'.",
                    normalized_model_id, model_phenomenon_id, normalized_phenomenon_id
                );
            }
            primary_model_by_phenomenon_id.insert(normalized_phenomenon_id, normalized_model_id);
        }

        for phenomenon_id in kind_by_phenomenon_id.keys() {
            if !primary_model_by_phenomenon_id.contains_key(phenomenon_id) {
                panic!(
                    "USF phenomenon bootstrap failed: phenomenon '{}' has no primary model assignment.",
                    phenomenon_id
                );
            }
        }
        for (phenomenon_id, kind) in &kind_by_phenomenon_id {
            if *kind != PhenomenonKind::MetricSurfaceDebug {
                continue;
            }
            let Some(primary_model_id) = primary_model_by_phenomenon_id.get(phenomenon_id) else {
                panic!(
                    "USF phenomenon bootstrap failed: metric_surface_debug phenomenon '{}' has no primary model assignment.",
                    phenomenon_id
                );
            };
            if !metric_surface_debug_by_model_id.contains_key(primary_model_id) {
                panic!(
                    "USF phenomenon bootstrap failed: primary model '{}' for metric_surface_debug phenomenon '{}' has no model field definition.",
                    primary_model_id, phenomenon_id
                );
            }
        }

        Self {
            kind_by_phenomenon_id,
            metric_surface_debug_by_model_id,
            primary_model_by_phenomenon_id,
            phenomenon_by_model_id,
        }
    }
}

impl PhenomenonDefinitionRegistry {
    pub fn kind_for(&self, phenomenon_id: &str) -> Option<PhenomenonKind> {
        self.kind_by_phenomenon_id.get(&normalize_identifier(phenomenon_id)).copied()
    }

    pub fn metric_surface_debug_for(&self, phenomenon_id: &str) -> Option<MetricSurfaceDebugFieldDefinition> {
        let primary_model = self.primary_model_for(phenomenon_id)?;
        self.metric_surface_debug_for_model(primary_model)
    }

    pub fn metric_surface_debug_for_model(&self, model_id: &str) -> Option<MetricSurfaceDebugFieldDefinition> {
        self.metric_surface_debug_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn primary_model_for(&self, phenomenon_id: &str) -> Option<&str> {
        self.primary_model_by_phenomenon_id
            .get(&normalize_identifier(phenomenon_id))
            .map(|value| value.as_str())
    }

    pub fn model_belongs_to_phenomenon(&self, model_id: &str, phenomenon_id: &str) -> bool {
        let normalized_model_id = normalize_identifier(model_id);
        let normalized_phenomenon_id = normalize_identifier(phenomenon_id);
        self.phenomenon_by_model_id
            .get(&normalized_model_id)
            .is_some_and(|value| value == &normalized_phenomenon_id)
    }
}

#[inline]
fn normalize_identifier(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}
