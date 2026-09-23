//! Canonical USF gravity fields.
//!
//! Gravity sources are semantic physical facts. Consumers query the field at a
//! canonical [`crate::spatial::UsfPosition`] and receive SI acceleration;
//! navigation, cameras, locomotion regimes and runtime Scale Slices do not own
//! gravity.
//!
//! Runtime evaluation is attached to the shared ancestor-closed
//! [`crate::spatial::UsfContextTopology`]. The current backend is exact but
//! hierarchical: child caches inherit sources inside their context and carry
//! the remainder as parent residuals. That gives us a real cross-scale field
//! stack now while leaving room to replace exact residual source lists with
//! bounded-error multipoles/grids later.

mod cache;
mod query;
mod source;

pub use query::{GravityFieldQuery, GravitySample, GravitySampleQuality};
pub use source::RadialGravitySource;

use bevy::{app::RunFixedMainLoop, prelude::*};

use crate::spatial::{UsfScaleLayer, UsfSpatialFrame};

use cache::{GravityFieldCache, prepare_gravity_field_cache};

/// Stable gravity-field runtime extension points.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GravitySet {
    PrepareCache,
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

        *sample = gravity.sample(&position, layer.scale());
    }
}

pub struct GravityPlugin;

impl Plugin for GravityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GravityFieldCache>()
            .configure_sets(
                RunFixedMainLoop,
                (GravitySet::PrepareCache, GravitySet::Sample).chain(),
            )
            .add_systems(
                RunFixedMainLoop,
                prepare_gravity_field_cache.in_set(GravitySet::PrepareCache),
            )
            .add_systems(
                RunFixedMainLoop,
                sample_gravity_receivers.in_set(GravitySet::Sample),
            );
    }
}
