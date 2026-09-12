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
}
