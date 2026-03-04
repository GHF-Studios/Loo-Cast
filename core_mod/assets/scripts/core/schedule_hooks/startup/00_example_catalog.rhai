fn run_startup_example_catalog(world) {
    run_reflection_core_suite();
    run_testing_bridge_smoke_suite();
    run_ecs_world_commands_suite(world);
    run_ecs_player_bundle_suite(world);
    run_ecs_entity_primitives_suite(world);
    run_ecs_query_messages_iterators_suite(world);
}
