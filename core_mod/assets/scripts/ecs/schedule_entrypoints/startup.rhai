use rhai_binding::testing as Testing;

fn main(world, params) {
    if !Testing::enabled() {
        return;
    }

    print("[Startup] startup test harness (entrypoint)");
    run_startup_test_catalog(world);
    world.flush();
}
