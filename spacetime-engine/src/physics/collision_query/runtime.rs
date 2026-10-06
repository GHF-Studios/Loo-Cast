//! ECS collection and provider ordering for canonical collision queries.

use super::{
    UsfCollisionQueryFrame,
    contract::{UsfCanonicalSweep, UsfCollisionQueryDemand},
};
use crate::spatial::UsfPosition;
use bevy::prelude::*;

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
    frame.finalize();
}

pub(in crate::physics) fn configure(app: &mut App) {
    app.init_resource::<UsfCollisionQueryFrame>()
        .configure_sets(
            PostUpdate,
            UsfCollisionQuerySet::Collect.after(crate::spatial::UsfSpatialSet::SyncSemantic),
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
