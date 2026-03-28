use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::rhai_binding::engine::statics::{USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODELS_BY_ID, USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID};

use super::types::PhenomenonKind;

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct PhenomenonDefinitionRegistry {
    pub kind_by_phenomenon_id: HashMap<String, PhenomenonKind>,
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
            kind_by_phenomenon_id.insert(normalized_phenomenon_id, PhenomenonKind::from_config_value(phenomenon.kind.as_str()));
        }

        let mut phenomenon_by_model_id = HashMap::new();
        for (model_id, model) in script_models {
            let normalized_model_id = normalize_identifier(&model_id);
            let normalized_phenomenon_id = normalize_identifier(&model.phenomenon_id);
            if !kind_by_phenomenon_id.contains_key(&normalized_phenomenon_id) {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' references unknown phenomenon '{}'.",
                    normalized_model_id, normalized_phenomenon_id
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

        Self {
            kind_by_phenomenon_id,
            primary_model_by_phenomenon_id,
            phenomenon_by_model_id,
        }
    }
}

impl PhenomenonDefinitionRegistry {
    pub fn kind_for(&self, phenomenon_id: &str) -> Option<PhenomenonKind> {
        self.kind_by_phenomenon_id.get(&normalize_identifier(phenomenon_id)).copied()
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
