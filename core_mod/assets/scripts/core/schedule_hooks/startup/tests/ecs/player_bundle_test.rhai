use core_mod_api::player::bundles::PlayerBundle as PlayerBundle;
use bevy::ecs::bundle::Bundle as Bundle;

private fn run_ecs_player_bundle_test(world) {
    print("[Startup][Player] bundle test");

    let player_bundle = PlayerBundle::new_default();
    player_bundle.test_print();
    let player_bundle = PlayerBundle::as_trait_obj(player_bundle, Bundle);

    world.spawn_single(player_bundle, |ent_world_mut| {
        print("[Startup][Player] spawned via trait object: " + ent_world_mut.id().to_string());
    });
}
