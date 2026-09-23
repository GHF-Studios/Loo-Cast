//! Non-local semantic navigation structure.
//!
//! Galaxy/cloud data is world state, not gameplay-scene geometry. A future map
//! or astronomical renderer may project these same semantic entities explicitly.

use bevy::{math::DVec3, prelude::*};

use crate::{
    ecs::UsfEntity,
    spatial::{SpatialScale, UsfPosition, UsfTravelInfluence},
    worldgen::GalaxyInterstellarMediumState,
};

const GALAXY_SCALE: i8 = 18;

fn scale(raw: i8) -> SpatialScale {
    SpatialScale::new(raw).expect("cosmic semantic scale is valid")
}

fn canonical(absolute: DVec3, scale: SpatialScale) -> UsfPosition {
    UsfPosition::from_scale_native_f64(absolute, scale, SpatialScale::MIN)
        .expect("cosmic semantic position must be canonical")
}

pub(super) fn spawn_navigation_structure(
    commands: &mut Commands,
    parent: Entity,
    state: GalaxyInterstellarMediumState,
) {
    let galaxy_scale = scale(GALAXY_SCALE);
    let center = DVec3::new(220.0, -8.0, -70.0);

    commands.spawn((
        Name::new("Host Galaxy Semantic Region"),
        ChildOf(parent),
        UsfEntity,
        canonical(center, galaxy_scale),
        UsfTravelInfluence::region(center, galaxy_scale, 310.0),
    ));

    let density = state.stellar_density.clamp(0.0, 1.0);
    let cloud_radius = 5.0 + f64::from(density) * 4.0;

    for arm in 0..3 {
        for step in 0..12 {
            let t = step as f64 / 11.0;
            let radius = 42.0 + t * 245.0;
            let angle = arm as f64 * std::f64::consts::TAU / 3.0 + t * 5.2;
            let absolute = center
                + DVec3::new(
                    angle.cos() * radius,
                    ((step * 17 + arm * 11) as f64).sin() * 3.5,
                    angle.sin() * radius,
                );

            commands.spawn((
                Name::new(format!("Galaxy Arm {arm} Medium {step}")),
                ChildOf(parent),
                UsfEntity,
                canonical(absolute, galaxy_scale),
                UsfTravelInfluence::medium(
                    absolute,
                    galaxy_scale,
                    cloud_radius,
                    cloud_radius * 0.25,
                    state.gas_fraction,
                    state.turbulence,
                    state.star_formation_potential * 0.35,
                ),
            ));
        }
    }
}
