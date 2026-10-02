// Reconstructible celestial presentation-height policy.
//
// This script owns the FINAL terrain-height formula for the scripted
// presentation surface. The host owns coordinates, seed, semantic scale,
// bounds, scheduling, revision invalidation and publication.
//
// Built-in Rhai math includes sin/cos/tan/sqrt/exp/ln/log/min/max/abs plus
// PI and E. Loo-Cast also adds:
//   value.pow(4) / value.pow(1.5)
//   lerp(a, b, t), saturate(x), smoothstep(a, b, x), remap(...)
//   print(...), debug(...), log_info(...), log_warn(...), log_error(...)
//
// TerrainContext:
//   ctx.canonical_height  // metres from the normal semantic terrain
//   ctx.radius            // body radius in metres
//   ctx.seed              // deterministic body seed
//   ctx.scale             // semantic Scale exponent
//   ctx.x / ctx.y / ctx.z // body-local unit direction
//
// Deterministic NoiseField:
//   let noise_rng = ctx.noise("mountains");
//   noise_rng.sample();                    // unit-frequency spherical noise
//   noise_rng.sample(8.0);                 // explicit frequency
//   noise_rng.sample_wavelength(500000.0); // approximate wavelength in metres
//   noise_rng.fbm(5, 2.0);                 // octaves, frequency
//   noise_rng.fbm(5, 2.0, 2.0, 0.5);       // + lacunarity, gain
//   noise_rng.ridged(5, 4.0);
//
// Example replacement formula:
//   let noise_rng = ctx.noise("base");
//   let value = noise_rng.fbm(5, 2.0, 2.0, 0.5);
//   value.pow(4) * 8000.0
//
// Default: preserve canonical semantic terrain.
fn height(ctx) {
    ctx.canonical_height
}
