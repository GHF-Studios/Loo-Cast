define_system(
    "chunk_loader.pre_update",
    "pre_update",
    "(Query<chunk_loader>, Query<chunk>)",
    function(chunk_loaders, chunks) 
        for chunk_loader in chunk_loaders {
            local current_chunk_positions = chunk_loader.get_current_chunk_positions()
            local detected_chunk_positions = chunk_loader.detect_chunk_positions()

            local old_chunk_positions = {}
            local current_chunk_positions = {}
            local new_chunk_positions = {}

            for _, pos in ipairs(previous_chunk_positions) do
                if table_contains(detected_chunk_positions, pos) then
                    table.insert(unchanged_chunk_positions, pos)
                else
                    table.insert(old_chunk_positions, pos)
                end
            end

            for _, pos in ipairs(detected_chunk_positions) do
                if not table_contains(previous_chunk_positions, pos) then
                    table.insert(new_chunk_positions, pos)
                end
            end

            chunk_loader.set_current_chunk_positions(current_chunk_positions)

            for old_chunk_pos in old_chunk_positions {
                local old_chunk = chunks.find(function(chunk) return chunk.pos == old_chunk_pos end)
                if old_chunk and old_chunk.loader.id == chunk_loader.id then
                    -- control flow undso is shit, weil das hier blockt ja so, weil das braucht bevy, aber das system hier läuft ja schon in bevy?
                    await_task("entity.despawn", old_chunk.entity)
                end
            }

            for new_chunk_pos in new_chunk_positions {
                local new_chunk = chunks.find(function(chunk) return chunk.pos == new_chunk_pos end)
                if not new_chunk then
                    -- control flow undso is shit, weil das hier blockt ja so, weil das braucht bevy, aber das system hier läuft ja schon in bevy?
                    await_task("chunk.spawn", new_chunk_pos, chunk_loader)
                end
            }
        }
    end
)