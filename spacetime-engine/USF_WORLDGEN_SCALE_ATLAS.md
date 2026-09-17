# USF World Generation Scale Atlas (+35 → 0)

**Status:** design baseline for the first top-down world-generation implementation
**Purpose:** define enough semantic meaning across all 36 non-negative spatial scales that Scale 0 can be generated as a consequence of higher-scale state instead of as an independent terrain generator.

This document is intentionally broader than `USF_SPATIAL_WORLD_ROADMAP.md`. The spatial roadmap defines the addressing/realization skeleton. This atlas defines the **domain meaning carried through that skeleton**.

## 0. Core stance

USF is not a 71-octave noise pyramid.

The first world-generation path must begin at Scale `+35`, establish a coarse universe state, and refine downward through every spatial scale until Scale `0`.

Adjacent scales may share the same physical domain or Phenomenon family. That is expected. What must not happen is silently skipping the semantic interpretation of a scale.

The first implementation is allowed to be phenomenological rather than physically exhaustive:

- qualitative realism over full first-principles cosmology,
- deterministic/coherent refinement over brute-force simulation,
- present-day-ish bootstrap parameters may be hardcoded,
- only demanded branches need semantic refinement/materialization,
- upward reaggregation is still deferred,
- lower-than-zero detail is future work.

The architecture must nevertheless make it possible to replace approximations later without changing the top-down contract.

---

# 1. Scale size convention

For spatial scale `S`:

- one native unit is `10^S` metres,
- one USF Chunk spans `1000` native units,
- therefore one USF Chunk spans `10^(S+3)` metres,
- one chunk at `S` contains exactly `10 × 10 × 10` child chunks at `S-1`.

**Domain assignment below follows chunk span more than native-unit size.**

This is important. For example, a Scale `+18` chunk spans about `10^21 m`, roughly `106,000` light-years: a natural galaxy-scale container even though the native unit at `+18` is much larger than a metre.

The `+35` root grid remains intentionally much larger than the observable universe. The game universe may therefore carry statistically homogeneous present-day cosmological state far beyond one observable horizon.

---

# 2. The 36-scale atlas

The entries below are **dominant concerns**, not exclusivity rules. Several Phenomena may overlap the same scale, and one Phenomenon family may span many scales.

| Scale | Approx. USF Chunk span | Dominant domain | Meaningful semantic state | Typical downward refinement |
|---:|---:|---|---|---|
| +35 | 1e38 m | Root cosmology / global statistical state | Global topology; present-day bootstrap epoch; mean energy/matter composition; background temperature; primordial-spectrum parameters | Seed the longest-wavelength perturbation band. No discrete objects. |
| +34 | 1e37 m | Ultra-long cosmological perturbations | Parent mean state + first spatial fluctuation band | Refine statistically consistent density/potential/velocity modes. |
| +33 | 1e36 m | Ultra-long cosmological perturbations | Long-wavelength density contrast, potential and bulk-flow summaries | Add the next shorter perturbation band; still no objects. |
| +32 | 1e35 m | Ultra-long cosmological perturbations | Coarse matter/radiation contrast and tidal context | Refine fields; preserve parent averages and boundary coherence. |
| +31 | 1e34 m | Ultra-long cosmological perturbations | Coarse density, potential, velocity/tidal statistics | Refine fields; no galaxy/halo identity yet. |
| +30 | 1e33 m | Ultra-long cosmological perturbations | Statistical cosmological field state | Inject shorter modes conditioned on parent state. |
| +29 | 1e32 m | Ultra-long cosmological perturbations | Statistical cosmological field state | Continue spectral refinement; still effectively homogeneous locally. |
| +28 | 1e31 m | Ultra-long cosmological perturbations | Statistical cosmological field state | Continue spectral refinement with exact canonical child addressing. |
| +27 | 1e30 m | Ultra-long cosmological perturbations | Statistical cosmological field state | Begin allowing weak large-scale anisotropy/flow structure to become spatially meaningful. |
| +26 | 1e29 m | Pre-web cosmological structure | Density/potential/flow fields with increasingly resolved contrast | Refine toward future void/wall/filament structure. |
| +25 | 1e28 m | Largest cosmological structure modes | Longest structure-forming modes that can matter to observable-scale regions | Refine toward horizon-scale cosmic-web state. |
| +24 | 1e27 m (~106 Gly) | Observable-universe-order cosmology | Horizon-scale matter/radiation distribution, expansion-era parameters, giant void/wall seeds | Produce coherent cosmic-web metrics; do not treat the observable horizon as a hard world boundary. |
| +23 | 1e26 m (~10.6 Gly) | Supervoid / wall / filament complexes | Large-scale density, potential and bulk-flow fields | Concentrate matter into coherent web regions; identify high-level basins/ridges. |
| +22 | 1e25 m (~1.06 Gly) | Cosmic web | Voids, walls, filaments, nodes; halo-mass potential | Refine filament/node structure and candidate cluster environments. |
| +21 | 1e24 m (~106 Mly) | Supercluster / large-void scale | Cluster-node fields, filament topology, large-scale tidal environment | Emit cluster/group population seeds conditioned by the web. |
| +20 | 1e23 m (~10.6 Mly) | Galaxy-cluster / group environment | Halo population statistics; intra-cluster medium summary; merger/tidal context | Refine individual massive halos, groups and their baryon reservoirs. |
| +19 | 1e22 m (~1.06 Mly) | Galaxy halo / local-group scale | Dark-matter halo state, circumgalactic gas, satellite population potential | Emit galaxy-scale structures, satellite/stream seeds and inflow context. |
| +18 | 1e21 m (~106 kly) | Galaxy scale | Disk/bulge/halo morphology; stellar/gas mass fields; metallicity and angular-momentum structure | Refine spiral/bar/elliptical structure, ISM distribution and stellar populations. |
| +17 | 1e20 m (~10.6 kly) | Galactic region / spiral-arm scale | Local stellar density, gas phase, radiation, metallicity, shear, gravitational/tidal field | Emit star-forming complexes, old stellar populations, clouds and remnants. |
| +16 | 1e19 m (~1.06 kly) | Interstellar complex scale | Molecular/atomic gas structure, stellar associations, radiation and feedback fields | Refine giant molecular clouds, clusters, bubbles and dense regions. |
| +15 | 1e18 m (~106 ly) | Giant molecular cloud / star-forming region | Cloud mass, turbulence, temperature, chemistry/metallicity summary, feedback and collapse propensity | Emit cloud substructure, protostellar cores and stellar-population seeds. |
| +14 | 1e17 m (~10.6 ly) | Stellar neighborhood / cluster scale | Star population, nearby cloud structure, radiation/wind/tidal environment | Refine individual stellar systems and local interstellar medium. |
| +13 | 1e16 m (~1.06 ly) | Outer stellar-domain scale | Host-star gravity/radiation, companion-star context, outer comet reservoir statistics | Emit individual system outskirts and long-period body populations. |
| +12 | 1e15 m (~6,700 AU) | Oort-cloud / system-outskirts scale | Outer-system body density, stellar tide, passing-star perturbation context | Refine comet reservoirs and bound/unbound small-body populations. |
| +11 | 1e14 m (~670 AU) | Outer planetary-system scale | Distant body populations, disk remnants, companion effects | Refine Kuiper/scattered-disc-like structures and major orbital architecture. |
| +10 | 1e13 m (~67 AU) | Planetary-system scale | Star, major planets, minor-body belts, radiation/wind, orbital resonances | Emit individual planetary bodies and local orbital neighborhoods. |
| +9 | 1e12 m (~6.7 AU) | Inner planetary-system scale | Planet/moon orbital regions, belts, local plasma/radiation environment | Refine individual body neighborhoods, rings, moons and debris. |
| +8 | 1e11 m (~0.67 AU) | Local orbital / stellar-environment scale | Near-body or near-star field, magnetosphere/wind context, body encounter state | Refine star/planet envelope and large body-bound fields. |
| +7 | 1e10 m (~10 million km) | Stellar / planetary envelope scale | Stellar corona/wind or planetary magnetosphere/large satellite system | Refine dominant body and its immediate bound environment. |
| +6 | 1e9 m (~1 million km) | Star / giant-planet body scale | Body mass/radius/rotation; coarse interior/envelope; atmospheric or plasma layers | Emit body-local structural fields; distinguish solid/liquid/gas/plasma domains. |
| +5 | 1e8 m (~100,000 km) | Global giant-planet / small-star / planetary-envelope scale | Global fluid layers, circulation, field geometry, major storms or moons | Refine global atmospheric/oceanic/interior structures. |
| +4 | 1e7 m (~10,000 km) | Terrestrial-planet global scale | Planet shape/gravity, crust/ocean/atmosphere inventory, rotation/insolation, global climate and tectonic state | Emit continents/ocean basins, tectonic provinces, climate circulation and hydrologic reservoirs. |
| +3 | 1e6 m (~1,000 km) | Continental / ocean-basin scale | Tectonics, elevation potential, lithology, ocean/ice, synoptic climate fields | Refine mountain belts, basins, drainage, weather regimes and broad ecosystems. |
| +2 | 1e5 m (~100 km) | Regional landscape / weather / hydrology scale | Regional geology, erosion potential, rivers, groundwater, soils, mesoscale climate, disturbance | Emit watersheds, valleys, local substrate fields and ecosystem-region seeds. |
| +1 | 1e4 m (~10 km) | Landscape / biome-mosaic scale | Terrain form, soil/substrate, hydrology, microclimate, disturbance history, colonization/resource fields | Resolve biome/ecosystem mosaics, forest/grassland/wetland/desert phenomena and major cave/rock structures. |
| 0 | 1e3 m (1 km) | Local terrain / ecosystem / coarse material scale | Meter-native terrain density, substrate/material composition summaries, water, soil state, local climate, ecosystem populations, caves/overhangs | Materialize playable terrain and local phenomena. Seed trees/vegetation/objects; finer anatomy/material microstructure continues below 0. |

---

# 3. Phenomenon families across the atlas

The scale rows above are not meant to become 36 hardcoded engine enums. They are an atlas describing where different self-hosted Phenomena become meaningful.

A first useful family map is:

## 3.1 CosmologicalBackground

**Typical scales:** `+35 .. +24`

Carries:

- cosmological epoch / bootstrap era,
- mean matter/energy composition,
- mean background radiation temperature,
- expansion/background parameters,
- statistical fluctuation-spectrum parameters,
- long-wavelength density/potential/velocity/tidal fields.

Refinement behavior:

- preserve parent averages,
- add shorter-wavelength perturbation bands as scale decreases,
- keep boundaries/neighbors coherent,
- do not invent discrete galaxies while the scale is too coarse.

This is the natural place for a mostly homogeneous top-level state.

## 3.2 CosmicMatterDistribution

**Typical scales:** `+24 .. +20`

Carries:

- matter-density contrast,
- gravitational-potential/tidal summaries,
- cosmic-web topology,
- void/wall/filament/node metrics,
- halo-population potential.

Refinement behavior:

- turn statistical perturbations into increasingly localized cosmic-web structure,
- emit seeds for halo/cluster populations where collapse is plausible,
- preserve the coarse mass budget inherited from the parent.

## 3.3 Halo / Galaxy Environment

**Typical scales:** `+20 .. +18`

Carries:

- halo mass and shape,
- angular momentum,
- merger/tidal context,
- circumgalactic/intracluster baryon reservoirs,
- satellite potential.

Refinement behavior:

- emit galaxies, satellites, streams, gas reservoirs,
- derive morphology tendencies from inherited angular momentum / merger / environment state,
- remain compatible with parent halo mass and matter fields.

## 3.4 GalaxyStructure / InterstellarMedium

**Typical scales:** `+18 .. +15`

Carries:

- disk/bulge/halo structure,
- stellar population fields,
- gas phases,
- metallicity,
- radiation,
- shear/turbulence,
- star-formation potential,
- feedback bubbles.

Refinement behavior:

- emit arm/bar/local-region structure,
- produce molecular-cloud complexes and stellar associations,
- propagate metallicity/radiation/turbulence context downward.

## 3.5 StellarFormation / StellarPopulation

**Typical scales:** `+16 .. +14`

Carries:

- cloud density/turbulence/temperature,
- collapse propensity,
- stellar population age/mass distribution,
- feedback,
- local radiation/winds.

Refinement behavior:

- emit stars and stellar-system seeds,
- emit remnants / clusters / unbound populations,
- pass host-environment chemistry and radiation to the systems below.

## 3.6 StellarSystem / OrbitalEnvironment

**Typical scales:** `+14 .. +8`

Carries:

- star/companion state,
- planetary-system architecture,
- disk/remnant population,
- major bodies,
- orbital resonances,
- radiation/wind,
- comet/small-body reservoirs.

Refinement behavior:

- emit individual planets/moons/belts/rings,
- refine local orbital neighborhoods,
- pass irradiation, tides, impactor environment and elemental inventory to body-local Phenomena.

## 3.7 StellarBody / PlanetaryBody

**Typical scales:** `+8 .. +4`

Carries:

- mass/radius/rotation,
- gravity,
- bulk composition,
- interior/envelope phase structure,
- atmosphere/ocean inventory,
- magnetic/plasma environment,
- insolation and tidal forcing.

Refinement behavior:

- separate interior, crust, ocean, atmosphere and plasma domains,
- emit global tectonic/climate/hydrologic state,
- establish the boundary conditions for geology and ecology.

## 3.8 Geology / Climate / Hydrology

**Typical scales:** `+4 .. 0`

Carries:

- lithology / crustal structure,
- tectonics,
- elevation potential,
- erosion/deposition,
- ocean/ice,
- atmospheric circulation,
- precipitation/temperature/radiation fields,
- rivers/groundwater,
- disturbance history.

Refinement behavior:

- generate terrain as a true volumetric/material field rather than a heightmap-only authority,
- generate drainage, caves, overhangs, strata, soils and local substrate state,
- provide ecological boundary conditions.

## 3.9 Ecology / Biome

**Typical scales:** `+2 .. 0` initially, continuing below `0` later

A biome must **not** be:

```text
temperature × humidity × height -> enum
```

Instead, biome/ecosystem state is a characterization/emergent result of interacting Phenomena:

- climate and seasonality,
- water availability and hydrology,
- substrate / chemistry / nutrients,
- disturbance (fire, flood, impact, erosion, disease, etc.),
- colonization history,
- competition,
- succession,
- resource/light fields,
- local topology.

A forest is therefore not merely a texture or density mask. It can be a localized ecological Phenomenon carrying population structure that later refines into stands, individual trees, branches, leaves, dead matter, fungi, etc.

At Scale `0`, individual vegetation may already exist as coarse localized entities/state while its detailed anatomy is deferred to negative scales.

## 3.10 Material / Substrate State

**Begins to matter strongly around `+1 .. 0`; continues deeply below `0`.**

Scale `0` is only metre-native, so this is not yet molecular material simulation. It is enough to carry meaningful coarse composition/state such as:

- material fractions / dominant constituents,
- porosity,
- compaction,
- water content,
- temperature,
- fracture/damage,
- organic content,
- phase / saturation,
- mechanically relevant derived properties.

This state is semantic.

Presentation and physics derive from it.

Example:

```text
clay composition + water content + compaction + temperature
                |
                +--> appearance: darker, smoother, wetter, altered cracks
                +--> mechanics: softer, lower bearing strength, more deformation
                +--> acoustics: changed footsteps/impact response
                +--> ecology: changed water/nutrient availability
```

The existing procedural cracked-clay asset recipe is therefore useful, but its eventual input should be semantic clay state rather than an isolated visual preset.

---

# 4. Top-down Phenomenon contract

The engine should not own one global switch statement saying what the universe is at every scale.

Generation is decentralized:

```text
parent semantic scope
    |
    +-- one or more Phenomena attached to that scope
            |
            | refine / interpret child scope
            v
child semantic scope
    |
    +-- inherited/derived Metrics
    +-- continued Phenomena
    +-- newly-emergent Phenomena
    `-- localized semantic entities/events
```

A Phenomenon is responsible for its domain rule, not global loading.

A refinement evaluation needs at minimum:

```text
PhenomenonEvaluationContext
    spatial_scope
    spatial_scale
    temporal_scale
    epoch/time coordinate
    exact child address
    world/phenomenon seed
    parent state
    permitted parent-neighbor / boundary context
```

And can produce:

```text
PhenomenonRefinement
    metrics / fields
    child phenomenon state/seeds
    localized semantic entities
    semantic events
    realization-facing descriptors
```

It does **not** create render meshes, colliders, GPU buffers or decide global streaming.

---

# 5. Recursive / nested / self-hosted generation

“Recursive” must mean more than recursively calling the same noise function.

A Phenomenon may:

- refine **itself** into a finer representation,
- spawn another Phenomenon kind when a new domain becomes meaningful,
- remain dormant/pass-through across scales where no new state is needed,
- disappear after transferring all relevant state to children,
- coexist with unrelated Phenomena over the same child scope.

Example causal lineage:

```text
CosmologicalBackground
    -> CosmicMatterDistribution
        -> Halo
            -> Galaxy
                -> InterstellarMedium
                    -> MolecularCloud
                        -> StellarPopulation / StellarSystem
                            -> PlanetaryBody
                                -> Geology + Atmosphere + Hydrology
                                    -> Ecosystem / Biome
                                        -> Forest
                                            -> Tree population
                                                -> individual tree ...
```

This lineage is illustrative, not mandatory. Different regions can branch differently.

The key invariant is:

> Lower-scale state must have a causal semantic path back to higher-scale state, even when intermediate models are approximate.

---

# 6. Parent and neighbor coherence

A child may not independently “roll a world.”

Refinement may consume:

- direct parent state,
- parent Metrics,
- exact child address,
- deterministic boundary/corner values,
- permitted neighboring-parent state,
- shared constraint seeds,
- domain-specific continuity constraints.

Different domains may use different methods:

- spectral/random-field refinement,
- interpolation,
- constrained stochastic sampling,
- graph/topology generation,
- relaxation,
- PDE approximations,
- rule systems,
- WFC-like methods,
- bespoke physical/phenomenological models.

The framework supplies identity/context. It does not mandate one coherence algorithm.

---

# 7. Temporal scale contract

Spatial and temporal scale are orthogonal.

There is **not** one correct temporal scale for Scale `+18`, Scale `0`, etc. A galaxy can have orbital, star-formation and transient Phenomena with very different characteristic times.

Nevertheless every Phenomenon evaluation should explicitly receive a temporal scale.

For the first top-down implementation:

```text
TemporalScale
    = explicit model-resolution / parameterization label

Epoch
    = fixed present-day bootstrap snapshot
```

No broad time evolution is required yet.

The important future-facing rule is:

> A Phenomenon must know the temporal scale at which its model is being asked to operate.

The temporal scale can later determine:

- effective timestep,
- which processes are averaged out,
- which events are explicit,
- coefficients/closures,
- stochastic rates,
- integrator/model selection,
- cache/reaggregation cadence.

A coarse temporal model is therefore not merely the same function called with a bigger `dt`; it may be a different parameterization of the same domain.

Do **not** yet freeze the exact global temporal exponent range merely to satisfy this first spatial-worldgen proof.

---

# 8. Present-day bootstrap root state

The first executable universe does not need to simulate the Big Bang forward for 13.8 billion years.

We may bootstrap Scale `+35` with a deliberately explicit “present-day-ish” cosmological snapshot.

Useful approximate inputs include:

- epoch / scale factor (`a ≈ 1`),
- age of order `13.8 Gyr`,
- CMB temperature of order `2.725 K`,
- matter fraction of order `0.3`,
- dark-energy fraction of order `0.7`,
- baryon fraction of order `0.05`,
- a primordial/late-time fluctuation amplitude and spectral slope,
- one deterministic universe seed.

These values are **bootstrap parameters**, not claims that the engine is already a precision cosmology simulator.

The first CosmologicalBackground Phenomenon uses them to create statistically homogeneous large-scale state plus coherent perturbations that gain detail as refinement proceeds.

Later, a true temporal/cosmological initialization path may begin from an early epoch and evolve forward.

---

# 9. First executable +35 -> 0 proof

The first runtime proof should be intentionally narrow but semantically real.

## 9.1 Demand

Request one canonical branch/region containing the eventual player start location, down to Scale `0`.

The worldgen layer must recursively ensure required parent semantic state exists before refining the child.

## 9.2 Required traversal

The request must cross every scale:

```text
+35, +34, +33, ... +2, +1, 0
```

No “jump directly from cosmology to terrain” shortcut.

A scale is allowed to perform a pass-through/refinement of the same Phenomenon family, but that step remains explicit and inspectable.

## 9.3 First synthetic-but-domain-aware generators

We do not need a complete physical model at every row.

We do need scale-aware transitions such as:

```text
+35..+25  CosmologicalBackground spectral refinement
+24..+20  CosmicMatterDistribution / web / halo tendency
+19..+18  Halo -> Galaxy state
+17..+15  Galaxy -> ISM / cloud state
+14..+10  Stellar environment -> planetary-system state
 +9..+4   Planetary system/body -> planetary boundary conditions
 +3..+1   Planet -> geology/climate/hydrology
      0   local volumetric terrain + substrate/ecosystem seed
```

Within those bands every individual scale still receives and emits explicit state.

## 9.4 Scale-0 realization

Only after Scale `0` semantic state exists should the current voxel representation ask it for a local field.

The existing standalone `ProceduralTerrain` base should then become temporary compatibility scaffolding and eventually disappear as world authority.

At this point true 3D density generation, caves/overhangs and terrain material state belong here.

The voxel system remains a realization/cache:

```text
Scale-0 semantic geology/substrate state
        |
        v
voxel field realization
        |
        v
dense samples
        |
        +-- Surface Nets mesh
        `-- collider
```

## 9.5 Inspection

Developer tooling should let us inspect the ancestry:

```text
S0 region
 <- S+1 parent
 <- S+2 parent
 ...
 <- S+35 root
```

and see which Phenomena contributed state at each step.

That is the proof that a local valley/forest/cave did not originate from an unrelated S0 noise seed.

---

# 10. Biome framework acceptance

The first biome framework does not need hundreds of biome types.

It must prove the *mechanism*:

1. planetary/global state constrains climate,
2. regional geology/hydrology constrains local substrate/water,
3. climate + substrate + hydrology + disturbance feed ecological state,
4. ecological state produces an emergent biome characterization,
5. localized ecosystem Phenomena can overlap,
6. a forest can exist as state, not as a decorative scatter pass,
7. later lower-scale refinement can turn that forest into individual organisms.

A useful first world may still contain only a few strong ecological outcomes.

The important thing is that they arise from the same top-down chain.

---

# 11. Procedural assets remain downstream

Procedural assets are still important, but they consume semantic state.

```text
semantic world
    |
    +-- material/substrate state
    +-- tree/organism state
    +-- weather state
    +-- event state
    |
    v
procedural realization
    +-- textures/materials
    +-- meshes/models
    +-- animation
    +-- sound effects
    +-- ambience/music
```

The cracked-clay work should therefore survive, but evolve into:

```text
ClayMaterialState
    -> CrackedClayPresentationRecipe
    -> generated PBR textures/material
```

instead of making the visual recipe the source of material truth.

---

# 12. Anti-goals for the first implementation

Do not:

- eagerly allocate the full 36-level tree,
- globally generate every chunk at every scale,
- make noise the semantic authority,
- make voxels the semantic authority,
- make one universal `PhenomenonState` struct containing every possible field,
- force every scale to invent a unique object type,
- assign one temporal scale to one spatial scale,
- solve upward reaggregation now,
- simulate 13.8 billion years just to prove refinement,
- implement realistic galaxy/planet/ecology physics before the chain works,
- bypass intermediate scales because the first approximation happens to know the final answer.

---

# 13. Immediate implementation order

1. **Freeze this atlas as the semantic target.**
2. Add the minimal canonical `SpatialScope/ChunkAddress` operations required to address parent/child scale scopes.
3. Add explicit `TemporalScale` to Phenomenon evaluation context without implementing general time evolution.
4. Add a sparse Phenomenon/refinement store keyed by canonical scope + phenomenon identity.
5. Implement the Scale `+35` present-day bootstrap `CosmologicalBackground`.
6. Implement one demand-driven refinement path through **every scale** to `0`.
7. Make each scale emit inspectable state, even when the same Phenomenon family continues.
8. At domain boundaries, spawn/hand off to the next phenomenon family.
9. Connect Scale `0` geology/substrate output to voxel realization.
10. Only then deepen the local 3D terrain, biome, material and procedural-asset models.

This is the fastest route that still preserves the actual vision.
