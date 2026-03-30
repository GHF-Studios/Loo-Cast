use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::rhai_binding::engine::statics::{USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE, USF_PHENOMENON_MODELS_BY_ID};
use crate::usf::scale::Scale;

use super::types::{MetricSurfaceDebugFieldDefinition, PhenomenonKind};

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct PhenomenonDefinitionRegistry {
    pub kind_by_phenomenon_id: HashMap<String, PhenomenonKind>,
    pub metric_surface_debug_by_model_id: HashMap<String, MetricSurfaceDebugFieldDefinition>,
    pub model_selection_by_phenomenon_scale: HashMap<String, String>,
    pub phenomenon_by_model_id: HashMap<String, String>,
}

impl Default for PhenomenonDefinitionRegistry {
    fn default() -> Self {
        let script_phenomena = USF_PHENOMENA_BY_ID().lock().unwrap().clone();
        let script_models = USF_PHENOMENON_MODELS_BY_ID().lock().unwrap().clone();
        let script_model_selection = USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE().lock().unwrap().clone();

        if script_phenomena.is_empty() {
            panic!("USF phenomenon bootstrap failed: no phenomena registered. Define at least one '*.phenomenon.rhai' file.");
        }
        if script_models.is_empty() {
            panic!("USF phenomenon bootstrap failed: no phenomenon models registered. Define at least one '*.phenomenon_model.rhai' file.");
        }
        if script_model_selection.is_empty() {
            panic!(
                "USF phenomenon bootstrap failed: no model selection assignments registered. \
                 Assign model selection per scale via set_model_for_* APIs."
            );
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

        let mut model_selection_by_phenomenon_scale = HashMap::new();
        for (selection_key_raw, model_id) in script_model_selection {
            let (phenomenon_id, scale_index) = parse_selection_key(selection_key_raw.as_str());
            let normalized_phenomenon_id = normalize_identifier(phenomenon_id);
            let normalized_model_id = normalize_identifier(model_id.as_str());
            if !kind_by_phenomenon_id.contains_key(&normalized_phenomenon_id) {
                panic!(
                    "USF phenomenon bootstrap failed: model selection references unknown phenomenon '{}'.",
                    normalized_phenomenon_id
                );
            }
            if scale_index >= Scale::SCALE_LEVEL_COUNT as u8 {
                panic!(
                    "USF phenomenon bootstrap failed: model selection references invalid scale {} for '{}'.",
                    scale_index, normalized_phenomenon_id
                );
            }
            let Some(model_phenomenon_id) = phenomenon_by_model_id.get(&normalized_model_id) else {
                panic!(
                    "USF phenomenon bootstrap failed: model selection references unknown model '{}'.",
                    normalized_model_id
                );
            };
            if model_phenomenon_id != &normalized_phenomenon_id {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' belongs to '{}', but is assigned to '{}@{}'.",
                    normalized_model_id, model_phenomenon_id, normalized_phenomenon_id, scale_index
                );
            }
            let selection_key = selection_key(normalized_phenomenon_id.as_str(), scale_index);
            model_selection_by_phenomenon_scale.insert(selection_key, normalized_model_id);
        }

        for phenomenon_id in kind_by_phenomenon_id.keys() {
            for scale_index in 0..(Scale::SCALE_LEVEL_COUNT as u8) {
                let key = selection_key(phenomenon_id.as_str(), scale_index);
                if !model_selection_by_phenomenon_scale.contains_key(&key) {
                    panic!(
                        "USF phenomenon bootstrap failed: phenomenon '{}' has no model assignment for scale {}.",
                        phenomenon_id, scale_index
                    );
                }
            }
        }
        for (phenomenon_id, kind) in &kind_by_phenomenon_id {
            if *kind != PhenomenonKind::MetricSurfaceDebug {
                continue;
            }
            for scale_index in 0..(Scale::SCALE_LEVEL_COUNT as u8) {
                let key = selection_key(phenomenon_id.as_str(), scale_index);
                let Some(model_id) = model_selection_by_phenomenon_scale.get(&key) else {
                    panic!(
                        "USF phenomenon bootstrap failed: metric_surface_debug phenomenon '{}' has no model assignment for scale {}.",
                        phenomenon_id, scale_index
                    );
                };
                if !metric_surface_debug_by_model_id.contains_key(model_id) {
                    panic!(
                        "USF phenomenon bootstrap failed: selected model '{}' for metric_surface_debug phenomenon '{}' at scale {} has no model field definition.",
                        model_id, phenomenon_id, scale_index
                    );
                }
            }
        }

        Self {
            kind_by_phenomenon_id,
            metric_surface_debug_by_model_id,
            model_selection_by_phenomenon_scale,
            phenomenon_by_model_id,
        }
    }
}

impl PhenomenonDefinitionRegistry {
    pub fn kind_for(&self, phenomenon_id: &str) -> Option<PhenomenonKind> {
        self.kind_by_phenomenon_id.get(&normalize_identifier(phenomenon_id)).copied()
    }

    pub fn metric_surface_debug_for(&self, phenomenon_id: &str) -> Option<MetricSurfaceDebugFieldDefinition> {
        self.metric_surface_debug_for_scale(phenomenon_id, Scale::MAX)
    }

    pub fn metric_surface_debug_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<MetricSurfaceDebugFieldDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.metric_surface_debug_for_model(model_id)
    }

    pub fn metric_surface_debug_for_model(&self, model_id: &str) -> Option<MetricSurfaceDebugFieldDefinition> {
        self.metric_surface_debug_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn primary_model_for(&self, phenomenon_id: &str) -> Option<&str> {
        self.model_for_scale(phenomenon_id, Scale::MAX)
    }

    pub fn model_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<&str> {
        let selection_key = selection_key(normalize_identifier(phenomenon_id).as_str(), scale.index_from_top());
        self.model_selection_by_phenomenon_scale.get(selection_key.as_str()).map(|value| value.as_str())
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

#[inline]
fn selection_key(phenomenon_id: &str, scale_index: u8) -> String {
    format!("{}@{}", normalize_identifier(phenomenon_id), scale_index)
}

fn parse_selection_key(selection_key: &str) -> (&str, u8) {
    let Some((phenomenon_id, scale_index)) = selection_key.split_once('@') else {
        panic!(
            "USF phenomenon bootstrap failed: invalid model selection key '{}'; expected '<phenomenon_id>@<scale_index>'.",
            selection_key
        );
    };
    let scale_index = scale_index.parse::<u8>().unwrap_or_else(|_| {
        panic!(
            "USF phenomenon bootstrap failed: invalid model selection scale in key '{}'; expected u8 scale index.",
            selection_key
        )
    });
    (phenomenon_id, scale_index)
}
