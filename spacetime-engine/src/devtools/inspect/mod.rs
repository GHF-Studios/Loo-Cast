//! Compose inspection frames, metadata, typed models, and visitor traits.
//!
//! ## Module map
//!
//! - `frame`: Per-frame inspection snapshot and edit/action request protocol.
//! - `metadata`: UI-agnostic semantic metadata used by inspection derives and hosts.
//! - `model`: Semantic inspection values, fields, actions and sections.
//! - `registry`: Explicit type-erased registration of inspectable Rust types.
//! - `traits`: Structured inspection traversal contracts.
//!
//! Reexports here define the supported surface; child modules hold its implementation.
//!

mod frame;
mod metadata;
mod model;
mod registry;
mod traits;

pub(super) use frame::begin_inspection_frame;
pub use frame::{InspectActionRequest, InspectEditRequest, InspectionFrame};
pub use metadata::{
    InspectAccess, InspectActionId, InspectFieldId, InspectFieldMetadata, InspectNumberFormat,
    InspectNumberInput, InspectSectionId, InspectTypeMetadata, InspectUnit, InspectWidgetId,
};
pub use model::{InspectAction, InspectField, InspectSection, InspectValue};
pub use registry::{AppInspectExt, InspectTypeRegistration, InspectTypeRegistry};
pub use traits::{Inspect, InspectFieldVisitor, InspectFieldVisitorMut};
