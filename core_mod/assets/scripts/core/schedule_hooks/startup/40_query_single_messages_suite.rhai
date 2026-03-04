fn run_query_single_messages_suite(world) {
    print("[Startup][ECS] query/single/messages suite");

    let all_entities = world.query_entities();
    print("[Startup][ECS] query_entities.len = " + all_entities.len());

    let player_entities = world.query_players();
    print("[Startup][ECS] query_players.len = " + player_entities.len());

    if player_entities.len() != 1 {
        print("[Startup][ECS][WARN] Expected exactly one player entity");
        return;
    }

    let player_from_query = player_entities.single();
    let player_from_world_single = world.single_player();
    print("[Startup][ECS] query.single = " + player_from_query.to_string());
    print("[Startup][ECS] world.single_player = " + player_from_world_single.to_string());

    let maybe_player = world.try_single_player();
    if maybe_player == () {
        print("[Startup][ECS][WARN] try_single_player returned unit unexpectedly");
    }

    world.write_probe_message("startup-message-0");
    world.write_probe_message("startup-message-1");

    let drained_batch = world.read_probe_messages();
    print("[Startup][ECS] drained_batch.len = " + drained_batch.len());
    if drained_batch.len() != 2 {
        print("[Startup][ECS][WARN] Expected exactly two probe messages");
    }

    for payload in drained_batch.to_array() {
        print("[Startup][ECS] probe_message = " + payload);
    }

    let drained_again = world.read_probe_messages();
    if drained_again.len() != 0 {
        print("[Startup][ECS][WARN] Expected message drain to be empty on second read");
    }
}
