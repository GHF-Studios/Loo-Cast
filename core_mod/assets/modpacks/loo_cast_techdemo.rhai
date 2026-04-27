fn register_modpack(ctx) {
    let modpack = ctx.start();

    modpack.name("Loo Cast Techdemo Modpack");
    modpack.mods(["loo_cast_techdemo"]);

    ctx.end(modpack);
}
