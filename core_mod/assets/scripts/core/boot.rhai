use rhai_binding::schedule_hooks as ScheduleHooks;

for hook in [
    "pre_startup",
    "startup",
    "post_startup",
    "first",
    "pre_update",
    "update",
    "post_update",
    "last",
] {
    ScheduleHooks::add(hook);
}
