fn register_mod(ctx) {
    ctx.register();
    ctx.set_priority(0);
    ctx.set_singleton_conflict_policy("scale", "replace_if_higher_priority");
    ctx.set_singleton_conflict_policy("dpt_schema", "replace_if_higher_priority");
    ctx.set_singleton_conflict_policy("zlm", "replace_if_higher_priority");
}
