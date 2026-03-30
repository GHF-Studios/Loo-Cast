use core_mod_api::usf::substrate as UsfSubstrate;

fn register_mod() {
    let mod_id = "demo";
    let scale_level_count = UsfSubstrate::scale_level_count();
    UsfSubstrate::set_usf_mod(
        mod_id,
        true,
        "usf_content/mods/demo/enabled",
    );
    UsfSubstrate::set_usf_mod_priority(mod_id, 0);
    UsfSubstrate::set_usf_mod_singleton_conflict_policy(mod_id, "scale", "replace_if_higher_priority");
    UsfSubstrate::set_usf_mod_singleton_conflict_policy(mod_id, "dpt_schema", "replace_if_higher_priority");
    UsfSubstrate::set_usf_mod_singleton_conflict_policy(mod_id, "zlm", "replace_if_higher_priority");

    UsfSubstrate::declare_mod_metric(mod_id, "solid_fill");
    UsfSubstrate::declare_mod_metric(mod_id, "root_pos_x");
    UsfSubstrate::declare_mod_metric(mod_id, "root_pos_y");
    UsfSubstrate::declare_mod_metric(mod_id, "root_pos_z");
    UsfSubstrate::declare_mod_metric_set(mod_id, "metric_set.demo.solid_fill");
    UsfSubstrate::declare_mod_zone(mod_id, "empty");
    UsfSubstrate::declare_mod_zone(mod_id, "spawn_buffer");
    UsfSubstrate::declare_mod_zone(mod_id, "solid");
    UsfSubstrate::declare_mod_phenomenon(mod_id, "phenomenon.demo.surface");
    UsfSubstrate::declare_mod_phenomenon_model(mod_id, "phenomenon_model.demo.surface.primary");

    for scale_index in 0..scale_level_count {
        UsfSubstrate::declare_mod_scale_binding(mod_id, scale_index);
        UsfSubstrate::declare_mod_dpt_schema(mod_id, scale_index);
        UsfSubstrate::declare_mod_zlm(mod_id, scale_index);
    }
}
