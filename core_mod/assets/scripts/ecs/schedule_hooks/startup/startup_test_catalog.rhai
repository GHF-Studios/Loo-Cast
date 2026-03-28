private fn run_startup_test_catalog(world) {
    run_startup_core_tests(world);
    run_startup_example_tests();
}

private fn run_startup_core_tests(world) {
    run_reflection_core_test();
    run_ecs_world_commands_test(world);
    run_ecs_player_bundle_test(world);
    run_ecs_entity_primitives_test(world);
    run_ecs_query_messages_iterators_test(world);
}

private fn run_startup_example_tests() {
    run_shop_working_example_test();
}
