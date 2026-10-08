//! View-owned observer state and bounded projection conversions.

use super::*;

/// Observer-relative USF presentation state.
///
/// A view context belongs to an observer/view entity rather than to the universe
/// globally. The current game owns one primary context on the active gameplay
/// camera; future portal/spectator/split-screen views can own additional
/// contexts without making semantic entities observer-aware.
#[derive(Component, Debug, Clone)]
pub struct UsfViewContext {
    /// Canonical universe position of the semantic observer anchor.
    anchor: UsfPosition,
    /// Runtime-chart position of that same semantic observer anchor.
    runtime_anchor: Vec3,
    /// Canonical SI motion of the semantic observer anchor.
    velocity_metres_per_second: DVec3,
    /// Render-space position of the dedicated USF projection camera.
    render_anchor: Vec3,
    /// Physical camera-eye offset from the semantic observer anchor, in SI metres.
    ///
    /// This is presentation-only state. It must participate in the same
    /// similarity transform as contextual geometry without becoming semantic
    /// observer motion or world-demand authority.
    projection_eye_offset_metres: DVec3,
    scale: SpatialScale,
    zoom: f32,
}

impl Default for UsfViewContext {
    fn default() -> Self {
        Self {
            anchor: UsfPosition::zero(SpatialScale::MAX),
            runtime_anchor: Vec3::ZERO,
            velocity_metres_per_second: DVec3::ZERO,
            render_anchor: Vec3::ZERO,
            projection_eye_offset_metres: DVec3::ZERO,
            scale: SpatialScale::MAX,
            zoom: 0.0,
        }
    }
}

impl UsfViewContext {
    /// Publish one coherent observer snapshot. The runtime and render anchors
    /// may differ, but neither changes the canonical semantic anchor.
    pub(super) fn sync_observer(
        &mut self,
        anchor: UsfPosition,
        runtime_anchor: Vec3,
        render_anchor: Vec3,
        velocity_metres_per_second: DVec3,
    ) {
        if self.anchor != anchor
            || self.runtime_anchor != runtime_anchor
            || self.render_anchor != render_anchor
            || self.velocity_metres_per_second != velocity_metres_per_second
        {
            self.anchor = anchor;
            self.runtime_anchor = runtime_anchor;
            self.render_anchor = render_anchor;
            self.velocity_metres_per_second = velocity_metres_per_second;
        }
    }

    pub const fn anchor(&self) -> &UsfPosition {
        &self.anchor
    }

    /// Runtime-chart position of the semantic observer anchor.
    pub const fn runtime_anchor(&self) -> Vec3 {
        self.runtime_anchor
    }

    pub const fn velocity_metres_per_second(&self) -> DVec3 {
        self.velocity_metres_per_second
    }

    /// Render-space origin corresponding to the semantic observer anchor.
    ///
    /// Camera eye height and third-person boom are presentation offsets from
    /// this origin; they must never translate the projected universe.
    pub const fn presentation_origin(&self) -> Vec3 {
        self.runtime_anchor
    }

    /// Render-space position of the dedicated USF projection camera.
    pub const fn render_anchor(&self) -> Vec3 {
        self.render_anchor
    }

    /// Physical camera-eye offset from the semantic observer anchor.
    pub const fn projection_eye_offset_metres(&self) -> DVec3 {
        self.projection_eye_offset_metres
    }

    /// Publishes view-only eye/boom placement without moving semantic authority.
    pub(crate) fn set_projection_eye_offset_metres(&mut self, offset: DVec3) {
        self.projection_eye_offset_metres = if offset.is_finite() {
            offset
        } else {
            DVec3::ZERO
        };
    }

    pub const fn scale(&self) -> SpatialScale {
        self.scale
    }

    pub const fn zoom(&self) -> f32 {
        self.zoom
    }

    pub fn continuous_exponent(&self) -> f32 {
        self.scale.exponent() as f32 + self.zoom
    }

    /// Changes observer scale without changing canonical observer position.
    pub fn add_zoom(&mut self, delta: f32, minimum: SpatialScale, maximum: SpatialScale) {
        let minimum = minimum.exponent() as f32;
        let maximum = maximum.exponent() as f32;
        let target = (self.continuous_exponent() + delta).clamp(minimum, maximum);
        self.set_continuous_exponent(target);
    }

    pub fn set_continuous_exponent(&mut self, exponent: f32) {
        let exponent = exponent.clamp(
            SpatialScale::MIN.exponent() as f32,
            SpatialScale::MAX.exponent() as f32,
        );

        if exponent >= SpatialScale::MAX.exponent() as f32 {
            self.scale = SpatialScale::MAX;
            self.zoom = 0.0;
            return;
        }

        let lower = exponent.floor() as i8;
        self.scale = SpatialScale::new(lower).expect("clamped USF view scale is valid");
        self.zoom = (exponent - lower as f32).clamp(0.0, 1.0);
    }

    /// Relative visual contribution of one of the two adjacent active scales.
    ///
    /// This first proof uses the result for visibility rather than alpha; the
    /// contract leaves room for proper morph/fade policies per realizer.
    pub fn contribution(&self, scale: SpatialScale) -> f32 {
        let t = self.zoom * self.zoom * (3.0 - 2.0 * self.zoom);
        if scale == self.scale {
            if self.scale == SpatialScale::MAX {
                1.0
            } else {
                1.0 - t
            }
        } else if self.zoom > 0.0 && scale.exponent() == self.scale.exponent() + 1 {
            t
        } else {
            0.0
        }
    }

    /// Contextual eligibility extends continuously upward through coarser
    /// Scale Slices. Residency and visibility remain downstream capability policy.
    pub fn context_scale_eligible(&self, scale: SpatialScale) -> bool {
        scale >= self.scale
    }

    /// f64 scale conversion at the final presentation-chart boundary.
    ///
    /// Keep the scale algebra in f64 until the final render transform so the
    /// 71-slice stack never relies on an intermediate f32 factor being finite.
    pub(crate) fn projection_factor_f64(&self, scale: SpatialScale) -> Option<f64> {
        let exponent_delta = f64::from(scale.exponent()) - f64::from(self.continuous_exponent());
        let factor = 10.0_f64.powf(exponent_delta);
        (factor.is_finite() && factor > 0.0).then_some(factor)
    }

    /// Converts geometry authored in `scale`-native units into current view units.
    pub fn projection_factor(&self, scale: SpatialScale) -> f32 {
        self.projection_factor_f64(scale)
            .map_or(f32::INFINITY, |factor| factor as f32)
    }

    /// Similarity ratio for one *distant* bounded semantic phenomenon.
    ///
    /// Its actual SI position, radius, and source Scale stay authoritative.
    /// Only the observer's disposable presentation receives a uniform ratio
    /// about the camera eye: every chunk of one phenomenon uses the SAME ratio
    /// derived from the semantic center, so its projected angular geometry is
    /// preserved and the seams between its realized chunks cannot drift.
    ///
    /// This is deliberately not chosen for near/inside-body geometry, which
    /// must compose with the physical local presentation and depth/occlusion.
    pub(crate) fn distant_presentation_compression(
        &self,
        phenomenon_center: &UsfPosition,
        conservative_radius_metres: f64,
    ) -> Option<f64> {
        if !conservative_radius_metres.is_finite() || conservative_radius_metres <= 0.0 {
            return None;
        }
        let delta_metres = phenomenon_center
            .relative_at_scale_bounded_f64(&self.anchor, SpatialScale::ZERO, f64::MAX)
            .ok()?
            - self.projection_eye_offset_metres;
        let distance_metres = delta_metres.length();
        // A far-field adapter must not compete with near-field physical depth.
        if !distance_metres.is_finite()
            || distance_metres <= conservative_radius_metres * 4.0
        {
            return None;
        }
        let metres_to_view = 10.0_f64.powf(-f64::from(self.continuous_exponent()));
        let uncompressed_distance = distance_metres * metres_to_view;
        let target_distance = f64::from(PRESENTATION_RELATIVE_BOUND) * 0.5;
        if !uncompressed_distance.is_finite() || uncompressed_distance <= target_distance {
            return None;
        }
        let ratio = target_distance / uncompressed_distance;
        (ratio.is_finite() && ratio > 0.0 && ratio < 1.0).then_some(ratio)
    }

    /// Projects a semantic-observer-relative SI vector into the bounded view
    /// chart while preserving the physical camera ray.
    ///
    /// The camera rig offset is subtracted *before* chart scaling. Therefore
    /// changing only the presentation exponent uniformly rescales the complete
    /// camera-relative scene and cannot manufacture parallax.
    pub(crate) fn project_relative_metres_from_eye(&self, relative_metres: DVec3) -> Option<DVec3> {
        let metres_to_view = 10.0_f64.powf(-f64::from(self.continuous_exponent()));
        let projected = (relative_metres - self.projection_eye_offset_metres) * metres_to_view;
        (metres_to_view.is_finite() && metres_to_view > 0.0 && projected.is_finite())
            .then_some(projected)
    }

    /// Same mapping for one vector expressed in an arbitrary Scale's native units.
    pub(crate) fn project_relative_native_from_eye(
        &self,
        relative_native: Vec3,
        scale: SpatialScale,
    ) -> Option<DVec3> {
        if !relative_native.is_finite() {
            return None;
        }
        let metres_per_native = scale.metres_per_native();
        if !metres_per_native.is_finite() || metres_per_native <= 0.0 {
            return None;
        }
        let relative_metres = DVec3::new(
            f64::from(relative_native.x) * metres_per_native,
            f64::from(relative_native.y) * metres_per_native,
            f64::from(relative_native.z) * metres_per_native,
        );
        self.project_relative_metres_from_eye(relative_metres)
    }

    /// Scale factor for the bounded *direct* contextual composition path.
    ///
    /// Arbitrarily distant/coarse phenomena belong in scenery/regional
    /// projection. Returning `None` here prevents a valid source-local chart
    /// from escaping the final f32 render chart through an enormous Scale
    /// Stack projection factor.
    pub fn direct_projection_factor(&self, scale: SpatialScale) -> Option<f32> {
        let factor = self.projection_factor_f64(scale)?;
        (factor <= DIRECT_PRESENTATION_SCALE_BOUND).then_some(factor as f32)
    }
}

#[cfg(test)]
mod distant_projection_tests {
    use super::*;

    #[test]
    fn distant_semantic_projection_is_view_scale_independent() {
        let observer = UsfPosition::zero(SpatialScale::ZERO);
        let distant = observer
            .translated_metres_f64(DVec3::X * 384_400_000.0)
            .unwrap();
        let mut view = UsfViewContext::default();
        view.sync_observer(observer, Vec3::ZERO, Vec3::ZERO, DVec3::ZERO);

        for exponent in [0.0, 1.0, 2.0] {
            view.set_continuous_exponent(exponent);
            let ratio = view.distant_presentation_compression(&distant, 1_737_400.0).unwrap();
            let point = view
                .project_relative_metres_from_eye(DVec3::X * 384_400_000.0)
                .unwrap() * ratio;
            let scale = view.projection_factor_f64(SpatialScale::new(5).unwrap()).unwrap() * ratio;
            let rendered_radius = 17.374 * scale;
            let rendered_angular_ratio = rendered_radius / point.length();
            let canonical_angular_ratio = 1_737_400.0 / 384_400_000.0;
            assert!((rendered_angular_ratio - canonical_angular_ratio).abs() < 1.0e-10);
            assert!((point.length() - f64::from(PRESENTATION_RELATIVE_BOUND) * 0.5).abs() < 0.01);
        }
    }

    #[test]
    fn nearby_semantic_world_must_not_enter_distant_projection() {
        let observer = UsfPosition::zero(SpatialScale::ZERO);
        let near = observer.translated_metres_f64(DVec3::Y * 6_371_000.0).unwrap();
        let mut view = UsfViewContext::default();
        view.sync_observer(observer, Vec3::ZERO, Vec3::ZERO, DVec3::ZERO);
        view.set_continuous_exponent(0.0);
        assert!(view.distant_presentation_compression(&near, 6_371_000.0).is_none());
    }
}
