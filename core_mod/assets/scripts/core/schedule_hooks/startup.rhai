fn main(world) {
    print("Hello from 'Startup' schedule hook!");

    // Reflection graph sanity checks (SexShop sketch module).
    shop::divisions::sex::test_function();
    shop::divisions::sex::SexShopProduct::test();
    let sus_item = shop::divisions::sex::SexShopProduct::new_("SuspiciouslyPenisShapedItem", 9.99);
    let name = sus_item.name();
    let price_usd = sus_item.price_usd();
    print("Created SexShopProduct '" + name + "' for " + price_usd + "USD");
    shop::divisions::sex::SexShopProduct::verify_price(price_usd);

    // ECS chain checks:
    // World -> Commands -> EntityCommands -> Commands -> EntityCommands
    world.commands(|commands| {
        commands.spawn_empty(|entity_commands| {
            print("Spawned empty via World::commands: " + entity_commands.id().to_string());

            entity_commands.commands(|commands_from_entity| {
                commands_from_entity.spawn_empty(|nested_entity_commands| {
                    print(
                        "Spawned empty via EntityCommands::commands: " +
                        nested_entity_commands.id().to_string()
                    );
                });
            });
        });
    });

    world.spawn_empty(|entity_world_mut| {
        print("Spawned empty via World::spawn_empty: " + entity_world_mut.id().to_string());
    });

    // Player bundle path (value-semantics + trait-object conversion).
    let player_bundle = player::bundles::PlayerBundle::new_default();
    player_bundle.test_print();
    let player_bundle = player::bundles::PlayerBundle::as_trait_obj(player_bundle, "ecs::bundle::Bundle");

    world.spawn_single(player_bundle, |ent_world_mut| {
        print("Spawned player via trait object: " + ent_world_mut.id().to_string());
    });

    world.flush();
}
