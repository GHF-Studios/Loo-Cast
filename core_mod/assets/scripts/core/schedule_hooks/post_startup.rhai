fn run_entity_primitives_suite(world) {
    print("[PostStartup] entity primitive suite");

    world.spawn_empty(|entity_world_mut| {
        let id = entity_world_mut.id();
        print("[PostStartup] id.to_string() = " + id.to_string());
        print("[PostStartup] id.index() = " + id.index().to_string());
        print("[PostStartup] id.gen() = " + id.gen().to_string());
    });

    world.commands(|commands| {
        commands.spawn_empty(|entity_commands| {
            let id = entity_commands.id();
            print("[PostStartup] commands id = " + id.to_string());
        });
    });

    world.flush();
}

fn main(world) {
    run_entity_primitives_suite(world);
}
