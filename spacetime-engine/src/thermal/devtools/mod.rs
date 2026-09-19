//! Structured semantic inspection and contextual gizmo support for thermal state.
//!
//! Thermal is intentionally the second proof case after Transform: most runtime
//! facts are observational, selected parameters use validated domain-owned edits,
//! and heat/cool/reset are explicit actions rather than fake mutable fields.

use bevy::prelude::*;

use crate::{
    devtools::{
        DeveloperFocus, DeveloperSet, DrawDepth, InspectAccess, InspectAction, InspectActionId,
        InspectActionRequest, InspectEditRequest, InspectField, InspectFieldId,
        InspectNumberFormat, InspectNumberInput, InspectSection, InspectSectionId, InspectUnit,
        InspectValue, InspectionFrame, StructureFrame, StructureItem, StructureItemId,
        StructureSelection, WorldDrawBatch, WorldDrawFrame,
    },
    ecs::UsfManifestations,
    view::PrimaryViewPresentation,
};

use super::{
    AMBIENT_TEMPERATURE_KELVIN, CombustibleMaterial, Combustion, Fuel, ThermalBody, ThermalField,
    ThermalImpulse, ThermalMaterial, ThermalSpatialSample,
};

const THERMAL_SECTION: InspectSectionId = InspectSectionId("thermal");
const MATERIAL_SECTION: InspectSectionId = InspectSectionId("thermal.material");
const COMBUSTION_SECTION: InspectSectionId = InspectSectionId("thermal.combustion");
pub(crate) const THERMAL_STRUCTURE: StructureItemId = StructureItemId("thermal.state");

const HEAT_CAPACITY_FIELD: InspectFieldId = InspectFieldId("thermal.body.heat_capacity");
const COOLING_FIELD: InspectFieldId = InspectFieldId("thermal.body.cooling");
const IGNITION_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.ignition");
const EXTINCTION_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.extinction");
const BURN_POWER_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.power");
const SELF_HEATING_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.self_heating");
const ENVIRONMENTAL_TRANSFER_FIELD: InspectFieldId =
    InspectFieldId("thermal.combustion.environmental_transfer");
const TRANSFER_RADIUS_FIELD: InspectFieldId = InspectFieldId("thermal.combustion.transfer_radius");

const HEAT_ACTION: InspectActionId = InspectActionId("thermal.heat");
const COOL_ACTION: InspectActionId = InspectActionId("thermal.cool");
const RESET_AMBIENT_ACTION: InspectActionId = InspectActionId("thermal.reset_ambient");
const THERMAL_NUDGE_JOULES: f32 = 10_000.0;

mod commit;
mod gizmo;
mod inspection;

use commit::{apply_thermal_inspection_actions, apply_thermal_inspection_edits};
use gizmo::collect_focused_thermal_gizmo;
use inspection::{collect_thermal_inspection, collect_thermal_structure};

pub(crate) fn configure(app: &mut App) {
    app.add_systems(
        PreUpdate,
        (
            apply_thermal_inspection_edits,
            apply_thermal_inspection_actions,
        ),
    )
    .add_systems(
        PostUpdate,
        collect_thermal_structure.in_set(DeveloperSet::CollectStructure),
    )
    .add_systems(
        PostUpdate,
        collect_thermal_inspection.in_set(DeveloperSet::CollectInspection),
    )
    .add_systems(
        PostUpdate,
        collect_focused_thermal_gizmo.in_set(DeveloperSet::CollectWorldDraw),
    );
}
