define_system("chunk_loader.pre_update", "pre_update", "(Query<chunk_loader>)", function(chunk_loaders, chunks) 
    for chunk_loader in chunk_loaders {
        local current_chunks = run_op("chunk_loader.get_current_chunks", chunk_loader);
        local detected_chunks = run_op("chunk_loader.detect_chunks", chunk_loader);
        local old_chunks, current_chunks, new_chunks = run_op("chunk_loader.filter_chunks", current_chunks, detected_chunks);
        try(run_op("chunk_loader.set_current_chunks", chunk_loader, current_chunks));
        
        for old_chunk in old_chunks {
            old_chunk = try(await_task("chunk.resolve_from_pos", old_chunk));
            if old_chunk.owner.id == chunk_loader.id then
                await_task("entity.despawn", old_chunk.entity);
            end
        }

        for new_chunk in new_chunks {
            if run_op("chunk.pos.exists", new_chunk) == false then
                await_task("chunk.spawn", new_chunk, chunk_loader);
            end
        }
    }
end);