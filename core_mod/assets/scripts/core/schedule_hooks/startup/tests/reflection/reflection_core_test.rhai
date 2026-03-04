use bevy::ecs as Ecs;

private fn run_reflection_core_test() {
    print("[Startup][Reflection] core smoke test");
    print("[Startup][Reflection] " + Ecs::bridge_info());
}
