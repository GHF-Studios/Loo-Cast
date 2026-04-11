fn register_modpack(ctx) {
    let modpack = ctx.start();

    modpack.id("loo_cast");
    modpack.name("Loo Cast");
    modpack.mods(["loo_cast_base"]);

    ctx.end(modpack);
}
