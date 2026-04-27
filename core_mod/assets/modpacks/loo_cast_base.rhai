fn register_modpack(ctx) {
    let modpack = ctx.start();

    modpack.name("Loo Cast Base Modpack");
    modpack.mods(["loo_cast_base"]);

    ctx.end(modpack);
}
