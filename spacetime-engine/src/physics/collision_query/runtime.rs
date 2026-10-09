//! ECS collection and provider ordering for canonical collision queries.

use super::{UsfCollisionQueryFrame, contract::UsfCanonicalSweep};
use bevy::prelude::*;

#[derive(Resource, Default)]
pub struct UsfProposedSweeps {
    sweeps: Vec<(UsfCanonicalSweep, f64)>,
}

impl UsfProposedSweeps {
    pub fn clear(&mut self) {
        self.sweeps.clear();
    }

    pub fn submit(&mut self, sweep: UsfCanonicalSweep, target_error_metres: f64) {
        self.sweeps.push((sweep, target_error_metres));
    }
}

#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UsfCollisionQuerySet {
    Reset,
    Collect,
    Providers,
    Finalize,
}

fn reset_proposed_sweeps(mut proposals: ResMut<UsfProposedSweeps>) {
    proposals.clear();
}

fn collect_proposed_collision_sweeps(
    proposals: Res<UsfProposedSweeps>,
    mut frame: ResMut<UsfCollisionQueryFrame>,
) {
    frame.begin_transaction();
    for (sweep, target_error_metres) in &proposals.sweeps {
        frame.submit_request(*sweep, *target_error_metres);
    }
}

fn finalize_collision_query_frame(mut frame: ResMut<UsfCollisionQueryFrame>) {
    frame.finalize_transaction();
}

pub(in crate::physics) fn configure(app: &mut App) {
    app.init_resource::<UsfCollisionQueryFrame>()
        .init_resource::<UsfProposedSweeps>()
        .configure_sets(
            FixedUpdate,
            UsfCollisionQuerySet::Reset.before(crate::game::locomotion::LocomotionSet::Prepare),
        )
        .configure_sets(
            FixedUpdate,
            UsfCollisionQuerySet::Collect.after(crate::game::locomotion::LocomotionSet::Prepare),
        )
        .configure_sets(
            FixedUpdate,
            UsfCollisionQuerySet::Providers.after(UsfCollisionQuerySet::Collect),
        )
        .configure_sets(
            FixedUpdate,
            UsfCollisionQuerySet::Finalize.after(UsfCollisionQuerySet::Providers),
        )
        .add_systems(
            FixedUpdate,
            reset_proposed_sweeps.in_set(UsfCollisionQuerySet::Reset),
        )
        .add_systems(
            FixedUpdate,
            collect_proposed_collision_sweeps.in_set(UsfCollisionQuerySet::Collect),
        )
        .add_systems(
            FixedUpdate,
            finalize_collision_query_frame.in_set(UsfCollisionQuerySet::Finalize),
        );
}
