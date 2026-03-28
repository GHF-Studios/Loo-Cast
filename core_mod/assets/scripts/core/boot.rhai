use rhai_binding::schedule_hooks as ScheduleHooks;
use core_mod_api::usf::zone as UsfZone;

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
// Boot script keeps schedule hooks and zone behavior defaults.
UsfZone::clear_phenomenon_kinds();
UsfZone::set_phenomenon_kind("void", "mandelbulb");
UsfZone::set_phenomenon_kind("arid", "mandelbulb");
UsfZone::set_phenomenon_kind("alpine", "mandelbulb");
UsfZone::set_phenomenon_kind("forest", "sierpinski_sponge");
UsfZone::set_phenomenon_kind("wetland", "sierpinski_sponge");

UsfZone::clear_density_profiles();
UsfZone::set_density_profile("void", 0.15, 0.00, 0.00, 0.25);
UsfZone::set_density_profile("arid", 0.45, 0.05, 0.00, 0.55);
UsfZone::set_density_profile("alpine", 0.55, 0.10, 0.00, 0.72);
UsfZone::set_density_profile("forest", 0.72, 0.14, 0.05, 0.88);
UsfZone::set_density_profile("wetland", 0.78, 0.20, 0.10, 0.95);
