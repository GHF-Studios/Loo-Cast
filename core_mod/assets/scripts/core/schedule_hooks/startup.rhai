fn main(world) {
    print("Hello from 'Startup' schedule hook!");

    let old_player_bundle = PlayerBundle("default", ());
    let player_bundle = player::bundles::PlayerBundle::new_default();
    player_bundle.test_print();
    
    world.spawn_single(player_bundle, |ent_world_mut| {
        print("Spawned player: " + ent_world_mut.id().to_string());
    });
    world.flush();
}