private fn run_ecs_world_commands_test(world) {
    print("[Startup][ECS] world/commands test");

    world.commands(|commands| {
        commands.spawn_empty(|entity_commands| {
            let id = entity_commands.id();
            print("[Startup][ECS] World::commands -> spawn_empty: " + id.to_string());

            entity_commands.commands(|commands_from_entity| {
                commands_from_entity.spawn_empty(|nested_entity_commands| {
                    let nested_id = nested_entity_commands.id();
                    print(
                        "[Startup][ECS] EntityCommands::commands -> spawn_empty: " +
                        nested_id.to_string()
                    );
                });
            });
        });
    });

    world.spawn_empty(|entity_world_mut| {
        let id = entity_world_mut.id();
        print("[Startup][ECS] World::spawn_empty: " + id.to_string());
    });
}
