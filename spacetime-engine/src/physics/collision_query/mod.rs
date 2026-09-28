//! Canonical swept-collision query contracts.
//!
//! High-speed USF collision is not a sequence of scale-local impacts. A single
//! canonical swept interval may be conservatively tested and progressively
//! refined by many Scale Slices, but none of those query representations owns a
//! physical response merely because it reported a candidate.
//!
//! Local [`crate::spatial::UsfScaleRoleMask::COLLISION`] realizations remain the
//! ordinary Avian/contact backend. `COLLISION_QUERY` is deliberately separate:
//! it exists to answer conservative future-motion questions without becoming an
//! impulse authority.

use bevy::{math::DVec3, prelude::*};

use crate::spatial::{SpatialScale, UsfPosition, UsfPositionError};

/// Policy demand for conservative swept-collision query support.
///
/// This component does not itself select voxel chunks or a Scale Slice. Query
/// providers consume it together with canonical position/motion and decide what
/// sparse hierarchy/caches are required to meet the requested error bound.
///
/// `lookahead_seconds` may exceed one fixed tick so residency can prepare ahead
/// of motion. The actual collision transaction still resolves one concrete
/// [`UsfCanonicalSweep`] at a time.
#[derive(Component, Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionQueryDemand {
    lookahead_seconds: f64,
    bounding_radius_metres: f64,
    target_error_metres: f64,
}

impl UsfCollisionQueryDemand {
    pub fn new(
        lookahead_seconds: f64,
        bounding_radius_metres: f64,
        target_error_metres: f64,
    ) -> Self {
        assert!(
            lookahead_seconds.is_finite() && lookahead_seconds > 0.0,
            "collision-query lookahead must be finite and positive"
        );
        assert!(
            bounding_radius_metres.is_finite() && bounding_radius_metres >= 0.0,
            "collision-query bounding radius must be finite and non-negative"
        );
        assert!(
            target_error_metres.is_finite() && target_error_metres > 0.0,
            "collision-query target error must be finite and positive"
        );

        Self {
            lookahead_seconds,
            bounding_radius_metres,
            target_error_metres,
        }
    }

    pub const fn lookahead_seconds(self) -> f64 {
        self.lookahead_seconds
    }

    pub const fn bounding_radius_metres(self) -> f64 {
        self.bounding_radius_metres
    }

    pub const fn target_error_metres(self) -> f64 {
        self.target_error_metres
    }
}

/// One authoritative physical-motion segment to test for collision.
///
/// `displacement_metres` is semantic SI displacement over `duration_seconds`.
/// The sweep is independent from whichever bounded Scale Slice/backend happens
/// to answer it.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCanonicalSweep {
    subject: Entity,
    start: UsfPosition,
    displacement_metres: DVec3,
    duration_seconds: f64,
    bounding_radius_metres: f64,
}

impl UsfCanonicalSweep {
    pub fn new(
        subject: Entity,
        start: UsfPosition,
        displacement_metres: DVec3,
        duration_seconds: f64,
        bounding_radius_metres: f64,
    ) -> Self {
        assert!(
            displacement_metres.is_finite(),
            "canonical collision sweep displacement must be finite"
        );
        assert!(
            duration_seconds.is_finite() && duration_seconds > 0.0,
            "canonical collision sweep duration must be finite and positive"
        );
        assert!(
            bounding_radius_metres.is_finite() && bounding_radius_metres >= 0.0,
            "canonical collision sweep bounding radius must be finite and non-negative"
        );

        Self {
            subject,
            start,
            displacement_metres,
            duration_seconds,
            bounding_radius_metres,
        }
    }

    pub const fn subject(self) -> Entity {
        self.subject
    }

    pub const fn start(self) -> UsfPosition {
        self.start
    }

    pub const fn displacement_metres(self) -> DVec3 {
        self.displacement_metres
    }

    pub const fn duration_seconds(self) -> f64 {
        self.duration_seconds
    }

    pub const fn bounding_radius_metres(self) -> f64 {
        self.bounding_radius_metres
    }

    pub fn velocity_metres_per_second(self) -> DVec3 {
        self.displacement_metres / self.duration_seconds
    }

    pub fn end(self) -> Result<UsfPosition, UsfPositionError> {
        self.start.translated_metres_f64(self.displacement_metres)
    }

    pub fn position_at(self, fraction: f64) -> Result<UsfPosition, UsfPositionError> {
        let fraction = fraction.clamp(0.0, 1.0);
        self.start
            .translated_metres_f64(self.displacement_metres * fraction)
    }
}

/// Closed normalized interval over one [`UsfCanonicalSweep`].
///
/// `0` is the sweep start and `1` is the requested end. Coarse providers should
/// conservatively *contain* the true impact fraction; refinement narrows this
/// interval rather than applying a response.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfSweepInterval {
    minimum: f64,
    maximum: f64,
}

impl UsfSweepInterval {
    pub const WHOLE: Self = Self {
        minimum: 0.0,
        maximum: 1.0,
    };

    pub fn new(minimum: f64, maximum: f64) -> Option<Self> {
        if !minimum.is_finite()
            || !maximum.is_finite()
            || minimum < 0.0
            || maximum > 1.0
            || minimum > maximum
        {
            return None;
        }
        Some(Self { minimum, maximum })
    }

    pub const fn minimum(self) -> f64 {
        self.minimum
    }

    pub const fn maximum(self) -> f64 {
        self.maximum
    }

    pub fn width(self) -> f64 {
        self.maximum - self.minimum
    }
}

/// Conservative collision possibility emitted by one query representation.
///
/// This is *evidence*, never response authority. Many candidates at different
/// scales may describe the same physical encounter.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionCandidate {
    authority: Entity,
    scale: SpatialScale,
    interval: UsfSweepInterval,
    error_bound_metres: f64,
}

impl UsfCollisionCandidate {
    pub fn new(
        authority: Entity,
        scale: SpatialScale,
        interval: UsfSweepInterval,
        error_bound_metres: f64,
    ) -> Self {
        assert!(
            error_bound_metres.is_finite() && error_bound_metres >= 0.0,
            "collision candidate error bound must be finite and non-negative"
        );
        Self {
            authority,
            scale,
            interval,
            error_bound_metres,
        }
    }

    pub const fn authority(self) -> Entity {
        self.authority
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn interval(self) -> UsfSweepInterval {
        self.interval
    }

    pub const fn error_bound_metres(self) -> f64 {
        self.error_bound_metres
    }
}

/// Refined geometric result for one canonical sweep.
///
/// A resolution is still only a query result. A higher-level collision episode
/// transaction decides whether to accept it and is the *only* layer allowed to
/// mutate canonical motion.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionResolution {
    authority: Entity,
    scale: SpatialScale,
    toi_fraction: f64,
    contact_position: UsfPosition,
    outward_normal: DVec3,
    error_bound_metres: f64,
}

impl UsfCollisionResolution {
    pub fn new(
        authority: Entity,
        scale: SpatialScale,
        toi_fraction: f64,
        contact_position: UsfPosition,
        outward_normal: DVec3,
        error_bound_metres: f64,
    ) -> Self {
        assert!(
            toi_fraction.is_finite() && (0.0..=1.0).contains(&toi_fraction),
            "collision resolution TOI must be a finite sweep fraction"
        );
        assert!(
            outward_normal.is_finite() && outward_normal.length_squared() > 0.0,
            "collision resolution normal must be finite and non-zero"
        );
        assert!(
            error_bound_metres.is_finite() && error_bound_metres >= 0.0,
            "collision resolution error bound must be finite and non-negative"
        );

        Self {
            authority,
            scale,
            toi_fraction,
            contact_position,
            outward_normal: outward_normal.normalize(),
            error_bound_metres,
        }
    }

    pub const fn authority(self) -> Entity {
        self.authority
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn toi_fraction(self) -> f64 {
        self.toi_fraction
    }

    pub const fn contact_position(self) -> UsfPosition {
        self.contact_position
    }

    pub const fn outward_normal(self) -> DVec3 {
        self.outward_normal
    }

    pub const fn error_bound_metres(self) -> f64 {
        self.error_bound_metres
    }
}
/// Stable identifier for one request inside a collision-query frame.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UsfCollisionQueryRequestId {
    frame_revision: u64,
    index: u32,
}

impl UsfCollisionQueryRequestId {
    pub const fn frame_revision(self) -> u64 {
        self.frame_revision
    }

    pub const fn index(self) -> u32 {
        self.index
    }
}

/// One canonical sweep submitted to capability query providers.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionQueryRequest {
    id: UsfCollisionQueryRequestId,
    sweep: UsfCanonicalSweep,
    target_error_metres: f64,
}

impl UsfCollisionQueryRequest {
    pub const fn id(self) -> UsfCollisionQueryRequestId {
        self.id
    }

    pub const fn sweep(self) -> UsfCanonicalSweep {
        self.sweep
    }

    pub const fn target_error_metres(self) -> f64 {
        self.target_error_metres
    }
}

/// One provider candidate associated with its canonical sweep request.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfCollisionCandidateRecord {
    request: UsfCollisionQueryRequestId,
    candidate: UsfCollisionCandidate,
}

impl UsfCollisionCandidateRecord {
    pub const fn request(self) -> UsfCollisionQueryRequestId {
        self.request
    }

    pub const fn candidate(self) -> UsfCollisionCandidate {
        self.candidate
    }
}

/// Per-frame collision-query transaction.
///
/// This is deliberately an observation/query surface today. Motion executors do
/// not consume it yet. The eventual transaction will become:
///
/// `prepare canonical motion -> providers/refinement -> accept one result -> commit`.
#[derive(Resource, Debug, Default)]
pub struct UsfCollisionQueryFrame {
    revision: u64,
    requests: Vec<UsfCollisionQueryRequest>,
    candidates: Vec<UsfCollisionCandidateRecord>,
}

impl UsfCollisionQueryFrame {
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn requests(
        &self,
    ) -> impl ExactSizeIterator<Item = UsfCollisionQueryRequest> + '_ {
        self.requests.iter().copied()
    }

    pub fn candidates(
        &self,
    ) -> impl ExactSizeIterator<Item = UsfCollisionCandidateRecord> + '_ {
        self.candidates.iter().copied()
    }

    pub fn candidates_for(
        &self,
        request: UsfCollisionQueryRequestId,
    ) -> impl Iterator<Item = UsfCollisionCandidate> + '_ {
        self.candidates
            .iter()
            .copied()
            .filter(move |record| record.request == request)
            .map(UsfCollisionCandidateRecord::candidate)
    }

    pub fn push_candidate(
        &mut self,
        request: UsfCollisionQueryRequestId,
        candidate: UsfCollisionCandidate,
    ) {
        if request.frame_revision != self.revision {
            return;
        }
        self.candidates.push(UsfCollisionCandidateRecord {
            request,
            candidate,
        });
    }

    fn begin_frame(&mut self) {
        self.revision = self.revision.wrapping_add(1).max(1);
        self.requests.clear();
        self.candidates.clear();
    }

    fn push_request(
        &mut self,
        sweep: UsfCanonicalSweep,
        target_error_metres: f64,
    ) {
        let index = u32::try_from(self.requests.len())
            .expect("collision-query request count exceeded u32");
        let id = UsfCollisionQueryRequestId {
            frame_revision: self.revision,
            index,
        };
        self.requests.push(UsfCollisionQueryRequest {
            id,
            sweep,
            target_error_metres,
        });
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfCollisionQuerySet {
    Collect,
    Providers,
    Finalize,
}

fn collect_shadow_collision_sweeps(
    fixed_time: Res<Time<Fixed>>,
    ownership: crate::ecs::UsfOwnershipQuery,
    runtimes: Query<(
        Entity,
        &crate::spatial::UsfCanonicalMotion,
        &UsfCollisionQueryDemand,
    )>,
    semantic_positions: Query<&UsfPosition>,
    mut frame: ResMut<UsfCollisionQueryFrame>,
) {
    frame.begin_frame();

    let duration = fixed_time.delta().as_secs_f64();
    if !duration.is_finite() || duration <= 0.0 {
        return;
    }

    let mut seen = std::collections::HashSet::<Entity>::new();

    for (runtime, motion, demand) in &runtimes {
        let Some(subject) = ownership.semantic_of(runtime) else {
            continue;
        };
        if !seen.insert(subject) {
            continue;
        }

        let Ok(&start) = semantic_positions.get(subject) else {
            continue;
        };

        let displacement = motion.velocity_metres_per_second() * duration;
        if !displacement.is_finite() || displacement.length_squared() <= f64::EPSILON {
            continue;
        }

        frame.push_request(
            UsfCanonicalSweep::new(
                subject,
                start,
                displacement,
                duration,
                demand.bounding_radius_metres(),
            ),
            demand.target_error_metres(),
        );
    }
}

fn finalize_collision_query_frame(mut frame: ResMut<UsfCollisionQueryFrame>) {
    frame.candidates.sort_by(|a, b| {
        a.request
            .cmp(&b.request)
            .then_with(|| {
                a.candidate
                    .interval()
                    .minimum()
                    .total_cmp(&b.candidate.interval().minimum())
            })
            .then_with(|| {
                a.candidate
                    .interval()
                    .maximum()
                    .total_cmp(&b.candidate.interval().maximum())
            })
            .then_with(|| {
                a.candidate
                    .authority()
                    .to_bits()
                    .cmp(&b.candidate.authority().to_bits())
            })
    });
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<UsfCollisionQueryFrame>()
        .configure_sets(
            PostUpdate,
            UsfCollisionQuerySet::Collect
                .after(crate::spatial::UsfSpatialSet::SyncSemantic),
        )
        .configure_sets(
            PostUpdate,
            UsfCollisionQuerySet::Providers.after(UsfCollisionQuerySet::Collect),
        )
        .configure_sets(
            PostUpdate,
            UsfCollisionQuerySet::Finalize.after(UsfCollisionQuerySet::Providers),
        )
        .add_systems(
            PostUpdate,
            collect_shadow_collision_sweeps.in_set(UsfCollisionQuerySet::Collect),
        )
        .add_systems(
            PostUpdate,
            finalize_collision_query_frame.in_set(UsfCollisionQuerySet::Finalize),
        );
}
