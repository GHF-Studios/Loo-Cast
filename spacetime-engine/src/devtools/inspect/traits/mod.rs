//! Structured inspection traversal contracts.

use std::any::Any;

use super::metadata::{InspectAccess, InspectFieldMetadata, InspectTypeMetadata};

/// UI-agnostic field traversal for inspectable values.
///
/// The derive emits this traversal. Presentation hosts choose how to render the
/// values and whether they possess a legal edit capability.
pub trait InspectFieldVisitor {
    fn field(&mut self, metadata: &'static InspectFieldMetadata, value: &dyn Any);
}

/// Mutable traversal deliberately exposes `&mut` only for fields whose metadata
/// declares [`InspectAccess::Direct`]. Validated/transactional/command fields are
/// visited through `read_only` and require an explicit adapter/request path.
pub trait InspectFieldVisitorMut {
    fn read_only(&mut self, metadata: &'static InspectFieldMetadata, value: &dyn Any);
    fn direct(&mut self, metadata: &'static InspectFieldMetadata, value: &mut dyn Any);
}

/// Type-level opt-in for reusable structured inspection.
///
/// `#[derive(Inspect)]` implements this trait for ordinary named-field structs.
/// Manual implementations remain a first-class path for computed properties,
/// setters, polymorphism, collections, or any type whose semantics are richer
/// than stored fields.
pub trait Inspect: Any {
    fn inspect_type_metadata() -> &'static InspectTypeMetadata
    where
        Self: Sized;

    fn visit_inspect_fields(&self, visitor: &mut dyn InspectFieldVisitor);
    fn visit_inspect_fields_mut(&mut self, visitor: &mut dyn InspectFieldVisitorMut);
}
