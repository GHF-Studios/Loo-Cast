//! Semantic inspection values, fields, actions and sections.

use bevy::prelude::{Entity, Vec3};

use super::metadata::{
    InspectAccess, InspectActionId, InspectFieldId, InspectNumberFormat,
    InspectNumberInput, InspectSectionId, InspectUnit,
};
use crate::devtools::StructureItemId;

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
