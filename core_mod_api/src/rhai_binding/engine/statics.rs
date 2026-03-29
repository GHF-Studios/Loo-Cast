use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use core_mod_macros::export_static;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScriptDptMetricDefinition {
    pub id: u16,
    pub name: String,
    pub value_type: String,
    pub semantics_tag: String,
    pub storage_class: String,
    pub derived: bool,
    pub min_scale_index: u8,
    pub max_scale_index: u8,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptDptSchemaDefinition {
    pub revision: u64,
    pub fallback_zone: String,
    pub metrics: Vec<ScriptDptMetricDefinition>,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptZlmMetricBandDefinition {
    pub metric_id: u16,
    pub min: f32,
    pub max: f32,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptZlmRuleDefinition {
    pub zone_type: String,
    pub metric_bands: Vec<ScriptZlmMetricBandDefinition>,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptZlmScaleDefinition {
    pub revision: u64,
    pub fallback_zone: String,
    pub rules: Vec<ScriptZlmRuleDefinition>,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptZoneDensityProfileDefinition {
    pub density_multiplier: f32,
    pub density_offset: f32,
    pub density_floor: f32,
    pub density_ceil: f32,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptScaleBindingDefinition {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ScriptMetricDefinition {
    pub id: u16,
    pub name: String,
    pub value_type: String,
    pub semantics_tag: String,
    pub storage_class: String,
    pub derived: bool,
    pub min_scale_index: u8,
    pub max_scale_index: u8,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptPhenomenonDefinition {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptPhenomenonModelDefinition {
    pub id: String,
    pub phenomenon_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptZonePhenomenonSupportDefinition {
    pub phenomenon_id: String,
    pub priority: i32,
    pub weight: f32,
    pub spawn_policy: String,
    pub max_active: u32,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptZoneSelectionPolicyDefinition {
    pub strategy: String,
}

export_static!(self, crate::rhai_binding::engine::statics::SCHEDULE_HOOKS: Lazy<Mutex<Vec<String>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_TYPES: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_DPT_SCHEMAS_BY_SCALE: Lazy<Mutex<HashMap<u8, ScriptDptSchemaDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZLM_SCALES_BY_SCALE: Lazy<Mutex<HashMap<u8, ScriptZlmScaleDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_DENSITY_PROFILE_BY_TYPE: Lazy<Mutex<HashMap<String, ScriptZoneDensityProfileDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_DPT_SAMPLER_IDS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_DPT_CATEGORIZER_IDS: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_SCALE_BINDINGS_BY_SCALE: Lazy<Mutex<HashMap<u8, ScriptScaleBindingDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_METRICS_BY_NAME: Lazy<Mutex<HashMap<String, ScriptMetricDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_METRIC_SETS_BY_ID: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_PHENOMENA_BY_ID: Lazy<Mutex<HashMap<String, ScriptPhenomenonDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE: Lazy<Mutex<HashMap<String, Vec<ScriptZonePhenomenonSupportDefinition>>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE: Lazy<Mutex<HashMap<String, ScriptZoneSelectionPolicyDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_PHENOMENON_MODELS_BY_ID: Lazy<Mutex<HashMap<String, ScriptPhenomenonModelDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_PRIMARY_PHENOMENON_MODEL_BY_PHENOMENON_ID: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(Default::default));
