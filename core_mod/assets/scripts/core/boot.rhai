use rhai_binding::schedule_entrypoints as ScheduleEntrypoints;

for entrypoint in [
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
    ScheduleEntrypoints::add(entrypoint);
}

// USF substrate content registration now comes from typed scripts under:
// core_mod/scripts/usf/*
// Boot script keeps schedule-entrypoint registration only.
