fn run_ecs_player_bundle_suite(world) {
    print("[Startup][Player] bundle suite");

    let player_bundle = core_mod_api::player::bundles::PlayerBundle::new_default();
    player_bundle.test_print();
    let player_bundle =
        core_mod_api::player::bundles::PlayerBundle::as_trait_obj(player_bundle, "bevy::ecs::bundle::Bundle");

    world.spawn_single(player_bundle, |ent_world_mut| {
        print("[Startup][Player] spawned via trait object: " + ent_world_mut.id().to_string());
    });
}
