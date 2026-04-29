use crate::bevy::prelude::*;

/// Generic field kinds that orchestration-capable systems may operate on.
///
/// This is intentionally broader than USF transform fields so the same protocol
/// can be reused by other subsystems later.
#[derive(Clone, Copy, Debug, Default, Reflect, PartialEq, Eq)]
pub enum OrchestrationFieldKind {
    Translation,
    Rotation,
    Scale,
    #[default]
    Other,
}

/// Cross-subsystem pressure state for runtime orchestration.
#[derive(Clone, Copy, Debug, Default, Reflect, PartialEq, Eq)]
pub enum OrchestrationPressure {
    #[default]
    Open,
    BoundaryOverlap,
    TimeoutRecovery,
    ExternalBackpressure,
}

/// Shared app-level orchestration state, usable by any subsystem.
#[derive(Resource, Debug, Default, Reflect)]
#[reflect(Resource)]
pub struct AppOrchestrationState {
    pub pressure: OrchestrationPressure,
    pub active_batches: u64,
    pub active_retries: u64,
}

/// Generic orchestration signals for diagnostics and future policy handlers.
#[derive(Message, Clone, Debug, Reflect)]
pub enum AppOrchestrationSignal {
    PressureChanged {
        pressure: OrchestrationPressure,
        source: String,
        details: String,
    },
    BoundaryCommit {
        field: OrchestrationFieldKind,
        actor_key: String,
    },
}

/// Per-frame player-local motion intent gathered from input systems and applied
/// by a single authoritative transform resolver.
#[derive(Resource, Clone, Debug, Reflect)]
#[reflect(Resource)]
pub struct PlayerMotionIntent {
    pub translation_delta: Vec3,
    pub rotation_delta: Vec3,
}
impl Default for PlayerMotionIntent {
    fn default() -> Self {
        Self {
            translation_delta: Vec3::ZERO,
            rotation_delta: Vec3::ZERO,
        }
    }
}
impl PlayerMotionIntent {
    pub fn clear(&mut self) {
        self.translation_delta = Vec3::ZERO;
        self.rotation_delta = Vec3::ZERO;
    }

    pub fn has_motion(&self) -> bool {
        self.translation_delta != Vec3::ZERO || self.rotation_delta != Vec3::ZERO
    }
}
