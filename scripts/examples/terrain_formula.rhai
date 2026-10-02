// Example: script owns the whole terrain-height formula.
//
// Enable the `worldgen/celestial_height.rhai` live target before using a
// formula like this. Return value is final radial displacement in metres.

fn height(ctx) {
    let continents = ctx.noise("continents");
    let mountains = ctx.noise("mountains");

    let broad = continents.fbm(4, 1.5, 2.0, 0.5);
    let ridges = mountains.ridged(6, 6.0, 2.0, 0.48);

    // Intentionally exaggerated to make live edits obvious.
    broad * 1800.0 + ridges.pow(3) * 6500.0
}
