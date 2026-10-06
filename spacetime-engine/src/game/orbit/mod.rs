//! Canonical orbital mechanics shared by simulation, prediction and presentation.
use crate::{
    physics::gravity::RadialGravitySource,
    spatial::{UsfCanonicalMotion, UsfMotionAuthority, UsfPosition},
};
use bevy::{app::RunFixedMainLoop, math::DVec3, prelude::*, time::Virtual};
use std::collections::HashMap;
const EPS: f64 = 1.0e-10;
const ITERS: usize = 16;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrbitalStateVector {
    pub position_metres: DVec3,
    pub velocity_metres_per_second: DVec3,
}
impl OrbitalStateVector {
    pub const fn new(position_metres: DVec3, velocity_metres_per_second: DVec3) -> Self {
        Self {
            position_metres,
            velocity_metres_per_second,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct KeplerianElements {
    pub semi_major_axis_metres: f64,
    pub eccentricity: f64,
    pub inclination_radians: f64,
    pub longitude_ascending_node_radians: f64,
    pub argument_periapsis_radians: f64,
    pub mean_anomaly_at_epoch_radians: f64,
    pub epoch_seconds: f64,
}
impl KeplerianElements {
    pub const fn new(a: f64, e: f64, i: f64, lan: f64, arg: f64, m: f64, epoch: f64) -> Self {
        Self {
            semi_major_axis_metres: a,
            eccentricity: e,
            inclination_radians: i,
            longitude_ascending_node_radians: lan,
            argument_periapsis_radians: arg,
            mean_anomaly_at_epoch_radians: m,
            epoch_seconds: epoch,
        }
    }
    pub fn from_state(state: OrbitalStateVector, mu: f64, epoch_seconds: f64) -> Option<Self> {
        let r = state.position_metres;
        let v = state.velocity_metres_per_second;
        if !r.is_finite() || !v.is_finite() || !mu.is_finite() || mu <= 0.0 {
            return None;
        }
        let radius = r.length();
        if radius <= EPS {
            return None;
        }
        let h = r.cross(v);
        let hl = h.length();
        if hl <= EPS {
            return None;
        }
        let hh = h / hl;
        let ev = v.cross(h) / mu - r / radius;
        let e = ev.length();
        if !e.is_finite() {
            return None;
        }
        let energy = 0.5 * v.length_squared() - mu / radius;
        let a = if energy.abs() > EPS {
            -mu / (2.0 * energy)
        } else {
            f64::INFINITY
        };
        let node = DVec3::Y.cross(h);
        let nl = node.length();
        let nh = if nl > EPS { node / nl } else { DVec3::X };
        let i = hh.y.clamp(-1.0, 1.0).acos();
        let lan = nh.z.atan2(nh.x).rem_euclid(std::f64::consts::TAU);
        let ph = if e > EPS { ev / e } else { nh };
        let arg = signed_angle(nh, ph, hh);
        let nu = signed_angle(ph, r / radius, hh);
        let m = if e < 1.0 { true_to_mean(nu, e) } else { 0.0 };
        Some(Self::new(a, e, i, lan, arg, m, epoch_seconds))
    }
    pub fn is_bound(self) -> bool {
        self.semi_major_axis_metres.is_finite()
            && self.semi_major_axis_metres > 0.0
            && self.eccentricity >= 0.0
            && self.eccentricity < 1.0
    }
    pub fn periapsis_radius_metres(self, mu: f64, state: OrbitalStateVector) -> Option<f64> {
        let h2 = state
            .position_metres
            .cross(state.velocity_metres_per_second)
            .length_squared();
        (h2.is_finite() && mu.is_finite() && mu > 0.0)
            .then_some(h2 / (mu * (1.0 + self.eccentricity)))
    }
    pub fn apoapsis_radius_metres(self) -> Option<f64> {
        self.is_bound()
            .then_some(self.semi_major_axis_metres * (1.0 + self.eccentricity))
    }
    pub fn mean_motion_radians_per_second(self, mu: f64) -> Option<f64> {
        if !self.is_bound() || !mu.is_finite() || mu <= 0.0 {
            return None;
        }
        Some((mu / self.semi_major_axis_metres.powi(3)).sqrt())
    }
    pub fn period_seconds(self, mu: f64) -> Option<f64> {
        Some(std::f64::consts::TAU / self.mean_motion_radians_per_second(mu)?)
    }
    pub fn time_to_periapsis_seconds(self, mu: f64) -> Option<f64> {
        let n = self.mean_motion_radians_per_second(mu)?;
        Some((-self.mean_anomaly_at_epoch_radians).rem_euclid(std::f64::consts::TAU) / n)
    }
    pub fn time_to_apoapsis_seconds(self, mu: f64) -> Option<f64> {
        let n = self.mean_motion_radians_per_second(mu)?;
        Some(
            (std::f64::consts::PI - self.mean_anomaly_at_epoch_radians)
                .rem_euclid(std::f64::consts::TAU)
                / n,
        )
    }
    pub fn state_at(self, epoch: f64, mu: f64) -> Option<OrbitalStateVector> {
        let n = self.mean_motion_radians_per_second(mu)?;
        let e = self.eccentricity;
        let m = (self.mean_anomaly_at_epoch_radians + n * (epoch - self.epoch_seconds))
            .rem_euclid(std::f64::consts::TAU);
        let ea = solve_e(m, e);
        let ce = ea.cos();
        let se = ea.sin();
        let d = 1.0 - e * ce;
        if d <= EPS {
            return None;
        }
        let a = self.semi_major_axis_metres;
        let beta = (1.0 - e * e).sqrt();
        let x = a * (ce - e);
        let y = a * beta * se;
        let vx = -a * n * se / d;
        let vy = a * n * beta * ce / d;
        let (p, q) = self.basis()?;
        let pos = p * x + q * y;
        let vel = p * vx + q * vy;
        (pos.is_finite() && vel.is_finite()).then_some(OrbitalStateVector::new(pos, vel))
    }
    fn basis(self) -> Option<(DVec3, DVec3)> {
        if !self.inclination_radians.is_finite()
            || !self.longitude_ascending_node_radians.is_finite()
            || !self.argument_periapsis_radians.is_finite()
        {
            return None;
        }
        let (sn, cn) = self.longitude_ascending_node_radians.sin_cos();
        let node = DVec3::new(cn, 0.0, sn);
        let (si, ci) = self.inclination_radians.sin_cos();
        let normal = (DVec3::Y * ci + node.cross(DVec3::Y) * si).normalize();
        let qn = normal.cross(node).normalize();
        let (sa, ca) = self.argument_periapsis_radians.sin_cos();
        let p = (node * ca + qn * sa).normalize();
        Some((p, normal.cross(p).normalize()))
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct KeplerianOrbitPropagation {
    primary: Entity,
    elements: KeplerianElements,
}
impl KeplerianOrbitPropagation {
    pub const fn new(primary: Entity, elements: KeplerianElements) -> Self {
        Self { primary, elements }
    }
    pub const fn primary(self) -> Entity {
        self.primary
    }
    pub const fn elements(self) -> KeplerianElements {
        self.elements
    }
}
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum OrbitalMechanicsSet {
    Propagate,
}
pub struct OrbitalMechanicsPlugin;
impl Plugin for OrbitalMechanicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            RunFixedMainLoop,
            propagate_keplerian_orbits.in_set(OrbitalMechanicsSet::Propagate),
        );
    }
}
fn propagate_keplerian_orbits(
    time: Res<Time<Virtual>>,
    mut state: ParamSet<(
        Query<(
            Entity,
            &UsfPosition,
            &RadialGravitySource,
            Option<&UsfCanonicalMotion>,
        )>,
        Query<(
            &KeplerianOrbitPropagation,
            &mut UsfPosition,
            &mut UsfCanonicalMotion,
        )>,
    )>,
) {
    let primaries = state
        .p0()
        .iter()
        .map(|(e, p, g, m)| {
            (
                e,
                (
                    *p,
                    *g,
                    m.map(|m| m.velocity_metres_per_second())
                        .unwrap_or(DVec3::ZERO),
                ),
            )
        })
        .collect::<HashMap<_, _>>();
    let epoch = time.elapsed_secs_f64();
    let mut orbiters = state.p1();
    for (o, mut p, mut m) in &mut orbiters {
        let Some((pp, g, pv)) = primaries.get(&o.primary()).copied() else {
            continue;
        };
        let Some(s) = o
            .elements()
            .state_at(epoch, g.gravitational_parameter_metres3_per_second2())
        else {
            continue;
        };
        let Ok(np) = pp.translated_metres_f64(s.position_metres) else {
            continue;
        };
        *p = np;
        m.set_authority(UsfMotionAuthority::CanonicalKinematics);
        m.set_velocity_metres_per_second(pv + s.velocity_metres_per_second);
        m.set_epoch_seconds(epoch);
    }
}
fn signed_angle(a: DVec3, b: DVec3, n: DVec3) -> f64 {
    n.dot(a.cross(b))
        .atan2(a.dot(b))
        .rem_euclid(std::f64::consts::TAU)
}
fn true_to_mean(nu: f64, e: f64) -> f64 {
    if e <= EPS {
        return nu.rem_euclid(std::f64::consts::TAU);
    }
    let h = nu * 0.5;
    let ea = 2.0 * ((1.0 - e).sqrt() * h.sin()).atan2((1.0 + e).sqrt() * h.cos());
    (ea - e * ea.sin()).rem_euclid(std::f64::consts::TAU)
}
fn solve_e(m: f64, e: f64) -> f64 {
    let mut x = if e < 0.8 { m } else { std::f64::consts::PI };
    for _ in 0..ITERS {
        let f = x - e * x.sin() - m;
        let d = 1.0 - e * x.cos();
        if d.abs() <= EPS {
            break;
        }
        let s = f / d;
        x -= s;
        if s.abs() <= 1.0e-13 {
            break;
        }
    }
    x
}
