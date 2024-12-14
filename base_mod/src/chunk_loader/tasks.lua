define_task("chunk_loader.spawn", function(entity_position, range)
    local entity_id = await_task("entity.create", entity_position);
    local chunk_loader_id = try(await_task("entity.upgrade", "chunk_loader", entity_id, range));
    return entity_id, chunk_loader_id
end);