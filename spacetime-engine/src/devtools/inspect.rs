use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectSectionId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectUnit(pub &'static str);

impl InspectUnit {
    pub const KELVIN: Self = Self("K");
    pub const JOULE: Self = Self("J");
    pub const WATT: Self = Self("W");
    pub const METER: Self = Self("m");
    pub const METERS_PER_SECOND: Self = Self("m/s");
    pub const METERS_PER_SECOND_SQUARED: Self = Self("m/s²");
    pub const DENSITY: Self = Self("kg/m³");
    pub const SPECIFIC_HEAT_CAPACITY: Self = Self("J/(kg·K)");
    pub const THERMAL_CONDUCTIVITY: Self = Self("W/(m·K)");
    pub const THERMAL_DIFFUSIVITY: Self = Self("m²/s");
}

#[derive(Debug, Clone, Copy)]
pub struct InspectNumberFormat {
    pub significant_digits: u8,
}

impl Default for InspectNumberFormat {
    fn default() -> Self {
        Self {
            significant_digits: 5,
        }
    }
}

impl InspectNumberFormat {
    pub const fn significant_digits(significant_digits: u8) -> Self {
        Self { significant_digits }
    }
}

#[derive(Debug, Clone)]
pub enum InspectValue {
    Text(String),
    Bool(bool),
    Integer(i64),
    Number {
        value: f64,
        format: InspectNumberFormat,
    },
    Quantity {
        value: f64,
        unit: InspectUnit,
        format: InspectNumberFormat,
    },
    Range {
        minimum: f64,
        maximum: f64,
        unit: InspectUnit,
        format: InspectNumberFormat,
    },
    Entity(Entity),
    Vec3(Vec3),
}

impl InspectValue {
    pub fn text(value: impl Into<String>) -> Self {
        Self::Text(value.into())
    }

    pub fn quantity(value: impl Into<f64>, unit: InspectUnit) -> Self {
        Self::Quantity {
            value: value.into(),
            unit,
            format: InspectNumberFormat::default(),
        }
    }

    pub fn quantity_with_format(
        value: impl Into<f64>,
        unit: InspectUnit,
        format: InspectNumberFormat,
    ) -> Self {
        Self::Quantity {
            value: value.into(),
            unit,
            format,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InspectField {
    pub label: String,
    pub symbol: Option<&'static str>,
    pub value: InspectValue,
}

impl InspectField {
    pub fn new(label: impl Into<String>, value: InspectValue) -> Self {
        Self {
            label: label.into(),
            symbol: None,
            value,
        }
    }

    pub fn symbol(mut self, symbol: &'static str) -> Self {
        self.symbol = Some(symbol);
        self
    }
}

#[derive(Debug, Clone)]
pub struct InspectSection {
    pub id: InspectSectionId,
    pub title: String,
    pub order: i32,
    pub fields: Vec<InspectField>,
}

impl InspectSection {
    pub fn new(id: InspectSectionId, title: impl Into<String>, order: i32) -> Self {
        Self {
            id,
            title: title.into(),
            order,
            fields: Vec::new(),
        }
    }

    pub fn field(mut self, field: InspectField) -> Self {
        self.fields.push(field);
        self
    }
}

/// One frame of structured Inspector data for the current developer focus.
///
/// This deliberately uses ordinary `ResMut` collection. Only one focus target is
/// inspected, so parallel append machinery would add complexity without useful
/// throughput.
#[derive(Resource, Debug, Default)]
pub struct InspectionFrame {
    sections: Vec<InspectSection>,
}

impl InspectionFrame {
    pub fn submit(&mut self, section: InspectSection) {
        assert!(
            !self.sections.iter().any(|existing| existing.id == section.id),
            "duplicate inspection section id {}",
            section.id.0,
        );
        self.sections.push(section);
    }

    pub fn sections(&self) -> &[InspectSection] {
        &self.sections
    }

    pub fn sorted_sections(&self) -> Vec<&InspectSection> {
        let mut sections = self.sections.iter().collect::<Vec<_>>();
        sections.sort_by(|a, b| {
            a.order
                .cmp(&b.order)
                .then_with(|| a.title.cmp(&b.title))
        });
        sections
    }

    fn clear(&mut self) {
        self.sections.clear();
    }
}

pub(super) fn clear_inspection_frame(mut frame: ResMut<InspectionFrame>) {
    frame.clear();
}
