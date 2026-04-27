use crate::bevy::prelude::*;
use crate::usf::scale::Scale;

#[derive(Reflect, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct MetricId(pub u16);

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub enum MetricValueType {
    U8,
    U16,
    I32,
    F32,
    F64,
}
impl MetricValueType {
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
impl Default for MetricValueType {
    fn default() -> Self {
        Self::F32
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub enum MetricStorageClass {
    Uniform,
    Brick,
}
impl MetricStorageClass {
    pub fn from_tag(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "uniform" => Some(Self::Uniform),
            "brick" => Some(Self::Brick),
            _ => None,
        }
    }
}
impl Default for MetricStorageClass {
    fn default() -> Self {
        Self::Brick
    }
}

#[derive(Reflect, Debug, Clone, PartialEq, Eq)]
pub struct MetricDefinition {
    pub id: MetricId,
    pub name: String,
    pub value_type: MetricValueType,
    pub semantics_tag: String,
    pub storage_class: MetricStorageClass,
    pub derived: bool,
    pub min_scale_index: u8,
    pub max_scale_index: u8,
}
impl MetricDefinition {
    pub fn applies_to_scale(&self, scale: Scale) -> bool {
        let index = scale.index_from_top();
        (self.min_scale_index..=self.max_scale_index).contains(&index)
    }
}
