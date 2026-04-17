fn register_mod(ctx) {
    let mod = ctx.start();

    mod.name("Loo Cast Base Mod");
    mod.scales([

    ]);
    mod.metrics([
        "baryons",
        "dark_energy",
        "dark_matter",
        "photons"
    ]);
    mod.phenomena([
        "universal.undefined", "universal.universe",
        "cosmological.cosmic_web",
        "galactic.galaxy_wall", "galactic.galaxy_filament", "galactic.galaxy_sheet", "galactic.galaxy_supercluster", "galactic.galaxy_cluster", "galactic.galaxy_group", "galactic.galaxy"
    ]);

    ctx.end(mod);
}
