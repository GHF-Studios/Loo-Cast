//! Observer-relative presentation scale over canonical USF space.
//!
//! The physical/runtime chart can remain fixed at S0 while presentation is
//! projected into units appropriate to the observer's current spatial scale.
//! A representation authored at scale S stores bounded S-native geometry and a
//! canonical anchor; it never needs a universe-wide float position.

use bevy::prelude::*;

use crate::spatial::{
    SpatialScale, UsfActiveScaleLayer, UsfFollowsActiveScale, UsfPosition, UsfScaleLayer,
    UsfScaleLayerFrames, UsfSpatialFrame,
};

const PRESENTATION_RELATIVE_BOUND: f32 = 1_000_000.0;
const CONTRIBUTION_EPSILON: f32 = 0.001;

/// Marks the runtime transform used as the primary observer/view anchor.
///
/// This is deliberately a view concern rather than player ownership. Editors,
/// portal views and other observers can later provide independent view frames.
#[derive(Component, Debug, Default)]
pub struct UsfViewAnchor;

/// One disposable visual representation authored in units native to `scale`.
///
/// `anchor` is semantic identity for the representation's local origin.
/// Geometry remains bounded around that origin.
#[derive(Component, Debug, Clone, Copy)]
pub struct UsfScalePresentation {
    anchor: UsfPosition,
    scale: SpatialScale,
}

/// Presentation geometry whose parent already owns the correct runtime position.
///
/// This is useful for actors such as the local player: physics remains S0 while
/// the visible child shrinks as the observer zooms outward.
#[derive(Component, Debug, Clone, Copy)]
pub struct UsfLocalScalePresentation {
    scale: SpatialScale,
}

impl UsfLocalScalePresentation {
    pub const fn new(scale: SpatialScale) -> Self {
        Self { scale }
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub(crate) fn set_scale(&mut self, scale: SpatialScale) {
        self.scale = scale;
    }
}

impl UsfScalePresentation {
    pub const fn new(anchor: UsfPosition, scale: SpatialScale) -> Self {
        Self { anchor, scale }
    }

    pub const fn anchor(self) -> UsfPosition {
        self.anchor
    }

    pub const fn scale(self) -> SpatialScale {
        self.scale
    }
}

/// Primary observer-relative spatial presentation frame.
///
/// `scale` is the lower integer scale and `zoom` is fractional progress toward
/// `scale + 1`. The observer's canonical identity never changes when zooming.
#[derive(Resource, Debug, Clone)]
pub struct UsfViewFrame {
    anchor: UsfPosition,
    runtime_anchor: Vec3,
    scale: SpatialScale,
    zoom: f32,
}

/// One scale requested by the observer's continuous transition window.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct UsfViewScaleDemand {
    scale: SpatialScale,
    contribution: f32,
}

impl UsfViewScaleDemand {
    pub const fn scale(self) -> SpatialScale {
        self.scale
    }

    pub const fn contribution(self) -> f32 {
        self.contribution
    }
}

impl Default for UsfViewFrame {
    fn default() -> Self {
        Self {
            anchor: UsfPosition::zero(SpatialScale::MAX),
            runtime_anchor: Vec3::ZERO,
            scale: SpatialScale::MAX,
            zoom: 0.0,
        }
    }
}

impl UsfViewFrame {
    pub const fn anchor(&self) -> &UsfPosition {
        &self.anchor
    }

    pub const fn runtime_anchor(&self) -> Vec3 {
        self.runtime_anchor
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

    pub fn interaction_scale(&self) -> SpatialScale {
        if self.scale == SpatialScale::MAX || self.zoom <= CONTRIBUTION_EPSILON {
            self.scale
        } else {
            SpatialScale::new(self.scale.exponent() + 1)
                .expect("fractional transition has a coarser interaction scale")
        }
    }

    pub fn dominant_scale(&self) -> SpatialScale {
        self.interaction_scale()
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

    /// The adjacent spatial scales whose representations are currently needed
    /// to realize the continuous observer view.
    ///
    /// This is *transition demand*, not distance LOD demand. A later visibility
    /// policy may request additional coarser representations at the same view
    /// scale for distant detail without changing this two-slot contract.
    pub fn active_scale_demands(&self) -> [Option<UsfViewScaleDemand>; 2] {
        let lower = UsfViewScaleDemand {
            scale: self.scale,
            contribution: self.contribution(self.scale),
        };

        let upper = if self.zoom > CONTRIBUTION_EPSILON && self.scale != SpatialScale::MAX {
            let scale = SpatialScale::new(self.scale.exponent() + 1)
                .expect("non-maximum view scale has an adjacent upper scale");
            Some(UsfViewScaleDemand {
                scale,
                contribution: self.contribution(scale),
            })
        } else {
            None
        };

        [Some(lower), upper]
    }

    /// Converts geometry authored in `scale`-native units into current view units.
    pub fn projection_factor(&self, scale: SpatialScale) -> f32 {
        10.0_f32.powf(scale.exponent() as f32 - self.continuous_exponent())
    }

    /// Temporary bridge while physical interaction remains an S0 manifestation.
    /// One decade of observer zoom receives matching physical traversal speed.
    /// Beyond S+1, navigation needs scale-specific/semantic manifestations rather
    /// than multiplying one local collider into absurd velocities.
    pub fn scale0_physical_navigation_factor(&self) -> f32 {
        10.0_f32.powf(self.continuous_exponent().clamp(0.0, 1.0))
    }
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<UsfViewFrame>();
}

/// Keeps the view anchored to an ordinary bounded runtime transform while
/// deriving its semantic position through the current local physical frame.
pub(super) fn sync_view_anchor(
    frame: Res<UsfSpatialFrame>,
    anchors: Query<&Transform, With<UsfViewAnchor>>,
    mut view: ResMut<UsfViewFrame>,
) {
    let Some(anchor) = anchors.iter().next() else {
        return;
    };

    let Ok(canonical) = frame.origin().translated_native(anchor.translation) else {
        error!(
            local_anchor = ?anchor.translation,
            "USF view anchor could not project into canonical space"
        );
        return;
    };

    view.anchor = canonical;
    view.runtime_anchor = anchor.translation;
}

/// Projects scale-authored presentation geometry around the observer without
/// modifying logical/physics transforms.
///
/// Current child presentations (voxel surfaces) have root parents with identity
/// rotation/scale, so child compensation is translation-only. General rotated
/// representation frames can later promote this to an explicit projection frame.
pub(super) fn project_local_scale_presentations(
    view: Res<UsfViewFrame>,
    active: Res<UsfActiveScaleLayer>,
    frames: Res<UsfScaleLayerFrames>,
    parents: Query<
        (&Transform, &UsfScaleLayer, Option<&UsfFollowsActiveScale>),
        Without<UsfLocalScalePresentation>,
    >,
    mut presentations: Query<(&mut UsfLocalScalePresentation, &ChildOf, &mut Transform)>,
) {
    let active_scale = active.scale();
    let observer_absolute = frames.absolute(active_scale, view.runtime_anchor());

    for (mut presentation, parent, mut transform) in &mut presentations {
        let Ok((parent_transform, layer, follows_active)) = parents.get(parent.0) else {
            continue;
        };
        presentation.set_scale(layer.scale());

        let observer_in_parent_chart = if follows_active.is_some() {
            view.runtime_anchor()
        } else {
            let converted = frames.convert_absolute(observer_absolute, active_scale, layer.scale());
            frames.runtime_from_absolute(layer.scale(), converted)
        };

        let factor = view.projection_factor(layer.scale());
        let desired_global = view.runtime_anchor()
            + (parent_transform.translation - observer_in_parent_chart) * factor;
        let delta = desired_global - parent_transform.translation;
        transform.translation = parent_transform.rotation.inverse() * delta;
        transform.scale = Vec3::splat(factor);
    }
}

pub(super) fn project_scale_presentations(
    view: Res<UsfViewFrame>,
    parents: Query<&Transform, Without<UsfScalePresentation>>,
    mut presentations: Query<(
        &UsfScalePresentation,
        Option<&ChildOf>,
        &mut Transform,
        &mut Visibility,
    )>,
) {
    for (presentation, parent, mut transform, mut visibility) in &mut presentations {
        let contribution = view.contribution(presentation.scale());
        if contribution <= CONTRIBUTION_EPSILON {
            *visibility = Visibility::Hidden;
            continue;
        }

        let Ok(relative) = presentation.anchor().relative_at_scale_bounded(
            view.anchor(),
            presentation.scale(),
            PRESENTATION_RELATIVE_BOUND,
        ) else {
            *visibility = Visibility::Hidden;
            continue;
        };

        let factor = view.projection_factor(presentation.scale());
        let desired_global = view.runtime_anchor() + relative * factor;

        transform.translation = if let Some(parent) = parent {
            let Ok(parent_transform) = parents.get(parent.0) else {
                *visibility = Visibility::Hidden;
                continue;
            };
            desired_global - parent_transform.translation
        } else {
            desired_global
        };
        transform.scale = Vec3::splat(factor);
        *visibility = Visibility::Inherited;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn midpoint_between_s0_and_s1_has_reciprocal_adjacent_projection_factors() {
        let mut view = UsfViewFrame::default();
        view.set_continuous_exponent(0.5);

        let s1 = SpatialScale::new(1).unwrap();
        assert!((view.projection_factor(SpatialScale::ZERO) - 10.0_f32.powf(-0.5)).abs() < 1.0e-6);
        assert!((view.projection_factor(s1) - 10.0_f32.powf(0.5)).abs() < 1.0e-6);
        assert!((view.contribution(SpatialScale::ZERO) - 0.5).abs() < 1.0e-6);
        assert!((view.contribution(s1) - 0.5).abs() < 1.0e-6);
    }

    #[test]
    fn crossing_an_integer_zoom_boundary_normalizes_to_the_next_scale() {
        let mut view = UsfViewFrame::default();
        view.add_zoom(1.0, SpatialScale::ZERO, SpatialScale::new(1).unwrap());

        assert_eq!(view.scale(), SpatialScale::new(1).unwrap());
        assert_eq!(view.zoom(), 0.0);
    }
}
