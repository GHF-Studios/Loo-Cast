fn main(world) {
    // print("Hello from 'Startup' schedule hook!");
    world.commands(|cmds| {
        let entity = cmds.spawn_empty(|ent_cmds| {
            print("Spawned an entity");
        });
    });
}