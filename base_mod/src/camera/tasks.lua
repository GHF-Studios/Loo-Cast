define_task("camera.spawn", function(ctx, entity_position)
    local entity_id = await_task("entity.create", entity_position);
    local camera_id = try(await_task("entity.upgrade", "camera_2d_bundle", entity_id));
    return entity_id, camera_id
end);