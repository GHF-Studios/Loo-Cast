private fn run_ecs_entity_primitives_test(world) {
    print("[Startup][Entity] primitive test");

    world.spawn_empty(|entity_world_mut| {
        let id = entity_world_mut.id();
        print("[Startup][Entity] id.to_string() = " + id.to_string());
    });

    world.commands(|commands| {
        commands.spawn_empty(|entity_commands| {
            let id = entity_commands.id();
            print("[Startup][Entity] commands id = " + id.to_string());
        });
    });
}
