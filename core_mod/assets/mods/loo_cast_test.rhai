fn register_mod(ctx) {
    let mod = ctx.start();

    mod.name("Loo Cast Test Mod");
    mod.scales([
        
    ]);
    mod.metrics([
        "test.test_metric"
    ]);
    mod.phenomena([
        "test.test_phenomenon"
    ]);

    ctx.end(mod);
}
