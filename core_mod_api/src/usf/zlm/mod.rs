use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::USF_ZLM_SCALES_BY_SCALE;
use crate::usf::content::UsfActiveContentProfile;
use crate::usf::definition::{DptMetricId, DptSchema, ZoneTypeId};
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct ZlmMetricBand {
    pub metric_id: DptMetricId,
    pub min: f32,
    pub max: f32,
}
impl ZlmMetricBand {
    pub fn contains(&self, value: f32) -> bool {
        value >= self.min && value <= self.max
    }
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct ZlmZoneRule {
    pub zone_type: ZoneTypeId,
    pub metric_bands: Vec<ZlmMetricBand>,
}

#[derive(Reflect, Debug, Clone, PartialEq)]
pub struct ZlmScaleDefinition {
    pub revision: u64,
    pub fallback_zone: ZoneTypeId,
    pub rules: Vec<ZlmZoneRule>,
}

#[derive(Resource, Debug, Clone)]
pub struct ZlmRegistry {
    pub maps_by_scale: HashMap<Scale, ZlmScaleDefinition>,
}
impl Default for ZlmRegistry {
    fn default() -> Self {
        let script_maps = script_zlm_overrides();
        if script_maps.is_empty() {
            panic!("USF ZLM bootstrap failed: no ZLM maps registered. Define at least one '*.zlm.rhai' file.");
        }
        let maps_by_scale = script_maps.into_iter().collect::<HashMap<_, _>>();

        Self { maps_by_scale }
    }
}
impl ZlmRegistry {
    pub fn classify(&self, scale: Scale, schema: &DptSchema, metric_values: &[f32]) -> ZoneTypeId {
        let Some(scale_map) = self.maps_by_scale.get(&scale) else {
            panic!("USF ZLM classification failed: missing map for scale index {}", scale.index_from_top());
        };

        for rule in &scale_map.rules {
            let mut is_match = true;
            for band in &rule.metric_bands {
                let Some(index) = schema.metric_index(band.metric_id) else {
                    is_match = false;
                    break;
                };
                let Some(value) = metric_values.get(index).copied() else {
                    is_match = false;
                    break;
                };
                if !band.contains(value) {
                    is_match = false;
                    break;
                }
            }

            if is_match {
                return rule.zone_type.clone();
            }
        }

        scale_map.fallback_zone.clone()
    }

    pub fn classify_with_scale_binding(
        &self,
        scale: Scale,
        schema: &DptSchema,
        metric_values: &[f32],
        active_content_profile: &UsfActiveContentProfile,
    ) -> ZoneTypeId {
        let _categorizer_id = active_content_profile
            .binding_for_scale(scale)
            .map(|binding| binding.dpt_categorizer_id.as_str())
            .unwrap_or_else(|| {
                panic!(
                    "USF ZLM classification failed: missing scale content binding for scale index {}",
                    scale.index_from_top()
                )
            });
        self.classify(scale, schema, metric_values)
    }

    pub fn validate_against(&self, active_content_profile: &UsfActiveContentProfile) -> Result<(), String> {
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(schema) = active_content_profile.schema_for_scale(scale) else {
                return Err(format!("missing schema while validating ZLM scale {}", scale.index_from_top()));
            };
            let Some(scale_map) = self.maps_by_scale.get(&scale) else {
                return Err(format!("missing ZLM map for scale {}", scale.index_from_top()));
            };
            if scale_map.revision == 0 {
                return Err(format!("ZLM map revision must be > 0 for scale {}", scale.index_from_top()));
            }
            if scale_map.revision < schema.revision {
                return Err(format!(
                    "ZLM map revision {} must be >= schema revision {} at scale {}",
                    scale_map.revision,
                    schema.revision,
                    scale.index_from_top()
                ));
            }
            if !active_content_profile.known_zone_types.contains(&scale_map.fallback_zone) {
                return Err(format!(
                    "ZLM fallback zone '{}' is not known at scale {}",
                    scale_map.fallback_zone.0,
                    scale.index_from_top()
                ));
            }
            for rule in &scale_map.rules {
                if !active_content_profile.known_zone_types.contains(&rule.zone_type) {
                    return Err(format!(
                        "ZLM rule references unknown zone '{}' at scale {}",
                        rule.zone_type.0,
                        scale.index_from_top()
                    ));
                }
                for band in &rule.metric_bands {
                    if !band.min.is_finite() || !band.max.is_finite() {
                        return Err(format!(
                            "non-finite metric range for metric {} at scale {}",
                            band.metric_id.0,
                            scale.index_from_top()
                        ));
                    }
                    if band.max < band.min {
                        return Err(format!(
                            "invalid metric range {}..{} for metric {} at scale {}",
                            band.min,
                            band.max,
                            band.metric_id.0,
                            scale.index_from_top()
                        ));
                    }
                    if schema.metric_index(band.metric_id).is_none() {
                        return Err(format!("metric {} not found in schema at scale {}", band.metric_id.0, scale.index_from_top()));
                    }
                }
            }
        }

        Ok(())
    }
}

fn normalize_zone_type(value: &str) -> ZoneTypeId {
    ZoneTypeId::new(value.trim().to_ascii_lowercase())
}

fn script_zlm_overrides() -> Vec<(Scale, ZlmScaleDefinition)> {
    let zlm_maps = USF_ZLM_SCALES_BY_SCALE().lock().unwrap().clone();
    let mut ordered = zlm_maps.into_iter().collect::<Vec<_>>();
    ordered.sort_by_key(|(scale_index, _)| *scale_index);

    ordered
        .into_iter()
        .filter_map(|(scale_index, definition)| {
            let Some(scale) = Scale::from_index_from_top(scale_index) else {
                return None;
            };
            let rules = definition
                .rules
                .into_iter()
                .map(|rule| ZlmZoneRule {
                    zone_type: normalize_zone_type(&rule.zone_type),
                    metric_bands: rule
                        .metric_bands
                        .into_iter()
                        .map(|band| ZlmMetricBand {
                            metric_id: DptMetricId(band.metric_id),
                            min: band.min,
                            max: band.max,
                        })
                        .collect(),
                })
                .collect();
            Some((
                scale,
                ZlmScaleDefinition {
                    revision: definition.revision,
                    fallback_zone: normalize_zone_type(&definition.fallback_zone),
                    rules,
                },
            ))
        })
        .collect()
}

fn validate_zlm_registry_system(active_content_profile: Res<UsfActiveContentProfile>, zlm_registry: Res<ZlmRegistry>) {
    if let Err(reason) = zlm_registry.validate_against(&active_content_profile) {
        panic!("USF ZLM registry validation failed: {reason}");
    }
}

pub(crate) struct ZlmPlugin;
impl Plugin for ZlmPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ZlmRegistry>()
            .add_systems(Startup, validate_zlm_registry_system.in_set(AppSet::Diagnostics));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::usf::content::UsfActiveContentProfile;
    use crate::usf::definition::{DptMetricDefinition, DptMetricId, DptMetricStorageClass, DptMetricValueType};
    use std::collections::HashMap;

    fn active_content_profile_for_tests() -> UsfActiveContentProfile {
        let mut known_zone_types = std::collections::HashSet::new();
        known_zone_types.insert(ZoneTypeId::new("void"));

        let mut schemas_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let scale_index = scale.index_from_top();
            schemas_by_scale.insert(
                scale,
                DptSchema {
                    revision: 1,
                    metrics: vec![DptMetricDefinition {
                        id: DptMetricId(0),
                        name: "temperature".to_string(),
                        value_type: DptMetricValueType::F32,
                        semantics_tag: "climate.temperature.normalized".to_string(),
                        storage_class: DptMetricStorageClass::Brick,
                        derived: false,
                        min_scale_index: scale_index,
                        max_scale_index: scale_index,
                    }],
                    fallback_zone: ZoneTypeId::new("void"),
                },
            );
        }

        UsfActiveContentProfile {
            profile_id: "content_profile.test.default".to_string(),
            configured_content_packages: vec![crate::usf::content::UsfConfiguredContentPackage {
                content_package_id: "content_package.test.default".to_string(),
                default_enabled: true,
                config_enabled_key: "usf_content/content_packages/test_default/enabled".to_string(),
                enabled: true,
            }],
            enabled_content_packages: std::collections::HashSet::from(["content_package.test.default".to_string()]),
            resolved_enabled_content_packages: vec!["content_package.test.default".to_string()],
            bindings_by_scale: (0..Scale::SCALE_LEVEL_COUNT)
                .filter_map(Scale::from_index_from_top)
                .map(|scale| {
                    (
                        scale,
                        crate::usf::content::ScaleContentBinding {
                            dpt_sampler_id: crate::usf::content::DPT_SAMPLER_KERNEL_DEFAULT_ID.to_string(),
                            dpt_categorizer_id: crate::usf::content::DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string(),
                            chunk_store_key: "chunk_store.test.default".to_string(),
                        },
                    )
                })
                .collect(),
            known_dpt_samplers: std::collections::HashSet::from([crate::usf::content::DPT_SAMPLER_KERNEL_DEFAULT_ID.to_string()]),
            known_dpt_categorizers: std::collections::HashSet::from([crate::usf::content::DPT_CATEGORIZER_KERNEL_ZLM_LOOKUP_ID.to_string()]),
            schemas_by_scale,
            known_zone_types,
        }
    }

    #[test]
    fn classify_uses_scale_map_fallback_zone_when_no_rule_matches() {
        let scale = Scale::MAX;
        let schema = DptSchema {
            revision: 1,
            metrics: vec![DptMetricDefinition {
                id: DptMetricId(0),
                name: "temperature".to_string(),
                value_type: crate::usf::definition::DptMetricValueType::F32,
                semantics_tag: "climate.temperature.normalized".to_string(),
                storage_class: crate::usf::definition::DptMetricStorageClass::Brick,
                derived: false,
                min_scale_index: 0,
                max_scale_index: Scale::SCALE_LEVEL_COUNT.saturating_sub(1),
            }],
            fallback_zone: ZoneTypeId::new("void"),
        };
        let mut maps_by_scale = HashMap::new();
        maps_by_scale.insert(
            scale,
            ZlmScaleDefinition {
                revision: 1,
                fallback_zone: ZoneTypeId::new("arid"),
                rules: Vec::new(),
            },
        );
        let zlm_registry = ZlmRegistry { maps_by_scale };

        let classified = zlm_registry.classify(scale, &schema, &[0.8]);
        assert_eq!(classified, ZoneTypeId::new("arid"));
    }

    #[test]
    fn validate_rejects_map_revision_that_lags_schema_revision() {
        let scale = Scale::MAX;
        let mut active_content_profile = active_content_profile_for_tests();
        active_content_profile.schemas_by_scale.get_mut(&scale).unwrap().revision = 2;

        let mut maps_by_scale = HashMap::new();
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(level) = Scale::from_index_from_top(index) else {
                continue;
            };
            maps_by_scale.insert(
                level,
                ZlmScaleDefinition {
                    revision: 2,
                    fallback_zone: ZoneTypeId::new("void"),
                    rules: Vec::new(),
                },
            );
        }
        maps_by_scale.get_mut(&scale).unwrap().revision = 1;
        let zlm_registry = ZlmRegistry { maps_by_scale };

        let error = zlm_registry.validate_against(&active_content_profile).unwrap_err();
        assert!(error.contains("must be >= schema revision"));
    }
}
