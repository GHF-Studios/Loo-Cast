fn run_query_single_messages_suite(world) {
    print("[Startup][ECS] query/single/messages suite");

    let entity_data = ecs::query::QueryData::single("ecs::entities::Entity");
    let all_entities = world.query(entity_data);
    print("[Startup][ECS] world.query(QueryData::single(...)).remaining_len = " + all_entities.remaining_len());

    let player_filter = ecs::query::QueryFilter::require("player::components::Player");
    let player_entities = world.query_filtered(entity_data, player_filter);
    print("[Startup][ECS] world.query_filtered(..., QueryFilter::require(...)).remaining_len = " + player_entities.remaining_len());

    if player_entities.remaining_len() != 1 {
        print("[Startup][ECS][WARN] Expected exactly one player entity");
        return;
    }

    let player_from_query = player_entities.next();
    print("[Startup][ECS] query.next = " + player_from_query.to_string());

    let maybe_player = player_entities.next();
    if maybe_player == () {
        print("[Startup][ECS] query.next returned unit as expected after consuming the single player");
    } else {
        print("[Startup][ECS][WARN] query.next returned an unexpected second value");
    }

    let all_entities_again = world.query(entity_data);
    let all_entities_array = all_entities_again.collect_remaining();
    print("[Startup][ECS] query.collect_remaining count = " + all_entities_array.len());

    world.write_probe_message("startup-message-0");
    world.write_probe_message("startup-message-1");

    let drained_iter = world.drain_probe_messages();
    print("[Startup][ECS] drained_iter.remaining_len = " + drained_iter.remaining_len());
    if drained_iter.remaining_len() != 2 {
        print("[Startup][ECS][WARN] Expected exactly two probe messages");
    }

    loop {
        let payload = drained_iter.next();
        if payload == () {
            break;
        }
        print("[Startup][ECS] probe_message = " + payload);
    }

    let drained_again = world.drain_probe_messages();
    if !drained_again.is_empty() {
        print("[Startup][ECS][WARN] Expected message drain to be empty on second read");
    }
}
