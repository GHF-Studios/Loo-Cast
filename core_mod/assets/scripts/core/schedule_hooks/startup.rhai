fn run_world_commands_suite(world) {
    print("[Startup] world/commands suite");

    world.commands(|commands| {
        commands.spawn_empty(|entity_commands| {
            let id = entity_commands.id();
            print("[Startup] World::commands -> spawn_empty: " + id.to_string());
            print(
                "[Startup] entity id parts: index=" +
                id.index().to_string() +
                ", gen=" +
                id.gen().to_string()
            );

            entity_commands.commands(|commands_from_entity| {
                commands_from_entity.spawn_empty(|nested_entity_commands| {
                    let nested_id = nested_entity_commands.id();
                    print(
                        "[Startup] EntityCommands::commands -> spawn_empty: " +
                        nested_id.to_string()
                    );
                });
            });
        });
    });

    world.spawn_empty(|entity_world_mut| {
        let id = entity_world_mut.id();
        print("[Startup] World::spawn_empty: " + id.to_string());
    });
}

fn run_player_bundle_suite(world) {
    print("[Startup] player bundle suite");

    let player_bundle = player::bundles::PlayerBundle::new_default();
    player_bundle.test_print();
    let player_bundle = player::bundles::PlayerBundle::as_trait_obj(player_bundle, "ecs::bundle::Bundle");

    world.spawn_single(player_bundle, |ent_world_mut| {
        print("[Startup] Spawned player via trait object: " + ent_world_mut.id().to_string());
    });
}

fn main(world) {
    print("[Startup] bridge integration suite");
    run_world_commands_suite(world);
    run_player_bundle_suite(world);
    world.flush();
}
