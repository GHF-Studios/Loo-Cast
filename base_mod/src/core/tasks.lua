define_task("core.spawn_main_camera", function()
    local entity_position = run_op("entity.position.new", 0.0, 0.0);
    local entity_id, camera_id = try(await_task("camera.spawn", entity_position));

    return entity_id, camera_id;
end);

define_task("core.spawn_start_chunks", function(range)
    local entity_ids = {};
    local chunk_ids = {};

    for x in -range..=range {
        for y in -range..=range {
            local chunk_position = run_op("chunk.position.new", x, y);
            local entity_id, chunk_id = try(await_task("chunk.spawn", chunk_position));
            entity_ids[(range * 2 + 1) * x + y] = entity_id;
            chunk_ids[(range * 2 + 1) * x + y] = chunk_id;
        }
    }

    return entity_ids, chunk_ids;
end);

define_task("core.spawn_start_chunk_actors", function(range) 
    local entity_ids = {};
    local chunk_actor_ids = {};

    for x in -range..=range {
        for y in -range..=range {
            local chunk_position = run_op("chunk.position.new", x, y);
            local entity_position = try(run_op("core.convert", "chunk.position", "entity.position", chunk_position));
            local entity_id, chunk_actor_id = try(await_task("chunk_actor.spawn", entity_position));
            entity_ids[(range * 2 + 1) * x + y] = entity_id;
            chunk_actor_ids[(range * 2 + 1) * x + y] = chunk_actor_id;
        }
    }

    return entity_ids, chunk_actor_ids;
end);