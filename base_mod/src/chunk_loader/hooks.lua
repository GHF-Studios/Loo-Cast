define_hook("on_spawn", "chunk_loader", function(chunk_loader)
    local current_chunks = run_op("chunk_loader.detect_chunks", chunk_loader);
    try(run_op("chunk_loader.set_current_chunks", chunk_loader, current_chunks));

    for current_chunk in current_chunks {
        if run_op("chunk.pos.exists", current_chunk) == false then
            await_task("chunk.spawn", current_chunk, chunk_loader);
        end
    }
end);

define_hook("on_despawn", "chunk_loader", function(chunk_loader)
    local current_chunks = run_op("chunk_loader.get_current_chunks", chunk_loader);

    for current_chunk in current_chunks {
        current_chunk = try(await_task("chunk.resolve_from_pos", current_chunk));
        if current_chunk.owner.id == chunk_loader.id then
            await_task("entity.despawn", current_chunk.entity);
        end
    }
end);