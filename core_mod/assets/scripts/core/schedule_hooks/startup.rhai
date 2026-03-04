fn main(world) {
    print("[Startup] bridge integration suite (entrypoint)");
    run_reflection_smoke_suite();
    run_world_commands_suite(world);
    run_player_bundle_suite(world);
    run_entity_primitives_suite(world);
    world.flush();
}
