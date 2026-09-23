use bevy::prelude::*;

const FRAME_EPSILON: f32 = 1.0e-6;

/// Persistent locomotion/gravity reference frame for a character.
///
/// Physical [`Transform::rotation`] may temporarily differ from this frame
/// while topology maps a manifestation through a portal. Gravity, jumping,
/// ground classification and player locomotion remain relative to `up` until
/// some explicit gameplay mechanism changes the frame itself.
#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct CharacterLocomotionFrame {
    pub up: Vec3,
}

impl Default for CharacterLocomotionFrame {
    fn default() -> Self {
        Self { up: Vec3::Y }
    }
}

impl CharacterLocomotionFrame {
    /// Returns a stable normalized up direction.
    pub fn up(&self) -> Vec3 {
        let up = self.up.normalize_or_zero();
        if up == Vec3::ZERO { Vec3::Y } else { up }
    }

    /// Removes pitch/roll from a physical pose relative to this frame while
    /// preserving as much of its heading as possible.
    ///
    /// This is used when a topology-specific temporary pose (for example a
    /// wall-to-floor portal mapping) finishes and the ordinary character body
    /// becomes authoritative again.
    pub fn aligned_rotation(&self, current: Quat) -> Quat {
        let up = self.up();
        let mut forward = reject(current * Vec3::NEG_Z, up).normalize_or_zero();

        if forward == Vec3::ZERO {
            let right = reject(current * Vec3::X, up).normalize_or_zero();
            if right != Vec3::ZERO {
                forward = up.cross(right).normalize_or_zero();
            }
        }

        if forward == Vec3::ZERO {
            forward = [Vec3::NEG_Z, Vec3::X, Vec3::Z, Vec3::NEG_X]
                .into_iter()
                .map(|axis| reject(axis, up).normalize_or_zero())
                .find(|axis| axis.length_squared() > FRAME_EPSILON)
                .unwrap_or(Vec3::NEG_Z);
        }

        let right = forward.cross(up).normalize_or_zero();
        if right == Vec3::ZERO {
            return Quat::IDENTITY;
        }
        let back = right.cross(up).normalize_or_zero();

        Quat::from_mat3(&Mat3::from_cols(right, up, back)).normalize()
    }
}

/// World-space basis used by view and locomotion input.
///
/// Unlike [`CharacterLocomotionFrame`], this frame is allowed to carry a
/// temporary topology-induced roll/pitch. `PlayerAim`-style local look input is
/// applied *after* this basis, so mouse input remains live while the basis
/// settles back toward the stable locomotion frame.
#[derive(Component, Reflect, Clone, Copy, Debug)]
#[reflect(Component)]
pub struct CharacterControlFrame {
    rotation: Quat,
    settle: Option<CharacterControlSettle>,
}

#[derive(Reflect, Clone, Copy, Debug)]
struct CharacterControlSettle {
    start: Quat,
    target: Quat,
    elapsed: f32,
    duration: f32,
    input_blend_duration: f32,
}

impl Default for CharacterControlFrame {
    fn default() -> Self {
        Self {
            rotation: Quat::IDENTITY,
            settle: None,
        }
    }
}

impl CharacterControlFrame {
    pub fn rotation(&self) -> Quat {
        self.rotation.normalize()
    }

    /// Immediately adopts one resolved control basis and cancels any transient settle.
    ///
    /// Use this at explicit pose transactions (for example vehicle exit), not
    /// for ordinary per-frame orientation response.
    pub fn snap_to(&mut self, rotation: Quat) {
        self.rotation = rotation.normalize();
        self.settle = None;
    }

    /// Starts a smooth basis transition without touching any local look state.
    pub fn begin_settle(
        &mut self,
        start: Quat,
        target: Quat,
        duration: f32,
        input_blend_duration: f32,
    ) {
        let start = start.normalize();
        let target = target.normalize();
        let duration = duration.max(0.0);

        self.rotation = start;
        if duration <= FRAME_EPSILON {
            self.snap_to(target);
            return;
        }

        self.settle = Some(CharacterControlSettle {
            start,
            target,
            elapsed: 0.0,
            duration,
            input_blend_duration: input_blend_duration.clamp(0.0, duration),
        });
    }

    /// Conforms the stable control basis to the current locomotion `up` while
    /// preserving as much tangent heading as possible.
    ///
    /// An active topology/portal settle remains authoritative until it finishes;
    /// planetary gravity therefore cannot stomp a transient mapped orientation.
    pub fn follow_locomotion_frame(&mut self, frame: &CharacterLocomotionFrame) {
        if self.settle.is_none() {
            self.rotation = frame.aligned_rotation(self.rotation);
        }
    }

    /// Existing momentum is never changed by this factor; callers use it only
    /// to soften freshly commanded locomotion while orientation is changing.
    pub fn movement_input_scale(&self) -> f32 {
        let Some(settle) = self.settle else {
            return 1.0;
        };
        if settle.input_blend_duration <= FRAME_EPSILON {
            return 1.0;
        }

        smoothstep((settle.elapsed / settle.input_blend_duration).clamp(0.0, 1.0))
    }

    fn tick(&mut self, dt: f32) {
        let Some(mut settle) = self.settle else {
            return;
        };

        settle.elapsed = (settle.elapsed + dt.max(0.0)).min(settle.duration);
        let t = smoothstep((settle.elapsed / settle.duration).clamp(0.0, 1.0));
        self.rotation = settle.start.slerp(settle.target, t).normalize();

        if settle.elapsed >= settle.duration - FRAME_EPSILON {
            self.rotation = settle.target;
            self.settle = None;
        } else {
            self.settle = Some(settle);
        }
    }
}

pub(super) fn settle_character_control_frames(
    time: Res<Time>,
    mut frames: Query<&mut CharacterControlFrame>,
) {
    for mut frame in &mut frames {
        frame.tick(time.delta_secs());
    }
}

fn smoothstep(t: f32) -> f32 {
    t * t * (3.0 - 2.0 * t)
}

fn reject(vector: Vec3, axis: Vec3) -> Vec3 {
    vector - axis * vector.dot(axis)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_frame_rebases_sideways_pose_upright() {
        let frame = CharacterLocomotionFrame::default();
        let sideways = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
        let rebased = frame.aligned_rotation(sideways);

        assert!((rebased * Vec3::Y - Vec3::Y).length() < 1.0e-5);
    }

    #[test]
    fn arbitrary_up_is_respected() {
        let frame = CharacterLocomotionFrame { up: Vec3::Z };
        let rebased = frame.aligned_rotation(Quat::IDENTITY);

        assert!((rebased * Vec3::Y - Vec3::Z).length() < 1.0e-5);
    }

    #[test]
    fn control_frame_follows_planetary_up_when_not_settling() {
        let frame = CharacterLocomotionFrame { up: Vec3::Z };
        let mut control = CharacterControlFrame::default();

        control.follow_locomotion_frame(&frame);

        assert!((control.rotation() * Vec3::Y - Vec3::Z).length() < 1.0e-5);
    }

    #[test]
    fn control_frame_settle_preserves_local_offset_contract() {
        let mut control = CharacterControlFrame::default();
        let start = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
        control.begin_settle(start, Quat::IDENTITY, 0.3, 0.1);

        assert!((control.rotation() * Vec3::Y - start * Vec3::Y).length() < 1.0e-5);
        assert_eq!(control.movement_input_scale(), 0.0);

        control.tick(0.15);
        assert!(control.movement_input_scale() > 0.999);
        assert!(control.rotation().dot(Quat::IDENTITY).abs() < 0.99999);

        control.tick(0.15);
        assert!(control.rotation().dot(Quat::IDENTITY).abs() > 0.99999);
    }
}
