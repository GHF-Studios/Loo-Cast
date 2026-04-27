define_system("core.startup", "startup", "()", function()
    spawn_task("core.debug_entrypoint")
end)