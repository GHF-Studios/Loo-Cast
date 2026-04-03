use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::rhai_binding::engine::statics::{USF_PHENOMENA_BY_ID, USF_PHENOMENON_MODEL_SELECTION_BY_PHENOMENON_SCALE, USF_PHENOMENON_MODELS_BY_ID};
use crate::usf::scale::Scale;

use super::components::{PhenomenonModelProjectionSpec, PhenomenonModelTopology};
use super::types::{
    InteractionTriggerDefinition, RealizationAudioEmitterDefinition, RealizationDensityFieldDefinition, RealizationMaterialProfileDefinition,
    RealizationParticleEmitterDefinition, PhenomenonKind, PhenomenonRealizationFieldContract, PhenomenonSimulationServiceDefinition,
};

#[derive(Resource, Reflect, Debug, Clone)]
#[reflect(Resource)]
pub struct PhenomenonDefinitionRegistry {
    pub kind_by_phenomenon_id: HashMap<String, PhenomenonKind>,
    pub realization_density_by_model_id: HashMap<String, RealizationDensityFieldDefinition>,
    pub realization_material_by_model_id: HashMap<String, RealizationMaterialProfileDefinition>,
    pub realization_collider_enabled_by_model_id: HashMap<String, bool>,
    pub simulation_service_by_model_id: HashMap<String, PhenomenonSimulationServiceDefinition>,
    pub realization_audio_emitter_by_model_id: HashMap<String, RealizationAudioEmitterDefinition>,
    pub realization_particle_emitter_by_model_id: HashMap<String, RealizationParticleEmitterDefinition>,
    pub interaction_trigger_by_model_id: HashMap<String, InteractionTriggerDefinition>,
    pub projection_contract_by_model_id: HashMap<String, PhenomenonModelProjectionSpec>,
    pub topology_by_model_id: HashMap<String, PhenomenonModelTopology>,
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
        for (phenomenon_id, phenomenon) in script_phenomena {
            let normalized_phenomenon_id = normalize_identifier(&phenomenon_id);
            let kind = PhenomenonKind::from_config_value(phenomenon.kind.as_str());
            kind_by_phenomenon_id.insert(normalized_phenomenon_id.clone(), kind);
        }

        let mut phenomenon_by_model_id = HashMap::new();
        let mut realization_density_by_model_id = HashMap::new();
        let mut realization_material_by_model_id = HashMap::new();
        let mut realization_collider_enabled_by_model_id = HashMap::new();
        let mut simulation_service_by_model_id = HashMap::new();
        let mut realization_audio_emitter_by_model_id = HashMap::new();
        let mut realization_particle_emitter_by_model_id = HashMap::new();
        let mut interaction_trigger_by_model_id = HashMap::new();
        let mut projection_contract_by_model_id = HashMap::new();
        let mut topology_by_model_id = HashMap::new();
        let mut support_chunk_radius_by_model_id = HashMap::new();
        for (model_id, model) in script_models {
            let normalized_model_id = normalize_identifier(&model_id);
            let normalized_phenomenon_id = normalize_identifier(&model.phenomenon_id);
            let Some(kind) = kind_by_phenomenon_id.get(&normalized_phenomenon_id).cloned() else {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' references unknown phenomenon '{}'.",
                    normalized_model_id, normalized_phenomenon_id
                );
            };
            let has_realization_density_contract = model.realization_density.is_some();
            if has_realization_density_contract {
                let Some(field) = model.realization_density else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         declares contract 'realization_density_field' but has no field definition. \
                         Call set_realization_density_field(...) in the model script.",
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
                realization_density_by_model_id.insert(
                    normalized_model_id.clone(),
                    RealizationDensityFieldDefinition {
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
            let has_realization_material_contract = model.realization_material.is_some();
            if has_realization_material_contract {
                let Some(material) = model.realization_material else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         declares contract 'realization_material_profile' but has no material profile definition. \
                         Call set_realization_material_profile(...) in the model script.",
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
                realization_material_by_model_id.insert(
                    normalized_model_id.clone(),
                    RealizationMaterialProfileDefinition {
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
            realization_collider_enabled_by_model_id.insert(normalized_model_id.clone(), model.realization_collider_enabled);
            let has_simulation_service_contract = model.simulation_service.is_some();
            if has_simulation_service_contract {
                let Some(simulation_service) = model.simulation_service else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         declares contract 'simulation_service' but has no simulation service definition. \
                         Call set_simulation_service(...) in the model script.",
                        normalized_model_id,
                        normalized_phenomenon_id,
                        kind.canonical_id()
                    );
                };
                if !simulation_service.target_hz.is_finite() || simulation_service.target_hz <= 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid simulation target_hz={}; expected finite > 0.",
                        normalized_model_id, simulation_service.target_hz
                    );
                }
                if !simulation_service.stability_bias.is_finite() {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid simulation stability_bias={}; expected finite.",
                        normalized_model_id, simulation_service.stability_bias
                    );
                }
                if !simulation_service.response_gain.is_finite() || simulation_service.response_gain <= 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid simulation response_gain={}; expected finite > 0.",
                        normalized_model_id, simulation_service.response_gain
                    );
                }
                simulation_service_by_model_id.insert(
                    normalized_model_id.clone(),
                    PhenomenonSimulationServiceDefinition {
                        target_hz: simulation_service.target_hz,
                        stability_bias: simulation_service.stability_bias,
                        response_gain: simulation_service.response_gain,
                    },
                );
            }
            let has_realization_audio_contract = model.realization_audio_emitter.is_some();
            if has_realization_audio_contract {
                let Some(audio_emitter) = model.realization_audio_emitter.clone() else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         declares contract 'realization_audio_emitter' but has no audio emitter definition. \
                         Call set_realization_audio_emitter(...) in the model script.",
                        normalized_model_id,
                        normalized_phenomenon_id,
                        kind.canonical_id()
                    );
                };
                let event_id = normalize_identifier(audio_emitter.event_id.as_str());
                if event_id.is_empty() {
                    panic!("USF phenomenon bootstrap failed: model '{}' has empty audio event_id.", normalized_model_id);
                }
                if !audio_emitter.gain.is_finite() || audio_emitter.gain < 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid audio gain={}; expected finite >= 0.",
                        normalized_model_id, audio_emitter.gain
                    );
                }
                if !audio_emitter.spatial_range.is_finite() || audio_emitter.spatial_range <= 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid audio spatial_range={}; expected finite > 0.",
                        normalized_model_id, audio_emitter.spatial_range
                    );
                }
                if !audio_emitter.start_offset_seconds.is_finite() || audio_emitter.start_offset_seconds < 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid audio start_offset_seconds={}; expected finite >= 0.",
                        normalized_model_id, audio_emitter.start_offset_seconds
                    );
                }
                realization_audio_emitter_by_model_id.insert(
                    normalized_model_id.clone(),
                    RealizationAudioEmitterDefinition {
                        event_id,
                        looped: audio_emitter.looped,
                        gain: audio_emitter.gain,
                        spatial_range: audio_emitter.spatial_range,
                        start_offset_seconds: audio_emitter.start_offset_seconds,
                    },
                );
            }
            let has_realization_particle_contract = model.realization_particle_emitter.is_some();
            if has_realization_particle_contract {
                let Some(particle_emitter) = model.realization_particle_emitter.clone() else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         declares contract 'realization_particle_emitter' but has no particle emitter definition. \
                         Call set_realization_particle_emitter(...) in the model script.",
                        normalized_model_id,
                        normalized_phenomenon_id,
                        kind.canonical_id()
                    );
                };
                let effect_id = normalize_identifier(particle_emitter.effect_id.as_str());
                if effect_id.is_empty() {
                    panic!("USF phenomenon bootstrap failed: model '{}' has empty particle effect_id.", normalized_model_id);
                }
                if !particle_emitter.emission_rate.is_finite() || particle_emitter.emission_rate < 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid particle emission_rate={}; expected finite >= 0.",
                        normalized_model_id, particle_emitter.emission_rate
                    );
                }
                if particle_emitter.burst_count == 0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid particle burst_count=0; expected >= 1.",
                        normalized_model_id
                    );
                }
                if !particle_emitter.lifetime_seconds.is_finite() || particle_emitter.lifetime_seconds <= 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid particle lifetime_seconds={}; expected finite > 0.",
                        normalized_model_id, particle_emitter.lifetime_seconds
                    );
                }
                if !particle_emitter.radius.is_finite() || particle_emitter.radius <= 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid particle radius={}; expected finite > 0.",
                        normalized_model_id, particle_emitter.radius
                    );
                }
                realization_particle_emitter_by_model_id.insert(
                    normalized_model_id.clone(),
                    RealizationParticleEmitterDefinition {
                        effect_id,
                        emission_rate: particle_emitter.emission_rate,
                        burst_count: particle_emitter.burst_count,
                        lifetime_seconds: particle_emitter.lifetime_seconds,
                        radius: particle_emitter.radius,
                    },
                );
            }
            let has_interaction_trigger_contract = model.interaction_trigger.is_some();
            if has_interaction_trigger_contract {
                let Some(interaction_trigger) = model.interaction_trigger.clone() else {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' belongs to phenomenon '{}' (kind='{}') \
                         declares contract 'interaction_trigger' but has no trigger definition. \
                         Call set_interaction_trigger(...) in the model script.",
                        normalized_model_id,
                        normalized_phenomenon_id,
                        kind.canonical_id()
                    );
                };
                let trigger_id = normalize_identifier(interaction_trigger.trigger_id.as_str());
                if trigger_id.is_empty() {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has empty interaction trigger_id.",
                        normalized_model_id
                    );
                }
                if !interaction_trigger.cooldown_seconds.is_finite() || interaction_trigger.cooldown_seconds < 0.0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid interaction cooldown_seconds={}; expected finite >= 0.",
                        normalized_model_id, interaction_trigger.cooldown_seconds
                    );
                }
                if interaction_trigger.max_targets == 0 {
                    panic!(
                        "USF phenomenon bootstrap failed: model '{}' has invalid interaction max_targets=0; expected >= 1.",
                        normalized_model_id
                    );
                }
                interaction_trigger_by_model_id.insert(
                    normalized_model_id.clone(),
                    InteractionTriggerDefinition {
                        trigger_id,
                        cooldown_seconds: interaction_trigger.cooldown_seconds,
                        max_targets: interaction_trigger.max_targets,
                    },
                );
            }
            let projection_metric_name = normalize_identifier(model.projection_metric_name.as_str());
            if projection_metric_name.is_empty() {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' has empty projection metric name.",
                    normalized_model_id
                );
            }
            if !model.projection_bias.is_finite() {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' has invalid projection_bias={}.",
                    normalized_model_id, model.projection_bias
                );
            }
            if !model.projection_gain.is_finite() || model.projection_gain <= 0.0 {
                panic!(
                    "USF phenomenon bootstrap failed: model '{}' has invalid projection_gain={}; expected finite > 0.",
                    normalized_model_id, model.projection_gain
                );
            }
            projection_contract_by_model_id.insert(
                normalized_model_id.clone(),
                PhenomenonModelProjectionSpec {
                    metric_name: projection_metric_name,
                    projection_bias: model.projection_bias,
                    projection_gain: model.projection_gain,
                },
            );
            let topology = parse_topology_tag(model.topology.as_str());
            let support_chunk_radius = match topology {
                PhenomenonModelTopology::MonolithicChunk => {
                    if model.support_chunk_radius != 0 {
                        panic!(
                            "USF phenomenon bootstrap failed: model '{}' is monolithic_chunk but declares support_chunk_radius={}; expected 0.",
                            normalized_model_id, model.support_chunk_radius
                        );
                    }
                    0
                }
                PhenomenonModelTopology::PartitionedByChunk => {
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
        Self {
            kind_by_phenomenon_id,
            realization_density_by_model_id,
            realization_material_by_model_id,
            realization_collider_enabled_by_model_id,
            simulation_service_by_model_id,
            realization_audio_emitter_by_model_id,
            realization_particle_emitter_by_model_id,
            interaction_trigger_by_model_id,
            projection_contract_by_model_id,
            topology_by_model_id,
            support_chunk_radius_by_model_id,
            model_selection_by_phenomenon_scale,
            phenomenon_by_model_id,
        }
    }
}

impl PhenomenonDefinitionRegistry {
    pub fn kind_for(&self, phenomenon_id: &str) -> Option<PhenomenonKind> {
        self.kind_by_phenomenon_id.get(&normalize_identifier(phenomenon_id)).cloned()
    }

    pub fn realization_density_for(&self, phenomenon_id: &str) -> Option<RealizationDensityFieldDefinition> {
        self.realization_density_for_scale(phenomenon_id, Scale::MAX)
    }

    pub fn realization_density_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<RealizationDensityFieldDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.realization_density_for_model(model_id)
    }

    pub fn realization_density_for_model(&self, model_id: &str) -> Option<RealizationDensityFieldDefinition> {
        self.realization_density_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn realization_material_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<RealizationMaterialProfileDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.realization_material_for_model(model_id)
    }

    pub fn realization_material_for_model(&self, model_id: &str) -> Option<RealizationMaterialProfileDefinition> {
        self.realization_material_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn realization_collider_enabled_for_scale(&self, phenomenon_id: &str, scale: Scale) -> bool {
        let Some(model_id) = self.model_for_scale(phenomenon_id, scale) else {
            return false;
        };
        self.realization_collider_enabled_for_model(model_id)
    }

    pub fn realization_collider_enabled_for_model(&self, model_id: &str) -> bool {
        self.realization_collider_enabled_by_model_id
            .get(&normalize_identifier(model_id))
            .copied()
            .unwrap_or(false)
    }

    pub fn simulation_service_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<PhenomenonSimulationServiceDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.simulation_service_for_model(model_id)
    }

    pub fn simulation_service_for_model(&self, model_id: &str) -> Option<PhenomenonSimulationServiceDefinition> {
        self.simulation_service_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn realization_audio_emitter_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<RealizationAudioEmitterDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.realization_audio_emitter_for_model(model_id)
    }

    pub fn realization_audio_emitter_for_model(&self, model_id: &str) -> Option<RealizationAudioEmitterDefinition> {
        self.realization_audio_emitter_by_model_id.get(&normalize_identifier(model_id)).cloned()
    }

    pub fn realization_particle_emitter_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<RealizationParticleEmitterDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.realization_particle_emitter_for_model(model_id)
    }

    pub fn realization_particle_emitter_for_model(&self, model_id: &str) -> Option<RealizationParticleEmitterDefinition> {
        self.realization_particle_emitter_by_model_id.get(&normalize_identifier(model_id)).cloned()
    }

    pub fn interaction_trigger_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<InteractionTriggerDefinition> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.interaction_trigger_for_model(model_id)
    }

    pub fn interaction_trigger_for_model(&self, model_id: &str) -> Option<InteractionTriggerDefinition> {
        self.interaction_trigger_by_model_id.get(&normalize_identifier(model_id)).cloned()
    }

    pub fn projection_contract_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<PhenomenonModelProjectionSpec> {
        let model_id = self.model_for_scale(phenomenon_id, scale)?;
        self.projection_contract_for_model(model_id)
    }

    pub fn projection_contract_for_model(&self, model_id: &str) -> Option<PhenomenonModelProjectionSpec> {
        self.projection_contract_by_model_id.get(&normalize_identifier(model_id)).cloned()
    }

    pub fn any_model_declares_realization_collider_contract(&self) -> bool {
        self.realization_collider_enabled_by_model_id.values().copied().any(|enabled| enabled)
    }

    pub fn any_model_declares_realization_audio_emitter_contract(&self) -> bool {
        !self.realization_audio_emitter_by_model_id.is_empty()
    }

    pub fn any_model_declares_realization_particle_emitter_contract(&self) -> bool {
        !self.realization_particle_emitter_by_model_id.is_empty()
    }

    pub fn any_model_declares_interaction_trigger_contract(&self) -> bool {
        !self.interaction_trigger_by_model_id.is_empty()
    }

    pub fn realization_field_contract_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<PhenomenonRealizationFieldContract> {
        self.realization_density_for_scale(phenomenon_id, scale)
            .map(PhenomenonRealizationFieldContract::DensityField)
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

    pub fn topology_for_model(&self, model_id: &str) -> Option<PhenomenonModelTopology> {
        self.topology_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn support_chunk_radius_for_model(&self, model_id: &str) -> Option<u16> {
        self.support_chunk_radius_by_model_id.get(&normalize_identifier(model_id)).copied()
    }

    pub fn topology_for_scale(&self, phenomenon_id: &str, scale: Scale) -> Option<PhenomenonModelTopology> {
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
fn parse_topology_tag(raw: &str) -> PhenomenonModelTopology {
    let normalized = raw.trim().to_ascii_lowercase();
    match normalized.as_str() {
        "monolithic_chunk" | "monolithic-chunk" | "monolithic" => PhenomenonModelTopology::MonolithicChunk,
        "partitioned_by_chunk" | "partitioned-by-chunk" | "partitioned" => PhenomenonModelTopology::PartitionedByChunk,
        _ => panic!(
            "USF phenomenon bootstrap failed: unsupported model topology '{}'; expected monolithic_chunk or partitioned_by_chunk.",
            normalized
        ),
    }
}
