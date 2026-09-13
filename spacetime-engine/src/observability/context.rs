use bevy::prelude::*;

/// One concrete thing currently under the developer's look ray.
///
/// `entity` is the spatial/concrete entity that was actually hit. For USF
/// manifestations, `semantic_entity` resolves through `UsfManifestationOf`;
/// for ordinary entities (including portals) the two identities are equal.
#[derive(Debug, Clone, Copy)]
pub struct DebugSelection {
    pub entity: Entity,
    pub semantic_entity: Entity,
    pub world_position: Vec3,
    pub distance_meters: f32,
}

/// Runtime point of view used by debug presentation.
///
/// Domain collectors do not query cameras. They publish observations against
/// this context; game adapters decide what observer and concrete thing are in
/// focus.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub struct DebugContext {
    pub observer: Option<Entity>,
    /// Compatibility view of the selected semantic identity.
    pub selected_entity: Option<Entity>,
    pub selection: Option<DebugSelection>,
}

impl DebugContext {
    pub fn clear_selection(&mut self) {
        self.selected_entity = None;
        self.selection = None;
    }

    pub fn set_selection(&mut self, selection: DebugSelection) {
        self.selected_entity = Some(selection.semantic_entity);
        self.selection = Some(selection);
    }
}
