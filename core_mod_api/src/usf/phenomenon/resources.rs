use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::rhai_binding::engine::statics::{USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE, USF_PHENOMENON_MODELS_BY_ID};
use crate::usf::scale::Scale;

use super::components::PhenomenaModelTopology;
use super::types::{
    ManifestationDensityFieldDefinition, ManifestationMaterialProfileDefinition, PhenomenonCapability, PhenomenonKind, PhenomenonManifestationFieldContract,
};

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct PhenomenonDefinitionRegistry {
    pub kind_by_phenomenon_id: HashMap<String, PhenomenonKind>,
    pub capabilities_by_phenomenon_id: HashMap<String, Vec<PhenomenonCapability>>,
    pub manifestation_density_by_model_id: HashMap<String, ManifestationDensityFieldDefinition>,
    pub manifestation_material_by_model_id: HashMap<String, ManifestationMaterialProfileDefinition>,
    pub topology_by_model_id: HashMap<String, PhenomenaModelTopology>,
    pub support_chunk_radius_by_model_id: HashMap<String, u16>,
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
        let mut capabilities_by_phenomenon_id = HashMap::new();
        for (phenomenon_id, phenomenon) in script_phenomena {
            let normalized_phenomenon_id = normalize_identifier(&phenomenon_id);
            let kind = PhenomenonKind::from_config_value(phenomenon.kind.as_str());
            let capabilities = parse_capability_list(phenomenon.capabilities.as_slice(), kind);
            kind_by_phenomenon_id.insert(normalized_phenomenon_id.clone(), kind);
            capabilities_by_phenomenon_id.insert(normalized_phenomenon_id, capabilities);
        }

        let mut phenomenon_by_model_id = HashMap::new();
        let mut manifestation_density_by_model_id = HashMap::new();
        let mut manifestation_material_by_model_id = HashMap::new();
        let mut topology_by_model_id = HashMap::new();
        let mut support_chunk_radius_by_model_id = HashMap::new();
        for (model_id, model) in script_models {
            let normalized_model_id = normalize_identifier(&model_id);
            let normalized_phenomenon_id = normalize_identifier(&model.phenomenon_id);
            let Some(kind) = kind_by_phenomenon_id.get(&normalized_phenomenon_id).copied() else {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' references unknown phenomenon '{}'.",
                    normalized_model_id, normalized_phenomenon_id
                );
            };
            let has_manifestation_density_capability = capabilities_by_phenomenon_id
                .get(&normalized_phenomenon_id)
                .is_some_and(|capabilities| capabilities.contains(&PhenomenonCapability::ManifestationDensityField));
            if has_manifestation_density_capability {
                let Some(field) = model.manifestation_density else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         requiring capability 'manifestation_density_field' but has no field definition. \
                         Call set_manifestation_density_field(...) in the model script.",
                        normalized_model_id,
                        normalized_phenomenon_id,
                        kind.canonical_id()
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
                manifestation_density_by_model_id.insert(
                    normalized_model_id.clone(),
                    ManifestationDensityFieldDefinition {
                        coarse_span_units: field.coarse_span_units,
                        detail_span_units: field.detail_span_units,
                        coarse_weight: field.coarse_weight,
                        detail_weight: field.detail_weight,
                        bias: field.bias,
                        gain: field.gain,
                        center: field.center,
                        seed_salt_coarse: field.seed_salt_coarse,
                        seed_salt_detail: field.seed_salt_detail,
                    },
                );
            }
            let has_manifestation_material_capability = capabilities_by_phenomenon_id
                .get(&normalized_phenomenon_id)
                .is_some_and(|capabilities| capabilities.contains(&PhenomenonCapability::ManifestationMaterialProfile));
            if has_manifestation_material_capability {
                let Some(material) = model.manifestation_material else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         requiring capability 'manifestation_material_profile' but has no material profile definition. \
                         Call set_manifestation_material_profile(...) in the model script.",
                        normalized_model_id,
                        normalized_phenomenon_id,
                        kind.canonical_id()
                    );
                };
                if !material.albedo_r.is_finite() || !material.albedo_g.is_finite() || !material.albedo_b.is_finite() {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid albedo components ({}, {}, {}).",
                        normalized_model_id, material.albedo_r, material.albedo_g, material.albedo_b
                    );
                }
                if !material.alpha.is_finite() || !(0.0..=1.0).contains(&material.alpha) {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid alpha={}; expected [0..1].",
                        normalized_model_id, material.alpha
                    );
                }
                if !material.perceptual_roughness.is_finite() || !(0.0..=1.0).contains(&material.perceptual_roughness) {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid perceptual_roughness={}; expected [0..1].",
                        normalized_model_id, material.perceptual_roughness
                    );
                }
                if !material.metallic.is_finite() || !(0.0..=1.0).contains(&material.metallic) {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid metallic={}; expected [0..1].",
                        normalized_model_id, material.metallic
                    );
                }
                if !material.emissive_strength.is_finite() || material.emissive_strength < 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid emissive_strength={}; expected >= 0.",
                        normalized_model_id, material.emissive_strength
                    );
                }
                manifestation_material_by_model_id.insert(
                    normalized_model_id.clone(),
                    ManifestationMaterialProfileDefinition {
                        albedo_r: material.albedo_r,
                        albedo_g: material.albedo_g,
                        albedo_b: material.albedo_b,
                        alpha: material.alpha,
                        perceptual_roughness: material.perceptual_roughness,
                        metallic: material.metallic,
                        emissive_strength: material.emissive_strength,
                    },
                );
            }
            let topology = parse_topology_tag(model.topology.as_str());
            let support_chunk_radius = match topology {
                PhenomenaModelTopology::MonolithicChunk => {
                    if model.support_chunk_radius != 0 {
                        panic!(
                            "USF phenomenon bootstrap failed: model '{}' is monolithic_chunk but declares support_chunk_radius={}; expected 0.",
                            normalized_model_id, model.support_chunk_radius
                        );
                    }
                    0
                }
                PhenomenaModelTopology::PartitionedByChunk => {
                    if model.support_chunk_radius == 0 {
                        panic!(
                            "USF phenomenon bootstrap failed: model '{}' is partitioned_by_chunk but declares support_chunk_radius=0; expected >= 1.",
                            normalized_model_id
                        );
                    }
                    model.support_chunk_radius
                }
            };
            topology_by_model_id.insert(normalized_model_id.clone(), topology);
            support_chunk_radius_by_model_id.insert(normalized_model_id.clone(), support_chunk_radius);
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
            let has_manifestation_density_capability = capabilities_by_phenomenon_id
                .get(phenomenon_id)
                .is_some_and(|capabilities| capabilities.contains(&PhenomenonCapability::ManifestationDensityField));
            for scale_index in 0..(Scale::SCALE_LEVEL_COUNT as u8) {
                let key = selection_key(phenomenon_id.as_str(), scale_index);
                let Some(model_id) = model_selection_by_phenomenon_scale.get(&key) else {
                    if has_manifestation_density_capability {
                        panic!(
                            "USF phenomenon bootstrap failed: phenomenon '{}' (kind='{}') requiring capability 'manifestation_density_field' \
                             has no model assignment for scale {}.",
                            phenomenon_id,
                            kind.canonical_id(),
                            scale_index
                        );
                    }
                    continue;
                };
                if has_manifestation_density_capability && !manifestation_density_by_model_id.contains_key(model_id) {
                    panic!(
                        "USF phenomenon bootstrap failed: selected model '{}' for phenomenon '{}' (kind='{}') at scale {} \
                         has no manifestation density field definition.",
                        model_id,
                        phenomenon_id,
                        kind.canonical_id(),
                        scale_index
                    );
                }
                let has_manifestation_material_capability = capabilities_by_phenomenon_id
                    .get(phenomenon_id)
                    .is_some_and(|capabilities| capabilities.contains(&PhenomenonCapability::ManifestationMaterialProfile));
                if has_manifestation_material_capability && !manifestation_material_by_model_id.contains_key(model_id) {
                    panic!(
                        "USF phenomenon bootstrap failed: selected model '{}' for phenomenon '{}' (kind='{}') at scale {} \
                         has no manifestation material profile definition.",
                        model_id,
                        phenomenon_id,
                        kind.canonical_id(),
                        scale_index
                    );
                }
            }
        }

        Self {
            kind_by_phenomenon_id,
            capabilities_by_phenomenon_id,
            manifestation_density_by_model_id,
            manifestation_material_by_model_id,
            topology_by_model_id,
            support_chunk_radius_by_model_id,
            model_selection_by_phenomenon_scale,
            phenomenon_by_model_id,
        }
    }
}

impl PhenomenonDefinitionRegistry {
    pub fn kind_for(&self, phenomenon_id: &str) -> Option<PhenomenonKind> {
        self.kind_by_phenomenon_id.get(&normalize_identifier(phenomenon_id)).copied()
    }

    pub fn capabilities_for(&self, phenomenon_id: &str) -> Option<&[PhenomenonCapability]> {
        self.capabilities_by_phenomenon_id.get(&normalize_identifier(phenomenon_id)).map(Vec::as_slice)
    }

    pub fn supports_capability_for_phenomenon(&self, phenomenon_id: &str, capability: PhenomenonCapability) -> bool {
        self.capabilities_for(phenomenon_id)
            .is_some_and(|capabilities| capabilities.contains(&capability))
    }

    pub fn manifestation_density_for(&self, phenomenon_id: &str) -> Option<ManifestationDensityFieldDefinition> {
        self.manifestation_density_for_scale(phenomenon_id, Scale::MAX)
    }

    pub fn manifestation_density_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<ManifestationDensityFieldDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.manifestation_density_for_model(model_id)
    }

    pub fn manifestation_density_for_model(&self, model_id: &str) -> Option<ManifestationDensityFieldDefinition> {
        self.manifestation_density_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn manifestation_material_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<ManifestationMaterialProfileDefinition> {
        if !self.supports_capability_for_phenomenon(phenomenon_id, PhenomenonCapability::ManifestationMaterialProfile) {
            return None;
        }
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.manifestation_material_for_model(model_id)
    }

    pub fn manifestation_material_for_model(&self, model_id: &str) -> Option<ManifestationMaterialProfileDefinition> {
        self.manifestation_material_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn manifestation_field_contract_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<PhenomenonManifestationFieldContract> {
        if !self.supports_capability_for_phenomenon(phenomenon_id, PhenomenonCapability::ManifestationDensityField) {
            return None;
        }
        self.manifestation_density_for_scale(phenomenon_id, scale)
            .map(PhenomenonManifestationFieldContract::DensityField)
    }

    pub fn model_selector_single(&self, phenomenon_id: &str, scale: Scale) -> Option<&str> {
        self.model_for_scale(phenomenon_id, scale)
    }

    pub fn model_selector_all(&self, phenomenon_id: &str) -> Vec<(Scale, &str)> {
        let mut selected = Vec::with_capacity(Scale::SCALE_LEVEL_COUNT as usize);
        for scale_index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                continue;
            };
            let Some(model_id) = self.model_for_scale(phenomenon_id, scale) else {
                continue;
            };
            selected.push((scale, model_id));
        }
        selected
    }

    pub fn model_selector_range(&self, phenomenon_id: &str, min_scale: Scale, max_scale: Scale) -> Vec<(Scale, &str)> {
        let mut lower = min_scale.index_from_top();
        let mut upper = max_scale.index_from_top();
        if lower > upper {
            std::mem::swap(&mut lower, &mut upper);
        }

        let mut selected = Vec::new();
        for scale_index in lower..=upper {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                continue;
            };
            let Some(model_id) = self.model_for_scale(phenomenon_id, scale) else {
                continue;
            };
            selected.push((scale, model_id));
        }
        selected
    }

    pub fn model_selector_set(&self, phenomenon_id: &str, scales: &[Scale]) -> Vec<(Scale, &str)> {
        let mut selected = Vec::new();
        for scale in scales {
            let Some(model_id) = self.model_for_scale(phenomenon_id, *scale) else {
                continue;
            };
            selected.push((*scale, model_id));
        }
        selected
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

    pub fn topology_for_model(&self, model_id: &str) -> Option<PhenomenaModelTopology> {
        self.topology_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn support_chunk_radius_for_model(&self, model_id: &str) -> Option<u16> {
        self.support_chunk_radius_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn topology_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<PhenomenaModelTopology> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.topology_for_model(model_id)
    }

    pub fn support_chunk_radius_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<u16> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.support_chunk_radius_for_model(model_id)
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

#[inline]
fn parse_topology_tag(raw: &str) -> PhenomenaModelTopology {
    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "monolithic_chunk" | "monolithic-chunk" | "monolithic" => PhenomenaModelTopology::MonolithicChunk,
        "partitioned_by_chunk" | "partitioned-by-chunk" | "partitioned" => PhenomenaModelTopology::PartitionedByChunk,
        _ => panic!(
            "USF phenomenon bootstrap failed: unsupported model topology '{}'; expected monolithic_chunk or partitioned_by_chunk.",
            normalized
        ),
    }
}

fn parse_capability_list(raw_capabilities: &[String], kind: PhenomenonKind) -> Vec<PhenomenonCapability> {
    let source = if raw_capabilities.is_empty() {
        kind.declared_capabilities()
            .iter()
            .map(|capability| capability.canonical_id().to_string())
            .collect::<Vec<_>>()
    } else {
        raw_capabilities.to_vec()
    };

    let mut parsed = Vec::<PhenomenonCapability>::new();
    for raw in source {
        let capability = PhenomenonCapability::try_from_config_value(raw.as_str()).unwrap_or_else(|error| {
            panic!(
                "USF phenomenon bootstrap failed: unknown capability '{}' for kind '{}': {}",
                raw,
                kind.canonical_id(),
                error
            )
        });
        if !parsed.contains(&capability) {
            parsed.push(capability);
        }
    }
    parsed
}
