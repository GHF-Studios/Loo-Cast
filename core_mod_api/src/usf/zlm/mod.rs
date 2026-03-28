use std::collections::HashMap;

use crate::bevy::prelude::*;
use crate::core::orchestration::AppSet;
use crate::rhai_binding::engine::statics::USF_ZLM_SCALES_BY_SCALE;
use crate::usf::definition::{DefinitionRegistry, DptMetricId, DptSchema, ScaleContentRegistry, ZoneTypeId};
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
        let mut maps_by_scale = HashMap::new();
        let script_maps = script_zlm_overrides();
        if script_maps.is_empty() {
            for index in 0..Scale::SCALE_LEVEL_COUNT {
                let Some(scale) = Scale::from_index_from_top(index) else {
                    continue;
                };
                maps_by_scale.insert(scale, baseline_zlm_for_scale(scale));
            }
        } else {
            for (scale, definition) in script_maps {
                maps_by_scale.insert(scale, definition);
            }
        }

        Self { maps_by_scale }
    }
}
impl ZlmRegistry {
    pub const DEFAULT_DPT_CATEGORIZER_ID: &'static str = "dpt_categorizer.debug.zlm_lookup.v1";

    pub fn classify(&self, scale: Scale, schema: &DptSchema, metric_values: &[f32]) -> ZoneTypeId {
        let Some(scale_map) = self.maps_by_scale.get(&scale) else {
            return schema.fallback_zone.clone();
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
        scale_content_registry: &ScaleContentRegistry,
    ) -> ZoneTypeId {
        let _categorizer_id = scale_content_registry
            .binding_for_scale(scale)
            .map(|binding| binding.dpt_categorizer_id.as_str())
            .unwrap_or(Self::DEFAULT_DPT_CATEGORIZER_ID);
        self.classify(scale, schema, metric_values)
    }

    pub fn validate_against(&self, definitions: &DefinitionRegistry) -> Result<(), String> {
        for index in 0..Scale::SCALE_LEVEL_COUNT {
            let Some(scale) = Scale::from_index_from_top(index) else {
                continue;
            };
            let Some(schema) = definitions.schema_for_scale(scale) else {
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
            if !definitions.known_zone_types.contains(&scale_map.fallback_zone) {
                return Err(format!(
                    "ZLM fallback zone '{}' is not known at scale {}",
                    scale_map.fallback_zone.0,
                    scale.index_from_top()
                ));
            }
            for rule in &scale_map.rules {
                if !definitions.known_zone_types.contains(&rule.zone_type) {
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

fn baseline_zlm_for_scale(_scale: Scale) -> ZlmScaleDefinition {
    ZlmScaleDefinition {
        revision: 1,
        fallback_zone: normalize_zone_type("void"),
        rules: vec![
            ZlmZoneRule {
                zone_type: normalize_zone_type("wetland"),
                metric_bands: vec![
                    ZlmMetricBand {
                        metric_id: DptMetricId(1),
                        min: 0.70,
                        max: 1.0,
                    },
                    ZlmMetricBand {
                        metric_id: DptMetricId(2),
                        min: 0.0,
                        max: 0.35,
                    },
                ],
            },
            ZlmZoneRule {
                zone_type: normalize_zone_type("forest"),
                metric_bands: vec![
                    ZlmMetricBand {
                        metric_id: DptMetricId(1),
                        min: 0.45,
                        max: 1.0,
                    },
                    ZlmMetricBand {
                        metric_id: DptMetricId(0),
                        min: 0.20,
                        max: 0.85,
                    },
                ],
            },
            ZlmZoneRule {
                zone_type: normalize_zone_type("alpine"),
                metric_bands: vec![ZlmMetricBand {
                    metric_id: DptMetricId(2),
                    min: 0.75,
                    max: 1.0,
                }],
            },
            ZlmZoneRule {
                zone_type: normalize_zone_type("arid"),
                metric_bands: vec![
                    ZlmMetricBand {
                        metric_id: DptMetricId(1),
                        min: 0.0,
                        max: 0.30,
                    },
                    ZlmMetricBand {
                        metric_id: DptMetricId(0),
                        min: 0.45,
                        max: 1.0,
                    },
                ],
            },
        ],
    }
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

fn validate_zlm_registry_system(definitions: Res<DefinitionRegistry>, zlm_registry: Res<ZlmRegistry>) {
    if let Err(reason) = zlm_registry.validate_against(&definitions) {
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
    use crate::usf::definition::{DptMetricDefinition, DptMetricId};
    use std::collections::HashMap;

    #[test]
    fn classify_uses_scale_map_fallback_zone_when_no_rule_matches() {
        let scale = Scale::MAX;
        let schema = DptSchema {
            revision: 1,
            metrics: vec![DptMetricDefinition {
                id: DptMetricId(0),
                name: "temperature".to_string(),
                primitive: true,
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
        let mut definitions = DefinitionRegistry::default();
        definitions.schemas_by_scale.get_mut(&scale).unwrap().revision = 2;

        let mut zlm_registry = ZlmRegistry::default();
        zlm_registry.maps_by_scale.get_mut(&scale).unwrap().revision = 1;

        let error = zlm_registry.validate_against(&definitions).unwrap_err();
        assert!(error.contains("must be >= schema revision"));
    }
}
