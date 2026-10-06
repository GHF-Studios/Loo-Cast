//! Generic pacing for disposable/reconstructible main-thread work.

use bevy::prelude::*;
use std::time::Instant;

const WORK_CLASS_COUNT: usize = 3;
const DEFAULT_TARGET_FRAME_SECONDS: f64 = 1.0 / 60.0;
const DEFAULT_RESERVED_SECONDS: f64 = 0.003;
const MINIMUM_DISCRETIONARY_SECONDS: f64 = 0.001;
const MINIMUM_PREDICTED_UNIT_SECONDS: f64 = 0.000_025;
const EWMA_ALPHA: f64 = 0.20;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ReconstructibleWorkClass {
    Planning,
    Publication,
    Maintenance,
}
impl ReconstructibleWorkClass {
    const fn index(self) -> usize {
        match self {
            Self::Planning => 0,
            Self::Publication => 1,
            Self::Maintenance => 2,
        }
    }
}

#[derive(Resource, Debug, Clone, Copy)]
pub struct ReconstructibleWorkPolicy {
    target_frame_seconds: f64,
    reserved_seconds: f64,
}
impl Default for ReconstructibleWorkPolicy {
    fn default() -> Self {
        Self {
            target_frame_seconds: DEFAULT_TARGET_FRAME_SECONDS,
            reserved_seconds: DEFAULT_RESERVED_SECONDS,
        }
    }
}
impl ReconstructibleWorkPolicy {
    pub fn for_target_hz(hz: f64) -> Self {
        let hz = if hz.is_finite() && hz > 1.0 { hz } else { 60.0 };
        Self {
            target_frame_seconds: 1.0 / hz,
            ..default()
        }
    }
    pub fn with_reserved_seconds(mut self, seconds: f64) -> Self {
        if seconds.is_finite() && seconds >= 0.0 {
            self.reserved_seconds = seconds;
        }
        self
    }
    pub const fn target_frame_seconds(self) -> f64 {
        self.target_frame_seconds
    }
}

#[derive(Debug, Clone, Copy)]
struct WorkClassState {
    predicted_seconds: f64,
    started_units: u32,
}
impl Default for WorkClassState {
    fn default() -> Self {
        Self {
            predicted_seconds: 0.000_25,
            started_units: 0,
        }
    }
}

#[derive(Resource, Debug)]
pub struct ReconstructibleFrameBudget {
    frame_started: Instant,
    previous_frame_started: Instant,
    allowance_seconds: f64,
    overshoot_debt_seconds: f64,
    classes: [WorkClassState; WORK_CLASS_COUNT],
}
impl Default for ReconstructibleFrameBudget {
    fn default() -> Self {
        let now = Instant::now();
        Self {
            frame_started: now,
            previous_frame_started: now,
            allowance_seconds: DEFAULT_TARGET_FRAME_SECONDS - DEFAULT_RESERVED_SECONDS,
            overshoot_debt_seconds: 0.0,
            classes: [WorkClassState::default(); WORK_CLASS_COUNT],
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ReconstructibleWorkToken {
    class: ReconstructibleWorkClass,
    started: Instant,
}

impl ReconstructibleFrameBudget {
    fn begin_frame(&mut self, policy: ReconstructibleWorkPolicy) {
        let now = Instant::now();
        let previous = now
            .duration_since(self.previous_frame_started)
            .as_secs_f64();
        self.previous_frame_started = now;
        self.frame_started = now;

        let target = policy
            .target_frame_seconds
            .max(MINIMUM_DISCRETIONARY_SECONDS);
        let overshoot = (previous - target).max(0.0);
        let slack = (target - previous).max(0.0);
        self.overshoot_debt_seconds =
            (self.overshoot_debt_seconds * 0.75 + overshoot * 0.50 - slack * 0.10).max(0.0);

        let base = (target - policy.reserved_seconds).max(MINIMUM_DISCRETIONARY_SECONDS);
        let max_repayment = (base - MINIMUM_DISCRETIONARY_SECONDS).max(0.0);
        let repayment = (self.overshoot_debt_seconds * 0.50).min(max_repayment);
        self.allowance_seconds = (base - repayment).max(MINIMUM_DISCRETIONARY_SECONDS);
        for class in &mut self.classes {
            class.started_units = 0;
        }
    }

    pub fn begin(&mut self, class: ReconstructibleWorkClass) -> Option<ReconstructibleWorkToken> {
        let state = self.classes[class.index()];
        let elapsed = self.frame_started.elapsed().as_secs_f64();
        let predicted = state.predicted_seconds.max(MINIMUM_PREDICTED_UNIT_SECONDS);
        if state.started_units > 0 && elapsed + predicted > self.allowance_seconds {
            return None;
        }
        self.classes[class.index()].started_units =
            self.classes[class.index()].started_units.saturating_add(1);
        Some(ReconstructibleWorkToken {
            class,
            started: Instant::now(),
        })
    }

    pub fn finish(&mut self, token: ReconstructibleWorkToken) {
        let sample = token.started.elapsed().as_secs_f64();
        if !sample.is_finite() {
            return;
        }
        let state = &mut self.classes[token.class.index()];
        state.predicted_seconds += (sample - state.predicted_seconds) * EWMA_ALPHA;
    }

    /// Run one synchronous reconstructible work unit when the frame budget admits it.
    pub fn try_run<T>(
        &mut self,
        class: ReconstructibleWorkClass,
        work: impl FnOnce() -> T,
    ) -> Option<T> {
        let token = self.begin(class)?;
        let result = work();
        self.finish(token);
        Some(result)
    }

    pub fn predicted_unit_seconds(&self, class: ReconstructibleWorkClass) -> f64 {
        self.classes[class.index()].predicted_seconds
    }
}

fn begin_reconstructible_frame(
    policy: Res<ReconstructibleWorkPolicy>,
    mut budget: ResMut<ReconstructibleFrameBudget>,
) {
    budget.begin_frame(*policy);
}

pub struct ReconstructibleWorkPlugin;
impl Plugin for ReconstructibleWorkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReconstructibleWorkPolicy>()
            .init_resource::<ReconstructibleFrameBudget>()
            .add_systems(First, begin_reconstructible_frame);
    }
}
