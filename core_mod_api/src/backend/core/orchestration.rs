use crate::bevy::prelude::*;

/// Repository-wide execution phases for app-level orchestration.
///
/// Subsystems should prefer attaching systems to these sets instead of creating
/// ad-hoc `before/after` relationships across plugins.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
pub enum AppSet {
    /// Collect raw player/debug/editor input and convert it into local intent state.
    InputGather,
    /// Apply domain intent systems that mutate local simulation inputs.
    Intent,
    /// Run simulation-time systems that are not boundary commits.
    Simulation,
    /// Resolve USF boundary/pivot commits and authoritative local window transforms.
    BoundaryResolve,
    /// Chunk/world orchestration, including batch planning and workflow dispatch.
    ChunkOrchestration,
    /// Camera/follower systems that must react after boundary resolution.
    Camera,
    /// Visual proxy updates and presentation-layer syncing.
    Presentation,
    /// Diagnostics, overlays, and instrumentation hooks.
    Diagnostics,
}

pub(crate) fn configure_app_sets(app: &mut App) {
    app.configure_sets(
        Update,
        (
            AppSet::InputGather,
            AppSet::Intent,
            AppSet::Simulation,
            AppSet::BoundaryResolve,
            AppSet::ChunkOrchestration,
            AppSet::Camera,
            AppSet::Presentation,
            AppSet::Diagnostics,
        )
            .chain(),
    );

    app.configure_sets(
        PostUpdate,
        (
            AppSet::InputGather,
            AppSet::Intent,
            AppSet::Simulation,
            AppSet::BoundaryResolve,
            AppSet::ChunkOrchestration,
            AppSet::Camera,
            AppSet::Presentation,
            AppSet::Diagnostics,
        )
            .chain(),
    );
}
