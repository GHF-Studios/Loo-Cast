use crate::bevy::prelude::*;
use std::f64::consts::{PI, TAU};

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub enum UsfFloatDomain {
    Multiplicative { pivot_factor: f64 },
    Linear { wrap_size: f64 },
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub struct UsfFloatPolicy {
    pub local_min: f64,
    pub local_max: f64,
    pub commit_buffer_ratio: f64,
    pub domain: UsfFloatDomain,
}
impl UsfFloatPolicy {
    pub fn commit_min(&self) -> f64 {
        match self.domain {
            UsfFloatDomain::Multiplicative { .. } => self.local_min * (1.0 - self.commit_buffer_ratio),
            UsfFloatDomain::Linear { .. } => {
                let span = (self.local_max - self.local_min).abs();
                self.local_min - (span * self.commit_buffer_ratio)
            }
        }
    }

    pub fn commit_max(&self) -> f64 {
        match self.domain {
            UsfFloatDomain::Multiplicative { .. } => self.local_max * (1.0 + self.commit_buffer_ratio),
            UsfFloatDomain::Linear { .. } => {
                let span = (self.local_max - self.local_min).abs();
                self.local_max + (span * self.commit_buffer_ratio)
            }
        }
    }
}

#[derive(Default, Clone, Copy, Debug, Reflect, PartialEq, Eq)]
pub struct UsfFloatPivotResult {
    /// Number of crossings below the lower commit boundary.
    /// For scale this means "towards finer detail" (zoom-in transitions).
    pub lower_crossings: i32,
    /// Number of crossings above the upper commit boundary.
    /// For scale this means "towards coarser detail" (zoom-out transitions).
    pub upper_crossings: i32,
}

#[derive(Clone, Copy, Debug, Reflect, PartialEq)]
pub struct UsfFloat {
    pub local: f64,
    /// Canonical cycle accumulator.
    /// - Multiplicative domain: order-of-magnitude cycle count.
    /// - Linear domain: wrap cycle count.
    pub canonical_cycles: i64,
}
impl Default for UsfFloat {
    fn default() -> Self {
        Self {
            local: 0.0,
            canonical_cycles: 0,
        }
    }
}
impl UsfFloat {
    pub fn new(local: f64) -> Self {
        Self { local, canonical_cycles: 0 }
    }

    pub fn set_local(&mut self, local: f64) {
        self.local = local;
    }

    pub fn fold_with_policy(&mut self, policy: UsfFloatPolicy) -> UsfFloatPivotResult {
        let mut result = UsfFloatPivotResult::default();
        let commit_min = policy.commit_min();
        let commit_max = policy.commit_max();

        match policy.domain {
            UsfFloatDomain::Multiplicative { pivot_factor } => {
                assert!(pivot_factor > 1.0, "Multiplicative pivot factor must be > 1.0");
                assert!(policy.local_min > 0.0, "Multiplicative policy requires local_min > 0.0");
                assert!(self.local > 0.0, "Multiplicative policy requires positive local value");

                while self.local <= commit_min {
                    self.local *= pivot_factor;
                    self.canonical_cycles -= 1;
                    result.lower_crossings += 1;
                }

                while self.local >= commit_max {
                    self.local /= pivot_factor;
                    self.canonical_cycles += 1;
                    result.upper_crossings += 1;
                }
            }
            UsfFloatDomain::Linear { wrap_size } => {
                assert!(wrap_size > 0.0, "Linear wrap_size must be > 0.0");
                while self.local <= commit_min {
                    self.local += wrap_size;
                    self.canonical_cycles -= 1;
                    result.lower_crossings += 1;
                }

                while self.local >= commit_max {
                    self.local -= wrap_size;
                    self.canonical_cycles += 1;
                    result.upper_crossings += 1;
                }
            }
        }

        result
    }

    pub fn resolved_value(&self, policy: UsfFloatPolicy) -> f64 {
        match policy.domain {
            UsfFloatDomain::Multiplicative { pivot_factor } => self.local * pivot_factor.powf(self.canonical_cycles as f64),
            UsfFloatDomain::Linear { wrap_size } => self.local + (self.canonical_cycles as f64 * wrap_size),
        }
    }
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub struct UsfTranslation {
    pub x: UsfFloat,
    pub y: UsfFloat,
    pub z: UsfFloat,
    pub policy: UsfFloatPolicy,
}
impl Default for UsfTranslation {
    fn default() -> Self {
        Self {
            x: UsfFloat::default(),
            y: UsfFloat::default(),
            z: UsfFloat::default(),
            policy: UsfFloatPolicy {
                local_min: -500.0,
                local_max: 500.0,
                commit_buffer_ratio: 0.1,
                domain: UsfFloatDomain::Linear { wrap_size: 1000.0 },
            },
        }
    }
}
impl UsfTranslation {
    pub fn set_local(&mut self, value: Vec3) {
        self.x.set_local(value.x as f64);
        self.y.set_local(value.y as f64);
        self.z.set_local(value.z as f64);
    }

    pub fn local(&self) -> Vec3 {
        Vec3::new(self.x.local as f32, self.y.local as f32, self.z.local as f32)
    }

    pub fn fold(&mut self) -> IVec3 {
        let px = self.x.fold_with_policy(self.policy);
        let py = self.y.fold_with_policy(self.policy);
        let pz = self.z.fold_with_policy(self.policy);

        IVec3::new(
            px.upper_crossings - px.lower_crossings,
            py.upper_crossings - py.lower_crossings,
            pz.upper_crossings - pz.lower_crossings,
        )
    }
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub struct UsfScale {
    pub uniform: UsfFloat,
    pub policy: UsfFloatPolicy,
}
impl Default for UsfScale {
    fn default() -> Self {
        Self {
            uniform: UsfFloat::new(1.0),
            policy: UsfFloatPolicy {
                local_min: 0.1,
                local_max: 10.0,
                commit_buffer_ratio: 0.1,
                domain: UsfFloatDomain::Multiplicative { pivot_factor: 10.0 },
            },
        }
    }
}
impl UsfScale {
    pub fn configure_window(&mut self, local_min: f64, local_max: f64, commit_buffer_ratio: f64) {
        self.policy.local_min = local_min;
        self.policy.local_max = local_max;
        self.policy.commit_buffer_ratio = commit_buffer_ratio;
    }

    pub fn set_local(&mut self, local: f64) {
        self.uniform.set_local(local);
    }

    pub fn fold(&mut self) -> UsfFloatPivotResult {
        self.uniform.fold_with_policy(self.policy)
    }

    pub fn local_f32(&self) -> f32 {
        self.uniform.local as f32
    }
}

#[derive(Clone, Debug, Reflect, PartialEq)]
pub struct UsfRotation {
    pub x: UsfFloat,
    pub y: UsfFloat,
    pub z: UsfFloat,
    pub policy: UsfFloatPolicy,
}
impl Default for UsfRotation {
    fn default() -> Self {
        Self {
            x: UsfFloat::default(),
            y: UsfFloat::default(),
            z: UsfFloat::default(),
            policy: UsfFloatPolicy {
                local_min: -PI,
                local_max: PI,
                commit_buffer_ratio: 0.1,
                domain: UsfFloatDomain::Linear { wrap_size: TAU },
            },
        }
    }
}
impl UsfRotation {
    pub fn add_local_delta(&mut self, delta_radians_xyz: Vec3) {
        self.x.local += delta_radians_xyz.x as f64;
        self.y.local += delta_radians_xyz.y as f64;
        self.z.local += delta_radians_xyz.z as f64;
    }

    pub fn fold(&mut self) -> IVec3 {
        let px = self.x.fold_with_policy(self.policy);
        let py = self.y.fold_with_policy(self.policy);
        let pz = self.z.fold_with_policy(self.policy);

        IVec3::new(
            px.upper_crossings - px.lower_crossings,
            py.upper_crossings - py.lower_crossings,
            pz.upper_crossings - pz.lower_crossings,
        )
    }

    pub fn local_quat(&self) -> Quat {
        Quat::from_euler(EulerRot::XYZ, self.x.local as f32, self.y.local as f32, self.z.local as f32)
    }
}

#[derive(Clone, Debug, Reflect, PartialEq, Default)]
pub struct UsfTransform {
    pub translation: UsfTranslation,
    pub rotation: UsfRotation,
    pub scale: UsfScale,
}
