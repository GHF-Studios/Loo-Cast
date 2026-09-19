//! Collision-constrained movement for the unused fraction of a crossing tick.

use std::time::Duration;

use avian3d::{
    character_controller::move_and_slide::{
        MoveAndSlide, MoveAndSlideConfig, MoveAndSlideHitResponse,
    },
    prelude::*,
};
use bevy::prelude::*;

use crate::{
    portal::topology::mapping::portal_plane,
    physics::topology::{SpatialSplitBox, partition_box_by_plane},
};

const DESTINATION_REMAINDER_SUBSTEPS: usize = 4;

pub(super) fn simulate_destination_remainder(
    move_and_slide: &MoveAndSlide,
    split_box: SpatialSplitBox,
    mut body: Transform,
    mut velocity: Vec3,
    destination: &Transform,
    remaining: f32,
    filter: &SpatialQueryFilter,
) -> (Transform, Vec3) {
    if remaining <= 0.0 {
        return (body, velocity);
    }

    let step = remaining / DESTINATION_REMAINDER_SUBSTEPS as f32;
    for _ in 0..DESTINATION_REMAINDER_SUBSTEPS {
        let collider = destination_half_collider(split_box, &body, destination, velocity)
            .unwrap_or_else(|| split_box.full_collider());

        let output = move_and_slide.move_and_slide(
            &collider,
            body.translation,
            body.rotation,
            velocity,
            Duration::from_secs_f32(step),
            &MoveAndSlideConfig::default(),
            filter,
            |_| MoveAndSlideHitResponse::Accept,
        );

        body.translation = output.position;
        velocity = output.projected_velocity;
    }

    (body, velocity)
}

fn destination_half_collider(
    split_box: SpatialSplitBox,
    body: &Transform,
    destination: &Transform,
    velocity: Vec3,
) -> Option<Collider> {
    let plane = portal_plane(destination)?;
    let partition = partition_box_by_plane(split_box, body, plane);
    if velocity.dot(plane.normal) >= 0.0 {
        partition.positive_collider()
    } else {
        partition.negative_collider()
    }
}
