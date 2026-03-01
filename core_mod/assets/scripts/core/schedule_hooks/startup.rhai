fn main(world) {
    print("Hello from 'Startup' schedule hook!");

    // Simple but (mostly) comprehensive test for the brand new scripting framework
    shop::divisions::sex::test_function();
    shop::divisions::sex::SexShopProduct::test();
    let sus_item = shop::divisions::sex::SexShopProduct::new_("SuspiciouslyPenisShapedItem", 9.99);
    let name = sus_item.name();
    let price_usd = sus_item.price_usd();
    print("Created SexShopProduct '" + name + "' for " + price_usd + "USD");
    shop::divisions::sex::SexShopProduct::verify_price(price_usd);

    // Advanced scripting test, but for the outdated scripting "framework"
    let player_bundle = player::bundles::PlayerBundle::new_default();
    
    player_bundle.test_print();
    
    let player_bundle = player::bundles::PlayerBundle::as_trait_obj(player_bundle, "ecs::bundle::Bundle");
    
    world.spawn_single(player_bundle, |ent_world_mut| {
        print("Spawned player: " + ent_world_mut.id().to_string());
    });
    world.flush();
}