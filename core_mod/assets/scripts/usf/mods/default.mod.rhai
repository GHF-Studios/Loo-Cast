use core_mod_api::usf::substrate as UsfSubstrate;

fn register_mod() {
    let mod_id = "mod.placeholder_gameplay.v1";
    let scale_level_count = UsfSubstrate::scale_level_count();
    UsfSubstrate::set_usf_mod(
        mod_id,
        true,
        "usf_content/mods/placeholder_gameplay/enabled",
    );
    UsfSubstrate::set_usf_mod_priority(mod_id, 0);
    // Base package explicitly allows higher-priority packages to override singleton scale contracts.
    UsfSubstrate::set_usf_mod_singleton_conflict_policy(mod_id, "scale_binding", "replace_if_higher_priority");
    UsfSubstrate::set_usf_mod_singleton_conflict_policy(mod_id, "dpt_schema", "replace_if_higher_priority");
    UsfSubstrate::set_usf_mod_singleton_conflict_policy(mod_id, "zlm", "replace_if_higher_priority");

    // Mod manifest: authoritative declaration of what this mod must provide.
    UsfSubstrate::declare_mod_metric(mod_id, "solid_fill");
    UsfSubstrate::declare_mod_metric_set(mod_id, "metric_set.placeholder.solid_fill.v1");
    UsfSubstrate::declare_mod_zone(mod_id, "empty");
    UsfSubstrate::declare_mod_zone(mod_id, "solid");
    UsfSubstrate::declare_mod_phenomenon(mod_id, "phenomenon.placeholder.metric_surface_debug");
    UsfSubstrate::declare_mod_phenomenon_model(mod_id, "phenomenon_model.placeholder.metric_surface_debug.primary");

    // Explicitly declare mod ownership for each scale requirement.
    for scale_index in 0..scale_level_count {
        UsfSubstrate::declare_mod_scale_binding(mod_id, scale_index);
        UsfSubstrate::declare_mod_dpt_schema(mod_id, scale_index);
        UsfSubstrate::declare_mod_zlm(mod_id, scale_index);
    }
}
