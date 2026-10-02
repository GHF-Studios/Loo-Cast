//! Detached local debug camera and observer-policy adapter.
//!
//! #57 detached-debug-freecam-v2
//!
//! Freecam never becomes gameplay/interaction authority. It may optionally
//! become the *presentation observer* through `UsfViewObservationOverride`,
//! while the gameplay subject continues to own canonical simulation position,
//! collision, refinement and dense spatial/materialization demand.

use bevy::{
    input::mouse::AccumulatedMouseMotion,
    prelude::*,
};

use crate::{
    devtools::DeveloperScriptWorkbench,
    game::{
        control::LocalViewTarget,
        player::{
            Player, PlayerController,
            cursor::CursorCapture,
            input::{PlayerAction, PlayerInputBindings},
        },
    },
    input_focus::InputFocus,
    physics::topology::UsfRuntimeOwnershipQuery,
    spatial::{
        UsfPosition, UsfScaleLayer, UsfViewDemandMode, UsfViewDemandPolicy,
        UsfViewObservationOverride,
    },
};

use super::PlayerCamera;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FreecamControlPolicy {
    /// Freecam consumes local movement/look intent; the gameplay subject stays still.
    #[default]
    Exclusive,
    /// Freecam moves while ordinary gameplay input is also delivered to the subject.
    Passthrough,
}

impl FreecamControlPolicy {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Passthrough => "passthrough",
        }
    }

    pub(crate) fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "exclusive" | "detached" | "blocked" => Some(Self::Exclusive),
            "passthrough" | "shared" => Some(Self::Passthrough),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) enum FreecamProjectionPolicy {
    /// The detached camera is a real presentation observer. USF projection
    /// remains active and is re-anchored from the freecam's canonical position.
    #[default]
    Follow,
    /// Freeze the USF presentation observer where it last was while the local
    /// camera flies through already-realized local geometry.
    Frozen,
    /// Disable the dedicated USF projection pass; inspect only local/raw scene state.
    Disabled,
}

impl FreecamProjectionPolicy {
    pub(crate) const fn label(self) -> &'static str {
        match self {
            Self::Follow => "follow",
            Self::Frozen => "frozen",
            Self::Disabled => "disabled",
        }
    }

    pub(crate) fn parse(raw: &str) -> Option<Self> {
        match raw.to_ascii_lowercase().as_str() {
            "follow" | "observer" | "normal" => Some(Self::Follow),
            "frozen" | "freeze" => Some(Self::Frozen),
            "disabled" | "off" | "bypass" => Some(Self::Disabled),
            _ => None,
        }
    }
}

#[derive(Resource, Debug, Clone)]
pub(crate) struct DebugFreecam {
    enabled: bool,
    translation_speed_mps: f32,
    boost_multiplier: f32,
    control_policy: FreecamControlPolicy,
    projection_policy: FreecamProjectionPolicy,
    view_demand_mode: UsfViewDemandMode,
}

impl DebugFreecam {
    pub const DEFAULT_TRANSLATION_SPEED_MPS: f32 = 20.0;
    pub const DEFAULT_BOOST_MULTIPLIER: f32 = 8.0;

    pub(crate) const fn enabled(&self) -> bool {
        self.enabled
    }

    pub(crate) fn set_enabled(&mut self, enabled: bool) {
        self.enabled = enabled;
    }

    pub(crate) const fn translation_speed_mps(&self) -> f32 {
        self.translation_speed_mps
    }

    pub(crate) fn set_translation_speed_mps(&mut self, value: f32) -> Result<(), String> {
        if !value.is_finite() || value < 0.0 {
            return Err("freecam translation speed must be finite and non-negative".to_string());
        }
        self.translation_speed_mps = value;
        Ok(())
    }

    pub(crate) const fn boost_multiplier(&self) -> f32 {
        self.boost_multiplier
    }

    pub(crate) fn set_boost_multiplier(&mut self, value: f32) -> Result<(), String> {
        if !value.is_finite() || value <= 0.0 {
            return Err("freecam boost multiplier must be finite and > 0".to_string());
        }
        self.boost_multiplier = value;
        Ok(())
    }

    pub(crate) const fn control_policy(&self) -> FreecamControlPolicy {
        self.control_policy
    }

    pub(crate) fn set_control_policy(&mut self, policy: FreecamControlPolicy) {
        self.control_policy = policy;
    }

    pub(crate) const fn projection_policy(&self) -> FreecamProjectionPolicy {
        self.projection_policy
    }

    pub(crate) fn set_projection_policy(&mut self, policy: FreecamProjectionPolicy) {
        self.projection_policy = policy;
    }

    pub(crate) const fn view_demand_mode(&self) -> UsfViewDemandMode {
        self.view_demand_mode
    }

    pub(crate) fn set_view_demand_mode(&mut self, mode: UsfViewDemandMode) {
        self.view_demand_mode = mode;
    }

    pub(crate) const fn consumes_gameplay_input(&self) -> bool {
        matches!(self.control_policy, FreecamControlPolicy::Exclusive)
    }
}

impl Default for DebugFreecam {
    fn default() -> Self {
        Self {
            enabled: false,
            translation_speed_mps: Self::DEFAULT_TRANSLATION_SPEED_MPS,
            boost_multiplier: Self::DEFAULT_BOOST_MULTIPLIER,
            control_policy: FreecamControlPolicy::Exclusive,
            projection_policy: FreecamProjectionPolicy::Follow,
            // Freeze presentation demand by default so flying the camera does not
            // heal/replace the scene being inspected. Dense physical demand is
            // player-owned and is never transferred to freecam at all.
            view_demand_mode: UsfViewDemandMode::Frozen,
        }
    }
}

pub(in crate::game::player) fn update_freecam(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mouse_motion: Res<AccumulatedMouseMotion>,
    bindings: Res<PlayerInputBindings>,
    focus: Res<InputFocus>,
    capture: Res<CursorCapture>,
    controller: Single<&PlayerController, With<Player>>,
    view_target: Single<&UsfScaleLayer, With<LocalViewTarget>>,
    settings: Res<DebugFreecam>,
    script_workbench: Res<DeveloperScriptWorkbench>,
    mut camera: Single<&mut Transform, With<PlayerCamera>>,
) {
    if !settings.enabled()
        || !capture.active()
        || focus.pointer_claimed()
        || focus.gameplay_claimed()
    {
        return;
    }

    let controller = controller.into_inner();
    let layer = view_target.into_inner();

    let look = mouse_motion.delta;
    if look != Vec2::ZERO {
        let yaw = -look.x * controller.look_sensitivity;
        let pitch = -look.y * controller.look_sensitivity;
        camera.rotation =
            (Quat::from_rotation_y(yaw) * camera.rotation * Quat::from_rotation_x(pitch))
                .normalize();
    }

    let pressed = |action| bindings.pressed_raw(action, &keyboard, &mouse);
    let forward = (
        pressed(PlayerAction::MoveForward) as i8
            - pressed(PlayerAction::MoveBackward) as i8
    ) as f32;
    let right = (
        pressed(PlayerAction::MoveRight) as i8
            - pressed(PlayerAction::MoveLeft) as i8
    ) as f32;
    let vertical = (
        pressed(PlayerAction::Ascend) as i8
            - pressed(PlayerAction::Descend) as i8
    ) as f32;

    let local_direction = Vec3::new(right, vertical, -forward).normalize_or_zero();
    if local_direction == Vec3::ZERO {
        return;
    }

    let boost = if pressed(PlayerAction::FastModifier) {
        settings.boost_multiplier()
    } else {
        1.0
    };
    let authored_speed = f64::from(settings.translation_speed_mps());
    let scripted_speed = script_workbench
        .apply_live_scalar(authored_speed)
        .clamp(0.0, 1.0e9) as f32;
    let distance_metres =
        scripted_speed * boost * time.delta_secs().max(0.0);
    let distance_native = layer.scale().metres_to_native_f32(distance_metres);
    let rotation = camera.rotation;
    camera.translation += rotation * local_direction * distance_native;
}

/// Reconciles freecam with the generic observer/view-demand policy owned by #40.
///
/// The canonical gameplay subject and its `SpatialDemandSource` / refinement
/// components are never edited here. Follow mode derives a *view-only* canonical
/// observer position from the subject anchor plus the bounded freecam offset.
pub(in crate::game::player) fn sync_freecam_observer_policy(
    settings: Res<DebugFreecam>,
    runtime_ownership: UsfRuntimeOwnershipQuery,
    subject: Single<(Entity, &Transform, &UsfScaleLayer), With<LocalViewTarget>>,
    semantic_positions: Query<&UsfPosition>,
    camera: Single<&Transform, With<PlayerCamera>>,
    mut observation: ResMut<UsfViewObservationOverride>,
    mut demand_policy: ResMut<UsfViewDemandPolicy>,
    mut previous_observation: Local<Option<UsfViewObservationOverride>>,
    mut previous_demand_mode: Local<Option<UsfViewDemandMode>>,
) {
    if !settings.enabled() {
        if let Some(previous) = previous_observation.take() {
            *observation = previous;
        }
        if let Some(previous) = previous_demand_mode.take() {
            demand_policy.set_mode(previous);
        }
        return;
    }

    if previous_observation.is_none() {
        *previous_observation = Some(*observation);
    }
    if previous_demand_mode.is_none() {
        *previous_demand_mode = Some(demand_policy.mode());
    }
    demand_policy.set_mode(settings.view_demand_mode());

    if matches!(settings.projection_policy(), FreecamProjectionPolicy::Disabled) {
        observation.clear();
        return;
    }
    if matches!(settings.projection_policy(), FreecamProjectionPolicy::Frozen)
        && observation.current().is_some()
    {
        return;
    }

    let (subject_entity, subject_transform, layer) = subject.into_inner();
    let Some(semantic_entity) = runtime_ownership.semantic_of(subject_entity) else {
        warn!(?subject_entity, "freecam view target has no semantic owner");
        observation.clear();
        return;
    };
    let Ok(&subject_anchor) = semantic_positions.get(semantic_entity) else {
        warn!(?semantic_entity, "freecam semantic owner has no canonical position");
        observation.clear();
        return;
    };

    let runtime_anchor = camera.translation;
    let runtime_offset = runtime_anchor - subject_transform.translation;
    match subject_anchor.translated_at_scale(layer.scale(), runtime_offset) {
        Ok(anchor) => observation.set(anchor, runtime_anchor),
        Err(error) => {
            warn!(?error, "freecam observer offset could not enter canonical USF space");
            observation.clear();
        }
    }
}
