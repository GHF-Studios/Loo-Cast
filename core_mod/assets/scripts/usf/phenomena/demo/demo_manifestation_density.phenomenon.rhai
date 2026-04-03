fn register_phenomenon(ctx) {
    ctx.set_id("demo_manifestation_density");
    ctx.register("manifestation_density_debug");
    ctx.add_capability("manifestation_material_profile");
    ctx.add_capability("manifestation_collider");
    ctx.add_capability("manifestation_audio_emitter");
    ctx.add_capability("manifestation_particle_emitter");
    ctx.add_capability("interaction_trigger");
}
