//! Authored celestial fixture using shared semantic fields and voxel realization.

mod definition;
mod landmarks;
mod player;
mod scenery;

use bevy::prelude::*;

use crate::spatial::{SpatialScale, UsfPosition};

use super::GameWorld;

pub(in crate::game) use landmarks::UniverseLandmarkIndex;

/// One canonical body-surface location selected by authored fixture policy.
///
/// This is bootstrap/navigation data, not terrain identity. The surface is
/// resolved from the same celestial field that owns voxel terrain; `up` is the
/// outward body-relative normal used to apply physical clearance and orientation.
#[derive(Debug, Clone, Copy)]
pub(super) struct BodySurfaceSite {
    body: Entity,
    surface: UsfPosition,
    up: Vec3,
    scale: SpatialScale,
}

impl BodySurfaceSite {
    pub(super) fn new(
        body: Entity,
        surface: UsfPosition,
        up: Vec3,
        scale: SpatialScale,
    ) -> Option<Self> {
        let up = up.normalize_or_zero();
        (up != Vec3::ZERO).then_some(Self {
            body,
            surface,
            up,
            scale,
        })
    }

    pub(super) const fn body(self) -> Entity {
        self.body
    }

    pub(super) const fn surface(self) -> UsfPosition {
        self.surface
    }

    pub(super) const fn up(self) -> Vec3 {
        self.up
    }

    pub(super) const fn scale(self) -> SpatialScale {
        self.scale
    }
}

/// The authored fixture's selected arrival location.
///
/// The fixture may choose Earth today and another body tomorrow; controlled
/// subject bootstrap consumes only this generic contract.
#[derive(Resource, Debug, Default, Clone, Copy)]
pub(super) struct FixtureArrivalSite(Option<BodySurfaceSite>);

impl FixtureArrivalSite {
    pub(super) fn set(&mut self, site: BodySurfaceSite) {
        self.0 = Some(site);
    }

    pub(super) const fn site(&self) -> Option<BodySurfaceSite> {
        self.0
    }
}

pub(super) fn configure(app: &mut App) {
    app.init_resource::<landmarks::UniverseLandmarkIndex>()
        .init_resource::<FixtureArrivalSite>();
    app.add_systems(
        OnEnter(GameWorld::CelestialFixture),
        (
            scenery::spawn_fixture,
            player::prepare_controlled_subject,
        )
            .chain(),
    )
    .add_systems(
        Update,
        scenery::audit_world_authority.run_if(in_state(GameWorld::CelestialFixture)),
    );
}
