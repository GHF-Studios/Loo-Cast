use bevy::prelude::*;

use super::{FocusTarget, StructureItemId};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectSectionId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectFieldId(pub &'static str);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectActionId(pub &'static str);

/// How an inspected value may be changed.
///
/// This is intentionally richer than a `read_only: bool`: presentation consumes
/// authority supplied by the domain; it never creates authority merely because
/// `&mut T` happens to be technically obtainable.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InspectAccess {
    #[default]
    ReadOnly,
    /// The inspected value itself is authoritative and may be mutated directly.
    Direct,
    /// Changes must pass through validation/setter logic.
    Validated,
    /// Changes belong to an authoring transaction/undo history.
    Transactional,
    /// Changes are requests/commands to the owning domain rather than field writes.
    Command,
}

impl InspectAccess {
    pub fn editable(self) -> bool {
        self != Self::ReadOnly
    }

    pub fn label(self) -> &'static str {
        match self {
            Self::ReadOnly => "read-only",
            Self::Direct => "direct",
            Self::Validated => "validated",
            Self::Transactional => "transactional",
            Self::Command => "command",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InspectUnit(pub &'static str);

impl InspectUnit {
    pub const KELVIN: Self = Self("K");
    pub const JOULE: Self = Self("J");
    pub const WATT: Self = Self("W");
    pub const HEAT_CAPACITY: Self = Self("J/K");
    pub const COOLING_COEFFICIENT: Self = Self("W/K");
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

/// UI hints for numeric input. These are presentation affordances, not domain
/// validation. Domains must still validate every edit request they accept.
#[derive(Debug, Clone, Copy)]
pub struct InspectNumberInput {
    pub speed: f64,
    pub minimum: Option<f64>,
    pub maximum: Option<f64>,
}

impl Default for InspectNumberInput {
    fn default() -> Self {
        Self {
            speed: 0.1,
            minimum: None,
            maximum: None,
        }
    }
}

impl InspectNumberInput {
    pub const fn speed(speed: f64) -> Self {
        Self {
            speed,
            minimum: None,
            maximum: None,
        }
    }

    pub const fn range(mut self, minimum: f64, maximum: f64) -> Self {
        self.minimum = Some(minimum);
        self.maximum = Some(maximum);
        self
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

    pub fn number(value: impl Into<f64>) -> Self {
        Self::Number {
            value: value.into(),
            format: InspectNumberFormat::default(),
        }
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

    pub fn as_number(&self) -> Option<f64> {
        match self {
            Self::Number { value, .. } | Self::Quantity { value, .. } => Some(*value),
            _ => None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct InspectField {
    pub id: Option<InspectFieldId>,
    pub label: String,
    pub symbol: Option<&'static str>,
    pub value: InspectValue,
    pub access: InspectAccess,
    pub hint: Option<String>,
    pub number_input: Option<InspectNumberInput>,
}

impl InspectField {
    pub fn new(label: impl Into<String>, value: InspectValue) -> Self {
        Self {
            id: None,
            label: label.into(),
            symbol: None,
            value,
            access: InspectAccess::ReadOnly,
            hint: None,
            number_input: None,
        }
    }

    pub fn symbol(mut self, symbol: &'static str) -> Self {
        self.symbol = Some(symbol);
        self
    }

    pub fn access(mut self, access: InspectAccess) -> Self {
        self.access = access;
        self
    }

    pub fn editable(mut self, id: InspectFieldId, access: InspectAccess) -> Self {
        debug_assert!(access.editable());
        self.id = Some(id);
        self.access = access;
        self
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn number_input(mut self, input: InspectNumberInput) -> Self {
        self.number_input = Some(input);
        self
    }
}

#[derive(Debug, Clone)]
pub struct InspectAction {
    pub id: InspectActionId,
    pub label: String,
    pub hint: Option<String>,
}

impl InspectAction {
    pub fn new(id: InspectActionId, label: impl Into<String>) -> Self {
        Self {
            id,
            label: label.into(),
            hint: None,
        }
    }

    pub fn hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }
}

#[derive(Debug, Clone)]
pub struct InspectSection {
    pub id: InspectSectionId,
    pub title: String,
    pub order: i32,
    pub structure_item: Option<StructureItemId>,
    pub contextual_gizmo: bool,
    pub fields: Vec<InspectField>,
    pub actions: Vec<InspectAction>,
}

impl InspectSection {
    pub fn new(id: InspectSectionId, title: impl Into<String>, order: i32) -> Self {
        Self {
            id,
            title: title.into(),
            order,
            structure_item: None,
            contextual_gizmo: false,
            fields: Vec::new(),
            actions: Vec::new(),
        }
    }

    pub fn for_structure(mut self, structure_item: StructureItemId) -> Self {
        self.structure_item = Some(structure_item);
        self
    }

    pub fn contextual_gizmo(mut self) -> Self {
        self.contextual_gizmo = true;
        self
    }

    pub fn field(mut self, field: InspectField) -> Self {
        self.fields.push(field);
        self
    }

    pub fn action(mut self, action: InspectAction) -> Self {
        self.actions.push(action);
        self
    }
}

/// Proposed semantic field edit emitted by an inspection UI host.
///
/// This is a request, never authority. The owning domain must re-check target,
/// access, invariants and persistence semantics before applying it.
#[derive(Message, Debug, Clone)]
pub struct InspectEditRequest {
    pub target: FocusTarget,
    pub section: InspectSectionId,
    pub field: InspectFieldId,
    pub value: InspectValue,
}

/// Contextual operation emitted by an inspection/gizmo UI host.
#[derive(Message, Debug, Clone, Copy)]
pub struct InspectActionRequest {
    pub target: FocusTarget,
    pub section: InspectSectionId,
    pub action: InspectActionId,
}

/// One frame of structured semantic inspection data for the canonical focus.
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

    pub fn section(&self, id: InspectSectionId) -> Option<&InspectSection> {
        self.sections.iter().find(|section| section.id == id)
    }

    pub fn sorted_sections(&self) -> Vec<&InspectSection> {
        self.sorted_sections_for(None)
    }

    /// `None` means whole-entity inspection and therefore returns all sections.
    pub fn sorted_sections_for(&self, structure_item: Option<StructureItemId>) -> Vec<&InspectSection> {
        let mut sections = self
            .sections
            .iter()
            .filter(|section| {
                structure_item.is_none() || section.structure_item == structure_item
            })
            .collect::<Vec<_>>();
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
