define_system("pre_startup", "core.pre_startup", function()
    await_task("core.spawn_main_camera");
    await_task("core.spawn_start_chunks");
    await_task("core.spawn_start_chunk_actors");
end);