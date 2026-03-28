use rhai_binding::schedule_hooks as ScheduleHooks;

for hook in [
    "pre_startup",
    "startup",
    "post_startup",
    "first",
    "pre_update",
    "substrate_pre_update",
    "zone_pre_update",
    "phenomenon_pre_update",
    "update",
    "substrate_update",
    "zone_update",
    "phenomenon_update",
    "post_update",
    "last",
] {
    ScheduleHooks::add(hook);
}

// USF substrate content registration now comes from typed scripts under:
// core_mod/scripts/usf/*
// Boot script keeps schedule-hook registration only.
