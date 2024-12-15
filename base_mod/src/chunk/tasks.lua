define_task("chunk.spawn", function(chunk_position, chunk_loader)
    -- Create entity
    local entity_position = run_op("core.convert", "chunk.position", "entity.position", chunk_position);
    local entity_id = await_task("entity.create", entity_position);

    -- Upgrade to chunk
    local chunk_id = try(await_task("entity.upgrade", "chunk", entity_id, chunk_loader));

    -- Construct sprite
    local color = 
    if (chunk_position.x + chunk_position.y) % 2 == 0 then
        color = run_op("color.new_srgba", 0.25, 0.25, 0.25, 0.5);
    else then
        color = run_op("color.new_srgba", 0.75, 0.75, 0.75, 0.5);
    end;
    local rect_min = run_op("vec2.new", -HALF_CHUNK_SIZE, -HALF_CHUNK_SIZE);
    local rect_max = run_op("vec2.new", HALF_CHUNK_SIZE, HALF_CHUNK_SIZE);
    local rect = run_op("rect.new", rect_min, rect_max);
    local sprite = run_op("sprite.new_default", color, rect);

    -- Upgrade to sprite bundle
    try(await_task("entity.upgrade", "sprite_bundle", entity_id, sprite));
    
    return entity_id, chunk_id
end);