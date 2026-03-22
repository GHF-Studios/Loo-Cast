use std::collections::{HashMap, HashSet};
use std::sync::Mutex;

use core_mod_macros::export_static;
use once_cell::sync::Lazy;

#[derive(Debug, Clone, Default)]
pub struct ScriptDptMetricDefinition {
    pub id: u16,
    pub name: String,
    pub primitive: bool,
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

export_static!(self, crate::rhai_binding::engine::statics::SCHEDULE_HOOKS: Lazy<Mutex<Vec<String>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_KIND_BY_TYPE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_TYPES: Lazy<Mutex<HashSet<String>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_DPT_SCHEMAS_BY_SCALE: Lazy<Mutex<HashMap<u8, ScriptDptSchemaDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZLM_SCALES_BY_SCALE: Lazy<Mutex<HashMap<u8, ScriptZlmScaleDefinition>>> = Lazy::new(Default::default));
