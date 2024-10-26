use bevy::prelude::*;
use crate::camera_2d_bundle::operations::*;
use crate::entity::structs::EntityPosition;
use crate::entity::operations::*;
use crate::operation::commands::*;

pub async fn spawn_camera(entity_position: EntityPosition) -> Result<(), String> {
    let create_entity_args = CreateEntityArgs { entity_position };
    let create_entity_result = run_op::<CreateEntity>(create_entity_args).await;
    let entity_id = match create_entity_result {
        CreateEntityResult::Ok { entity_id } => entity_id,
        CreateEntityResult::Err(_) => {
            return Err("Failed to create entity!".to_string());
        }
    };

    debug!("Created entity '{}'", entity_id);

    let upgrade_to_camera_2d_bundle_args = UpgradeToCamera2dBundleArgs {
        target_entity_id: entity_id,
        ..Default::default()
    };
    let upgrade_to_camera_2d_bundle_result = run_op::<UpgradeToCamera2dBundle>(upgrade_to_camera_2d_bundle_args).await;
    match upgrade_to_camera_2d_bundle_result {
        UpgradeToCamera2dBundleResult::Ok(_) => {},
        UpgradeToCamera2dBundleResult::Err(_) => {
            return Err(format!("Failed to upgrade entity '{}' to camera 2d bundle!", entity_id));
        }
    };

    debug!("Upgraded entity '{}' to camera 2d bundle", entity_id);

    Ok(())
}