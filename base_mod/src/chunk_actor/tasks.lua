define_task("chunk_actor.spawn", function(entity_position)
    -- Create entity
    local entity_id = await_task("entity.create", entity_position);

    -- Upgrade to chunk actor
    local chunk_actor_id = try(await_task("entity.upgrade", "chunk_actor", entity_id));

    -- Construct sprite
    local color = run_op("color.new_srgba", 0.0, 1.0, 0.0, 1.0);
    local rect_min = run_op("vec2.new", -HALF_CHUNK_ACTOR_SIZE, -HALF_CHUNK_ACTOR_SIZE);
    local rect_max = run_op("vec2.new", HALF_CHUNK_ACTOR_SIZE, HALF_CHUNK_ACTOR_SIZE);
    local rect = run_op("rect.new", rect_min, rect_max);
    local sprite = run_op("sprite.new_default", color, rect);

    -- Upgrade to sprite bundle
    try(await_task("entity.upgrade", "sprite_bundle", entity_id, sprite));
    
    return entity_id, chunk_actor_id
end);