use std::collections::HashSet;

use crate::bevy::prelude::*;
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct DptMetricId(pub u16);

#[derive(Reflect, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct ZoneTypeId(pub String);
impl ZoneTypeId {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub enum DptMetricValueType {
    U8,
    U16,
    I32,
    F32,
    F64,
}
impl DptMetricValueType {
    pub fn from_tag(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "u8" => Some(Self::U8),
            "u16" => Some(Self::U16),
            "i32" => Some(Self::I32),
            "f32" => Some(Self::F32),
            "f64" => Some(Self::F64),
            _ => None,
        }
    }
}
impl Default for DptMetricValueType {
    fn default() -> Self {
        Self::F32
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub enum DptMetricStorageClass {
    Uniform,
    Brick,
}
impl DptMetricStorageClass {
    pub fn from_tag(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "uniform" => Some(Self::Uniform),
            "brick" => Some(Self::Brick),
            _ => None,
        }
    }
}
impl Default for DptMetricStorageClass {
    fn default() -> Self {
        Self::Brick
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct DptMetricDefinition {
    pub id: DptMetricId,
    pub name: String,
    pub value_type: DptMetricValueType,
    pub semantics_tag: String,
    pub storage_class: DptMetricStorageClass,
    pub derived: bool,
    pub min_scale_index: u8,
    pub max_scale_index: u8,
}
impl DptMetricDefinition {
    pub fn applies_to_scale(&self, scale: Scale) -> bool {
        let index = scale.index_from_top();
        (self.min_scale_index..=self.max_scale_index).contains(&index)
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct DptSchema {
    pub revision: u64,
    pub metrics: Vec<DptMetricDefinition>,
    pub fallback_zone: ZoneTypeId,
}
impl DptSchema {
    pub fn metric_index(&self, metric_id: DptMetricId) -> Option<usize> {
        self.metrics.iter().position(|metric| metric.id == metric_id)
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.metrics.is_empty() {
            return Err("DPT schema must contain at least one metric".to_string());
        }
        if self.revision == 0 {
            return Err("DPT schema revision must be > 0".to_string());
        }
        if self.fallback_zone.0.trim().is_empty() {
            return Err("DPT schema fallback zone must not be empty".to_string());
        }

        let mut seen_metric_ids = HashSet::<DptMetricId>::new();
        let mut seen_metric_names = HashSet::<String>::new();
        for metric in &self.metrics {
            if !seen_metric_ids.insert(metric.id) {
                return Err(format!("duplicate DPT metric id {}", metric.id.0));
            }
            let normalized_name = metric.name.trim().to_ascii_lowercase();
            if normalized_name.is_empty() {
                return Err(format!("metric {} has an empty name", metric.id.0));
            }
            if !seen_metric_names.insert(normalized_name.clone()) {
                return Err(format!("duplicate DPT metric name '{normalized_name}'"));
            }
            let normalized_semantics = metric.semantics_tag.trim().to_ascii_lowercase();
            if normalized_semantics.is_empty() {
                return Err(format!("metric {} has an empty semantics_tag", metric.id.0));
            }
            if metric.min_scale_index >= Scale::SCALE_LEVEL_COUNT {
                return Err(format!(
                    "metric {} has min_scale_index={} outside [0..{}]",
                    metric.id.0,
                    metric.min_scale_index,
                    Scale::SCALE_LEVEL_COUNT.saturating_sub(1)
                ));
            }
            if metric.max_scale_index >= Scale::SCALE_LEVEL_COUNT {
                return Err(format!(
                    "metric {} has max_scale_index={} outside [0..{}]",
                    metric.id.0,
                    metric.max_scale_index,
                    Scale::SCALE_LEVEL_COUNT.saturating_sub(1)
                ));
            }
            if metric.min_scale_index > metric.max_scale_index {
                return Err(format!(
                    "metric {} has invalid scale range [{}..{}]",
                    metric.id.0, metric.min_scale_index, metric.max_scale_index
                ));
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn schema_validate_rejects_duplicate_metric_id() {
        let schema = DptSchema {
            revision: 1,
            metrics: vec![
                DptMetricDefinition {
                    id: DptMetricId(0),
                    name: "temperature".to_string(),
                    value_type: DptMetricValueType::F32,
                    semantics_tag: "climate.temperature".to_string(),
                    storage_class: DptMetricStorageClass::Brick,
                    derived: false,
                    min_scale_index: 0,
                    max_scale_index: 0,
                },
                DptMetricDefinition {
                    id: DptMetricId(0),
                    name: "humidity".to_string(),
                    value_type: DptMetricValueType::F32,
                    semantics_tag: "climate.humidity".to_string(),
                    storage_class: DptMetricStorageClass::Brick,
                    derived: false,
                    min_scale_index: 0,
                    max_scale_index: 0,
                },
            ],
            fallback_zone: ZoneTypeId::new("void"),
        };

        let error = schema.validate().unwrap_err();
        assert!(error.contains("duplicate DPT metric id"));
    }
}
