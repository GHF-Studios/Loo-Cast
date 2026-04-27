fn register_modpack(ctx) {
    let modpack = ctx.start();

    modpack.name("Loo Cast Test Modpack");
    modpack.mods(["loo_cast_test"]);

    ctx.end(modpack);
}
