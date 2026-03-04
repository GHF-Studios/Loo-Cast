fn run_query_single_messages_suite(world) {
    print("[Startup][ECS] query/single/messages suite");

    let entity_data = ecs::query::QueryData::of("ecs::entities::Entity");
    let all_entities = world.query(entity_data);
    print("[Startup][ECS] world.query(QueryData::of(...)).len = " + all_entities.len());

    let player_filter = ecs::query::QueryFilter::of("player::components::Player");
    let player_entities = world.query_filtered(entity_data, player_filter);
    print("[Startup][ECS] world.query_filtered(..., QueryFilter::of(...)).len = " + player_entities.len());

    if player_entities.len() != 1 {
        print("[Startup][ECS][WARN] Expected exactly one player entity");
        return;
    }

    let player_from_query = player_entities.single();
    let player_from_try_single = player_entities.try_single();
    print("[Startup][ECS] query.single = " + player_from_query.to_string());
    print("[Startup][ECS] query.try_single = " + player_from_try_single.to_string());

    let maybe_player = player_entities.try_single();
    if maybe_player == () {
        print("[Startup][ECS][WARN] query.try_single returned unit unexpectedly");
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
