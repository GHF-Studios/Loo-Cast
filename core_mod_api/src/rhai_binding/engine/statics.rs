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
pub struct ScriptScaleDefinition {
    pub dpt_sampler_id: String,
    pub dpt_categorizer_id: String,
    pub chunk_store_key: String,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptUsfModpackDefinition {
    pub mod_ids: Vec<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptUsfModDefinition {
    pub priority: i32,
    pub dependencies: HashSet<String>,
    pub load_after: HashSet<String>,
    pub conflicts_with: HashSet<String>,
    pub scale_conflict_policy: ScriptSingletonConflictPolicy,
    pub dpt_schema_conflict_policy: ScriptSingletonConflictPolicy,
    pub zlm_conflict_policy: ScriptSingletonConflictPolicy,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ScriptSingletonConflictPolicy {
    #[default]
    HardError,
    Replace,
    ReplaceIfHigherPriority,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptUsfModManifestDefinition {
    pub required_metrics: HashSet<String>,
    pub required_metric_sets: HashSet<String>,
    pub required_zone_types: HashSet<String>,
    pub required_phenomena: HashSet<String>,
    pub required_phenomenon_models: HashSet<String>,
    pub required_scales: HashSet<u8>,
    pub required_dpt_schema_scales: HashSet<u8>,
    pub required_zlm_scales: HashSet<u8>,
}

#[derive(Debug, Clone, Default)]
pub struct ScriptUsfModContribution {
    pub zone_types: HashSet<String>,
    pub dpt_schemas_by_scale: HashMap<u8, ScriptDptSchemaDefinition>,
    pub zlm_scales_by_scale: HashMap<u8, ScriptZlmScaleDefinition>,
    pub zone_density_profile_by_type: HashMap<String, ScriptZoneDensityProfileDefinition>,
    pub scales_by_index: HashMap<u8, ScriptScaleDefinition>,
    pub metrics_by_name: HashMap<String, ScriptMetricDefinition>,
    pub metric_sets_by_id: HashMap<String, Vec<String>>,
    pub phenomena_by_id: HashMap<String, ScriptPhenomenonDefinition>,
    pub zone_phenomenon_support_by_zone_type: HashMap<String, Vec<ScriptZonePhenomenonSupportDefinition>>,
    pub zone_selection_policy_by_zone_type: HashMap<String, ScriptZoneSelectionPolicyDefinition>,
    pub phenomenon_models_by_id: HashMap<String, ScriptPhenomenonModelDefinition>,
    pub phenomenon_model_selection_by_phenomenon_scale: HashMap<String, String>,
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
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScriptManifestationDensityDefinition {
    pub coarse_span_units: f64,
    pub detail_span_units: f64,
    pub coarse_weight: f32,
    pub detail_weight: f32,
    pub bias: f32,
    pub gain: f32,
    pub center: f32,
    pub seed_salt_coarse: u64,
    pub seed_salt_detail: u64,
}

impl Default for ScriptManifestationDensityDefinition {
    fn default() -> Self {
        Self {
            coarse_span_units: 320.0,
            detail_span_units: 128.0,
            coarse_weight: 0.82,
            detail_weight: 0.18,
            bias: 0.66,
            gain: 3.0,
            center: 0.5,
            seed_salt_coarse: 0xa5a5_35f4_9be3_c211_u64,
            seed_salt_detail: 0x8b8b_4fb7_0a7f_6611_u64,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ScriptManifestationMaterialDefinition {
    pub albedo_r: f32,
    pub albedo_g: f32,
    pub albedo_b: f32,
    pub alpha: f32,
    pub perceptual_roughness: f32,
    pub metallic: f32,
    pub emissive_strength: f32,
}

impl Default for ScriptManifestationMaterialDefinition {
    fn default() -> Self {
        Self {
            albedo_r: 0.54,
            albedo_g: 0.68,
            albedo_b: 0.93,
            alpha: 1.0,
            perceptual_roughness: 0.9,
            metallic: 0.0,
            emissive_strength: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ScriptPhenomenonModelDefinition {
    pub id: String,
    pub phenomenon_id: String,
    pub topology: String,
    pub support_chunk_radius: u16,
    pub projection_metric_name: String,
    pub projection_bias: f32,
    pub projection_gain: f32,
    pub manifestation_density: Option<ScriptManifestationDensityDefinition>,
    pub manifestation_material: Option<ScriptManifestationMaterialDefinition>,
    pub manifestation_collider_enabled: bool,
}
impl Default for ScriptPhenomenonModelDefinition {
    fn default() -> Self {
        Self {
            id: String::new(),
            phenomenon_id: String::new(),
            topology: "monolithic_chunk".to_string(),
            support_chunk_radius: 0,
            projection_metric_name: "demo_mass_density".to_string(),
            projection_bias: 0.0,
            projection_gain: 1.0,
            manifestation_density: None,
            manifestation_material: None,
            manifestation_collider_enabled: false,
        }
    }
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
export_static!(
    self,
    crate::rhai_binding::engine::statics::USF_MODS_BY_ID: Lazy<Mutex<HashMap<String, ScriptUsfModDefinition>>> =
        Lazy::new(Default::default)
);
export_static!(
    self,
    crate::rhai_binding::engine::statics::USF_MOD_MANIFESTS_BY_ID:
        Lazy<Mutex<HashMap<String, ScriptUsfModManifestDefinition>>> = Lazy::new(Default::default)
);
export_static!(
    self,
    crate::rhai_binding::engine::statics::USF_MODPACKS_BY_ID: Lazy<Mutex<HashMap<String, ScriptUsfModpackDefinition>>> =
        Lazy::new(Default::default)
);
export_static!(
    self,
    crate::rhai_binding::engine::statics::USF_MOD_CONTRIBUTIONS_BY_ID: Lazy<Mutex<HashMap<String, ScriptUsfModContribution>>> =
        Lazy::new(Default::default)
);
export_static!(self, crate::rhai_binding::engine::statics::USF_SCALES_BY_INDEX: Lazy<Mutex<HashMap<u8, ScriptScaleDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_METRICS_BY_NAME: Lazy<Mutex<HashMap<String, ScriptMetricDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_METRIC_SETS_BY_ID: Lazy<Mutex<HashMap<String, Vec<String>>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_PHENOMENA_BY_ID: Lazy<Mutex<HashMap<String, ScriptPhenomenonDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_PHENOMENON_SUPPORT_BY_ZONE_TYPE: Lazy<Mutex<HashMap<String, Vec<ScriptZonePhenomenonSupportDefinition>>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_ZONE_SELECTION_POLICY_BY_ZONE_TYPE: Lazy<Mutex<HashMap<String, ScriptZoneSelectionPolicyDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_PHENOMENON_MODELS_BY_ID: Lazy<Mutex<HashMap<String, ScriptPhenomenonModelDefinition>>> = Lazy::new(Default::default));
export_static!(self, crate::rhai_binding::engine::statics::USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE: Lazy<Mutex<HashMap<String, String>>> = Lazy::new(Default::default));
