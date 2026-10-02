// Reconstructible celestial presentation-height policy.
//
// TEST PLANET PRESET
// ------------------
// Deliberately exaggerated Earth-ish terrain for exercising multiscale
// realization, streaming, collision, high-speed travel and presentation.
//
// This is still a radial height field: it can produce mountains, trenches,
// ridges and canyon-like valleys, but not true caves/overhangs. Those require
// a volumetric density-field scripting contract instead of height(direction).
//
// `ctx.canonical_height` remains underneath this preset, so the script augments
// semantic terrain rather than replacing the field with an unrelated sphere.

fn height(ctx) {
    let radius = ctx.radius;

    // Very broad uplift/subsidence: continent-sized highlands and basins.
    let continents = ctx.noise("continents")
        .fbm(5, radius / 2400000.0, 2.0, 0.52);
    let broad_relief = continents * 3200.0;

    // Broad tectonic mask. Mountain ranges only dominate selected regions,
    // rather than covering the entire planet uniformly.
    let tectonics = ctx.noise("tectonic-belts")
        .fbm(4, radius / 3200000.0, 2.05, 0.54);
    let mountain_mask = smoothstep(-0.30, 0.50, tectonics);

    // Sharp kilometer-scale range skeleton. Ridged noise + a high power makes
    // narrow high peaks instead of rounded Perlin blobs.
    let ridge = ctx.noise("major-ridges")
        .ridged(7, radius / 650000.0, 2.05, 0.52);
    let ridge01 = saturate(ridge * 0.5 + 0.5);
    let major_mountains =
        ridge01.pow(5) * mountain_mask * 15500.0;

    // Smaller nested ridges break the major ranges into believable-looking
    // serrated peaks and secondary chains.
    let secondary = ctx.noise("secondary-ridges")
        .ridged(6, radius / 170000.0, 2.12, 0.48);
    let secondary01 = saturate(secondary * 0.5 + 0.5);
    let secondary_mountains =
        secondary01.pow(4) * mountain_mask * 3600.0;

    // Narrow negative features create deep valleys / canyon networks. They are
    // strongest inside mountainous regions but still appear in uplands.
    let valley_field = ctx.noise("valley-network")
        .fbm(6, radius / 420000.0, 2.0, 0.50);
    let valley_line =
        saturate(1.0 - abs(valley_field) * 2.35).pow(5);
    let valleys =
        -valley_line * (2200.0 + mountain_mask * 5200.0);

    // Mid/fine roughness so slopes are not perfectly smooth between the giant
    // forms. Kept relatively small so it does not turn into visual static.
    let roughness = ctx.noise("rock-detail")
        .fbm(5, radius / 70000.0, 2.18, 0.46) * 850.0;

    ctx.canonical_height
        + broad_relief
        + major_mountains
        + secondary_mountains
        + valleys
        + roughness
}
