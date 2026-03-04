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
    rhai_binding::schedule_hooks::add(hook);
}
