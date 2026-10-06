//! Physical standing-pose proof for disembark.

use super::*;

/// Prove a walkable player-hull pose before disembark changes semantic or
/// manifestation state. A missing/non-walkable hit leaves control on the ship.
pub(super) fn resolve_disembark_pose(
    spatial_query: &SpatialQuery,
    physics_charts: &UsfPhysicsSliceQuery,
    ship_entity: Entity,
    ship_transform: &Transform,
    ship_layer: &UsfScaleLayer,
    ship_frame: &CharacterLocomotionFrame,
    ship_hull: &PhysicalBoxHull,
    player_entity: Entity,
    player_hull: &PhysicalBoxHull,
    player_movement: &CharacterMovementConfig,
    player_exclusions: Option<&KinematicQueryExclusions>,
    aligned_player: Quat,
) -> Option<Vec3> {
    let exit_offset =
        ship_transform.rotation * Vec3::X * ship_layer.scale().metres_to_native_f32(4.0);
    let exit_column = ship_transform.translation + exit_offset;

    let ship_support_metres =
        ship_hull.projection_radius_metres(ship_transform.rotation, ship_frame.up());
    let player_support_metres =
        player_hull.projection_radius_metres(aligned_player, ship_frame.up());
    let probe_lift_metres = player_support_metres + LANDING_PROBE_METRES;
    let probe_distance_metres =
        ship_support_metres + player_support_metres + LANDING_PROBE_METRES * 2.0;

    let exit_probe_start =
        exit_column + ship_frame.up() * ship_layer.scale().metres_to_native_f32(probe_lift_metres);
    let exit_direction = Dir3::new(-ship_frame.up()).ok()?;
    let exit_filter = physics_charts.filter_for_scale(
        ship_layer.scale(),
        std::iter::once(player_entity)
            .chain(std::iter::once(ship_entity))
            .chain(player_exclusions.into_iter().flat_map(|items| items.iter())),
    );
    let exit_config = ShapeCastConfig {
        max_distance: ship_layer
            .scale()
            .metres_to_native_f32(probe_distance_metres),
        ignore_origin_penetration: true,
        ..default()
    };
    let player_collider = player_hull.collider(ship_layer.scale());
    let exit_hit = spatial_query.cast_shape(
        &player_collider,
        exit_probe_start,
        aligned_player,
        exit_direction,
        &exit_config,
        &exit_filter,
    )?;
    (exit_hit.normal1.dot(ship_frame.up()) >= player_movement.min_ground_dot)
        .then_some(exit_probe_start - ship_frame.up() * exit_hit.distance.max(0.0))
}
