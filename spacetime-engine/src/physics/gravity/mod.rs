//! Canonical USF gravity.
//!
//! Gravity sources are semantic physical facts. Consumers query the field at a
//! canonical [`crate::spatial::UsfPosition`] and receive SI acceleration;
//! navigation, cameras, locomotion regimes and runtime Scale Slices do not own
//! gravity.
//!
//! The current evaluator is deliberately exact and direct. The typed query
//! boundary is retained; speculative generic cache/multipole machinery is not.
//! A hierarchical representation can be added behind the same query once real
//! source count, accuracy and performance requirements define that contract.

mod query;
mod source;

pub use query::{GravityFieldQuery, GravitySample};
pub use source::RadialGravitySource;

use bevy::{app::RunFixedMainLoop, prelude::*};

use crate::spatial::{UsfScaleLayer, UsfSpatialFrame};

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GravitySet {
    Sample,
}

fn sample_gravity_receivers(
    frame: Res<UsfSpatialFrame>,
    gravity: GravityFieldQuery,
    mut receivers: Query<(&Transform, &UsfScaleLayer, &mut GravitySample)>,
) {
    for (transform, layer, mut sample) in &mut receivers {
        let Ok(position) = frame
            .origin()
            .translated_at_scale(layer.scale(), transform.translation)
        else {
            *sample = GravitySample::default();
            continue;
        };

        *sample = gravity.sample(&position);
    }
}

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(RunFixedMainLoop, GravitySet::Sample)
            .add_systems(
                RunFixedMainLoop,
                sample_gravity_receivers.in_set(GravitySet::Sample),
            );
    }
}
