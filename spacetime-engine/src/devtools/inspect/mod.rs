mod frame;
mod metadata;
mod model;
mod registry;
mod traits;

pub use frame::{InspectActionRequest, InspectEditRequest, InspectionFrame};
pub(super) use frame::clear_inspection_frame;
pub use metadata::{
    InspectAccess, InspectActionId, InspectFieldId, InspectFieldMetadata,
    InspectNumberFormat, InspectNumberInput, InspectSectionId, InspectTypeMetadata,
    InspectUnit, InspectWidgetId,
};
pub use model::{InspectAction, InspectField, InspectSection, InspectValue};
pub use registry::{AppInspectExt, InspectTypeRegistration, InspectTypeRegistry};
pub use traits::{Inspect, InspectFieldVisitor, InspectFieldVisitorMut};

#[cfg(test)]
mod tests;
