fn register_mod(ctx) {
    let mod = ctx.start();

    mod.id("loo_cast_base");
    mod.name("Loo Cast Base Mod");
    mod.metrics(["baryons", "dark_energy", "dark_matter", "photons"]);

    ctx.end(mod);
}
