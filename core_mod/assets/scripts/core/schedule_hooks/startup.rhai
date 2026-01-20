fn main(world) {
    print("Hello from 'Startup' schedule hook!");

    // Spawn a player using the PlayerBundle with default constructor
    world.spawn_single(PlayerBundle("default", ()), |ent_world_mut| {
        print("Spawned player: " + ent_world_mut.id().to_string());
    });
    
    world.flush();
}