//! Per-frame query requests and provider observations.

use bevy::prelude::*;

use super::contract::{UsfCanonicalSweep, UsfCollisionCandidate, UsfSweepResolution};

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
pub struct UsfCollisionCandidateObservation {
    request: UsfCollisionQueryRequestId,
    candidate: UsfCollisionCandidate,
}

impl UsfCollisionCandidateObservation {
    pub const fn request(self) -> UsfCollisionQueryRequestId {
        self.request
    }

    pub const fn candidate(self) -> UsfCollisionCandidate {
        self.candidate
    }
}

/// One fixed-step collision transaction. Providers publish both diagnostics
/// and a bounded answer before the motion executor commits its proposed path.
#[derive(Resource, Debug, Default)]
pub struct UsfCollisionQueryFrame {
    revision: u64,
    requests: Vec<UsfCollisionQueryRequest>,
    candidates: Vec<UsfCollisionCandidateObservation>,
    resolutions: Vec<(UsfCollisionQueryRequestId, UsfSweepResolution)>,
}

impl UsfCollisionQueryFrame {
    pub const fn revision(&self) -> u64 {
        self.revision
    }

    pub fn requests(&self) -> impl ExactSizeIterator<Item = UsfCollisionQueryRequest> + '_ {
        self.requests.iter().copied()
    }

    pub fn candidates(
        &self,
    ) -> impl ExactSizeIterator<Item = UsfCollisionCandidateObservation> + '_ {
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
            .map(UsfCollisionCandidateObservation::candidate)
    }

    pub fn resolution_for(
        &self,
        request: UsfCollisionQueryRequestId,
    ) -> Option<UsfSweepResolution> {
        self.resolutions
            .iter()
            .filter_map(|(id, resolution)| (*id == request).then_some(*resolution))
            .min_by(|left, right| {
                left.safe_fraction()
                    .total_cmp(&right.safe_fraction())
                    .then_with(|| left.tie_priority().cmp(&right.tie_priority()))
            })
    }

    pub fn request_for_subject(&self, subject: Entity) -> Option<UsfCollisionQueryRequest> {
        self.requests
            .iter()
            .copied()
            .find(|request| request.sweep().subject() == subject)
    }

    pub fn publish_resolution(
        &mut self,
        request: UsfCollisionQueryRequestId,
        resolution: UsfSweepResolution,
    ) {
        if request.frame_revision != self.revision {
            return;
        }
        self.resolutions.push((request, resolution));
    }

    pub fn publish_candidate(
        &mut self,
        request: UsfCollisionQueryRequestId,
        candidate: UsfCollisionCandidate,
    ) {
        if request.frame_revision != self.revision {
            return;
        }
        self.candidates
            .push(UsfCollisionCandidateObservation { request, candidate });
    }

    pub(super) fn begin_transaction(&mut self) {
        self.revision = self.revision.wrapping_add(1).max(1);
        self.requests.clear();
        self.candidates.clear();
        self.resolutions.clear();
    }

    pub(super) fn submit_request(&mut self, sweep: UsfCanonicalSweep, target_error_metres: f64) {
        let index =
            u32::try_from(self.requests.len()).expect("collision-query request count exceeded u32");
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

    /// Stable ordering for provider results published in this frame.
    pub(super) fn finalize_transaction(&mut self) {
        self.candidates.sort_by(|a, b| {
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
}
