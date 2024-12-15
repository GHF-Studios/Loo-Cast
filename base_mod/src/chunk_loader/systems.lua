define_params("chunk_loader.pre_update", "Query<chunk_loader>")
define_system("pre_update", "chunk_loader.pre_update", function(chunk_loaders) 
    for chunk_loader in chunk_loaders {
        local old_chunk, new_chunks = run_op("chunk_loader.detect_delta_chunks", chunk_loader);
        
        for old_chunk in old_chunks {
            if old_chunk.owner.id == chunk_loader.id then
                await_task("entity.despawn", old_chunk.entity);
            end
        }

        for new_chunk in new_chunks {
            if new_chunk.owner.id == chunk_loader.id then
                await_task("chunk.spawn", new_chunk.pos);
            end
        }
    }
end);