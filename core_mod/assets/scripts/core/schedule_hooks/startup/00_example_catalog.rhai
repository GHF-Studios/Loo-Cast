private fn run_startup_example_catalog(world) {
    run_startup_production_suites(world);
    run_startup_testing_suites();
}

private fn run_startup_production_suites(world) {
    run_reflection_core_suite();
    run_ecs_world_commands_suite(world);
    run_ecs_player_bundle_suite(world);
    run_ecs_entity_primitives_suite(world);
    run_ecs_query_messages_iterators_suite(world);
}

private fn run_startup_testing_suites() {
    if rhai_binding::testing::enabled() {
        run_testing_bridge_smoke_suite();
    } else {
        print("[Startup][Testing] skipped (set rhai_binding/testing_enabled = true in config.toml)");
    }
}
