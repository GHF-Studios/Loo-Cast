fn main(world) {
    print("Hello from 'Startup' schedule hook!");

    // world.commands(|cmds| {
    //     let ent_id = cmds.spawn_empty(|ent_cmds| {
    //         
    //         ent_cmds.id
    //     });
    // 
    //     print("Spawned an entity: " + ent_id.to_string());
    // });

    world.spawn_single(Bundle(#{"Player": #{}}), |ent_world_mut| {
        print("Spawned player: " + ent_world_mut.id.to_string());
    });
}