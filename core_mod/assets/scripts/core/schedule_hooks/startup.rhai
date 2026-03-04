fn main(world) {
    print("[Startup] bridge integration suite (entrypoint)");
    run_startup_example_catalog(world);
    world.flush();
}
