use bevy::prelude::*;

/// Runtime point of view used by debug presentation.
///
/// Domain collectors do not query cameras. They publish world-space observations;
/// rendering resolves billboards and observer-relative slices from this context.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct DebugContext {
    pub observer: Option<Entity>,
    pub selected_entity: Option<Entity>,
}
